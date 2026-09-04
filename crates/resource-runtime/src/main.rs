use anyhow::{Context, Result};
use clap::{Parser, Subcommand, ValueEnum};
use gameverse_resource_manifest::{from_gameverse_toml, resolve_and_validate};
use gameverse_resource_runtime::{HostSide, Limits, ResourceHost};
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "gameverse-resource-host",
    about = "Validate and run a sandboxed GameVerse resource"
)]
struct Cli {
    #[command(subcommand)]
    command: Command,
}
#[derive(Subcommand)]
enum Command {
    Validate {
        #[arg(long)]
        manifest: PathBuf,
    },
    Run {
        #[arg(long)]
        manifest: PathBuf,
        #[arg(long, value_enum)]
        side: Side,
    },
}
#[derive(Clone, ValueEnum)]
enum Side {
    Client,
    Server,
}

fn load(path: &PathBuf) -> Result<(PathBuf, gameverse_resource_manifest::ResourceManifest)> {
    let text = std::fs::read_to_string(path).with_context(|| format!("read {}", path.display()))?;
    Ok((
        path.parent()
            .context("manifest has no parent")?
            .to_path_buf(),
        from_gameverse_toml(&text)?,
    ))
}
fn main() -> Result<()> {
    match Cli::parse().command {
        Command::Validate { manifest } => {
            let (root, manifest) = load(&manifest)?;
            let files = resolve_and_validate(&root, &manifest)?;
            println!(
                "{}",
                serde_json::json!({"valid":true,"resource":manifest.name,"files":files})
            );
        }
        Command::Run { manifest, side } => {
            let (root, manifest) = load(&manifest)?;
            let mut host = ResourceHost::new(
                root,
                manifest,
                match side {
                    Side::Client => HostSide::Client,
                    Side::Server => HostSide::Server,
                },
                Limits::default(),
            )?;
            host.start()?;
            println!(
                "{}",
                serde_json::json!({"state":"started","resource":host.manifest().name})
            );
        }
    }
    Ok(())
}
