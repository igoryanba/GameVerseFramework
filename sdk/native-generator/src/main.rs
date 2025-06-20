use anyhow::Result;
use clap::{Parser, Subcommand};
use tracing::{info};
use std::path::PathBuf;

mod fivem_parser;
mod rust_generator;
mod typescript_generator;
mod native_types;
mod utils;
mod rdr2_parser;

use fivem_parser::FiveMDocParser;
use rust_generator::RustWrapperGenerator;
use typescript_generator::TypeScriptGenerator;

/// GameVerse Native Functions Generator - Automatic FiveM compatibility layer
#[derive(Parser)]
#[command(name = "generate-natives")]
#[command(about = "Generate type-safe wrappers for FiveM native functions")]
#[command(version = "0.1.0")]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    /// Generate native function wrappers from FiveM documentation
    Generate {
        /// Source of FiveM documentation (URL or local path)
        #[arg(long, default_value = "https://docs.fivem.net/natives/")]
        source: String,
        
        /// Target game: gta5 (FiveM) | rdr2 (RedM)
        #[arg(long, value_enum, default_value_t = GameTarget::Gta5)]
        game: GameTarget,
        
        /// Optional path to a local directory containing FiveM natives documentation (e.g., a clone of natives-master)
        /// If provided, this will be used instead of the --source URL for categories found locally.
        #[arg(long)]
        local_natives_path: Option<PathBuf>,
        
        /// Optional path to a local directory containing RDR2 natives documentation (Markdown from rdr3-nativedb)
        #[arg(long)]
        rdr2_local_path: Option<PathBuf>,
        
        /// Optional path to a TOML file with native function overrides and configurations.
        #[arg(long)]
        native_configs_path: Option<PathBuf>,
        
        /// Target language for generation
        #[arg(long, value_enum)]
        target: TargetLanguage,
        
        /// Output directory for generated files
        #[arg(long, short, default_value = "./generated")]
        output: PathBuf,
        
        /// Generate only specific categories
        #[arg(long)]
        categories: Option<Vec<String>>,
        
        /// Enable validation of generated wrappers
        #[arg(long)]
        validate: bool,
        
        /// Generate VS Code IntelliSense data
        #[arg(long)]
        intellisense: bool,
    },
    
    /// Validate existing native function wrappers
    Validate {
        /// Path to generated wrappers
        #[arg(long, short)]
        path: PathBuf,
        
        /// Category to validate
        #[arg(long)]
        category: Option<String>,
    },
    
    /// Test generated wrappers against FiveM behavior
    Test {
        /// Path to generated wrappers
        #[arg(long, short)]
        path: PathBuf,
        
        /// Run comprehensive tests
        #[arg(long)]
        comprehensive: bool,
    },
    
    /// Update local FiveM documentation cache
    Update {
        /// Force update even if cache is fresh
        #[arg(long)]
        force: bool,
    },
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum TargetLanguage {
    Rust,
    TypeScript,
    Both,
}

#[derive(clap::ValueEnum, Clone, Debug)]
enum GameTarget {
    Gta5,
    Rdr2,
}

