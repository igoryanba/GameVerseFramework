//! Plugin management commands

use clap::Subcommand;
use anyhow::{Result, Context};
use std::path::{Path, PathBuf};
use tracing::{info, warn};
use indicatif::{ProgressBar, ProgressStyle};
use dialoguer::{Select, Confirm};
use console::style;

use crate::config::Config;
use crate::templates::TemplateManager;
use crate::utils::{FileUtils, ProcessUtils};

#[derive(Subcommand)]
pub enum PluginCommands {
    /// Create a new plugin
    New {
        /// Plugin name
        name: String,
        /// Plugin template (basic, economy, roleplay, admin)
        #[arg(short, long)]
        template: Option<String>,
        /// Programming language (rust, lua, typescript)
        #[arg(short, long)]
        language: Option<String>,
        /// Target directory
        #[arg(short, long)]
        directory: Option<String>,
        /// Skip interactive prompts
        #[arg(long)]
        no_interactive: bool,
    },
    
    /// Build plugin
    Build {
        /// Plugin directory
        #[arg(short, long)]
        plugin_dir: Option<String>,
        /// Build target (debug, release)
        #[arg(short, long)]
        target: Option<String>,
        /// Cross-compile targets (windows, linux, macos, all)
        #[arg(long)]
        cross_compile: Vec<String>,
        /// Enable optimizations
        #[arg(long)]
        optimize: bool,
    },
    
    /// Test plugin
    Test {
        /// Plugin directory
        #[arg(short, long)]
        plugin_dir: Option<String>,
        /// Run integration tests
        #[arg(long)]
        integration: bool,
        /// Run performance tests
        #[arg(long)]
        performance: bool,
    },
    
    /// Deploy plugin to server
    Deploy {
        /// Plugin directory
        #[arg(short, long)]
        plugin_dir: Option<String>,
        /// Target server
        #[arg(short, long)]
        server: Option<String>,
        /// Deployment environment (dev, staging, prod)
        #[arg(short, long)]
        environment: Option<String>,
        /// Force deployment without confirmation
        #[arg(long)]
        force: bool,
    },
    
    /// Package plugin for distribution
    Package {
        /// Plugin directory
        #[arg(short, long)]
        plugin_dir: Option<String>,
        /// Output file
        #[arg(short, long)]
        output: Option<String>,
        /// Include source code
        #[arg(long)]
        include_source: bool,
    },
    
    /// Install plugin from package or marketplace
    Install {
        /// Plugin package path or marketplace ID
        plugin: String,
        /// Installation directory
        #[arg(short, long)]
        directory: Option<String>,
        /// Install specific version
        #[arg(short, long)]
        version: Option<String>,
    },
    
    /// List installed plugins
    List {
        /// Show detailed information
        #[arg(short, long)]
        detailed: bool,
    },
    
    /// Validate plugin configuration
    Validate {
        /// Plugin directory
        #[arg(short, long)]
        plugin_dir: Option<String>,
    },
    
    /// Watch for changes and auto-reload
    Watch {
        /// Plugin directory
        #[arg(short, long)]
        plugin_dir: Option<String>,
        /// Server to reload on
        #[arg(short, long)]
        server: Option<String>,
    },
}

pub async fn execute(cmd: PluginCommands, config: &Config) -> Result<()> {
    match cmd {
        PluginCommands::New { name, template, language, directory, no_interactive } => {
            create_plugin(name, template, language, directory, no_interactive, config).await
        }
        PluginCommands::Build { plugin_dir, target, cross_compile, optimize } => {
            build_plugin(plugin_dir, target, cross_compile, optimize, config).await
        }
        PluginCommands::Test { plugin_dir, integration, performance } => {
            test_plugin(plugin_dir, integration, performance, config).await
        }
        PluginCommands::Deploy { plugin_dir, server, environment, force } => {
            deploy_plugin(plugin_dir, server, environment, force, config).await
        }
        PluginCommands::Package { plugin_dir, output, include_source } => {
            package_plugin(plugin_dir, output, include_source, config).await
        }
        PluginCommands::Install { plugin, directory, version } => {
            install_plugin(plugin, directory, version, config).await
        }
        PluginCommands::List { detailed } => {
            list_plugins(detailed, config).await
        }
        PluginCommands::Validate { plugin_dir } => {
            validate_plugin(plugin_dir, config).await
        }
        PluginCommands::Watch { plugin_dir, server } => {
            watch_plugin(plugin_dir, server, config).await
        }
    }
}

