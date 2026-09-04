use anyhow::Result;
use clap::Parser;
use gameverse_client::m2::Client;
use gameverse_protocol::presence_v2::{
    Appearance, CombatPresentation, Locomotion, PlayerFrame, Transform, MAX_PLAYERS,
};
use std::{
    net::SocketAddr,
    path::{Path, PathBuf},
    time::{Duration, Instant},
};
use tokio::task::JoinSet;

#[derive(Parser)]
struct Args {
    #[arg(long, default_value = "127.0.0.1:30122")]
    server: SocketAddr,
    #[arg(long)]
    cert: PathBuf,
    #[arg(long, default_value_t = 10)]
    duration: u64,
    #[arg(long, default_value_t = 1)]
    clients: usize,
    #[arg(long)]
    report: Option<PathBuf>,
}

#[derive(Default)]
struct BotReport {
    published: u64,
    received: u64,
    errors: u64,
}

async fn bot(
    index: usize,
    server: SocketAddr,
    cert: &Path,
    duration: Duration,
) -> Result<BotReport> {
    let client = Client::connect(server, cert, None).await?;
    let start = Instant::now();
    let mut tick = tokio::time::interval(Duration::from_millis(50));
    tick.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
    let mut sequence = 1_u64;
    let mut report = BotReport::default();
    while start.elapsed() < duration {
        tokio::select! {
            _ = tick.tick() => {
                sequence += 1;
                let t = start.elapsed().as_secs_f32() + index as f32 * 0.1;
                match client.publish(PlayerFrame {
                    sequence,
                    client_tick: start.elapsed().as_millis() as u64,
                    transform: Transform {
                        position: [t.cos() * 25.0, t.sin() * 25.0, 20.0],
                        rotation: [0.0, 0.0, 0.0, 1.0],
                        velocity: [-t.sin(), t.cos(), 0.0],
                    },
                    appearance: Some(Appearance { model_hash: 0x705e61f2 }),
                    locomotion: Locomotion::Run,
                    combat: CombatPresentation {
                        aiming: false,
                        shooting: false,
                        reloading: false,
                        dead: false,
                        weapon_hash: 0,
                        aim_target: None,
                    },
                    vehicle: None,
                }) {
                    Ok(()) => report.published += 1,
                    Err(_) => report.errors += 1,
                }
            },
            frame = client.read_frame() => match frame {
                Ok(_) => report.received += 1,
                Err(_) => report.errors += 1,
            },
            _ = tokio::signal::ctrl_c() => break,
        }
    }
    client.close().await?;
    Ok(report)
}

#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    anyhow::ensure!(
        (1..=MAX_PLAYERS).contains(&args.clients),
        "clients must be between 1 and {MAX_PLAYERS}"
    );
    anyhow::ensure!(args.duration > 0, "duration must be positive");
    let mut bots = JoinSet::new();
    for index in 0..args.clients {
        let cert = args.cert.clone();
        let duration = Duration::from_secs(args.duration);
        let server = args.server;
        bots.spawn(async move { bot(index, server, &cert, duration).await });
    }
    let mut published = 0_u64;
    let mut received = 0_u64;
    let mut errors = 0_u64;
    let mut completed = 0_usize;
    while let Some(result) = bots.join_next().await {
        let report = result??;
        published += report.published;
        received += report.received;
        errors += report.errors;
        completed += 1;
    }
    let report = serde_json::json!({
        "clients": args.clients,
        "completed": completed,
        "duration_seconds": args.duration,
        "published": published,
        "received": received,
        "errors": errors,
        "clean_shutdown": completed == args.clients && errors == 0,
    });
    if let Some(path) = args.report {
        std::fs::write(path, serde_json::to_vec_pretty(&report)?)?;
    }
    println!("{report}");
    Ok(())
}
