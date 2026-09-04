//! Unreliable, bounded Presence v2 movement channel.
//! Lifecycle and session configuration continue to use reliable QUIC streams.
use anyhow::Result;
use bytes::Bytes;
use gameverse_protocol::presence_v2::{decode_frame, encode_frame, ServerFrame};

pub const MAX_DATAGRAM: usize = 16 * 1024;

pub fn send_frame(connection: &quinn::Connection, frame: &ServerFrame) -> Result<()> {
    let bytes = encode_frame(frame)?;
    anyhow::ensure!(
        bytes.len() <= MAX_DATAGRAM,
        "Presence v2 datagram exceeds {MAX_DATAGRAM} bytes"
    );
    connection.send_datagram(Bytes::from(bytes))?;
    Ok(())
}

pub async fn read_frame(connection: &quinn::Connection) -> Result<ServerFrame> {
    let bytes = connection.read_datagram().await?;
    anyhow::ensure!(
        bytes.len() <= MAX_DATAGRAM,
        "Presence v2 datagram exceeds {MAX_DATAGRAM} bytes"
    );
    Ok(decode_frame(&bytes)?)
}
