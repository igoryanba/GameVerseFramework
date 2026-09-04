use anyhow::{Context, Result};
use clap::Parser;
use gameverse_resource_manifest::from_gameverse_toml;
use gameverse_resource_runtime::{HostSide, Limits, ResourceEvent, ResourceHost};
use serde_json::json;
use std::path::PathBuf;

#[derive(Parser)]
struct Cli {
    #[arg(long)]
    manifest: PathBuf,
    #[arg(long)]
    report: Option<PathBuf>,
}
fn main() -> Result<()> {
    let cli = Cli::parse();
    let root = cli.manifest.parent().context("manifest has no parent")?;
    let manifest = from_gameverse_toml(&std::fs::read_to_string(&cli.manifest)?)?;
    let mut client =
        ResourceHost::new(root, manifest.clone(), HostSide::Client, Limits::default())?;
    let mut server = ResourceHost::new(root, manifest, HostSide::Server, Limits::default())?;
    client.start()?;
    server.start()?;
    client.dispatch(&ResourceEvent {
        resource: "compat-basic".into(),
        name: "fixture:server-ready".into(),
        source: None,
        target: Some(1),
        arguments: vec![json!(40)],
        correlation_id: None,
    })?;
    let to_server = client.drain_outbound();
    for event in &to_server {
        server.dispatch(event)?;
    }
    let to_client = server.drain_outbound();
    client.advance(10)?;
    client.advance(5)?;
    let scheduled = client.drain_outbound();
    let callback = client.call_callback("fixture:add", vec![json!(2), json!(3)])?;
    let export = client.call_export("fixture_answer", vec![])?;
    client.stop()?;
    server.stop()?;
    let report = json!({"passed":to_server.iter().any(|v|v.name=="fixture:client-ack") && to_client.iter().any(|v|v.name=="fixture:accepted") && scheduled.len()==2 && callback==vec![json!(5)] && export==vec![json!(42)], "client_to_server":to_server, "server_to_client":to_client, "scheduled":scheduled, "callback":callback, "export":export, "cleanup":{"client":format!("{:?}",client.state()),"server":format!("{:?}",server.state())}});
    let output = serde_json::to_string_pretty(&report)?;
    if let Some(path) = cli.report {
        std::fs::write(path, &output)?;
    }
    println!("{output}");
    anyhow::ensure!(report["passed"] == true, "acceptance failed");
    Ok(())
}
