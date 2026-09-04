use crate::presence_m2::MetricsHandle;
use anyhow::{Context, Result};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use sqlx::PgPool;
use std::net::SocketAddr;
use subtle::ConstantTimeEq;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::watch,
    time::{timeout, Duration},
};

const MAX_REQUEST_BYTES: usize = 16 * 1024;

#[derive(Clone)]
pub struct AdminConfig {
    pub address: SocketAddr,
    pub token_hash: [u8; 32],
    pub actor_account_id: i64,
}

impl AdminConfig {
    pub fn new(address: SocketAddr, token: &str, actor_account_id: i64) -> Result<Self> {
        anyhow::ensure!(
            token.len() >= 32 && token.len() <= 512,
            "admin token must contain 32-512 characters"
        );
        anyhow::ensure!(actor_account_id > 0, "admin actor account ID is invalid");
        Ok(Self {
            address,
            token_hash: Sha256::digest(token.as_bytes()).into(),
            actor_account_id,
        })
    }
}

pub async fn serve(
    config: AdminConfig,
    pool: PgPool,
    metrics: MetricsHandle,
    mut shutdown: watch::Receiver<bool>,
) -> Result<()> {
    let listener = TcpListener::bind(config.address).await?;
    println!(
        "{}",
        json!({"event":"admin_ready","address":listener.local_addr()?.to_string()})
    );
    loop {
        tokio::select! {
            changed = shutdown.changed() => if changed.is_err() || *shutdown.borrow() { return Ok(()); },
            accepted = listener.accept() => {
                let (stream, _) = accepted?;
                let config = config.clone();
                let pool = pool.clone();
                let metrics = metrics.clone();
                tokio::spawn(async move { let _ = handle(stream, config, pool, metrics).await; });
            }
        }
    }
}

struct Request {
    method: String,
    path: String,
    authorization: Option<String>,
    body: Vec<u8>,
}

async fn handle(
    mut stream: TcpStream,
    config: AdminConfig,
    pool: PgPool,
    metrics: MetricsHandle,
) -> Result<()> {
    let request = match read_request(&mut stream).await {
        Ok(value) => value,
        Err(error) => {
            return write_response(
                &mut stream,
                "400 Bad Request",
                json!({"error":"bad_request","message":error.to_string()}),
            )
            .await
        }
    };
    let authorized = request
        .authorization
        .as_deref()
        .and_then(|value| value.strip_prefix("Bearer "))
        .map(|token| {
            let supplied: [u8; 32] = Sha256::digest(token.as_bytes()).into();
            bool::from(supplied.ct_eq(&config.token_hash))
        })
        .unwrap_or(false);
    if !authorized {
        return write_response(
            &mut stream,
            "401 Unauthorized",
            json!({"error":"unauthorized"}),
        )
        .await;
    }
    let role: Option<String> = sqlx::query_scalar("SELECT role FROM accounts WHERE id=$1 AND NOT EXISTS(SELECT 1 FROM bans WHERE account_id=$1 AND revoked_at IS NULL AND (expires_at IS NULL OR expires_at>now()))")
        .bind(config.actor_account_id).fetch_optional(&pool).await?;
    if !matches!(role.as_deref(), Some("moderator" | "administrator")) {
        return write_response(&mut stream, "403 Forbidden", json!({"error":"forbidden"})).await;
    }
    let response = dispatch(
        &request,
        config.actor_account_id,
        role.as_deref().unwrap_or_default(),
        &pool,
        &metrics,
    )
    .await;
    match response {
        Ok(value) => write_response(&mut stream, "200 OK", value).await,
        Err(AdminError::NotFound) => {
            write_response(&mut stream, "404 Not Found", json!({"error":"not_found"})).await
        }
        Err(AdminError::Forbidden) => {
            write_response(&mut stream, "403 Forbidden", json!({"error":"forbidden"})).await
        }
        Err(AdminError::Invalid(message)) => {
            write_response(
                &mut stream,
                "400 Bad Request",
                json!({"error":"invalid_request","message":message}),
            )
            .await
        }
        Err(AdminError::Database(error)) => Err(error),
    }
}

