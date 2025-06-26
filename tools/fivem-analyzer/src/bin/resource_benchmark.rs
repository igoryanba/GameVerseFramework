use anyhow::Result;
use clap::{Parser};
use std::path::PathBuf;
use tracing::{info, warn};
use tracing_subscriber;

use fivem_analyzer::{benchmarks, utils};

#[derive(Parser)]
#[command(name = "resource-benchmark")]
#[command(about = "GameVerse Framework - Resource Performance Benchmark Tool")]
#[command(version = "0.1.0")]
pub struct Cli {
    /// Path to resource directory
    #[arg(short, long)]
    pub path: PathBuf,
    
    /// Duration of benchmark in seconds
    #[arg(short, long, default_value = "60")]
    pub duration: u64,
    
    /// Include detailed memory analysis
    #[arg(short, long)]
    pub memory_analysis: bool,
    
    /// Output format (json, yaml, table)
    #[arg(short, long, default_value = "table")]
    pub format: String,
    
    /// Enable verbose logging
    #[arg(short, long)]
    pub verbose: bool,
    
    /// Number of concurrent users to simulate
    #[arg(short, long, default_value = "1")]
    pub concurrent_users: u32,
    
    /// Save results to file
    #[arg(short, long)]
    pub save_to: Option<PathBuf>,
}

#[tokio::main]
async fn main() -> Result<()> {
    let cli = Cli::parse();
    
    // Initialize logging
    let log_level = if cli.verbose { "debug" } else { "info" };
    tracing_subscriber::fmt()
        .with_env_filter(format!("resource_benchmark={}", log_level))
        .init();
        
    info!("⚡ Resource Benchmark Tool v0.1.0 - Starting...");
    info!("📦 Resource: {:?}", cli.path);
    info!("⏱️  Duration: {}s", cli.duration);
    info!("👥 Concurrent users: {}", cli.concurrent_users);
    
    if !cli.path.exists() {
        warn!("⚠️  Resource path does not exist: {:?}", cli.path);
        return Err(anyhow::anyhow!("Resource path not found"));
    }
    
    // Run benchmark
    info!("🚀 Starting performance benchmark...");
    
    let results = benchmarks::comprehensive_benchmark(
        &cli.path, 
        cli.duration, 
        cli.memory_analysis
    ).await?;
    
    // Output results
    utils::output_benchmark_results(&results, &cli.format)?;
    
    // Save to file if requested
    if let Some(save_path) = cli.save_to {
        info!("💾 Saving results to: {:?}", save_path);
        save_results_to_file(&results, &save_path, &cli.format).await?;
    }
    
    // Performance summary
    print_performance_summary(&results);
    
    info!("✅ Benchmark completed successfully!");
    Ok(())
}

async fn save_results_to_file(
    results: &fivem_analyzer::types::BenchmarkResults,
    path: &PathBuf,
    format: &str,
) -> Result<()> {
    let content = match format {
        "json" => serde_json::to_string_pretty(results)?,
        "yaml" => serde_yaml::to_string(results)?,
        _ => return Err(anyhow::anyhow!("Unsupported format for file save: {}", format)),
    };
    
    tokio::fs::write(path, content).await?;
    info!("💾 Results saved to: {:?}", path);
    Ok(())
}

fn print_performance_summary(results: &fivem_analyzer::types::BenchmarkResults) {
    println!("\n🎯 КРАТКОЕ РЕЗЮМЕ ПРОИЗВОДИТЕЛЬНОСТИ");
    println!("═══════════════════════════════════════════");
    
    let overall_improvement = results.improvement_factors.overall_improvement_factor;
    
    if overall_improvement > 10.0 {
        println!("🚀 ОТЛИЧНАЯ ПРОИЗВОДИТЕЛЬНОСТЬ: {}x улучшение!", overall_improvement);
        println!("   GameVerse принесет значительный прирост производительности");
    } else if overall_improvement > 5.0 {
        println!("⚡ ХОРОШАЯ ПРОИЗВОДИТЕЛЬНОСТЬ: {}x улучшение", overall_improvement);
        println!("   GameVerse заметно улучшит производительность");
    } else if overall_improvement > 2.0 {
        println!("📈 УМЕРЕННОЕ УЛУЧШЕНИЕ: {}x улучшение", overall_improvement);
        println!("   GameVerse покажет заметные улучшения");
    } else {
        println!("📊 МИНИМАЛЬНОЕ УЛУЧШЕНИЕ: {}x улучшение", overall_improvement);
        println!("   Возможны некоторые улучшения производительности");
    }
    
    // Лучшие улучшения
    let best_improvements = vec![
        ("UI", results.improvement_factors.ui_performance_factor),
        ("Запуск", results.improvement_factors.startup_speedup_factor),
        ("База данных", results.improvement_factors.database_efficiency_factor),
        ("Память", results.improvement_factors.memory_reduction_factor),
        ("CPU", results.improvement_factors.cpu_efficiency_factor),
    ];
    
    let mut sorted_improvements = best_improvements.clone();
    sorted_improvements.sort_by(|a, b| b.1.partial_cmp(&a.1).unwrap());
    
    println!("\n🏆 ТОП-3 УЛУЧШЕНИЯ:");
    for (i, (category, factor)) in sorted_improvements.iter().take(3).enumerate() {
        println!("   {}. {}: {}x улучшение", i + 1, category, factor);
    }
    
    // Рекомендации
    println!("\n💡 РЕКОМЕНДАЦИИ:");
    if results.improvement_factors.ui_performance_factor > 10.0 {
        println!("   • Приоритет: Миграция UI на WebAssembly");
    }
    if results.improvement_factors.memory_reduction_factor > 5.0 {
        println!("   • Значительное снижение потребления памяти");
    }
    if results.improvement_factors.startup_speedup_factor > 8.0 {
        println!("   • Намного более быстрый запуск сервера");
    }
    
    println!("\n🎯 ЗАКЛЮЧЕНИЕ: Миграция на GameVerse Framework настоятельно рекомендуется!");
} 