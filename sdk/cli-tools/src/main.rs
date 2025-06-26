//! # GameVerse CLI Tool
//!
//! Command-line interface for GameVerse Framework plugin development.
//! Provides tools for creating, building, testing, and deploying plugins.

use clap::{Parser, Subcommand};
use anyhow::Result;
use tracing::info;
use colored::*;

mod commands;
mod templates;
mod config;
mod utils;

use commands::{
    plugin::PluginCommands,
    server::ServerCommands,
    marketplace::MarketplaceCommands,
    templates::TemplatesCommands,
    sdk::SdkCommands,
};

/// GameVerse Framework CLI - Plugin Development Tools
#[derive(Parser)]
#[command(
    name = "gameverse",
    version = env!("CARGO_PKG_VERSION"),
    about = "GameVerse Framework CLI for plugin development",
    long_about = r#"
GameVerse Framework CLI provides comprehensive tools for developing, 
building, testing, and deploying plugins for the GameVerse Framework.

This tool is designed to provide superior developer experience compared 
to FiveM tools with modern features like hot reload, type safety, and 
cross-platform support.

Examples:
  gameverse plugin new my-economy --template economy --language rust
  gameverse plugin build --release --target all
  gameverse plugin deploy --server localhost:8080
  gameverse server start --dev
"#
)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    
    /// Enable verbose logging
    #[arg(short, long, global = true)]
    pub verbose: bool,
    
    /// Configuration file path
    #[arg(short, long, global = true)]
    pub config: Option<String>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Plugin development commands
    #[command(subcommand)]
    Plugin(PluginCommands),
    
    /// Server management commands  
    #[command(subcommand)]
    Server(ServerCommands),
    
    /// Plugin marketplace commands
    #[command(subcommand)]
    Marketplace(MarketplaceCommands),
    
    /// Template management commands
    #[command(subcommand)]
    Templates(TemplatesCommands),
    
    /// SDK management commands (e.g., update native SDK)
    #[command(subcommand)]
    Sdk(SdkCommands),
    
    /// Initialize GameVerse project
    Init {
        /// Project name
        name: String,
        /// Project template (server, client, plugin)
        #[arg(short, long, default_value = "server")]
        template: String,
        /// Target directory
        #[arg(short, long)]
        directory: Option<String>,
    },
    
    /// Show version information
    Version,
    
    /// Generate shell completions
    Completions {
        /// Shell type (bash, zsh, fish, powershell)
        shell: String,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    // Печатаем ASCII-логотип
    print_logo();

    let cli = Cli::parse();
    
    // Initialize logging
    init_logging(cli.verbose)?;
    
    info!("🚀 GameVerse CLI started");
    
    // Load configuration
    let config = config::load_config(cli.config.as_deref()).await?;
    
    // Execute command
    match cli.command {
        Commands::Plugin(cmd) => {
            commands::plugin::execute(cmd, &config).await?;
        }
        Commands::Server(cmd) => {
            commands::server::execute(cmd, &config).await?;
        }
        Commands::Marketplace(cmd) => {
            commands::marketplace::execute(cmd, &config).await?;
        }
        Commands::Templates(cmd) => {
            commands::templates::execute(cmd, &config).await?;
        }
        Commands::Sdk(cmd) => {
            commands::sdk::execute(cmd, &config).await?;
        }
        Commands::Init { name, template, directory } => {
            commands::init::execute(name, template, directory, &config).await?;
        }
        Commands::Version => {
            commands::version::execute().await?;
        }
        Commands::Completions { shell } => {
            commands::completions::execute(shell).await?;
        }
    }
    
    info!("✅ GameVerse CLI completed successfully");
    Ok(())
}

fn init_logging(verbose: bool) -> Result<()> {
    let filter = if verbose {
        "gameverse_cli=debug,info"
    } else {
        "gameverse_cli=info"
    };
    
    tracing_subscriber::fmt()
        .with_env_filter(filter)
        .with_target(false)
        .with_thread_ids(false)
        .with_file(false)
        .with_line_number(false)
        .compact()
        .init();
    
    Ok(())
}

fn print_logo() {
    // Простое ASCII с цветами (Green + Cyan gradient)
    let logo = r#"
   ██████╗  █████╗ ███╗   ███╗███████╗██╗   ██╗███████╗██████╗ ███████╗
  ██╔════╝ ██╔══██╗████╗ ████║██╔════╝██║   ██║██╔════╝██╔══██╗██╔════╝
  ██║  ███╗███████║██╔████╔██║█████╗  ██║   ██║█████╗  ██████╔╝███████╗
  ██║   ██║██╔══██║██║╚██╔╝██║██╔══╝  ╚██╗ ██╔╝██╔══╝  ██╔══██╗╚════██║
  ╚██████╔╝██║  ██║██║ ╚═╝ ██║███████╗ ╚████╔╝ ███████╗██║  ██║███████║
   ╚═════╝ ╚═╝  ╚═╝╚═╝     ╚═╝╚══════╝  ╚═══╝  ╚══════╝╚═╝  ╚═╝╚══════╝
    "#;

    for (i, line) in logo.lines().enumerate() {
        let colored_line = if i % 2 == 0 {
            line.green().bold()
        } else {
            line.cyan().bold()
        };
        println!("{}", colored_line);
    }

    println!("{}", "Type `gameverse --help` to explore commands".bright_white());
}