impl Default for GameTarget {
    fn default() -> Self { GameTarget::Gta5 }
}

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    let cli = Cli::parse();
    
    match cli.command {
        Commands::Generate { 
            source, 
            game,
            local_natives_path,
            rdr2_local_path,
            native_configs_path,
            target, 
            output, 
            categories, 
            validate, 
            intellisense 
        } => {
            info!("🚀 Starting native function generation from: {}", source);
            if let Some(local_path) = &local_natives_path {
                info!("📁 Using local natives path: {}", local_path.display());
            }
            if let Some(rdr2_path) = &rdr2_local_path {
                info!("📁 Using local RDR2 natives path: {}", rdr2_path.display());
            }
            if let Some(config_path_val) = &native_configs_path {
                info!("⚙️ Using native configs from: {}", config_path_val.display());
            } else {
                info!("⚙️ No native_configs.toml path provided. Proceeding without overrides.");
            }
            let parser_choice = match game {
                GameTarget::Gta5 => {
                    info!("🎮 Target game: GTA V / FiveM");
                    EitherParser::FiveM(FiveMDocParser::new(local_natives_path.clone(), native_configs_path.clone()))
                },
                GameTarget::Rdr2 => {
                    info!("🎮 Target game: RDR2 / RedM");
                    let path_for_rdr2 = rdr2_local_path.clone();
                    EitherParser::Rdr2(crate::rdr2_parser::Rdr2DocParser::new(path_for_rdr2.or(local_natives_path.clone()), native_configs_path.clone()))
                }
            };
            
            info!("📖 Determining categories and parsing documentation...");
            let natives = parser_choice.parse(&source, categories.as_ref(), local_natives_path.as_ref()).await?;
            
            info!("✅ Parsed {} native functions in {} categories", 
                  natives.total_functions(), natives.categories().len());
            
            // Filter by categories if specified
            let filtered_natives = if let Some(cats) = categories {
                // Если указан "ALL", не фильтруем
                if cats.len() == 1 && cats[0].to_uppercase() == "ALL" {
                    natives
                } else {
                    natives.filter_categories(&cats)
                }
            } else {
                natives
            };
            
            // Generate wrappers based on target language
            match target {
                TargetLanguage::Rust => {
                    info!("🦀 Generating Rust wrappers...");
                    let generator = RustWrapperGenerator::new();
                    generator.generate_all(&filtered_natives, &output.join("rust"))?;
                }
                TargetLanguage::TypeScript => {
                    info!("📘 Generating TypeScript definitions...");
                    let generator = TypeScriptGenerator::new();
                    generator.generate_all(&filtered_natives, &output.join("typescript"))?;
                }
                TargetLanguage::Both => {
                    info!("🔄 Generating both Rust and TypeScript...");
                    let rust_gen = RustWrapperGenerator::new();
                    let ts_gen = TypeScriptGenerator::new();
                    
                    rust_gen.generate_all(&filtered_natives, &output.join("rust"))?;
                    ts_gen.generate_all(&filtered_natives, &output.join("typescript"))?;
                }
            }
            
            // Generate VS Code IntelliSense data
            if intellisense {
                info!("🧠 Generating VS Code IntelliSense data...");
                utils::generate_vscode_intellisense(&filtered_natives, &output.join("vscode"))?;
            }
            
            // Validate generated wrappers
            if validate {
                info!("✅ Validating generated wrappers...");
                utils::validate_generated_wrappers(&output)?;
            }
            
            info!("🎉 Native function generation completed successfully!");
            info!("📂 Output directory: {}", output.display());
            info!("📊 Generated wrappers for {} functions", filtered_natives.total_functions());
        }
        
        Commands::Validate { path, category } => {
            info!("🔍 Validating native function wrappers at: {}", path.display());
            utils::validate_wrappers_at_path(&path, category.as_deref())?;
            info!("✅ Validation completed");
        }
        
        Commands::Test { path, comprehensive } => {
            info!("🧪 Testing native function wrappers at: {}", path.display());
            utils::test_wrappers(&path, comprehensive)?;
            info!("✅ Testing completed");
        }
        
        Commands::Update { force } => {
            info!("🔄 Updating FiveM native documentation cache...");
            let parser = FiveMDocParser::new(None, None);
            parser.update_cache(force).await?;
            info!("✅ Documentation cache updated");
        }
    }
    
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[tokio::test]
    async fn test_cli_parsing() {
        // Basic CLI parsing tests
        let cli = Cli::try_parse_from(&[
            "generate-natives",
            "generate",
            "--target", "rust",
            "--output", "/tmp/test"
        ]).unwrap();
        
        match cli.command {
            Commands::Generate { target, output, .. } => {
                assert!(matches!(target, TargetLanguage::Rust));
                assert_eq!(output, PathBuf::from("/tmp/test"));
            }
            _ => panic!("Expected Generate command"),
        }
    }
    
    #[test]
    fn test_target_language_parsing() {
        use clap::ValueEnum;
        
        assert!(matches!(TargetLanguage::from_str("rust", true), Ok(TargetLanguage::Rust)));
        assert!(matches!(TargetLanguage::from_str("type-script", true), Ok(TargetLanguage::TypeScript)));
        assert!(matches!(TargetLanguage::from_str("both", true), Ok(TargetLanguage::Both)));
    }
}

enum EitherParser {
    FiveM(FiveMDocParser),
    Rdr2(crate::rdr2_parser::Rdr2DocParser),
}

impl EitherParser {
    async fn parse(&self, source: &str, cats: Option<&Vec<String>>, local_natives_path: Option<&std::path::PathBuf>) -> anyhow::Result<crate::native_types::NativeCollection> {
        match self {
            EitherParser::FiveM(p) => {
                if source == "local" && local_natives_path.is_some() {
                    // Use local parsing method when source is "local" and path is provided
                    p.parse_from_local_only(local_natives_path.unwrap().to_str().unwrap(), cats)
                } else {
                    p.parse_from_url(source, cats).await
                }
            },
            EitherParser::Rdr2(p) => {
                if source == "local" && local_natives_path.is_some() {
                    // Use local parsing method when source is "local" and path is provided
                    p.parse_from_local_only(local_natives_path.unwrap().to_str().unwrap(), cats)
                } else {
                    p.parse_from_url(source, cats).await
                }
            },
        }
    }
} 