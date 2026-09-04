use anyhow::Result;
use clap::Parser;
use gameverse_transport::{generate_identity, server_endpoint};
use std::{net::SocketAddr, path::PathBuf, time::Duration};

#[derive(Parser)]
struct Args {
    #[arg(long, default_value = "0.0.0.0:30122")]
    bind: SocketAddr,
    #[arg(long)]
    cert: PathBuf,
    #[arg(long)]
    key: PathBuf,
    #[arg(long)]
    init_identity: bool,
    #[arg(long)]
    duration: Option<u64>,
    #[arg(long, env = "DATABASE_URL")]
    database_url: Option<String>,
}
#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    if args.init_identity {
        return generate_identity(&args.cert, &args.key);
    }
    let endpoint = server_endpoint(args.bind, &args.cert, &args.key)?;
    println!(
        "{}",
        serde_json::json!({"event":"m2_ready","address":endpoint.local_addr()?.to_string()})
    );
    let (tx, rx) = tokio::sync::watch::channel(false);
    tokio::spawn(async move {
        if let Some(seconds) = args.duration {
            tokio::select! { _=tokio::signal::ctrl_c()=>{}, _=tokio::time::sleep(Duration::from_secs(seconds))=>{} }
        } else {
            let _ = tokio::signal::ctrl_c().await;
        }
        let _ = tx.send(true);
    });
    let report = if let Some(database_url) = args.database_url {
        let store = gameverse_rp::persistence::PostgresStore::connect(&database_url, 16).await?;
        store.migrate().await?;
        gameverse_server::presence_m2::run_alpha(endpoint, store, rx).await?
    } else {
        gameverse_server::presence_m2::run(endpoint, rx).await?
    };
    println!("{}", report);
    Ok(())
}
