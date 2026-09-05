//! Bounded native bootstrap protocol. It never carries memory addresses.
use crate::{Error, MAX_FRAME};
use serde::{Deserialize, Serialize};

pub const VERSION: u16 = 1;
pub const DEFAULT_PIPE: &str = r"\\.\pipe\gameverse-bootstrap-v1";

#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
#[serde(tag = "type", rename_all = "snake_case", deny_unknown_fields)]
pub enum Message {
    BootstrapHello {
        schema_version: u16,
        bootstrap_build: String,
        gta_edition: String,
        gta_build: String,
        fingerprint: String,
        capabilities: Vec<String>,
    },
    BootstrapStage {
        schema_version: u16,
        monotonic_ms: u64,
        stage: Stage,
    },
    BootstrapFailure {
        schema_version: u16,
        code: String,
        message: String,
    },
    BootstrapCommand {
        schema_version: u16,
        command: Command,
    },
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Stage {
    Loaded,
    Verified,
    FrontendReady,
    WorldRequested,
    WorldReady,
    AdapterReady,
    Failed,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Command {
    BeginWorld,
    Abort,
    Shutdown,
}

impl Message {
    pub fn valid(&self) -> bool {
        match self {
            Self::BootstrapHello {
                schema_version,
                bootstrap_build,
                gta_edition,
                gta_build,
                fingerprint,
                capabilities,
            } => {
                *schema_version == VERSION
                    && !bootstrap_build.is_empty()
                    && bootstrap_build.len() <= 64
                    && gta_edition == "enhanced"
                    && gta_build.len() <= 64
                    && fingerprint.len() == 64
                    && fingerprint.bytes().all(|b| b.is_ascii_hexdigit())
                    && capabilities.len() <= 16
                    && capabilities.iter().all(|v| !v.is_empty() && v.len() <= 64)
            }
            Self::BootstrapStage { schema_version, .. }
            | Self::BootstrapFailure { schema_version, .. }
            | Self::BootstrapCommand { schema_version, .. } => *schema_version == VERSION,
        }
        &&match self {
            Self::BootstrapFailure { code, message, .. } => {
                !code.is_empty() && code.len() <= 64 && !message.is_empty() && message.len() <= 512
            }
            _ => true,
        }
    }
}

pub fn encode(message: &Message) -> Result<Vec<u8>, Error> {
    if !message.valid() {
        return Err(Error::Values);
    }
    let body = serde_json::to_vec(message)?;
    if body.is_empty() || body.len() > MAX_FRAME {
        return Err(Error::Length);
    }
    let mut frame = (body.len() as u32).to_be_bytes().to_vec();
    frame.extend(body);
    Ok(frame)
}

pub fn decode(body: &[u8]) -> Result<Message, Error> {
    if body.is_empty() || body.len() > MAX_FRAME {
        return Err(Error::Length);
    }
    let message: Message = serde_json::from_slice(body)?;
    if !message.valid() {
        return Err(Error::Values);
    }
    Ok(message)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn rejects_address_leaks_and_unknown_fields() {
        let valid =
            br#"{"type":"bootstrap_stage","schema_version":1,"monotonic_ms":10,"stage":"loaded"}"#;
        assert!(decode(valid).unwrap().valid());
        assert!(decode(br#"{"type":"bootstrap_failure","schema_version":1,"code":"bad","message":"safe","address":"0x1234"}"#).is_err());
    }
}
