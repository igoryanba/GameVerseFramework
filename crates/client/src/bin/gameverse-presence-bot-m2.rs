use anyhow::Result;
use clap::Parser;
use gameverse_client::m2::Client;
use gameverse_protocol::presence_v2::{
    Appearance, CombatPresentation, Locomotion, PlayerFrame, Transform,
};
use std::{
    net::SocketAddr,
    path::PathBuf,
    time::{Duration, Instant},
};

#[derive(Parser)]
struct Args {
    #[arg(long, default_value = "127.0.0.1:30122")]
    server: SocketAddr,
    #[arg(long)]
    cert: PathBuf,
    #[arg(long, default_value_t = 10)]
    duration: u64,
    #[arg(long)]
    report: Option<PathBuf>,
}
#[tokio::main]
async fn main() -> Result<()> {
    let args = Args::parse();
    let client = Client::connect(args.server, &args.cert, None).await?;
    let start = Instant::now();
    let mut tick = tokio::time::interval(Duration::from_millis(50));
    let mut sequence = 1_u64;
    let mut received = 0_u64;
    while start.elapsed() < Duration::from_secs(args.duration) {
        tokio::select! {
            _=tick.tick()=>{sequence+=1;let t=start.elapsed().as_secs_f32();client.publish(PlayerFrame { sequence, client_tick:start.elapsed().as_millis() as u64, transform:Transform { position:[t.cos()*5.0,t.sin()*5.0,20.0], rotation:[0.0,0.0,0.0,1.0], velocity:[-t.sin(),t.cos(),0.0] }, appearance:Some(Appearance{model_hash:0x705e61f2}), locomotion:Locomotion::Run, combat:CombatPresentation { aiming:false,shooting:false,reloading:false,dead:false,weapon_hash:0,aim_target:None }, vehicle:None })?;},
            frame=client.read_frame()=>{let _=frame?;received+=1;},
            _=tokio::signal::ctrl_c()=>break,
        }
    }
    let session = client.session;
    client.close().await?;
    let report = serde_json::json!({"session":session,"published":sequence-1,"received":received,"clean_shutdown":true});
    if let Some(path) = args.report {
        std::fs::write(path, serde_json::to_vec_pretty(&report)?)?;
    }
    println!("{report}");
    Ok(())
}
