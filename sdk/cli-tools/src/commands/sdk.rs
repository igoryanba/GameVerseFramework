use anyhow::Result;
use clap::Subcommand;
use tracing::{info, error};
use std::process::Command;

use crate::config::Config;

/// SDK-related commands (e.g., update native SDK)
#[derive(Subcommand, Debug)]
pub enum SdkCommands {
    /// Update the locally cached/generated SDK (native wrappers, types, etc.)
    Update {
        /// Optional path to output directory (defaults to ./generated_sdk)
        #[arg(short, long)]
        output: Option<String>,
    },
}

/// Entry point executed from main.rs
pub async fn execute(cmd: SdkCommands, _config: &Config) -> Result<()> {
    match cmd {
        SdkCommands::Update { output } => {
            // For now we just log; real implementation will invoke native-generator
            let out_dir = output.unwrap_or_else(|| "generated_sdk".into());
            info!("Starting SDK update. Output dir: {}", out_dir);
            // Пытаемся запустить native-generator, если он находится в стандартном пути репозитория
            let gen_path = "../native-generator"; // относительный путь от cli-tools

            let status = Command::new("cargo")
                .args([
                    "run",
                    "--quiet",
                    "--package",
                    "native-generator",
                    "--bin",
                    "native-generator",
                    "--",
                    "generate",
                    "--target",
                    "rust",
                    "--source",
                    "local",
                    "--output",
                    &out_dir,
                ])
                .current_dir(gen_path)
                .status();

            match status {
                Ok(st) if st.success() => {
                    info!("✅ SDK successfully generated in {}", out_dir);
                }
                Ok(st) => {
                    error!("SDK generation exited with status {}", st);
                }
                Err(e) => {
                    error!("Failed to launch native-generator: {}", e);
                }
            }
        }
    }
    Ok(())
} 