//! QUIC endpoints and bounded framing. Certificate trust is always explicit.
pub mod presence;
use anyhow::Result;
use gameverse_protocol::{decode, encode, frame_length, Message};
pub use quinn;
use std::{net::SocketAddr, path::Path, sync::Arc, time::Duration};
use tokio::io::{AsyncRead, AsyncReadExt, AsyncWrite, AsyncWriteExt};

pub const HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(5);
pub const IDLE_TIMEOUT: Duration = Duration::from_secs(15);
pub const INPUT_CAPACITY: usize = 128;

pub async fn read_message<R: AsyncRead + Unpin>(reader: &mut R) -> Result<Message> {
    let mut prefix = [0; 4];
    reader.read_exact(&mut prefix).await?;
    let length = frame_length(prefix)?;
    let mut body = vec![0; length];
    reader.read_exact(&mut body).await?;
    Ok(decode(&body)?)
}

pub async fn write_message<W: AsyncWrite + Unpin>(writer: &mut W, message: &Message) -> Result<()> {
    writer.write_all(&encode(message)?).await?;
    writer.flush().await?;
    Ok(())
}

fn transport_config() -> Arc<quinn::TransportConfig> {
    let mut config = quinn::TransportConfig::default();
    config.max_idle_timeout(Some(IDLE_TIMEOUT.try_into().expect("valid idle timeout")));
    config.max_concurrent_bidi_streams(1_u32.into());
    config.max_concurrent_uni_streams(0_u32.into());
    config.receive_window(256_000_u32.into());
    config.stream_receive_window(128_000_u32.into());
    config.send_window(128_000);
    Arc::new(config)
}

pub fn generate_identity(cert_path: &Path, key_path: &Path) -> Result<()> {
    use std::io::Write;
    // Never silently replace an identity clients already trust.
    anyhow::ensure!(
        !cert_path.exists() && !key_path.exists(),
        "identity already exists"
    );
    let cert = rcgen::generate_simple_self_signed(vec!["localhost".into()])?;
    let mut key = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(key_path)?;
    key.write_all(&cert.serialize_private_key_der())?;
    let mut output = std::fs::OpenOptions::new()
        .write(true)
        .create_new(true)
        .open(cert_path)?;
    output.write_all(&cert.serialize_der()?)?;
    Ok(())
}

pub fn server_endpoint(
    address: SocketAddr,
    cert_path: &Path,
    key_path: &Path,
) -> Result<quinn::Endpoint> {
    let mut config = quinn::ServerConfig::with_single_cert(
        vec![rustls::Certificate(std::fs::read(cert_path)?)],
        rustls::PrivateKey(std::fs::read(key_path)?),
    )?;
    config.transport = transport_config();
    Ok(quinn::Endpoint::server(config, address)?)
}

pub fn client_endpoint(cert_path: &Path) -> Result<quinn::Endpoint> {
    let mut roots = rustls::RootCertStore::empty();
    roots.add(&rustls::Certificate(std::fs::read(cert_path)?))?;
    let mut config = quinn::ClientConfig::with_root_certificates(roots);
    config.transport_config(transport_config());
    let mut endpoint = quinn::Endpoint::client("127.0.0.1:0".parse()?)?;
    endpoint.set_default_client_config(config);
    Ok(endpoint)
}

#[cfg(test)]
mod tests {
    use super::*;
    #[tokio::test]
    async fn rejects_oversize_before_reading_body() {
        let (mut tx, mut rx) = tokio::io::duplex(4);
        tx.write_all(&65537_u32.to_be_bytes()).await.unwrap();
        let result = tokio::time::timeout(Duration::from_millis(100), read_message(&mut rx)).await;
        assert!(result.unwrap().is_err());
    }
    #[tokio::test]
    async fn truncated_frame_fails() {
        let (mut tx, mut rx) = tokio::io::duplex(64);
        tx.write_all(&10_u32.to_be_bytes()).await.unwrap();
        tx.write_all(b"{").await.unwrap();
        drop(tx);
        assert!(read_message(&mut rx).await.is_err());
    }
}
