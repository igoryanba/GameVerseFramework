//! Version command

use anyhow::Result;
use console::style;

pub async fn execute() -> Result<()> {
    let version = env!("CARGO_PKG_VERSION");
    let name = env!("CARGO_PKG_NAME");
    let description = env!("CARGO_PKG_DESCRIPTION");
    
    println!("{} {}", style(name).cyan().bold(), style(version).green());
    println!("{}", style(description).dim());
    println!();
    
    // Build information
    println!("Build Information:");
    let rustc_version = std::env::var("RUSTC_VERSION").unwrap_or_else(|_| "unknown".to_string());
    let target = std::env::var("TARGET").unwrap_or_else(|_| std::env::consts::ARCH.to_string());
    
    println!("  Rust Version: {}", rustc_version);
    println!("  Target: {}", target);
    println!("  Profile: {}", if cfg!(debug_assertions) { "debug" } else { "release" });
    
    #[cfg(feature = "git_info")]
    {
        let git_hash = std::env::var("GIT_HASH").unwrap_or_else(|_| "unknown".to_string());
        let git_branch = std::env::var("GIT_BRANCH").unwrap_or_else(|_| "unknown".to_string());
        println!("  Git Commit: {}", git_hash);
        println!("  Git Branch: {}", git_branch);
    }
    
    println!();
    println!("Repository: {}", env!("CARGO_PKG_REPOSITORY"));
    println!("License: {}", env!("CARGO_PKG_LICENSE"));
    
    Ok(())
} 