//! Template management for GameVerse plugins

use anyhow::{Result, Context};
use std::path::{Path, PathBuf};
use std::collections::HashMap;
use serde::{Deserialize, Serialize};
use handlebars::Handlebars;
use walkdir::WalkDir;
use chrono::Datelike;

use crate::config::{Config, TemplateConfig};

pub struct TemplateManager {
    config: TemplateConfig,
    handlebars: Handlebars<'static>,
    templates_dir: PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateMetadata {
    pub name: String,
    pub description: String,
    pub languages: Vec<String>,
    pub author: String,
    pub version: String,
    pub gameverse_version: String,
    pub features: Vec<String>,
    pub variables: HashMap<String, TemplateVariable>,
    pub dependencies: Option<HashMap<String, String>>,
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateInfo {
    pub template: TemplateMetadata,
    pub languages: HashMap<String, LanguageConfig>,
    pub variables: HashMap<String, VariableConfig>,
    pub dependencies: Option<HashMap<String, String>>,
    pub features: Option<HashMap<String, bool>>,
    pub structure: Option<HashMap<String, Vec<String>>>,
}

// Новая структура для парсинга файла template.toml напрямую
#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateFile {
    pub template: TemplateSection,
    pub languages: HashMap<String, LanguageConfig>,
    pub variables: HashMap<String, VariableConfig>,
    pub dependencies: Option<HashMap<String, String>>,
    pub features: Option<HashMap<String, bool>>,
    pub structure: Option<HashMap<String, Vec<String>>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateSection {
    pub name: String,
    pub description: String,
    pub author: String,
    pub version: String,
    pub gameverse_version: String,
    pub category: Option<String>,
    pub tags: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguageConfig {
    pub min_version: Option<String>,
    pub features: Option<Vec<String>>,
    pub frameworks: Option<Vec<String>>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct VariableConfig {
    pub description: String,
    #[serde(rename = "type")]
    pub var_type: String,
    pub default: Option<serde_json::Value>,
    pub validation: Option<String>,
    pub transform: Option<String>,
    pub min: Option<f64>,
    pub max: Option<f64>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct TemplateVariable {
    pub description: String,
    pub default: Option<String>,
    pub required: bool,
    pub var_type: String, // string, number, boolean
}

#[derive(Debug, Serialize)]
struct TemplateContext {
    plugin_name: String,
    plugin_author: String,
    plugin_license: String,
    plugin_version: String,
    plugin_description: String,
    language: String,
    current_year: i32,
    custom_vars: HashMap<String, String>,
}

impl TemplateManager {
    pub fn new(config: &TemplateConfig) -> Self {
        let templates_dir = if let Some(local_dir) = &config.local_dir {
            // Если путь относительный, сделать его относительно текущей рабочей директории
            if local_dir.is_relative() {
                std::env::current_dir()
                    .unwrap_or_else(|_| PathBuf::from("."))
                    .join(local_dir)
            } else {
                local_dir.clone()
            }
        } else {
                dirs::cache_dir()
                    .unwrap_or_else(|| PathBuf::from("."))
                    .join("gameverse")
                    .join("templates")
        };

        let mut handlebars = Handlebars::new();
        handlebars.set_strict_mode(false);
        
        // Register built-in helpers
        handlebars.register_helper("upper", Box::new(uppercase_helper));
        handlebars.register_helper("lower", Box::new(lowercase_helper));
        handlebars.register_helper("snake_case", Box::new(snake_case_helper));
        handlebars.register_helper("kebab_case", Box::new(kebab_case_helper));
        handlebars.register_helper("camelcase", Box::new(camelcase_helper));

        Self {
            config: config.clone(),
            handlebars,
            templates_dir,
        }
    }

    /// Получить информацию о конкретном шаблоне
    pub async fn get_template_info(&self, template_name: &str) -> Result<TemplateInfo> {
        self.download_templates().await?;

        let templates_path = self.templates_dir.clone();

        let template_dir = templates_path.join(template_name);
        if !template_dir.exists() {
            return Err(anyhow::anyhow!("Template '{}' not found", template_name));
        }

        let metadata_file = template_dir.join("template.toml");
        if !metadata_file.exists() {
            return Err(anyhow::anyhow!("Template metadata not found for '{}'", template_name));
        }

        let content = tokio::fs::read_to_string(&metadata_file).await?;
        
        // Try to parse as TemplateFile first (new format)
        if let Ok(template_file) = toml::from_str::<TemplateFile>(&content) {
            let template_info = TemplateInfo {
                template: TemplateMetadata {
                    name: template_file.template.name.clone(),
                    description: template_file.template.description,
                    author: template_file.template.author,
                    version: template_file.template.version,
                    gameverse_version: template_file.template.gameverse_version,
                    category: template_file.template.category,
                    tags: template_file.template.tags,
                    languages: template_file.languages.keys().cloned().collect(),
                    features: template_file.features.as_ref().map(|f| 
                        f.iter().filter(|(_, &enabled)| enabled).map(|(name, _)| name.clone()).collect()
                    ).unwrap_or_default(),
                    variables: HashMap::new(), // Will be converted from VariableConfig to TemplateVariable
                    dependencies: template_file.dependencies.clone(),
                },
                languages: template_file.languages,
                variables: template_file.variables,
                dependencies: template_file.dependencies,
                features: template_file.features,
                structure: template_file.structure,
            };
            Ok(template_info)
        } else {
            // Fallback to legacy format
            let template_info: TemplateInfo = toml::from_str(&content)
                .context(format!("Failed to parse template.toml for '{}'", template_name))?;
            Ok(template_info)
        }
    }

    /// Валидация шаблона
    pub async fn validate_template(&self, template_path: &Path) -> Result<ValidationResult> {
        let mut result = ValidationResult {
            valid: true,
            errors: Vec::new(),
            warnings: Vec::new(),
        };

        // Проверка существования template.toml
        let metadata_file = template_path.join("template.toml");
        if !metadata_file.exists() {
            result.valid = false;
            result.errors.push("template.toml file is missing".to_string());
            return Ok(result);
        }

        // Валидация структуры template.toml
        let content = tokio::fs::read_to_string(&metadata_file).await?;
        match toml::from_str::<TemplateInfo>(&content) {
            Ok(template_info) => {
                // Валидация переменных
                for (name, var_config) in &template_info.variables {
                    if let Some(validation) = &var_config.validation {
                        if regex::Regex::new(validation).is_err() {
                            result.warnings.push(format!("Invalid regex pattern for variable '{}': {}", name, validation));
                        }
                    }
                }

                // Проверка существования файлов для каждого языка
                if let Some(structure) = &template_info.structure {
                    for (language, files) in structure {
                        let lang_dir = template_path.join(language);
                        if !lang_dir.exists() {
                            result.warnings.push(format!("Language directory '{}' not found", language));
                        } else {
                            for file in files {
                                let file_path = lang_dir.join(file);
                                if !file_path.exists() && !file.ends_with('/') {
                                    result.warnings.push(format!("Template file '{}' not found for language '{}'", file, language));
                                }
                            }
                        }
                    }
                }
            }
            Err(e) => {
                result.valid = false;
                result.errors.push(format!("Invalid template.toml format: {}", e));
            }
        }

        Ok(result)
    }

    /// Получить версии шаблона
    pub async fn get_template_versions(&self, template_name: &str) -> Result<Vec<String>> {
        self.download_templates().await?;

        let template_dir = self.templates_dir.join(template_name);
        if !template_dir.exists() {
            return Err(anyhow::anyhow!("Template '{}' not found", template_name));
        }

        // Для простоты, возвращаем текущую версию
        // В будущем можно добавить поддержку git tags
        let metadata_file = template_dir.join("template.toml");
        if metadata_file.exists() {
            let content = tokio::fs::read_to_string(&metadata_file).await?;
            let template_info: TemplateInfo = toml::from_str(&content)?;
            Ok(vec![template_info.template.version])
        } else {
            Ok(vec!["latest".to_string()])
        }
    }

    /// Создать кастомный шаблон из существующего проекта
    pub async fn create_custom_template(&self, source_dir: &Path, template_name: &str) -> Result<()> {
        let template_dir = self.templates_dir.join("custom").join(template_name);
        tokio::fs::create_dir_all(&template_dir).await?;

        // Копирование файлов с заменой переменных
        self.copy_and_templateize(source_dir, &template_dir).await?;

        // Создание template.toml
        let template_info = TemplateInfo {
            template: TemplateMetadata {
                name: template_name.to_string(),
                description: format!("Custom template based on {}", source_dir.display()),
                languages: vec!["rust".to_string()], // По умолчанию
                author: "Custom".to_string(),
                version: "1.0.0".to_string(),
                gameverse_version: ">=0.2.0".to_string(),
                features: vec![],
                variables: HashMap::new(),
                dependencies: None,
                category: Some("custom".to_string()),
                tags: Some(vec!["custom".to_string()]),
            },
            languages: HashMap::new(),
            variables: HashMap::new(),
            dependencies: None,
            features: None,
            structure: None,
        };

        let metadata_content = toml::to_string_pretty(&template_info)?;
        tokio::fs::write(template_dir.join("template.toml"), metadata_content).await?;

        Ok(())
    }

    async fn copy_and_templateize(&self, source: &Path, dest: &Path) -> Result<()> {
        for entry in WalkDir::new(source) {
            let entry = entry?;
            let src_path = entry.path();
            let relative_path = src_path.strip_prefix(source)?;
            let dest_path = dest.join(relative_path);

            if entry.file_type().is_dir() {
                tokio::fs::create_dir_all(&dest_path).await?;
            } else if entry.file_type().is_file() {
                if let Some(parent) = dest_path.parent() {
                    tokio::fs::create_dir_all(parent).await?;
                }

                // Простая templateization - заменяем название проекта на переменную
                if is_template_file(&src_path) {
                    let content = tokio::fs::read_to_string(&src_path).await?;
                    let project_name = source.file_name()
                        .and_then(|n| n.to_str())
                        .unwrap_or("project");
                    
                    let templated_content = content.replace(project_name, "{{plugin_name}}");
                    tokio::fs::write(&dest_path, templated_content).await?;
                } else {
                    tokio::fs::copy(&src_path, &dest_path).await?;
                }
            }
        }
        Ok(())
    }

    pub async fn download_templates(&self) -> Result<()> {
        // If local_dir is set, check if templates_dir exists (which is now resolved)
        if self.config.local_dir.is_some() {
            if self.templates_dir.exists() {
                return Ok(());
            } else {
                return Err(anyhow::anyhow!("Local template directory does not exist: {}", self.templates_dir.display()));
            }
        }

        // Create templates directory
        tokio::fs::create_dir_all(&self.templates_dir).await?;

        // Check if templates are cached and still valid
        let cache_file = self.templates_dir.join(".cache_info");
        if cache_file.exists() {
            let cache_info: CacheInfo = {
                let content = tokio::fs::read_to_string(&cache_file).await?;
                serde_json::from_str(&content)?
            };

            let hours_since_update = (std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs() - cache_info.last_update) / 3600;

            if hours_since_update < self.config.cache_duration {
                return Ok(()); // Cache is still valid
            }
        }

        // Download or update templates
        if self.templates_dir.join(".git").exists() {
            // Update existing repository
            crate::utils::ProcessUtils::run_command("git", &["pull"], Some(&self.templates_dir)).await?;
        } else {
            // Clone repository
            crate::utils::ProcessUtils::run_command(
                "git",
                &["clone", &self.config.repository, self.templates_dir.to_str().unwrap()],
                None,
            ).await?;
        }

        // Update cache info
        let cache_info = CacheInfo {
            last_update: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)?
                .as_secs(),
        };
        let cache_content = serde_json::to_string_pretty(&cache_info)?;
        tokio::fs::write(&cache_file, cache_content).await?;

        Ok(())
    }

    pub async fn list_templates(&self) -> Result<Vec<TemplateMetadata>> {
        self.download_templates().await?;

        let templates_path = self.templates_dir.clone();

        let mut templates = Vec::new();
        
        for entry in WalkDir::new(&templates_path)
            .min_depth(1)
            .max_depth(1)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_type().is_dir())
        {
            let template_dir = entry.path();
            let metadata_file = template_dir.join("template.toml");
            
            if metadata_file.exists() {
                let content = tokio::fs::read_to_string(&metadata_file).await?;
                
                // Try to parse as TemplateFile (new format) first
                if let Ok(template_file) = toml::from_str::<TemplateFile>(&content) {
                    let mut metadata = TemplateMetadata {
                        name: template_file.template.name.clone(),
                        description: template_file.template.description,
                        author: template_file.template.author,
                        version: template_file.template.version,
                        gameverse_version: template_file.template.gameverse_version,
                        category: template_file.template.category,
                        tags: template_file.template.tags,
                        languages: template_file.languages.keys().cloned().collect(),
                        features: vec![], // Convert from HashMap to Vec if needed
                        variables: HashMap::new(), // Convert VariableConfig to TemplateVariable if needed
                        dependencies: template_file.dependencies,
                    };
                    
                    // Convert features from HashMap<String, bool> to Vec<String>
                    if let Some(features) = &template_file.features {
                        metadata.features = features.iter()
                            .filter(|(_, &enabled)| enabled)
                            .map(|(name, _)| name.clone())
                            .collect();
                    }
                    
                    templates.push(metadata);
                } else if let Ok(template_info) = toml::from_str::<TemplateInfo>(&content) {
                    let mut metadata = template_info.template;
                    // Convert languages from HashMap keys to Vec
                    metadata.languages = template_info.languages.keys().cloned().collect();
                    templates.push(metadata);
                } else if let Ok(mut metadata) = toml::from_str::<TemplateMetadata>(&content) {
                    // Convert languages from HashMap keys to Vec
                    if metadata.languages.is_empty() {
                        // Try to extract from template_info structure
                        if let Ok(template_info) = toml::from_str::<TemplateInfo>(&content) {
                            metadata.languages = template_info.languages.keys().cloned().collect();
                        }
                    }
                templates.push(metadata);
                }
            }
        }

        Ok(templates)
    }

    pub async fn create_plugin(
        &self,
        template_name: &str,
        language: &str,
        plugin_name: &str,
        target_dir: &Path,
        config: &Config,
    ) -> Result<()> {
        self.download_templates().await?;

        let templates_path = self.templates_dir.clone();

        let template_dir = templates_path.join(template_name);
        if !template_dir.exists() {
            return Err(anyhow::anyhow!("Template '{}' not found", template_name));
        }

        // Load template metadata
        let metadata_file = template_dir.join("template.toml");
        let metadata: TemplateMetadata = if metadata_file.exists() {
            let content = tokio::fs::read_to_string(&metadata_file).await?;
            
            // Try to parse as TemplateFile (new format) first
            if let Ok(template_file) = toml::from_str::<TemplateFile>(&content) {
                TemplateMetadata {
                    name: template_file.template.name.clone(),
                    description: template_file.template.description,
                    author: template_file.template.author,
                    version: template_file.template.version,
                    gameverse_version: template_file.template.gameverse_version,
                    category: template_file.template.category,
                    tags: template_file.template.tags,
                    languages: template_file.languages.keys().cloned().collect(),
                    features: template_file.features.as_ref().map(|f| 
                        f.iter().filter(|(_, &enabled)| enabled).map(|(name, _)| name.clone()).collect()
                    ).unwrap_or_default(),
                    variables: HashMap::new(), // Convert VariableConfig to TemplateVariable if needed
                    dependencies: template_file.dependencies,
                }
            } else {
                // Fallback to legacy format
            toml::from_str(&content)?
            }
        } else {
            TemplateMetadata {
                name: template_name.to_string(),
                description: "Basic plugin template".to_string(),
                languages: vec!["rust".to_string(), "lua".to_string(), "typescript".to_string()],
                author: "GameVerse Team".to_string(),
                version: "1.0.0".to_string(),
                gameverse_version: ">=0.1.0".to_string(),
                features: vec![],
                variables: HashMap::new(),
                dependencies: None,
                category: Some("basic".to_string()),
                tags: Some(vec!["basic".to_string()]),
            }
        };

        // Check if language is supported
        if !metadata.languages.contains(&language.to_string()) {
            return Err(anyhow::anyhow!(
                "Language '{}' is not supported by template '{}'",
                language,
                template_name
            ));
        }

        // Create template context
        let context = TemplateContext {
            plugin_name: plugin_name.to_string(),
            plugin_author: config.plugin.default_author.clone()
                .unwrap_or_else(|| "GameVerse Developer".to_string()),
            plugin_license: config.plugin.default_license.clone(),
            plugin_version: "1.0.0".to_string(),
            plugin_description: format!("A {} plugin for GameVerse Framework", plugin_name),
            language: language.to_string(),
            current_year: chrono::Utc::now().year(),
            custom_vars: HashMap::new(),
        };

        // Create target directory
        tokio::fs::create_dir_all(target_dir).await?;

        // Process template files
        let language_dir = template_dir.join(language);
        let source_dir = if language_dir.exists() {
            language_dir
        } else {
            template_dir.clone()
        };

        self.process_template_directory(&source_dir, target_dir, &context).await?;

        Ok(())
    }

    async fn process_template_directory(
        &self,
        source_dir: &Path,
        target_dir: &Path,
        context: &TemplateContext,
    ) -> Result<()> {
        for entry in WalkDir::new(source_dir).into_iter().filter_map(|e| e.ok()) {
            let source_path = entry.path();
            
            // Skip template metadata files
            if source_path.file_name() == Some(std::ffi::OsStr::new("template.toml")) {
                continue;
            }

            let relative_path = source_path.strip_prefix(source_dir)?;
            let target_path = target_dir.join(relative_path);

            if entry.file_type().is_dir() {
                tokio::fs::create_dir_all(&target_path).await?;
            } else {
                // Process template file
                let content = tokio::fs::read_to_string(source_path).await
                    .context(format!("Failed to read template file: {}", source_path.display()))?;

                let processed_content = if is_template_file(source_path) {
                    self.handlebars.render_template(&content, context)
                        .context(format!("Failed to process template: {}", source_path.display()))?
                } else {
                    content
                };

                // Create parent directories
                if let Some(parent) = target_path.parent() {
                    tokio::fs::create_dir_all(parent).await?;
                }

                tokio::fs::write(&target_path, processed_content).await
                    .context(format!("Failed to write file: {}", target_path.display()))?;

                // Copy file permissions
                #[cfg(unix)]
                {
                    use std::os::unix::fs::PermissionsExt;
                    let metadata = tokio::fs::metadata(source_path).await?;
                    let permissions = std::fs::Permissions::from_mode(metadata.permissions().mode());
                    tokio::fs::set_permissions(&target_path, permissions).await?;
                }
            }
        }

        Ok(())
    }
}

#[derive(Serialize, Deserialize)]
#[derive(Debug)]
pub struct ValidationResult {
    pub valid: bool,
    pub errors: Vec<String>,
    pub warnings: Vec<String>,
}

#[derive(Serialize, Deserialize)]
struct CacheInfo {
    last_update: u64,
}

fn is_template_file(path: &Path) -> bool {
    // Files that should be processed as templates
    match path.extension().and_then(|ext| ext.to_str()) {
        Some("toml") | Some("rs") | Some("lua") | Some("ts") | Some("js") | 
        Some("tsx") | Some("jsx") | Some("md") | Some("txt") | Some("json") => true,
        _ => false,
    }
}

// Handlebars helpers
fn uppercase_helper(
    h: &handlebars::Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    let param = h.param(0).unwrap().value().as_str().unwrap();
    out.write(&param.to_uppercase())?;
    Ok(())
}

fn lowercase_helper(
    h: &handlebars::Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    let param = h.param(0).unwrap().value().as_str().unwrap();
    out.write(&param.to_lowercase())?;
    Ok(())
}

fn snake_case_helper(
    h: &handlebars::Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    let param = h.param(0).unwrap().value().as_str().unwrap();
    let snake_case = param
        .chars()
        .map(|c| if c.is_uppercase() { format!("_{}", c.to_lowercase()) } else { c.to_string() })
        .collect::<String>()
        .trim_start_matches('_')
        .replace(' ', "_")
        .replace('-', "_")
        .to_lowercase();
    out.write(&snake_case)?;
    Ok(())
}

fn kebab_case_helper(
    h: &handlebars::Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    let param = h.param(0).unwrap().value().as_str().unwrap();
    let kebab_case = param
        .chars()
        .map(|c| if c.is_uppercase() { format!("-{}", c.to_lowercase()) } else { c.to_string() })
        .collect::<String>()
        .trim_start_matches('-')
        .replace(' ', "-")
        .replace('_', "-")
        .to_lowercase();
    out.write(&kebab_case)?;
    Ok(())
}

fn camelcase_helper(
    h: &handlebars::Helper,
    _: &Handlebars,
    _: &handlebars::Context,
    _: &mut handlebars::RenderContext,
    out: &mut dyn handlebars::Output,
) -> handlebars::HelperResult {
    let param = h.param(0).unwrap().value().as_str().unwrap();
    let camel_case = param
        .split('-')
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                None => String::new(),
                Some(first) => first.to_uppercase().collect::<String>() + &chars.collect::<String>(),
            }
        })
        .collect::<String>();
    out.write(&camel_case)?;
    Ok(())
} 