enum AdminError {
    NotFound,
    Forbidden,
    Invalid(String),
    Database(anyhow::Error),
}
type AuditRow = (i64, String, Option<String>, Option<String>, String);
impl From<sqlx::Error> for AdminError {
    fn from(value: sqlx::Error) -> Self {
        Self::Database(value.into())
    }
}

async fn dispatch(
    request: &Request,
    actor: i64,
    role: &str,
    pool: &PgPool,
    metrics: &MetricsHandle,
) -> std::result::Result<Value, AdminError> {
    if request.method == "GET" && request.path == "/v1/admin/sessions" {
        let snapshot = metrics.snapshot();
        return Ok(
            json!({"active":snapshot.players,"accepted_total":snapshot.accepted_sessions,"disconnects_total":snapshot.disconnects}),
        );
    }
    if request.method == "GET" && request.path == "/v1/admin/audit" {
        let rows: Vec<AuditRow> = sqlx::query_as("SELECT id,action,target_type,target_id,created_at::text FROM audit_events ORDER BY id DESC LIMIT 100").fetch_all(pool).await?;
        return Ok(
            json!({"events":rows.into_iter().map(|row| json!({"id":row.0,"action":row.1,"target_type":row.2,"target_id":row.3,"created_at":row.4})).collect::<Vec<_>>() }),
        );
    }
    let Some((account_text, action)) = account_action(&request.path) else {
        return Err(AdminError::NotFound);
    };
    if request.method != "POST" {
        return Err(AdminError::NotFound);
    }
    let account_id: i64 = account_text
        .parse()
        .map_err(|_| AdminError::Invalid("invalid account ID".into()))?;
    if account_id <= 0 || account_id == actor {
        return Err(AdminError::Invalid("invalid moderation target".into()));
    }
    let body: Value = serde_json::from_slice(&request.body)
        .map_err(|_| AdminError::Invalid("body must be JSON".into()))?;
    let mut transaction = pool.begin().await?;
    let target_role: Option<String> =
        sqlx::query_scalar("SELECT role FROM accounts WHERE id=$1 FOR UPDATE")
            .bind(account_id)
            .fetch_optional(&mut *transaction)
            .await?;
    let Some(target_role) = target_role else {
        return Err(AdminError::NotFound);
    };
    if target_role == "administrator" || (target_role == "moderator" && role != "administrator") {
        return Err(AdminError::Forbidden);
    }
    match action {
        "ban" => {
            let reason = body
                .get("reason")
                .and_then(Value::as_str)
                .unwrap_or("")
                .trim();
            if reason.is_empty() || reason.len() > 512 {
                return Err(AdminError::Invalid(
                    "ban reason must contain 1-512 characters".into(),
                ));
            }
            sqlx::query("INSERT INTO bans(account_id,reason,created_by) VALUES($1,$2,$3)")
                .bind(account_id)
                .bind(reason)
                .bind(actor)
                .execute(&mut *transaction)
                .await?;
            sqlx::query(
                "UPDATE sessions SET revoked_at=now() WHERE account_id=$1 AND revoked_at IS NULL",
            )
            .bind(account_id)
            .execute(&mut *transaction)
            .await?;
        }
        "unban" => {
            if role != "administrator" {
                return Err(AdminError::Forbidden);
            }
            let result = sqlx::query(
                "UPDATE bans SET revoked_at=now() WHERE account_id=$1 AND revoked_at IS NULL",
            )
            .bind(account_id)
            .execute(&mut *transaction)
            .await?;
            if result.rows_affected() == 0 {
                return Err(AdminError::NotFound);
            }
        }
        _ => return Err(AdminError::NotFound),
    }
    sqlx::query("INSERT INTO audit_events(actor_account_id,action,target_type,target_id,details) VALUES($1,$2,'account',$3,$4)")
        .bind(actor).bind(action).bind(account_id.to_string()).bind(body).execute(&mut *transaction).await?;
    transaction.commit().await?;
    Ok(json!({"status":"ok","action":action,"account_id":account_id}))
}

