use anyhow::Result;
use clap::Parser;
use gameverse_transport::{generate_identity, server_endpoint};
use std::{net::SocketAddr, path::PathBuf, time::Duration};

#[derive(Parser)]
struct Args {
    #[arg(long, default_value = "127.0.0.1:30121")]
    bind: SocketAddr,
    #[arg(long)]
    cert: PathBuf,
    #[arg(long)]
    key: PathBuf,
    /// Create a localhost identity and exit. Does not replace existing files.
    #[arg(long)]
    init_identity: bool,
    /// Stop after this many seconds; omitted means run until Ctrl+C.
    #[arg(long)]
    duration: Option<u64>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    if args.init_identity {
        return generate_identity(&args.cert, &args.key);
    }
    anyhow::ensure!(
        args.bind.ip().is_loopback(),
        "M1 accepts loopback binding only"
    );
    let endpoint = server_endpoint(args.bind, &args.cert, &args.key)?;
    println!(
        "{}",
        serde_json::json!({"event":"ready", "address":endpoint.local_addr()?.to_string()})
    );
    let (tx, rx) = tokio::sync::watch::channel(false);
    tokio::spawn(async move {
        if let Some(seconds) = args.duration {
            tokio::select! { _ = tokio::signal::ctrl_c() => {}, _ = tokio::time::sleep(Duration::from_secs(seconds)) => {} }
        } else {
            let _ = tokio::signal::ctrl_c().await;
        }
        let _ = tx.send(true);
    });
    println!("{}", gameverse_server::presence::run(endpoint, rx).await?);
    Ok(())
}
