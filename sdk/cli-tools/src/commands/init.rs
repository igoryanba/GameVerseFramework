//! Project initialization command

use anyhow::Result;
use crate::config::Config;

pub async fn execute(
    name: String,
    template: String,
    directory: Option<String>,
    _config: &Config,
) -> Result<()> {
    println!("🚀 Initializing GameVerse project: {}", name);
    println!("Template: {}", template);
    if let Some(dir) = directory {
        println!("Directory: {}", dir);
    }
    
    // TODO: Implement project initialization
    println!("✅ Project initialization will be implemented in next version");
    
    Ok(())
} 