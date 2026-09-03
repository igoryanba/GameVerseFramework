use anyhow::Result;
use clap::Parser;
use gameverse_client::{Client, GameAdapter, MemoryAdapter, Replica};
use std::{
    net::SocketAddr,
    path::PathBuf,
    time::{Duration, Instant},
};

#[derive(Parser)]
struct Args {
    #[arg(long, default_value = "127.0.0.1:30120")]
    server: SocketAddr,
    #[arg(long)]
    cert: PathBuf,
    #[arg(long, default_value_t = 10)]
    duration: u64,
    #[arg(long, default_value_t = 3.0)]
    move_seconds: f32,
    #[arg(long, default_value_t = 1.0, allow_hyphen_values = true)]
    dx: f32,
    #[arg(long, default_value_t = 0.0, allow_hyphen_values = true)]
    dy: f32,
    #[arg(long)]
    reconnect_after: Option<u64>,
    #[arg(long)]
    report: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    anyhow::ensure!(
        args.duration > 0 && args.move_seconds.is_finite() && args.move_seconds >= 0.0,
        "invalid duration"
    );
    anyhow::ensure!(
        gameverse_protocol::valid_direction([args.dx, args.dy]),
        "invalid direction"
    );
    let mut client = Client::connect(args.server, &args.cert).await?;
    let mut sessions = vec![client.session];
    let mut replica = Replica::default();
    replica.apply(client.snapshots.borrow_and_update().clone());
    let mut adapter = MemoryAdapter {
        direction: [args.dx, args.dy],
        move_seconds: args.move_seconds,
        state: None,
        stopped: false,
    };
    let started = Instant::now();
    let mut ticker = tokio::time::interval(Duration::from_millis(50));
    ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
    let stop = tokio::time::sleep(Duration::from_secs(args.duration));
    tokio::pin!(stop);
    let mut snapshots_received = 1_u64;
    let mut reconnected = false;
    let mut convergence_state = None;
    loop {
        tokio::select! {
            _ = &mut stop => break,
            _ = tokio::signal::ctrl_c() => break,
            changed = client.snapshots.changed() => {
                changed?;
                let state = client.snapshots.borrow_and_update().clone();
                if replica.apply(state) {
                    snapshots_received += 1;
                    adapter.apply_snapshot(replica.authoritative().unwrap());
                    // Capture before either process disconnects; final snapshots may correctly contain despawns.
                    if convergence_state.is_none() && started.elapsed().as_secs_f64() >= args.duration as f64 - 1.0 {
                        convergence_state = replica.authoritative().cloned();
                    }
                }
            }
            _ = ticker.tick() => {
                if !reconnected && args.reconnect_after.is_some_and(|s| started.elapsed().as_secs() >= s) {
                    client.close().await?;
                    tokio::time::sleep(Duration::from_millis(250)).await;
                    client = Client::connect(args.server, &args.cert).await?;
                    sessions.push(client.session);
                    replica = Replica::default();
                    replica.apply(client.snapshots.borrow_and_update().clone());
                    reconnected = true;
                }
                client.input(adapter.input(started.elapsed().as_secs_f32())).await?;
            }
        }
    }
    client.close().await?;
    adapter.shutdown();
    let report = serde_json::json!({"sessions":sessions, "snapshots":snapshots_received, "elapsed_seconds":started.elapsed().as_secs_f64(), "convergence_state":convergence_state, "final_state":replica.authoritative(), "clean_shutdown":adapter.stopped});
    if let Some(path) = args.report {
        std::fs::write(path, serde_json::to_vec_pretty(&report)?)?;
    }
    println!("{report}");
    Ok(())
}
