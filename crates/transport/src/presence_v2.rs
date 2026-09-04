//! Unreliable, bounded Presence v2 movement channel.
//! Lifecycle and session configuration continue to use reliable QUIC streams.
use anyhow::Result;
use bytes::Bytes;
use gameverse_protocol::{
    control_v2::{decode_realtime, encode_realtime, RealtimeMessage},
    presence_v2::ServerFrame,
};

pub const MAX_DATAGRAM: usize = 16 * 1024;

pub fn send_frame(connection: &quinn::Connection, frame: &ServerFrame) -> Result<()> {
    send_realtime(
        connection,
        &RealtimeMessage::Server {
            frame: frame.clone(),
        },
    )
}

pub fn send_realtime(connection: &quinn::Connection, message: &RealtimeMessage) -> Result<()> {
    let bytes = encode_realtime(message)?;
    anyhow::ensure!(
        bytes.len() <= MAX_DATAGRAM,
        "Presence v2 datagram exceeds {MAX_DATAGRAM} bytes"
    );
    connection.send_datagram(Bytes::from(bytes))?;
    Ok(())
}

pub async fn read_frame(connection: &quinn::Connection) -> Result<ServerFrame> {
    match read_realtime(connection).await? {
        RealtimeMessage::Server { frame } => Ok(frame),
        _ => anyhow::bail!("expected server realtime frame"),
    }
}

pub async fn read_realtime(connection: &quinn::Connection) -> Result<RealtimeMessage> {
    let bytes = connection.read_datagram().await?;
    anyhow::ensure!(
        bytes.len() <= MAX_DATAGRAM,
        "Presence v2 datagram exceeds {MAX_DATAGRAM} bytes"
    );
    Ok(decode_realtime(&bytes)?)
}
