//! Bounded native bootstrap protocol. Normal pipe telemetry never carries raw
//! memory, absolute addresses, credentials, or user data.
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
    TelemetryHelloV1 {
        schema_version: u16,
        probe_build: String,
        gta_build: String,
        fingerprint: String,
        capabilities: Vec<String>,
    },
    TelemetrySnapshotV1 {
        schema_version: u16,
        snapshot: TelemetrySnapshotV1,
    },
    TelemetryCandidatesV1 {
        schema_version: u16,
        candidates: Vec<TelemetryCandidateV1>,
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
    StartTelemetry,
    MarkStage,
    FinishTelemetry,
    BeginWorld,
    Abort,
    Shutdown,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TelemetryModuleV1 {
    pub name: String,
    pub image_size: u64,
    pub file_version: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TelemetrySectionV1 {
    pub name: String,
    pub virtual_size: u64,
    pub characteristics: u32,
    pub committed_pages: u32,
    pub executable_pages: u32,
    pub readonly_pages: u32,
    pub changed_pages: u32,
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub changed_page_rvas: Vec<u32>,
    pub aggregate_sha256: String,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TelemetryReadinessV1 {
    pub window_visible: bool,
    pub window_responsive: bool,
    pub scripthook_loaded: bool,
    pub shvdn_loaded: bool,
    pub adapter_loaded: bool,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TelemetrySnapshotV1 {
    pub monotonic_ms: u64,
    pub stage: String,
    pub modules: Vec<TelemetryModuleV1>,
    pub sections: Vec<TelemetrySectionV1>,
    pub readiness: TelemetryReadinessV1,
}

/// Candidate metadata is written only to the ignored local research report.
/// `rva` is image-relative and must never be forwarded to UI or normal logs.
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TelemetryCandidateV1 {
    pub candidate_id: String,
    pub rva: u32,
    pub section: String,
    pub unique_match_count: u32,
    pub call_count: u64,
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TelemetryReportV1 {
    pub schema_version: u16,
    pub run_fingerprint: String,
    pub snapshots: Vec<TelemetrySnapshotV1>,
    pub candidates: Vec<TelemetryCandidateV1>,
    pub errors: Vec<String>,
    pub classification: String,
}

impl Message {
    pub fn valid(&self) -> bool {
        let envelope_valid = match self {
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
            | Self::BootstrapCommand { schema_version, .. }
            | Self::TelemetrySnapshotV1 { schema_version, .. }
            | Self::TelemetryCandidatesV1 { schema_version, .. } => *schema_version == VERSION,
            Self::TelemetryHelloV1 {
                schema_version,
                probe_build,
                gta_build,
                fingerprint,
                capabilities,
            } => {
                *schema_version == VERSION
                    && !probe_build.is_empty()
                    && probe_build.len() <= 64
                    && !gta_build.is_empty()
                    && gta_build.len() <= 64
                    && fingerprint.len() == 64
                    && fingerprint.bytes().all(|b| b.is_ascii_hexdigit())
                    && capabilities.len() <= 16
                    && capabilities.iter().all(|v| !v.is_empty() && v.len() <= 64)
            }
        };
        let payload_valid = match self {
            Self::BootstrapFailure { code, message, .. } => {
                !code.is_empty() && code.len() <= 64 && !message.is_empty() && message.len() <= 512
            }
            Self::TelemetrySnapshotV1 { snapshot, .. } => snapshot.valid(),
            Self::TelemetryCandidatesV1 { candidates, .. } => {
                candidates.len() <= 16
                    && candidates.iter().all(|candidate| {
                        !candidate.candidate_id.is_empty()
                            && candidate.candidate_id.len() <= 64
                            && !candidate.section.is_empty()
                            && candidate.section.len() <= 16
                            && candidate.unique_match_count <= 1024
                    })
            }
            _ => true,
        };
        envelope_valid && payload_valid
    }
}

impl TelemetrySnapshotV1 {
    fn valid(&self) -> bool {
        !self.stage.is_empty()
            && self.stage.len() <= 64
            && self.modules.len() <= 256
            && self.sections.len() <= 96
            && self.modules.iter().all(|v| {
                !v.name.is_empty()
                    && v.name.len() <= 260
                    && v.file_version.len() <= 64
                    && v.image_size <= 4 * 1024 * 1024 * 1024
            })
            && self.sections.iter().all(|v| {
                !v.name.is_empty()
                    && v.name.len() <= 16
                    && v.virtual_size <= 4 * 1024 * 1024 * 1024
                    && v.changed_page_rvas.len() <= 256
                    && v.changed_page_rvas.windows(2).all(|pair| pair[0] < pair[1])
                    && v.aggregate_sha256.len() == 64
                    && v.aggregate_sha256.bytes().all(|b| b.is_ascii_hexdigit())
            })
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

    #[test]
    fn bounds_telemetry_and_roundtrips_commands() {
        let snapshot = Message::TelemetrySnapshotV1 {
            schema_version: VERSION,
            snapshot: TelemetrySnapshotV1 {
                monotonic_ms: 42,
                stage: "image_verified".into(),
                modules: vec![TelemetryModuleV1 {
                    name: "GTA5_Enhanced.exe".into(),
                    image_size: 56_064_632,
                    file_version: "1.0.1158.13".into(),
                }],
                sections: vec![TelemetrySectionV1 {
                    name: ".text".into(),
                    virtual_size: 1024,
                    characteristics: 0x6000_0020,
                    committed_pages: 1,
                    executable_pages: 1,
                    readonly_pages: 1,
                    changed_pages: 0,
                    changed_page_rvas: vec![],
                    aggregate_sha256: "00".repeat(32),
                }],
                readiness: TelemetryReadinessV1 {
                    window_visible: false,
                    window_responsive: true,
                    scripthook_loaded: false,
                    shvdn_loaded: false,
                    adapter_loaded: false,
                },
            },
        };
        let frame = encode(&snapshot).unwrap();
        assert_eq!(decode(&frame[4..]).unwrap(), snapshot);
        let command = Message::BootstrapCommand {
            schema_version: VERSION,
            command: Command::StartTelemetry,
        };
        assert_eq!(decode(&encode(&command).unwrap()[4..]).unwrap(), command);

        let candidates = Message::TelemetryCandidatesV1 {
            schema_version: VERSION,
            candidates: vec![TelemetryCandidateV1 {
                candidate_id: "transition_ref_a".into(),
                rva: 0x11c_52f0,
                section: ".text".into(),
                unique_match_count: 1,
                call_count: 0,
            }],
        };
        assert_eq!(
            decode(&encode(&candidates).unwrap()[4..]).unwrap(),
            candidates
        );
    }
}
