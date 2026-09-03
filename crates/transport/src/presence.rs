//! The same M0 QUIC transport and framing, carrying presence v1 messages.
use anyhow::Result;
use gameverse_protocol::{
    frame_length,
    presence::{decode, encode, Message},
};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};
pub async fn read_message<R: AsyncRead + Unpin>(reader: &mut R) -> Result<Message> {
    let mut prefix = [0; 4];
    reader.read_exact(&mut prefix).await?;
    let mut bytes = vec![0; frame_length(prefix)?];
    reader.read_exact(&mut bytes).await?;
    Ok(decode(&bytes)?)
}
pub async fn write_message<W: AsyncWrite + Unpin>(writer: &mut W, message: &Message) -> Result<()> {
    writer.write_all(&encode(message)?).await?;
    writer.flush().await?;
    Ok(())
}
