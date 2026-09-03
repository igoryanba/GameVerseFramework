//! A second network player. By default follows a small circle near the real player.
use anyhow::Result;
use clap::Parser;
use gameverse_client::presence::Client;
use gameverse_protocol::presence::PlayerState;
use std::time::{Duration, Instant};
#[derive(Parser)]
struct Args {
    #[arg(long, default_value = "127.0.0.1:30121")]
    server: std::net::SocketAddr,
    #[arg(long)]
    cert: std::path::PathBuf,
    #[arg(long, default_value_t = 60)]
    duration: u64,
    #[arg(long)]
    report: Option<std::path::PathBuf>,
    /// Anchor once next to the first observed real player, on its ground plane.
    #[arg(long, default_value_t = 0x705e61f2)]
    model_hash: u32,
}
#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let mut client = Client::connect(args.server, &args.cert).await?;
    let start = Instant::now();
    let mut clock = tokio::time::interval(Duration::from_millis(50));
    clock.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);
    let mut anchor = None;
    let mut received = 0_u64;
    let mut last_remote = None;
    let mut published = 0;
    let mut last_heartbeat = Instant::now() - Duration::from_secs(1);
    while start.elapsed() < Duration::from_secs(args.duration) {
        tokio::select! {
            _=tokio::signal::ctrl_c()=>break,
            changed=client.snapshots.changed()=>{changed?;let s=client.snapshots.borrow_and_update().clone();
                if let Some(remote)=s.entities.into_iter().find(|e|e.id!=client.entity) {received+=1;if anchor.is_none(){anchor=Some(remote.state.position);}last_remote=Some(remote);
                    if received==1||received.is_multiple_of(100) {println!("{}",serde_json::json!({"event":"real_player_observed","frames":received,"remote":last_remote}));}
                }
            }
            _=clock.tick()=>{if let Some(origin)=anchor {
                let t=start.elapsed().as_secs_f32()*0.3;let yaw=t+std::f32::consts::FRAC_PI_2;
                let state=PlayerState{timestamp_ms:start.elapsed().as_millis() as u64,position:[origin[0]+3.0+t.cos()*2.0,origin[1]+t.sin()*2.0,origin[2]],rotation:[0.0,0.0,(yaw/2.0).sin(),(yaw/2.0).cos()],velocity:[-0.6*t.sin(),0.6*t.cos(),0.0],model_hash:args.model_hash,health:200,armor:0,movement:1,weapon_hash:0xa2719263};
                client.publish(state).await?;published+=1;
            } else if last_heartbeat.elapsed() >= Duration::from_secs(1) {
                client.heartbeat().await?; last_heartbeat=Instant::now();
            }}
        }
    }
    client.close().await?;
    let report = serde_json::json!({"elapsed_seconds":start.elapsed().as_secs_f64(),"received_remote_states":received,"published":published,"last_remote":last_remote,"clean_shutdown":true});
    if let Some(path) = args.report {
        std::fs::write(path, serde_json::to_vec_pretty(&report)?)?;
    }
    println!("{report}");
    anyhow::ensure!(
        received > 0 && published > 0,
        "no peer observed; integration was not exercised"
    );
    Ok(())
}
