use anyhow::Result;
use clap::Parser;
use gameverse_transport::{generate_identity, server_endpoint};
use sha2::{Digest, Sha256};
use std::{net::SocketAddr, path::PathBuf, time::Duration};

#[derive(Parser)]
struct Args {
    #[arg(long, default_value = "0.0.0.0:30122")]
    bind: SocketAddr,
    #[arg(long)]
    cert: PathBuf,
    #[arg(long)]
    key: PathBuf,
    #[arg(long)]
    init_identity: bool,
    #[arg(long)]
    duration: Option<u64>,
    #[arg(long, env = "DATABASE_URL")]
    database_url: Option<String>,
    #[arg(long, default_value = "127.0.0.1:30123")]
    http_bind: SocketAddr,
    #[arg(long, env = "GAMEVERSE_ADMIN_BIND")]
    admin_bind: Option<String>,
    #[arg(long, env = "GAMEVERSE_ADMIN_TOKEN", hide_env_values = true)]
    admin_token: Option<String>,
    #[arg(long, env = "GAMEVERSE_ADMIN_ACTOR_ID")]
    admin_actor_id: Option<String>,
}
#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    if args.init_identity {
        return generate_identity(&args.cert, &args.key);
    }
    let store = if let Some(database_url) = args.database_url.as_deref() {
        let store = gameverse_rp::persistence::PostgresStore::connect(database_url, 16).await?;
        store.migrate().await?;
        Some(store)
    } else {
        None
    };
    let endpoint = server_endpoint(args.bind, &args.cert, &args.key)?;
    println!(
        "{}",
        serde_json::json!({"event":"m2_ready","address":endpoint.local_addr()?.to_string()})
    );
    let (tx, rx) = tokio::sync::watch::channel(false);
    let metrics = gameverse_server::presence_m2::MetricsHandle::default();
    let certificate_sha256 = format!("{:X}", Sha256::digest(std::fs::read(&args.cert)?));
    let directory = gameverse_server::health::DirectoryInfo {
        server_id: "local-alpha".into(),
        name: "GameVerse RP Alpha".into(),
        description: "Закрытая исследовательская RP-альфа".into(),
        mode: "Roleplay".into(),
        address: endpoint.local_addr()?.to_string(),
        max_players: 32,
        gta_build: gameverse_protocol::adapter::GAME_VERSION.into(),
        certificate_sha256,
    };
    let health = tokio::spawn(gameverse_server::health::serve(
        args.http_bind,
        metrics.clone(),
        directory,
        rx.clone(),
    ));
    let admin_bind = args
        .admin_bind
        .as_deref()
        .filter(|value| !value.is_empty())
        .map(str::parse)
        .transpose()?;
    let admin_token = args
        .admin_token
        .as_deref()
        .filter(|value| !value.is_empty());
    let admin_actor_id = args
        .admin_actor_id
        .as_deref()
        .filter(|value| !value.is_empty())
        .map(str::parse)
        .transpose()?;
    let admin = match (admin_bind, admin_token, admin_actor_id, store.as_ref()) {
        (Some(address), Some(token), Some(actor), Some(store)) => {
            let config = gameverse_server::admin::AdminConfig::new(address, token, actor)?;
            Some(tokio::spawn(gameverse_server::admin::serve(
                config,
                store.pool().clone(),
                metrics.clone(),
                rx.clone(),
            )))
        }
        (None, None, None, _) => None,
        _ => anyhow::bail!(
            "admin API requires database, bind address, token and actor account ID together"
        ),
    };
    tokio::spawn(async move {
        if let Some(seconds) = args.duration {
            tokio::select! { _=tokio::signal::ctrl_c()=>{}, _=tokio::time::sleep(Duration::from_secs(seconds))=>{} }
        } else {
            let _ = tokio::signal::ctrl_c().await;
        }
        let _ = tx.send(true);
    });
    let report = if let Some(store) = store {
        gameverse_server::presence_m2::run_alpha_with_metrics(endpoint, store, rx, metrics).await?
    } else {
        gameverse_server::presence_m2::run_with_metrics(endpoint, rx, metrics).await?
    };
    health.abort();
    if let Some(admin) = admin {
        admin.abort();
    }
    println!("{}", report);
    Ok(())
}
