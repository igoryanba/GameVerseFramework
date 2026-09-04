//! Bounded local protocol between the WebView host and the Rust bridge.
use anyhow::Result;
use gameverse_protocol::MAX_FRAME;
use serde::{de::DeserializeOwned, Deserialize, Serialize};
use serde_json::Value;
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

pub const VERSION: u16 = 1;
pub const DEFAULT_PIPE: &str = r"\\.\pipe\gameverse-ui-v1";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UiRequest {
    pub schema_version: u16,
    pub request_id: String,
    pub command: String,
    #[serde(default)]
    pub payload: Value,
}

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct UiResponse {
    pub schema_version: u16,
    pub request_id: String,
    pub ok: bool,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error_code: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub message: Option<String>,
    #[serde(default)]
    pub payload: Value,
}

impl UiRequest {
    pub fn valid(&self) -> bool {
        self.schema_version == VERSION
            && valid_id(&self.request_id)
            && matches!(
                self.command.as_str(),
                "ui.hello"
                    | "ui.ready"
                    | "auth.login"
                    | "auth.register"
                    | "auth.resume"
                    | "auth.logout"
                    | "characters.list"
                    | "characters.create"
                    | "characters.select"
                    | "chat.send"
                    | "inventory.request"
                    | "shop.catalog"
                    | "shop.buy"
                    | "job.start"
                    | "job.finish"
                    | "session.reconnect"
            )
            && self.payload.is_object()
    }
}

impl UiResponse {
    pub fn success(request_id: impl Into<String>, payload: Value) -> Self {
        Self {
            schema_version: VERSION,
            request_id: request_id.into(),
            ok: true,
            error_code: None,
            message: None,
            payload,
        }
    }

    pub fn error(request_id: impl Into<String>, code: &str, message: impl Into<String>) -> Self {
        Self {
            schema_version: VERSION,
            request_id: request_id.into(),
            ok: false,
            error_code: Some(code.into()),
            message: Some(message.into()),
            payload: serde_json::json!({}),
        }
    }

    pub fn valid(&self) -> bool {
        self.schema_version == VERSION
            && valid_id(&self.request_id)
            && self
                .error_code
                .as_ref()
                .is_none_or(|value| value.len() <= 64)
            && self.message.as_ref().is_none_or(|value| value.len() <= 512)
            && self.payload.is_object()
    }
}

fn valid_id(value: &str) -> bool {
    !value.is_empty()
        && value.len() <= 96
        && value
            .bytes()
            .all(|byte| byte.is_ascii_alphanumeric() || b"._:-".contains(&byte))
}

pub async fn read<T: DeserializeOwned>(reader: &mut (impl AsyncRead + Unpin)) -> Result<T> {
    let mut prefix = [0_u8; 4];
    reader.read_exact(&mut prefix).await?;
    let length = gameverse_protocol::frame_length(prefix)?;
    let mut body = vec![0; length];
    reader.read_exact(&mut body).await?;
    Ok(serde_json::from_slice(&body)?)
}

pub async fn write<T: Serialize>(writer: &mut (impl AsyncWrite + Unpin), value: &T) -> Result<()> {
    let body = serde_json::to_vec(value)?;
    anyhow::ensure!(
        !body.is_empty() && body.len() <= MAX_FRAME,
        "invalid UI frame length"
    );
    writer.write_all(&(body.len() as u32).to_be_bytes()).await?;
    writer.write_all(&body).await?;
    writer.flush().await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validates_allowlist_ids_and_payloads() {
        let request = UiRequest {
            schema_version: VERSION,
            request_id: "request-1".into(),
            command: "auth.login".into(),
            payload: serde_json::json!({"login":"user"}),
        };
        assert!(request.valid());
        assert!(!UiRequest {
            command: "process.start".into(),
            ..request.clone()
        }
        .valid());
        assert!(!UiRequest {
            request_id: "bad id".into(),
            ..request.clone()
        }
        .valid());
        assert!(!UiRequest {
            payload: Value::Null,
            ..request
        }
        .valid());
    }

    #[tokio::test]
    async fn length_prefix_roundtrip_and_oversize_rejection() {
        let response = UiResponse::success("request-1", serde_json::json!({"stage":"ready"}));
        let (mut writer, mut reader) = tokio::io::duplex(MAX_FRAME + 4);
        write(&mut writer, &response).await.unwrap();
        assert_eq!(read::<UiResponse>(&mut reader).await.unwrap(), response);
        let (mut writer, _) = tokio::io::duplex(MAX_FRAME + 4);
        assert!(write(
            &mut writer,
            &serde_json::json!({"value":"x".repeat(MAX_FRAME)})
        )
        .await
        .is_err());
    }
}
