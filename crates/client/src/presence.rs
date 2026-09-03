//! Presence v1 session. M0 Client and transport configuration remain unchanged.
use anyhow::Result;
use gameverse_protocol::{
    presence::{Message, PlayerState, Snapshot, VERSION},
    EntityId, SessionId,
};
use gameverse_transport::{
    client_endpoint,
    presence::{read_message, write_message},
    quinn, HANDSHAKE_TIMEOUT, IDLE_TIMEOUT,
};
use std::{net::SocketAddr, path::Path, time::Instant};
use tokio::{sync::watch, task::JoinHandle, time::timeout};
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
    pub async fn publish(&mut self, state: PlayerState) -> Result<()> {
        self.sequence += 1;
        timeout(
            IDLE_TIMEOUT,
            write_message(
                &mut self.send,
                &Message::PlayerState {
                    sequence: self.sequence,
                    state,
                },
            ),
        )
        .await??;
        Ok(())
    }
    pub async fn heartbeat(&mut self) -> Result<()> {
        timeout(
            HANDSHAKE_TIMEOUT,
            write_message(&mut self.send, &Message::Heartbeat),
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

/// One snapshot interval of visual delay. Never feeds interpolated poses upstream.
#[derive(Default)]
pub struct Replica {
    previous: Option<Snapshot>,
    current: Option<Snapshot>,
    received: Option<Instant>,
}
impl Replica {
    pub fn apply(&mut self, snapshot: Snapshot, now: Instant) -> bool {
        if !snapshot.valid()
            || self
                .current
                .as_ref()
                .is_some_and(|s| snapshot.tick <= s.tick)
        {
            return false;
        }
        self.previous = self.current.replace(snapshot);
        self.received = Some(now);
        true
    }
    pub fn render(&self, now: Instant) -> Option<Snapshot> {
        let mut out = self.current.clone()?;
        let alpha = now.saturating_duration_since(self.received?).as_secs_f32() / 0.1;
        let alpha = alpha.clamp(0.0, 1.0);
        if let Some(old) = &self.previous {
            for entity in &mut out.entities {
                if let Some(previous) = old
                    .entities
                    .iter()
                    .find(|p| p.id == entity.id && p.state.model_hash == entity.state.model_hash)
                {
                    let distance: f32 = entity
                        .state
                        .position
                        .iter()
                        .zip(previous.state.position)
                        .map(|(a, b)| (a - b).powi(2))
                        .sum();
                    // Teleports snap; ordinary movement interpolates. No unbounded extrapolation.
                    if distance <= 100.0 {
                        for i in 0..3 {
                            entity.state.position[i] = previous.state.position[i]
                                + (entity.state.position[i] - previous.state.position[i]) * alpha;
                        }
                        entity.state.rotation =
                            slerp(previous.state.rotation, entity.state.rotation, alpha);
                    }
                }
            }
        }
        Some(out)
    }
}
pub fn slerp(a: [f32; 4], mut b: [f32; 4], t: f32) -> [f32; 4] {
    let mut dot: f32 = a.iter().zip(b).map(|(a, b)| a * b).sum();
    if dot < 0.0 {
        b = b.map(|x| -x);
        dot = -dot;
    }
    let mut out = [0.0; 4];
    if dot > 0.9995 {
        for i in 0..4 {
            out[i] = a[i] + t * (b[i] - a[i]);
        }
    } else {
        let angle = dot.clamp(-1.0, 1.0).acos();
        let den = angle.sin();
        for i in 0..4 {
            out[i] = ((1.0 - t) * angle).sin() / den * a[i] + (t * angle).sin() / den * b[i];
        }
    }
    let norm = out.iter().map(|x| x * x).sum::<f32>().sqrt();
    out.map(|x| x / norm)
}

#[cfg(test)]
mod presence_tests {
    use super::*;
    use gameverse_protocol::presence::Entity;
    use std::time::Duration;
    #[test]
    fn visual_delay_shortest_rotation_and_despawn() {
        let id = EntityId {
            slot: 0,
            generation: 1,
        };
        let state = PlayerState {
            timestamp_ms: 0,
            position: [0.0; 3],
            rotation: [0.0, 0.0, 0.0, 1.0],
            velocity: [0.0; 3],
            model_hash: 1,
            health: 200,
            armor: 0,
            movement: 1,
            weapon_hash: 0,
        };
        let now = Instant::now();
        let mut replica = Replica::default();
        let first = Snapshot {
            tick: 1,
            ack: 0,
            entities: vec![Entity { id, state }],
        };
        replica.apply(first.clone(), now);
        let mut next = first;
        next.tick = 2;
        next.entities[0].state.position[2] = 4.0;
        replica.apply(next, now);
        let rendered = replica.render(now + Duration::from_millis(50)).unwrap();
        assert_eq!(rendered.entities[0].state.position[2], 2.0);
        assert_eq!(
            slerp([0.0, 0.0, 0.0, 1.0], [0.0, 0.0, 0.0, -1.0], 0.5),
            [0.0, 0.0, 0.0, 1.0]
        );
        replica.apply(
            Snapshot {
                tick: 3,
                ack: 0,
                entities: vec![],
            },
            now,
        );
        assert!(replica.render(now).unwrap().entities.is_empty());
    }
}
