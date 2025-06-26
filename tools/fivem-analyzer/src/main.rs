use anyhow::Result;
use clap::{Parser, Subcommand};
use std::path::PathBuf;
use tracing::{info, error};
use tracing_subscriber;

pub mod qbcore;
pub mod esx;
pub mod analysis_engine;
pub mod types;
pub mod migration;
pub mod benchmarks;
pub mod utils;

#[derive(Parser)]
#[command(name = "fivem-analyzer")]
#[command(about = "GameVerse Framework - FiveM Resource Analysis and Migration Tool")]
#[command(version = "0.1.0")]
#[command(author = "GameVerse Team")]
pub struct Cli {
    #[command(subcommand)]
    pub command: Commands,
    
    /// Enable verbose logging
    #[arg(short, long)]
    pub verbose: bool,
    
    /// Output format (json, yaml, table)
    #[arg(short, long, default_value = "table")]
    pub format: String,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Analyze QBCore resources
    QBCore {
        /// Path to QBCore resource directory
        #[arg(short, long)]
        path: PathBuf,
        
        /// Generate migration report
        #[arg(short, long)]
        migration_report: bool,
        
        /// Analyze dependencies
        #[arg(short, long)]
        dependencies: bool,
        
        /// Performance benchmarking
        #[arg(short, long)]
        benchmark: bool,
    },
    
    /// Analyze ESX resources
    ESX {
        /// Path to ESX resource directory
        #[arg(short, long)]
        path: PathBuf,
        
        /// Generate migration report
        #[arg(short, long)]
        migration_report: bool,
    },
    
    /// Download and analyze popular FiveM resources
    Download {
        /// Framework type (qbcore, esx, all)
        #[arg(short, long, default_value = "qbcore")]
        framework: String,
        
        /// Number of resources to download
        #[arg(short, long, default_value = "10")]
        count: u32,
        
        /// Output directory
        #[arg(short, long, default_value = "./downloaded_resources")]
        output: PathBuf,
    },
    
    /// Generate comprehensive migration plan
    MigrationPlan {
        /// Path to FiveM server directory
        #[arg(short, long)]
        server_path: PathBuf,
        
        /// Target GameVerse configuration path
        #[arg(short, long)]
        target_path: Option<PathBuf>,
        
        /// Generate automated migration scripts
        #[arg(short, long)]
        generate_scripts: bool,
    },
    
    /// Performance comparison between FiveM and GameVerse
    Benchmark {
        /// Path to resource for benchmarking
        #[arg(short, long)]
        resource_path: PathBuf,
        
        /// Duration of benchmark in seconds
        #[arg(short, long, default_value = "60")]
        duration: u64,
        
        /// Include memory analysis
        #[arg(short, long)]
        memory_analysis: bool,
    },
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Initialize logging
    let log_level = if cli.verbose { "debug" } else { "info" };
    tracing_subscriber::fmt()
        .with_env_filter(format!("fivem_analyzer={}", log_level))
        .init();
        
    info!("🚀 GameVerse FiveM Analyzer v0.1.0 - Starting analysis...");
    
    match cli.command {
        Commands::QBCore { 
            path, 
            migration_report, 
            dependencies, 
            benchmark 
        } => {
            info!("📊 Analyzing QBCore resource at: {:?}", path);
            
            let mut analyzer = qbcore::QBCoreAnalyzer::new(path.clone());
            let report = analyzer.analyze_resource().await?;
            
            utils::output_report(&report, &cli.format)?;
            
            if migration_report {
                info!("📋 Generating migration report...");
                let migration_plan = migration::generate_migration_plan(&report)?;
                utils::output_migration_plan(&migration_plan, &cli.format)?;
            }
            
            if dependencies {
                info!("🔗 Analyzing dependencies...");
                let deps = analyzer.analyze_dependencies().await?;
                utils::output_dependencies(&deps, &cli.format)?;
            }
            
            if benchmark {
                info!("⚡ Running performance benchmark...");
                let bench_results = benchmarks::benchmark_resource(&path).await?;
                utils::output_benchmark_results(&bench_results, &cli.format)?;
            }
        },
        
        Commands::ESX { path, migration_report } => {
            info!("📊 Analyzing ESX resource at: {:?}", path);
            
            let mut analyzer = esx::ESXAnalyzer::new(path);
            let report = analyzer.analyze_resource().await?;
            
            utils::output_report(&report, &cli.format)?;
            
            if migration_report {
                let migration_plan = migration::generate_migration_plan(&report)?;
                utils::output_migration_plan(&migration_plan, &cli.format)?;
            }
        },
        
        Commands::Download { framework, count, output } => {
            info!("📥 Downloading {} {} resources to {:?}", count, framework, output);
            
            match framework.as_str() {
                "qbcore" => qbcore::download_popular_resources(count, &output).await?,
                "esx" => esx::download_popular_resources(count, &output).await?,
                "all" => {
                    qbcore::download_popular_resources(count / 2, &output.join("qbcore")).await?;
                    esx::download_popular_resources(count / 2, &output.join("esx")).await?;
                },
                _ => {
                    error!("❌ Unknown framework: {}", framework);
                    return Err(anyhow::anyhow!("Unsupported framework"));
                }
            }
        },
        
        Commands::MigrationPlan { 
            server_path, 
            target_path, 
            generate_scripts 
        } => {
            info!("🔄 Generating migration plan for server: {:?}", server_path);
            
            let engine = analysis_engine::AnalysisEngine::new();
            let plan = engine.generate_comprehensive_migration_plan(
                &server_path, 
                target_path.as_ref()
            ).await?;
            
            utils::output_migration_plan(&plan, &cli.format)?;
            
            if generate_scripts {
                info!("📜 Generating automated migration scripts...");
                migration::generate_migration_scripts(&plan, &server_path)?;
            }
        },
        
        Commands::Benchmark { 
            resource_path, 
            duration, 
            memory_analysis 
        } => {
            info!("⚡ Benchmarking resource: {:?} for {}s", resource_path, duration);
            
            let results = benchmarks::comprehensive_benchmark(
                &resource_path, 
                duration, 
                memory_analysis
            ).await?;
            
            utils::output_benchmark_results(&results, &cli.format)?;
        }
    }
    
    info!("✅ Analysis completed successfully!");
    Ok(())
} 