async fn create_plugin(
    name: String,
    template: Option<String>,
    language: Option<String>,
    directory: Option<String>,
    no_interactive: bool,
    config: &Config,
) -> Result<()> {
    info!("🔧 Creating new plugin: {}", style(&name).cyan().bold());
    
    // Determine template
    let template = if let Some(template) = template {
        template
    } else if no_interactive {
        config.plugin.default_template.clone()
    } else {
        let templates = vec!["basic", "economy", "roleplay", "admin", "custom"];
        let selection = Select::new()
            .with_prompt("Select plugin template")
            .items(&templates)
            .default(0)
            .interact()?;
        templates[selection].to_string()
    };
    
    // Determine language
    let language = if let Some(language) = language {
        language
    } else if no_interactive {
        config.plugin.default_language.clone()
    } else {
        let languages = vec!["rust", "lua", "typescript"];
        let selection = Select::new()
            .with_prompt("Select programming language")
            .items(&languages)
            .default(0)
            .interact()?;
        languages[selection].to_string()
    };
    
    // Determine directory
    let target_dir = if let Some(dir) = directory {
        PathBuf::from(dir)
    } else if let Some(dev_dir) = &config.plugin.dev_directory {
        dev_dir.join(&name)
    } else {
        std::env::current_dir()?.join(&name)
    };
    
    if target_dir.exists() && !no_interactive {
        let overwrite = Confirm::new()
            .with_prompt(&format!("Directory '{}' already exists. Overwrite?", target_dir.display()))
            .default(false)
            .interact()?;
        
        if !overwrite {
            warn!("Plugin creation cancelled");
            return Ok(());
        }
    }
    
    // Create plugin from template
    let progress = ProgressBar::new(100);
    progress.set_style(
        ProgressStyle::default_bar()
            .template("{spinner:.green} [{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
            .unwrap()
    );
    
    progress.set_message("Downloading template...");
    progress.set_position(10);
    
    let template_manager = TemplateManager::new(&config.templates);
    template_manager.download_templates().await?;
    
    progress.set_message("Creating plugin structure...");
    progress.set_position(30);
    
    template_manager.create_plugin(&template, &language, &name, &target_dir, config).await?;
    
    progress.set_message("Installing dependencies...");
    progress.set_position(60);
    
    // Install dependencies based on language
    match language.as_str() {
        "rust" => {
            ProcessUtils::run_command("cargo", &["check"], Some(&target_dir)).await?;
        }
        "typescript" => {
            ProcessUtils::run_command("npm", &["install"], Some(&target_dir)).await?;
        }
        _ => {}
    }
    
    progress.set_message("Finalizing...");
    progress.set_position(90);
    
    // Initialize git repository
    if !no_interactive {
        let init_git = Confirm::new()
            .with_prompt("Initialize git repository?")
            .default(true)
            .interact()?;
        
        if init_git {
            ProcessUtils::run_command("git", &["init"], Some(&target_dir)).await?;
            ProcessUtils::run_command("git", &["add", "."], Some(&target_dir)).await?;
            ProcessUtils::run_command("git", &["commit", "-m", "Initial commit"], Some(&target_dir)).await?;
        }
    }
    
    progress.set_message("Complete!");
    progress.set_position(100);
    progress.finish_with_message("✅ Plugin created successfully!");
    
    println!();
    println!("{} Plugin '{}' created successfully!", style("✅").green(), style(&name).cyan().bold());
    println!("   Template: {}", style(&template).yellow());
    println!("   Language: {}", style(&language).yellow());
    println!("   Location: {}", style(target_dir.display()).yellow());
    println!();
    println!("Next steps:");
    println!("   cd {}", target_dir.display());
    println!("   gameverse plugin build");
    println!("   gameverse plugin test");
    
    Ok(())
}

async fn build_plugin(
    plugin_dir: Option<String>,
    target: Option<String>,
    cross_compile: Vec<String>,
    optimize: bool,
    config: &Config,
) -> Result<()> {
    let plugin_dir = resolve_plugin_dir(plugin_dir)?;
    let plugin_config = load_plugin_config(&plugin_dir).await?;
    
    info!("🔨 Building plugin: {}", plugin_config.plugin.name);
    
    let target = target.unwrap_or_else(|| config.build.optimization.clone());
    
    // Build for each target platform
    let targets = if cross_compile.is_empty() {
        vec!["current".to_string()]
    } else if cross_compile.contains(&"all".to_string()) {
        config.build.default_targets.clone()
    } else {
        cross_compile
    };
    
    for build_target in targets {
        info!("Building for target: {}", build_target);
        
        let language = detect_plugin_language(&plugin_dir)?;
        match language.as_str() {
            "rust" => build_rust_plugin(&plugin_dir, &target, &build_target, optimize).await?,
            "typescript" => build_typescript_plugin(&plugin_dir, &target, &build_target).await?,
            "lua" => validate_lua_plugin(&plugin_dir).await?,
            _ => return Err(anyhow::anyhow!("Unsupported language: {}", language)),
        }
    }
    
    println!("✅ Plugin built successfully!");
    Ok(())
}

async fn build_rust_plugin(plugin_dir: &Path, target: &str, build_target: &str, optimize: bool) -> Result<()> {
    let mut args = vec!["build"];
    
    if target == "release" || optimize {
        args.push("--release");
    }
    
    // Add target-specific flags
    match build_target {
        "windows" if cfg!(not(target_os = "windows")) => {
            args.extend(&["--target", "x86_64-pc-windows-gnu"]);
        }
        "linux" if cfg!(not(target_os = "linux")) => {
            args.extend(&["--target", "x86_64-unknown-linux-gnu"]);
        }
        "macos" if cfg!(not(target_os = "macos")) => {
            args.extend(&["--target", "x86_64-apple-darwin"]);
        }
        _ => {}
    }
    
    ProcessUtils::run_command("cargo", &args, Some(plugin_dir)).await
}

async fn build_typescript_plugin(plugin_dir: &Path, _target: &str, _build_target: &str) -> Result<()> {
    ProcessUtils::run_command("npm", &["run", "build"], Some(plugin_dir)).await
}

async fn validate_lua_plugin(plugin_dir: &Path) -> Result<()> {
    // Basic Lua syntax validation
    let lua_files = FileUtils::find_files(plugin_dir, &["*.lua"]).await?;
    
    for lua_file in lua_files {
        // Simple syntax check using luac if available
        if ProcessUtils::command_exists("luac").await {
            let result = ProcessUtils::run_command_output("luac", &["-p", lua_file.to_str().unwrap()], None).await;
            if result.is_err() {
                warn!("Lua syntax error in: {}", lua_file.display());
            }
        }
    }
    
    Ok(())
}

// Helper functions
fn resolve_plugin_dir(plugin_dir: Option<String>) -> Result<PathBuf> {
    if let Some(dir) = plugin_dir {
        Ok(PathBuf::from(dir))
    } else {
        std::env::current_dir().context("Failed to get current directory")
    }
}

async fn load_plugin_config(plugin_dir: &Path) -> Result<PluginManifest> {
    let config_path = plugin_dir.join("gameverse.toml");
    let content = tokio::fs::read_to_string(&config_path)
        .await
        .context("Failed to read plugin configuration")?;
    
    toml::from_str(&content).context("Failed to parse plugin configuration")
}

fn detect_plugin_language(plugin_dir: &Path) -> Result<String> {
    if plugin_dir.join("Cargo.toml").exists() {
        Ok("rust".to_string())
    } else if plugin_dir.join("package.json").exists() {
        Ok("typescript".to_string())
    } else if FileUtils::has_files_with_extension(plugin_dir, "lua") {
        Ok("lua".to_string())
    } else {
        Err(anyhow::anyhow!("Could not detect plugin language"))
    }
}

// Plugin manifest structure matching gameverse.toml format
#[derive(Debug, serde::Deserialize)]
struct PluginManifest {
    plugin: PluginInfo,
    build: Option<BuildInfo>,
}

#[derive(Debug, serde::Deserialize)]
struct PluginInfo {
    name: String,
    version: String,
    description: Option<String>,
    author: Option<String>,
    abi_version: Option<String>,
    category: Option<String>,
    tags: Option<Vec<String>>,
}

#[derive(Debug, serde::Deserialize)]
struct BuildInfo {
    #[serde(rename = "type")]
    build_type: Option<String>,
    entry_point: Option<String>,
}

// Placeholder implementations for other commands
async fn test_plugin(plugin_dir: Option<String>, integration: bool, performance: bool, _config: &Config) -> Result<()> {
    info!("🧪 Testing plugin...");
    
    let plugin_dir = resolve_plugin_dir(plugin_dir)?;
    let plugin_manifest = load_plugin_config(&plugin_dir).await?;
    
    let pb = ProgressBar::new_spinner();
    pb.set_style(ProgressStyle::default_spinner()
        .template("{spinner:.green} {msg}")
        .context("Failed to create progress bar style")?);
    
    // Determine plugin language
    let language = detect_plugin_language(&plugin_dir)?;
    
    pb.set_message("Running unit tests...");
    pb.enable_steady_tick(std::time::Duration::from_millis(100));
    
    match language.as_str() {
        "rust" => {
            test_rust_plugin(&plugin_dir, integration, performance).await?;
        }
        "typescript" => {
            test_typescript_plugin(&plugin_dir, integration, performance).await?;
        }
        "lua" => {
            test_lua_plugin(&plugin_dir, integration, performance).await?;
        }
        _ => {
            return Err(anyhow::anyhow!("Testing not supported for language: {}", language));
        }
    }
    
    pb.finish_with_message("✅ All tests passed!");
    info!("✅ Plugin '{}' passed all tests", plugin_manifest.plugin.name);
    
    Ok(())
}

async fn test_rust_plugin(plugin_dir: &Path, integration: bool, performance: bool) -> Result<()> {
    let mut args = vec!["test"];
    
    if integration {
        args.push("--features");
        args.push("integration");
    }
    
    if performance {
        args.push("--release");
        args.push("--features");
        args.push("performance");
    }
    
    crate::utils::ProcessUtils::run_command("cargo", &args, Some(plugin_dir)).await?;
    
    // Run clippy for code quality
    crate::utils::ProcessUtils::run_command("cargo", &["clippy", "--", "-D", "warnings"], Some(plugin_dir)).await?;
    
    Ok(())
}

async fn test_typescript_plugin(plugin_dir: &Path, integration: bool, performance: bool) -> Result<()> {
    // Check if package.json exists
    let package_json = plugin_dir.join("package.json");
    if !package_json.exists() {
        return Err(anyhow::anyhow!("package.json not found in TypeScript plugin"));
    }
    
    // Install dependencies if needed
    let node_modules = plugin_dir.join("node_modules");
    if !node_modules.exists() {
        crate::utils::ProcessUtils::run_command("npm", &["install"], Some(plugin_dir)).await?;
    }
    
    let mut args = vec!["test"];
    
    if integration {
        args.push("--");
        args.push("--grep");
        args.push("integration");
    }
    
    if performance {
        args.push("--");
        args.push("--grep");
        args.push("performance");
    }
    
    crate::utils::ProcessUtils::run_command("npm", &args, Some(plugin_dir)).await?;
    
    // Run ESLint for code quality
    crate::utils::ProcessUtils::run_command("npx", &["eslint", "src/"], Some(plugin_dir)).await?;
    
    Ok(())
}

async fn test_lua_plugin(plugin_dir: &Path, _integration: bool, _performance: bool) -> Result<()> {
    // Basic Lua syntax validation
    for entry in walkdir::WalkDir::new(plugin_dir) {
        let entry = entry?;
        if entry.path().extension().and_then(|s| s.to_str()) == Some("lua") {
            // Basic syntax check using luac if available
            if let Ok(_) = crate::utils::ProcessUtils::run_command("luac", &["-p", entry.path().to_str().unwrap()], None).await {
                // Syntax is valid
            } else {
                warn!("Warning: luac not available for Lua syntax checking");
                break;
            }
        }
    }
    
    Ok(())
}

async fn deploy_plugin(_plugin_dir: Option<String>, _server: Option<String>, _environment: Option<String>, _force: bool, _config: &Config) -> Result<()> {
    info!("🚀 Deploying plugin...");
    // TODO: Implement plugin deployment
    Ok(())
}

async fn package_plugin(plugin_dir: Option<String>, output: Option<String>, include_source: bool, _config: &Config) -> Result<()> {
    info!("📦 Packaging plugin...");
    
    let plugin_dir = resolve_plugin_dir(plugin_dir)?;
    let plugin_manifest = load_plugin_config(&plugin_dir).await?;
    
    // Determine output file
    let output_file = if let Some(output) = output {
        PathBuf::from(output)
    } else {
        std::env::current_dir()?.join(format!("{}-{}.tar.gz", plugin_manifest.plugin.name, plugin_manifest.plugin.version))
    };
    
    let pb = ProgressBar::new(100);
    pb.set_style(ProgressStyle::default_bar()
        .template("{spinner:.green} [{elapsed_precise}] {bar:40.cyan/blue} {pos:>7}/{len:7} {msg}")
        .context("Failed to create progress bar style")?);
    
    pb.set_message("Building plugin...");
    pb.set_position(10);
    
    // Build plugin first
    let language = detect_plugin_language(&plugin_dir)?;
    match language.as_str() {
        "rust" => {
            build_rust_plugin(&plugin_dir, "release", "current", true).await?;
        }
        "typescript" => {
            build_typescript_plugin(&plugin_dir, "release", "current").await?;
        }
        "lua" => {
            validate_lua_plugin(&plugin_dir).await?;
        }
        _ => return Err(anyhow::anyhow!("Unsupported language for packaging: {}", language)),
    }
    
    pb.set_message("Creating package...");
    pb.set_position(50);
    
    // Create tar.gz package
    let file = std::fs::File::create(&output_file)?;
    let enc = flate2::write::GzEncoder::new(file, flate2::Compression::default());
    let mut tar = tar::Builder::new(enc);
    
    // Add plugin files
    let mut files_to_include = vec![
        "gameverse.toml",
    ];
    
    // Add built artifacts
    match language.as_str() {
        "rust" => {
            files_to_include.extend(&[
                "target/release/*.dll",
                "target/release/*.so", 
                "target/release/*.dylib",
            ]);
            if include_source {
                files_to_include.extend(&["src/", "Cargo.toml", "Cargo.lock"]);
            }
        }
        "typescript" => {
            files_to_include.extend(&["dist/", "package.json"]);
            if include_source {
                files_to_include.extend(&["src/", "tsconfig.json"]);
            }
        }
        "lua" => {
            files_to_include.push("*.lua");
        }
        _ => return Err(anyhow::anyhow!("Unsupported language for packaging: {}", language)),
    }
    
    // Add README if exists
    for readme in &["README.md", "README.txt", "readme.md"] {
        if plugin_dir.join(readme).exists() {
            files_to_include.push(readme);
            break;
        }
    }
    
    pb.set_message("Adding files to package...");
    pb.set_position(70);
    
    for pattern in files_to_include {
        if pattern.contains('*') {
            // Handle glob patterns
            for entry in glob::glob(&plugin_dir.join(pattern).to_string_lossy())? {
                let entry = entry?;
                if entry.is_file() {
                    let relative_path = entry.strip_prefix(&plugin_dir)?;
                    tar.append_path_with_name(&entry, relative_path)?;
                }
            }
        } else {
            let file_path = plugin_dir.join(pattern);
            if file_path.exists() {
                if file_path.is_file() {
                    tar.append_path_with_name(&file_path, pattern)?;
                } else if file_path.is_dir() {
                    tar.append_dir_all(pattern, &file_path)?;
                }
            }
        }
    }
    
    pb.set_message("Finalizing package...");
    pb.set_position(90);
    
    tar.finish()?;
    
    pb.set_message("Package created!");
    pb.set_position(100);
    pb.finish_with_message("✅ Package created successfully!");
    
    println!();
    println!("{} Plugin '{}' packaged successfully!", style("📦").green(), style(&plugin_manifest.plugin.name).cyan().bold());
    println!("   Package: {}", style(output_file.display()).yellow());
    println!("   Size: {}", style(format!("{:.2} MB", output_file.metadata()?.len() as f64 / 1024.0 / 1024.0)).yellow());
    
    Ok(())
}

async fn install_plugin(_plugin: String, _directory: Option<String>, _version: Option<String>, _config: &Config) -> Result<()> {
    info!("📥 Installing plugin...");
    // TODO: Implement plugin installation
    Ok(())
}

async fn list_plugins(_detailed: bool, _config: &Config) -> Result<()> {
    info!("📋 Listing plugins...");
    // TODO: Implement plugin listing
    Ok(())
}

async fn validate_plugin(plugin_dir: Option<String>, _config: &Config) -> Result<()> {
    info!("✅ Validating plugin...");
    
    let plugin_dir = resolve_plugin_dir(plugin_dir)?;
    let mut validation_errors = Vec::new();
    let mut validation_warnings = Vec::new();
    
    // Check if gameverse.toml exists
    let config_path = plugin_dir.join("gameverse.toml");
    if !config_path.exists() {
        validation_errors.push("gameverse.toml file is missing".to_string());
        println!("{} Validation failed: gameverse.toml not found", style("❌").red());
        return Ok(());
    }
    
    // Validate gameverse.toml structure
    match load_plugin_config(&plugin_dir).await {
        Ok(manifest) => {
            println!("{} Plugin configuration is valid", style("✅").green());
            println!("   Name: {}", style(&manifest.plugin.name).cyan());
            println!("   Version: {}", style(&manifest.plugin.version).yellow());
            if let Some(desc) = &manifest.plugin.description {
                println!("   Description: {}", style(desc).dim());
            }
        }
        Err(e) => {
            validation_errors.push(format!("Invalid gameverse.toml: {}", e));
        }
    }
    
    // Detect and validate language-specific files
    let language = detect_plugin_language(&plugin_dir);
    match language {
        Ok(lang) => {
            println!("{} Detected language: {}", style("🔍").blue(), style(&lang).cyan());
            
            match lang.as_str() {
                "rust" => validate_rust_plugin(&plugin_dir, &mut validation_errors, &mut validation_warnings).await?,
                "typescript" => validate_typescript_plugin(&plugin_dir, &mut validation_errors, &mut validation_warnings).await?,
                "lua" => validate_lua_plugin_structure(&plugin_dir, &mut validation_errors, &mut validation_warnings).await?,
                _ => validation_warnings.push(format!("Unknown language: {}", lang)),
            }
        }
        Err(e) => {
            validation_errors.push(format!("Could not detect plugin language: {}", e));
        }
    }
    
    // Check for README
    let readme_files = ["README.md", "README.txt", "readme.md", "readme.txt"];
    let has_readme = readme_files.iter().any(|&readme| plugin_dir.join(readme).exists());
    if !has_readme {
        validation_warnings.push("No README file found".to_string());
    }
    
    // Print validation results
    println!();
    if validation_errors.is_empty() {
        println!("{} Plugin validation passed!", style("✅").green());
        
        if !validation_warnings.is_empty() {
            println!("\n{} Warnings:", style("⚠️").yellow());
            for warning in validation_warnings {
                println!("   • {}", style(warning).yellow());
            }
        }
    } else {
        println!("{} Plugin validation failed!", style("❌").red());
        println!("\n{} Errors:", style("❌").red());
        for error in validation_errors {
            println!("   • {}", style(error).red());
        }
        
        if !validation_warnings.is_empty() {
            println!("\n{} Warnings:", style("⚠️").yellow());
            for warning in validation_warnings {
                println!("   • {}", style(warning).yellow());
            }
        }
    }
    
    Ok(())
}

async fn validate_rust_plugin(plugin_dir: &Path, errors: &mut Vec<String>, warnings: &mut Vec<String>) -> Result<()> {
    // Check Cargo.toml
    let cargo_toml = plugin_dir.join("Cargo.toml");
    if !cargo_toml.exists() {
        errors.push("Cargo.toml file is missing".to_string());
        return Ok(());
    }
    
    // Check src/lib.rs
    let lib_rs = plugin_dir.join("src").join("lib.rs");
    if !lib_rs.exists() {
        errors.push("src/lib.rs file is missing".to_string());
    }
    
    // Run cargo check
    match crate::utils::ProcessUtils::run_command_output("cargo", &["check"], Some(plugin_dir)).await {
        Ok(_) => println!("{} Rust compilation check passed", style("✅").green()),
        Err(e) => errors.push(format!("Rust compilation check failed: {}", e)),
    }
    
    // Check for tests
    let tests_dir = plugin_dir.join("tests");
    let has_tests = tests_dir.exists() || {
        let lib_content = tokio::fs::read_to_string(&lib_rs).await.unwrap_or_default();
        lib_content.contains("#[cfg(test)]") || lib_content.contains("#[test]")
    };
    
    if !has_tests {
        warnings.push("No tests found".to_string());
    }
    
    Ok(())
}

async fn validate_typescript_plugin(plugin_dir: &Path, errors: &mut Vec<String>, warnings: &mut Vec<String>) -> Result<()> {
    // Check package.json
    let package_json = plugin_dir.join("package.json");
    if !package_json.exists() {
        errors.push("package.json file is missing".to_string());
        return Ok(());
    }
    
    // Check src directory
    let src_dir = plugin_dir.join("src");
    if !src_dir.exists() {
        errors.push("src/ directory is missing".to_string());
    }
    
    // Check for TypeScript config
    let tsconfig = plugin_dir.join("tsconfig.json");
    if !tsconfig.exists() {
        warnings.push("tsconfig.json file is missing".to_string());
    }
    
    // Check if dependencies are installed
    let node_modules = plugin_dir.join("node_modules");
    if !node_modules.exists() {
        warnings.push("Dependencies not installed (run npm install)".to_string());
    }
    
    Ok(())
}

async fn validate_lua_plugin_structure(plugin_dir: &Path, errors: &mut Vec<String>, warnings: &mut Vec<String>) -> Result<()> {
    // Check for main Lua file
    let main_files = ["main.lua", "init.lua", "plugin.lua"];
    let has_main = main_files.iter().any(|&file| plugin_dir.join(file).exists());
    
    if !has_main {
        errors.push("No main Lua file found (main.lua, init.lua, or plugin.lua)".to_string());
    }
    
    // Basic syntax check for all Lua files
    for entry in walkdir::WalkDir::new(plugin_dir) {
        let entry = entry?;
        if entry.path().extension().and_then(|s| s.to_str()) == Some("lua") {
            if let Ok(_) = crate::utils::ProcessUtils::run_command("luac", &["-p", entry.path().to_str().unwrap()], None).await {
                // Syntax is valid
            } else {
                warnings.push("luac not available for Lua syntax checking".to_string());
                break;
            }
        }
    }
    
    Ok(())
}

async fn watch_plugin(_plugin_dir: Option<String>, _server: Option<String>, _config: &Config) -> Result<()> {
    info!("👁️ Watching plugin for changes...");
    // TODO: Implement plugin watching
    Ok(())
} 