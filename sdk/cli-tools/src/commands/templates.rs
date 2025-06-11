//! Template management commands

use clap::Subcommand;
use anyhow::Result;
use console::style;
use tracing::info;

use crate::config::Config;
use crate::templates::TemplateManager;

#[derive(Subcommand)]
pub enum TemplatesCommands {
    /// List available templates
    List {
        /// Show detailed information
        #[arg(short, long)]
        detailed: bool,
        /// Filter by category
        #[arg(long)]
        category: Option<String>,
    },
    
    /// Show template information
    Info {
        /// Template name
        name: String,
    },
    
    /// Update template repository
    Update,
    
    /// Validate template
    Validate {
        /// Template directory path
        path: String,
    },
    
    /// Create custom template from existing project
    Create {
        /// Source project directory
        source: String,
        /// Template name
        name: String,
    },
}

pub async fn execute(cmd: TemplatesCommands, config: &Config) -> Result<()> {
    match cmd {
        TemplatesCommands::List { detailed, category } => {
            list_templates(detailed, category, config).await
        }
        TemplatesCommands::Info { name } => {
            show_template_info(name, config).await
        }
        TemplatesCommands::Update => {
            update_templates(config).await
        }
        TemplatesCommands::Validate { path } => {
            validate_template(path, config).await
        }
        TemplatesCommands::Create { source, name } => {
            create_template(source, name, config).await
        }
    }
}

async fn list_templates(detailed: bool, category: Option<String>, config: &Config) -> Result<()> {
    info!("📋 Listing available templates...");
    
    let template_manager = TemplateManager::new(&config.templates);
    let templates = template_manager.list_templates().await?;
    
    let filtered_templates: Vec<_> = if let Some(cat) = category {
        templates.into_iter()
            .filter(|t| t.category.as_ref().map_or(false, |c| c == &cat))
            .collect()
    } else {
        templates
    };
    
    if filtered_templates.is_empty() {
        println!("{} No templates found", style("ℹ️").blue());
        return Ok(());
    }
    
    println!("{} Available Templates:", style("📋").green());
    println!();
    
    for template in filtered_templates {
        println!("{} {}", style("•").cyan(), style(&template.name).bold());
        println!("  {}", style(&template.description).dim());
        
        if detailed {
            println!("  Author: {}", style(&template.author).yellow());
            println!("  Version: {}", style(&template.version).yellow());
            println!("  GameVerse: {}", style(&template.gameverse_version).yellow());
            
            if let Some(category) = &template.category {
                println!("  Category: {}", style(category).magenta());
            }
            
            if let Some(tags) = &template.tags {
                println!("  Tags: {}", style(tags.join(", ")).dim());
            }
            
            println!("  Languages: {}", style(template.languages.join(", ")).cyan());
            
            if !template.features.is_empty() {
                println!("  Features: {}", style(template.features.join(", ")).green());
            }
        }
        
        println!();
    }
    
    println!("Use {} to see detailed information about a template", 
             style("gameverse templates info <name>").cyan());
    
    Ok(())
}

async fn show_template_info(name: String, config: &Config) -> Result<()> {
    info!("🔍 Showing template information: {}", name);
    
    let template_manager = TemplateManager::new(&config.templates);
    let template_info = template_manager.get_template_info(&name).await?;
    
    println!("{} Template: {}", style("📄").blue(), style(&template_info.template.name).bold());
    println!();
    
    // Basic information
    println!("{}", style("Basic Information:").underlined());
    println!("  Description: {}", template_info.template.description);
    println!("  Author: {}", template_info.template.author);
    println!("  Version: {}", template_info.template.version);
    println!("  GameVerse Version: {}", template_info.template.gameverse_version);
    
    if let Some(category) = &template_info.template.category {
        println!("  Category: {}", category);
    }
    
    if let Some(tags) = &template_info.template.tags {
        println!("  Tags: {}", tags.join(", "));
    }
    
    println!();
    
    // Languages
    println!("{}", style("Supported Languages:").underlined());
    for (lang, config) in &template_info.languages {
        println!("  {} {}", style("•").cyan(), style(lang).bold());
        if let Some(min_version) = &config.min_version {
            println!("    Min Version: {}", min_version);
        }
        if let Some(features) = &config.features {
            println!("    Features: {}", features.join(", "));
        }
        if let Some(frameworks) = &config.frameworks {
            println!("    Frameworks: {}", frameworks.join(", "));
        }
    }
    
    println!();
    
    // Variables
    if !template_info.variables.is_empty() {
        println!("{}", style("Template Variables:").underlined());
        for (name, var_config) in &template_info.variables {
            println!("  {} {}", style("•").cyan(), style(name).bold());
            println!("    Description: {}", var_config.description);
            println!("    Type: {}", var_config.var_type);
            if let Some(default) = &var_config.default {
                println!("    Default: {}", default);
            }
            if let Some(validation) = &var_config.validation {
                println!("    Validation: {}", validation);
            }
        }
        println!();
    }
    
    // Features
    if let Some(features) = &template_info.features {
        println!("{}", style("Features:").underlined());
        for (feature, enabled) in features {
            let status = if *enabled { style("✅").green() } else { style("❌").red() };
            println!("  {} {}", status, feature);
        }
        println!();
    }
    
    // Dependencies
    if let Some(dependencies) = &template_info.dependencies {
        println!("{}", style("Dependencies:").underlined());
        for (name, version) in dependencies {
            println!("  {} {}: {}", style("•").cyan(), name, version);
        }
        println!();
    }
    
    println!("Use {} to create a plugin with this template", 
             style(format!("gameverse plugin new <name> --template {}", name)).cyan());
    
    Ok(())
}

async fn update_templates(config: &Config) -> Result<()> {
    info!("🔄 Updating template repository...");
    
    let template_manager = TemplateManager::new(&config.templates);
    template_manager.download_templates().await?;
    
    println!("{} Template repository updated successfully!", style("✅").green());
    
    Ok(())
}

async fn validate_template(path: String, config: &Config) -> Result<()> {
    info!("✅ Validating template: {}", path);
    
    let template_manager = TemplateManager::new(&config.templates);
    let template_path = std::path::Path::new(&path);
    
    let result = template_manager.validate_template(template_path).await?;
    
    if result.valid {
        println!("{} Template validation passed!", style("✅").green());
        
        if !result.warnings.is_empty() {
            println!("\n{} Warnings:", style("⚠️").yellow());
            for warning in result.warnings {
                println!("   • {}", style(warning).yellow());
            }
        }
    } else {
        println!("{} Template validation failed!", style("❌").red());
        
        if !result.errors.is_empty() {
            println!("\n{} Errors:", style("❌").red());
            for error in result.errors {
                println!("   • {}", style(error).red());
            }
        }
        
        if !result.warnings.is_empty() {
            println!("\n{} Warnings:", style("⚠️").yellow());
            for warning in result.warnings {
                println!("   • {}", style(warning).yellow());
            }
        }
    }
    
    Ok(())
}

async fn create_template(source: String, name: String, config: &Config) -> Result<()> {
    info!("🔧 Creating custom template: {} from {}", name, source);
    
    let template_manager = TemplateManager::new(&config.templates);
    let source_path = std::path::Path::new(&source);
    
    if !source_path.exists() {
        return Err(anyhow::anyhow!("Source directory does not exist: {}", source));
    }
    
    template_manager.create_custom_template(source_path, &name).await?;
    
    println!("{} Custom template '{}' created successfully!", style("✅").green(), style(&name).cyan());
    println!("   Source: {}", style(source).yellow());
    println!("   Template: {}", style(&name).cyan());
    
    Ok(())
} 