fn account_action(path: &str) -> Option<(&str, &str)> {
    let value = path.strip_prefix("/v1/admin/accounts/")?;
    let (account, action) = value.split_once('/')?;
    if account.contains('/') || !matches!(action, "ban" | "unban") {
        return None;
    }
    Some((account, action))
}

async fn read_request(stream: &mut TcpStream) -> Result<Request> {
    let mut bytes = Vec::with_capacity(2048);
    let header_end;
    loop {
        if bytes.len() >= MAX_REQUEST_BYTES {
            anyhow::bail!("request exceeds 16 KiB");
        }
        let mut chunk = [0_u8; 2048];
        let count = timeout(Duration::from_secs(2), stream.read(&mut chunk)).await??;
        anyhow::ensure!(count > 0, "request ended before headers");
        bytes.extend_from_slice(&chunk[..count]);
        if let Some(position) = bytes.windows(4).position(|value| value == b"\r\n\r\n") {
            header_end = position + 4;
            break;
        }
    }
    let headers =
        std::str::from_utf8(&bytes[..header_end]).context("request headers are not UTF-8")?;
    let mut lines = headers.split("\r\n");
    let mut first = lines.next().unwrap_or_default().split_whitespace();
    let method = first.next().unwrap_or_default().to_string();
    let path = first.next().unwrap_or_default().to_string();
    anyhow::ensure!(
        first.next() == Some("HTTP/1.1") && first.next().is_none(),
        "invalid request line"
    );
    let mut authorization = None;
    let mut content_length = 0_usize;
    for line in lines.filter(|line| !line.is_empty()) {
        let (name, value) = line.split_once(':').context("invalid header")?;
        match name.trim().to_ascii_lowercase().as_str() {
            "authorization" => authorization = Some(value.trim().to_string()),
            "content-length" => {
                content_length = value.trim().parse().context("invalid content length")?
            }
            _ => {}
        }
    }
    anyhow::ensure!(
        content_length <= MAX_REQUEST_BYTES - header_end,
        "request body exceeds limit"
    );
    while bytes.len() < header_end + content_length {
        let mut chunk = [0_u8; 2048];
        let count = timeout(Duration::from_secs(2), stream.read(&mut chunk)).await??;
        anyhow::ensure!(count > 0, "request body is truncated");
        bytes.extend_from_slice(&chunk[..count]);
        anyhow::ensure!(bytes.len() <= MAX_REQUEST_BYTES, "request exceeds 16 KiB");
    }
    Ok(Request {
        method,
        path,
        authorization,
        body: bytes[header_end..header_end + content_length].to_vec(),
    })
}

async fn write_response(stream: &mut TcpStream, status: &str, body: Value) -> Result<()> {
    let body = body.to_string();
    let reply = format!("HTTP/1.1 {status}\r\nContent-Type: application/json\r\nContent-Length: {}\r\nConnection: close\r\nX-Content-Type-Options: nosniff\r\nCache-Control: no-store\r\nReferrer-Policy: no-referrer\r\n\r\n{body}", body.len());
    timeout(Duration::from_secs(2), stream.write_all(reply.as_bytes())).await??;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn moderation_paths_are_exact() {
        assert_eq!(
            account_action("/v1/admin/accounts/42/ban"),
            Some(("42", "ban"))
        );
        assert_eq!(
            account_action("/v1/admin/accounts/42/unban"),
            Some(("42", "unban"))
        );
        assert_eq!(account_action("/v1/admin/accounts/42/kick"), None);
    }
    #[test]
    fn tokens_require_sufficient_entropy_length() {
        assert!(AdminConfig::new("127.0.0.1:1".parse().unwrap(), "short", 1).is_err());
        assert!(AdminConfig::new("127.0.0.1:1".parse().unwrap(), &"a".repeat(32), 1).is_ok());
    }
}
