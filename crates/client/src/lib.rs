//! Standalone client session and replaceable game adapter; no game-process access.
pub mod ipc;
#[cfg(windows)]
pub mod ipc_m2;
pub mod m2;
pub mod presence;
pub mod ui;
use anyhow::Result;
use gameverse_protocol::{EntityId, Message, SessionId, Snapshot, VERSION};
use gameverse_transport::{
    client_endpoint, quinn, read_message, write_message, HANDSHAKE_TIMEOUT, IDLE_TIMEOUT,
};
use std::{net::SocketAddr, path::Path};
use tokio::{sync::watch, task::JoinHandle, time::timeout};

/// The adapter owns presentation, never authoritative state or network sessions.
pub trait GameAdapter {
    fn input(&mut self, elapsed_seconds: f32) -> [f32; 2];
    fn apply_snapshot(&mut self, snapshot: &Snapshot);
    fn shutdown(&mut self);
}

pub struct MemoryAdapter {
    pub direction: [f32; 2],
    pub move_seconds: f32,
    pub state: Option<Snapshot>,
    pub stopped: bool,
}
impl GameAdapter for MemoryAdapter {
    fn input(&mut self, elapsed_seconds: f32) -> [f32; 2] {
        if elapsed_seconds < self.move_seconds {
            self.direction
        } else {
            [0.0; 2]
        }
    }
    fn apply_snapshot(&mut self, snapshot: &Snapshot) {
        self.state = Some(snapshot.clone());
    }
    fn shutdown(&mut self) {
        self.stopped = true;
    }
}

#[derive(Default)]
pub struct Replica {
    previous: Option<Snapshot>,
    current: Option<Snapshot>,
}
impl Replica {
    /// Full snapshots supersede old state, including entities that disappeared.
    pub fn apply(&mut self, state: Snapshot) -> bool {
        if self.current.as_ref().is_some_and(|s| state.tick <= s.tick) {
            return false;
        }
        self.previous = self.current.replace(state);
        true
    }
    pub fn authoritative(&self) -> Option<&Snapshot> {
        self.current.as_ref()
    }
    /// Render interpolation never writes back into authoritative state.
    pub fn interpolated(&self, alpha: f32) -> Option<Snapshot> {
        let mut rendered = self.current.clone()?;
        let alpha = if alpha.is_finite() {
            alpha.clamp(0.0, 1.0)
        } else {
            1.0
        };
        if let Some(previous) = &self.previous {
            for entity in &mut rendered.entities {
                if let Some(old) = previous.entities.iter().find(|e| e.id == entity.id) {
                    for axis in 0..2 {
                        entity.position[axis] = old.position[axis]
                            + (entity.position[axis] - old.position[axis]) * alpha;
                    }
                }
            }
        }
        Some(rendered)
    }
}

pub struct Client {
    pub session: SessionId,
    pub entity: EntityId,
    pub snapshots: watch::Receiver<Snapshot>,
    endpoint: quinn::Endpoint,
    connection: quinn::Connection,
    send: quinn::SendStream,
    reader: JoinHandle<Result<()>>,
    sequence: u64,
}
impl Client {
    pub async fn connect(address: SocketAddr, cert: &Path) -> Result<Self> {
        Self::connect_version(address, cert, VERSION).await
    }
    pub async fn connect_version(address: SocketAddr, cert: &Path, version: u16) -> Result<Self> {
        anyhow::ensure!(
            address.ip().is_loopback(),
            "M0 clients connect to loopback only"
        );
        let endpoint = client_endpoint(cert)?;
        let (connection, send, mut recv, session, entity, initial) =
            timeout(HANDSHAKE_TIMEOUT, async {
                let connection = endpoint.connect(address, "localhost")?.await?;
                let (mut send, mut recv) = connection.open_bi().await?;
                write_message(&mut send, &Message::Hello { version }).await?;
                let (session, entity) = match read_message(&mut recv).await? {
                    Message::Welcome {
                        version: VERSION,
                        session,
                        entity,
                    } => (session, entity),
                    Message::Reject { reason } => anyhow::bail!("server rejected: {reason}"),
                    _ => anyhow::bail!("invalid welcome"),
                };
                let initial = match read_message(&mut recv).await? {
                    Message::Snapshot { state } => state,
                    _ => anyhow::bail!("missing initial snapshot"),
                };
                Ok::<_, anyhow::Error>((connection, send, recv, session, entity, initial))
            })
            .await??;
        let (tx, snapshots) = watch::channel(initial);
        let reader = tokio::spawn(async move {
            loop {
                match timeout(IDLE_TIMEOUT, read_message(&mut recv)).await?? {
                    Message::Snapshot { state } => {
                        tx.send_replace(state);
                    }
                    _ => anyhow::bail!("unexpected server message"),
                }
            }
        });
        Ok(Self {
            session,
            entity,
            snapshots,
            endpoint,
            connection,
            send,
            reader,
            sequence: 0,
        })
    }
    pub async fn input(&mut self, direction: [f32; 2]) -> Result<()> {
        self.sequence += 1;
        timeout(
            IDLE_TIMEOUT,
            write_message(
                &mut self.send,
                &Message::Input {
                    sequence: self.sequence,
                    direction,
                },
            ),
        )
        .await??;
        Ok(())
    }
    pub async fn close(&mut self) -> Result<()> {
        let result = timeout(HANDSHAKE_TIMEOUT, async {
            write_message(
                &mut self.send,
                &Message::Disconnect {
                    reason: "client shutdown".into(),
                },
            )
            .await?;
            self.send.finish().await?;
            Ok::<_, anyhow::Error>(())
        })
        .await;
        self.connection.close(0_u32.into(), b"client shutdown");
        self.reader.abort();
        self.endpoint.wait_idle().await;
        result??;
        Ok(())
    }
}
impl Drop for Client {
    fn drop(&mut self) {
        self.reader.abort();
        self.connection.close(0_u32.into(), b"client dropped");
        self.endpoint.close(0_u32.into(), b"client dropped");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gameverse_protocol::{Entity, Snapshot};
    fn snapshot(tick: u64, x: f32) -> Snapshot {
        Snapshot {
            tick,
            ack: tick,
            entities: vec![Entity {
                id: EntityId {
                    slot: 0,
                    generation: 1,
                },
                position: [x, 0.0],
            }],
        }
    }
    #[test]
    fn loss_reordering_duplicate_despawn_and_interpolation() {
        let mut replica = Replica::default();
        assert!(replica.apply(snapshot(1, 1.0)));
        assert!(replica.apply(snapshot(4, 4.0))); // snapshots 2 and 3 lost
        assert!(!replica.apply(snapshot(2, 2.0)));
        assert!(!replica.apply(snapshot(4, 99.0)));
        assert_eq!(
            replica.interpolated(0.5).unwrap().entities[0].position[0],
            2.5
        );
        assert_eq!(
            replica.authoritative().unwrap().entities[0].position[0],
            4.0
        );
        replica.apply(Snapshot {
            tick: 5,
            ack: 5,
            entities: vec![],
        });
        assert!(replica.authoritative().unwrap().entities.is_empty());
    }
}
