use crate::presence_m2::MetricsHandle;
use anyhow::Result;
use std::net::SocketAddr;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::{TcpListener, TcpStream},
    sync::watch,
    time::{timeout, Duration},
};

pub async fn serve(
    address: SocketAddr,
    metrics: MetricsHandle,
    mut shutdown: watch::Receiver<bool>,
) -> Result<()> {
    let listener = TcpListener::bind(address).await?;
    println!(
        "{}",
        serde_json::json!({"event":"http_ready","address":listener.local_addr()?.to_string()})
    );
    loop {
        tokio::select! {
            changed = shutdown.changed() => if changed.is_err() || *shutdown.borrow() { return Ok(()); },
            accepted = listener.accept() => {
                let (stream, _) = accepted?;
                let metrics = metrics.clone();
                tokio::spawn(async move { let _ = handle(stream, metrics).await; });
            }
        }
    }
}

async fn handle(mut stream: TcpStream, metrics: MetricsHandle) -> Result<()> {
    let mut request = [0_u8; 2048];
    let count = timeout(Duration::from_secs(2), stream.read(&mut request)).await??;
    let first = std::str::from_utf8(&request[..count])?
        .lines()
        .next()
        .unwrap_or_default();
    let path = first
        .strip_prefix("GET ")
        .and_then(|value| value.split_once(' ').map(|pair| pair.0));
    let (status, content_type, body) = response(path, &metrics);
    let reply = format!(
        "HTTP/1.1 {status}\r\nContent-Type: {content_type}\r\nContent-Length: {}\r\nConnection: close\r\nX-Content-Type-Options: nosniff\r\nCache-Control: no-store\r\n\r\n{body}",
        body.len()
    );
    timeout(Duration::from_secs(2), stream.write_all(reply.as_bytes())).await??;
    Ok(())
}

fn response(path: Option<&str>, metrics: &MetricsHandle) -> (&'static str, &'static str, String) {
    match path {
        Some("/v1/health") => ("200 OK", "application/json", "{\"status\":\"ok\"}".into()),
        Some("/v1/ready") => (
            "200 OK",
            "application/json",
            "{\"status\":\"ready\"}".into(),
        ),
        Some("/v1/version") => (
            "200 OK",
            "application/json",
            serde_json::json!({
                "build": env!("CARGO_PKG_VERSION"),
                "control_protocol": gameverse_protocol::control_v2::VERSION,
                "presence_protocol": gameverse_protocol::presence_v2::VERSION
            })
            .to_string(),
        ),
        Some("/v1/metrics") => {
            let value = metrics.snapshot();
            let body = format!(
                concat!(
                    "gameverse_players {}\n",
                    "gameverse_accepted_sessions_total {}\n",
                    "gameverse_disconnects_total {}\n",
                    "gameverse_datagrams_received_total {}\n",
                    "gameverse_datagrams_sent_total {}\n",
                    "gameverse_datagrams_dropped_total {}\n",
                    "gameverse_bytes_received_total {}\n",
                    "gameverse_bytes_sent_total {}\n",
                    "gameverse_tick_max_microseconds {}\n"
                ),
                value.players,
                value.accepted_sessions,
                value.disconnects,
                value.received_datagrams,
                value.sent_datagrams,
                value.dropped_datagrams,
                value.received_bytes,
                value.sent_bytes,
                value.max_tick_micros
            );
            ("200 OK", "text/plain; version=0.0.4", body)
        }
        _ => (
            "404 Not Found",
            "application/json",
            "{\"error\":\"not_found\"}".into(),
        ),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn health_routes_are_bounded_and_versioned() {
        let metrics = MetricsHandle::default();
        assert_eq!(response(Some("/v1/health"), &metrics).0, "200 OK");
        assert!(response(Some("/v1/metrics"), &metrics)
            .2
            .contains("gameverse_players 0"));
        assert_eq!(response(Some("/health"), &metrics).0, "404 Not Found");
        assert_eq!(response(Some("/v2/health"), &metrics).0, "404 Not Found");
    }
}
