use anyhow::Result;
use clap::Parser;
#[derive(Parser)]
struct Args {
    #[arg(long, default_value = "127.0.0.1:30122")]
    server: std::net::SocketAddr,
    #[arg(long)]
    cert: std::path::PathBuf,
    #[arg(long,default_value=gameverse_protocol::adapter::DEFAULT_PIPE)]
    pipe: String,
    #[arg(long,default_value=gameverse_client::ui::DEFAULT_PIPE)]
    ui_pipe: String,
    #[arg(long, default_value_t = 86400)]
    duration: u64,
}
#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    #[cfg(windows)]
    {
        gameverse_client::ipc_m2::run(
            &args.pipe,
            &args.ui_pipe,
            args.server,
            &args.cert,
            std::time::Duration::from_secs(args.duration),
        )
        .await
    }
    #[cfg(not(windows))]
    {
        let _ = args;
        anyhow::bail!("GTA M2 bridge requires Windows")
    }
}
