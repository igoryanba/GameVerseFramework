use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

#[derive(Parser)]
#[command(
    name = "fivem-analyzer",
    about = "Static FiveM resource compatibility inventory",
    version = "0.2.0"
)]
struct Cli {
    #[arg(short, long)]
    path: PathBuf,
    #[arg(short, long, default_value = "json")]
    format: String,
    #[arg(long)]
    gameverse_manifest: Option<PathBuf>,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let report = fivem_analyzer::analyze(&cli.path)?;
    if let Some(path) = cli.gameverse_manifest {
        std::fs::write(path, fivem_analyzer::to_gameverse_toml(&report)?)?;
    }
    match cli.format.as_str() {
        "json" => println!("{}", serde_json::to_string_pretty(&report)?),
        "yaml" => println!("{}", serde_yaml::to_string(&report)?),
        other => anyhow::bail!("unsupported format {other}; use json or yaml"),
    }
    Ok(())
}
