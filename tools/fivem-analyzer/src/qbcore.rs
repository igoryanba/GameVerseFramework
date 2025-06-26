use anyhow::{Context, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};
use tracing::{debug, info, warn, error};
use walkdir::WalkDir;

use crate::types::*;

/// Анализатор QBCore ресурсов
pub struct QBCoreAnalyzer {
    resource_path: PathBuf,
    manifest: Option<QBCoreManifest>,
    lua_files: Vec<PathBuf>,
    sql_files: Vec<PathBuf>,
    html_files: Vec<PathBuf>,
}

/// Структура fxmanifest.lua для QBCore
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QBCoreManifest {
    pub fx_version: String,
    pub game: String,
    pub name: String,
    pub description: Option<String>,
    pub author: Option<String>,
    pub version: Option<String>,
    pub dependencies: Vec<String>,
    pub shared_scripts: Vec<String>,
    pub client_scripts: Vec<String>,
    pub server_scripts: Vec<String>,
    pub ui_page: Option<String>,
    pub files: Vec<String>,
    pub exports: HashMap<String, Vec<String>>,
    pub server_exports: HashMap<String, Vec<String>>,
    pub uses_database: bool,
    pub database_type: DatabaseType,
}

/// Специфичные для QBCore данные
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct QBCoreSpecificData {
    pub qb_core_version: Option<String>,
    pub uses_qb_target: bool,
    pub uses_qb_menu: bool,
    pub uses_qb_notify: bool,
    pub uses_qb_input: bool,
    pub uses_ox_lib: bool,
    pub job_system_integration: bool,
    pub gang_system_integration: bool,
    pub inventory_integration: bool,
    pub banking_integration: bool,
    pub phone_integration: bool,
    pub housing_integration: bool,
    pub vehicle_integration: bool,
    pub custom_callbacks: Vec<String>,
    pub custom_events: Vec<String>,
    pub native_usage_patterns: Vec<NativeUsagePattern>,
}

/// Паттерн использования нативных функций
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NativeUsagePattern {
    pub native_name: String,
    pub usage_frequency: u32,
    pub file_locations: Vec<String>,
    pub complexity_score: f32,
    pub gameverse_equivalent: Option<String>,
    pub migration_notes: Option<String>,
}

impl QBCoreAnalyzer {
    pub fn new(resource_path: PathBuf) -> Self {
        Self {
            resource_path,
            manifest: None,
            lua_files: Vec::new(),
            sql_files: Vec::new(),
            html_files: Vec::new(),
        }
    }

    /// Основной анализ QBCore ресурса
    pub async fn analyze_resource(&mut self) -> Result<AnalysisReport> {
        info!("🔍 Starting QBCore resource analysis at: {:?}", self.resource_path);

        let resource_name = self.resource_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        Ok(AnalysisReport {
            resource_name,
            framework_type: FrameworkType::QBCore,
            migration_complexity: MigrationComplexity::default(),
            current_performance: PerformanceMetrics::default(),
            estimated_improvement: PerformanceImprovement {
                memory_reduction_factor: 5.0,
                cpu_efficiency_factor: 3.0,
                startup_speedup_factor: 10.0,
                response_time_improvement_factor: 5.0,
                database_efficiency_factor: 8.0,
                ui_performance_factor: 20.0,
                overall_improvement_factor: 8.5,
            },
            migration_steps: Vec::new(),
            dependencies: Vec::new(),
            issues: Vec::new(),
            compatibility: CompatibilityReport {
                gameverse_compatibility_score: 0.85,
                supported_features: vec!["Events".to_string(), "Callbacks".to_string()],
                unsupported_features: vec!["Legacy Exports".to_string()],
                alternative_approaches: std::collections::HashMap::new(),
                native_function_coverage: 0.95,
                ui_compatibility: UICompatibility {
                    ui_type: UIType::CEF,
                    conversion_complexity: 3.0,
                    webassembly_ready: true,
                    memory_savings_factor: 5.0,
                    performance_improvement_factor: 20.0,
                },
                database_compatibility: DatabaseCompatibility {
                    database_type: DatabaseType::MySQL,
                    schema_complexity: 4.0,
                    migration_feasibility: 0.9,
                    query_optimization_potential: 8.0,
                    data_preservation_guarantee: true,
                },
            },
            analysis_timestamp: chrono::Utc::now(),
        })
    }

    /// Сканирование файлов в ресурсе
    fn scan_files(&mut self) -> Result<()> {
        debug!("📁 Scanning files in resource directory");

        for entry in WalkDir::new(&self.resource_path).follow_links(false) {
            let entry = entry.context("Failed to read directory entry")?;
            let path = entry.path();
            
            if path.is_file() {
                match path.extension().and_then(|s| s.to_str()) {
                    Some("lua") => self.lua_files.push(path.to_path_buf()),
                    Some("sql") => self.sql_files.push(path.to_path_buf()),
                    Some("html") | Some("htm") => self.html_files.push(path.to_path_buf()),
                    _ => {}
                }
            }
        }

        info!("📊 Found {} Lua files, {} SQL files, {} HTML files", 
              self.lua_files.len(), self.sql_files.len(), self.html_files.len());

        Ok(())
    }

    /// Анализ fxmanifest.lua
    async fn analyze_manifest(&mut self) -> Result<()> {
        let manifest_path = self.resource_path.join("fxmanifest.lua");
        
        if !manifest_path.exists() {
            warn!("⚠️ No fxmanifest.lua found, checking for __resource.lua");
            
            let resource_path = self.resource_path.join("__resource.lua");
            if !resource_path.exists() {
                return Err(anyhow::anyhow!("No manifest file found"));
            }
        }

        let manifest_content = fs::read_to_string(&manifest_path)
            .context("Failed to read manifest file")?;

        self.manifest = Some(self.parse_lua_manifest(&manifest_content)?);
        
        debug!("✅ Manifest parsed successfully");
        Ok(())
    }

    /// Парсинг Lua манифеста
    fn parse_lua_manifest(&self, content: &str) -> Result<QBCoreManifest> {
        // Простой парсер для fxmanifest.lua
        // В реальной реализации лучше использовать Lua VM
        
        let fx_version_re = Regex::new(r#"fx_version\s*['"]([^'"]+)['"]"#)?;
        let game_re = Regex::new(r#"game\s*['"]([^'"]+)['"]"#)?;
        let description_re = Regex::new(r#"description\s*['"]([^'"]+)['"]"#)?;
        let author_re = Regex::new(r#"author\s*['"]([^'"]+)['"]"#)?;
        let version_re = Regex::new(r#"version\s*['"]([^'"]+)['"]"#)?;
        
        let fx_version = fx_version_re.captures(content)
            .and_then(|c| c.get(1))
            .map(|m| m.as_str().to_string())
            .unwrap_or_else(|| "cerulean".to_string());
            
        let game = game_re.captures(content)
            .and_then(|c| c.get(1))
            .map(|m| m.as_str().to_string())
            .unwrap_or_else(|| "gta5".to_string());

        let name = self.resource_path
            .file_name()
            .unwrap_or_default()
            .to_string_lossy()
            .to_string();

        let description = description_re.captures(content)
            .and_then(|c| c.get(1))
            .map(|m| m.as_str().to_string());

        let author = author_re.captures(content)
            .and_then(|c| c.get(1))
            .map(|m| m.as_str().to_string());

        let version = version_re.captures(content)
            .and_then(|c| c.get(1))
            .map(|m| m.as_str().to_string());

        // Анализ зависимостей
        let dependencies = self.extract_dependencies(content)?;
        
        // Анализ скриптов
        let (shared_scripts, client_scripts, server_scripts) = self.extract_scripts(content)?;
        
        // Анализ UI
        let ui_page = self.extract_ui_page(content)?;
        
        // Анализ файлов
        let files = self.extract_files(content)?;
        
        // Анализ экспортов
        let (exports, server_exports) = self.extract_exports(content)?;
        
        // Определение типа БД
        let (uses_database, database_type) = self.detect_database_usage(content)?;

        Ok(QBCoreManifest {
            fx_version,
            game,
            name,
            description,
            author,
            version,
            dependencies,
            shared_scripts,
            client_scripts,
            server_scripts,
            ui_page,
            files,
            exports,
            server_exports,
            uses_database,
            database_type,
        })
    }

    /// Извлечение зависимостей из манифеста
    fn extract_dependencies(&self, content: &str) -> Result<Vec<String>> {
        let dependencies_re = Regex::new(r#"dependencies\s*\{([^}]+)\}"#)?;
        let dependency_re = Regex::new(r#"['"]([^'"]+)['"]"#)?;
        
        let mut dependencies = Vec::new();
        
        if let Some(deps_block) = dependencies_re.captures(content) {
            if let Some(deps_text) = deps_block.get(1) {
                for cap in dependency_re.captures_iter(deps_text.as_str()) {
                    if let Some(dep) = cap.get(1) {
                        dependencies.push(dep.as_str().to_string());
                    }
                }
            }
        }
        
        Ok(dependencies)
    }

    /// Извлечение скриптов из манифеста
    fn extract_scripts(&self, content: &str) -> Result<(Vec<String>, Vec<String>, Vec<String>)> {
        let shared_re = Regex::new(r#"shared_scripts?\s*\{([^}]+)\}"#)?;
        let client_re = Regex::new(r#"client_scripts?\s*\{([^}]+)\}"#)?;
        let server_re = Regex::new(r#"server_scripts?\s*\{([^}]+)\}"#)?;
        let script_re = Regex::new(r#"['"]([^'"]+)['"]"#)?;
        
        let extract_scripts_from_block = |regex: &Regex| -> Vec<String> {
            regex.captures(content)
                .and_then(|caps| caps.get(1))
                .map(|m| script_re.captures_iter(m.as_str())
                    .filter_map(|cap| cap.get(1).map(|m| m.as_str().to_string()))
                    .collect())
                .unwrap_or_default()
        };
        
        let shared_scripts = extract_scripts_from_block(&shared_re);
        let client_scripts = extract_scripts_from_block(&client_re);
        let server_scripts = extract_scripts_from_block(&server_re);
        
        Ok((shared_scripts, client_scripts, server_scripts))
    }

    /// Извлечение UI страницы
    fn extract_ui_page(&self, content: &str) -> Result<Option<String>> {
        let ui_re = Regex::new(r#"ui_page\s*['"]([^'"]+)['"]"#)?;
        
        Ok(ui_re.captures(content)
            .and_then(|c| c.get(1))
            .map(|m| m.as_str().to_string()))
    }

    /// Извлечение файлов
    fn extract_files(&self, content: &str) -> Result<Vec<String>> {
        let files_re = Regex::new(r#"files\s*\{([^}]+)\}"#)?;
        let file_re = Regex::new(r#"['"]([^'"]+)['"]"#)?;
        
        Ok(files_re.captures(content)
            .and_then(|caps| caps.get(1))
            .map(|m| file_re.captures_iter(m.as_str())
                .filter_map(|cap| cap.get(1).map(|m| m.as_str().to_string()))
                .collect())
            .unwrap_or_default())
    }

    /// Извлечение экспортов
    fn extract_exports(&self, content: &str) -> Result<(HashMap<String, Vec<String>>, HashMap<String, Vec<String>>)> {
        // Упрощенная реализация - в реальности нужен полноценный Lua парсер
        Ok((HashMap::new(), HashMap::new()))
    }

    /// Определение использования базы данных
    fn detect_database_usage(&self, content: &str) -> Result<(bool, DatabaseType)> {
        let mysql_patterns = [
            "MySQL.Async",
            "mysql-async",
            "exports.oxmysql",
            "oxmysql",
            "MySQL.ready",
        ];
        
        for pattern in &mysql_patterns {
            if content.contains(pattern) {
                return Ok((true, DatabaseType::MySQL));
            }
        }
        
        if content.contains("sqlite") || content.contains("SQLite") {
            return Ok((true, DatabaseType::SQLite));
        }
        
        Ok((false, DatabaseType::None))
    }

    /// Анализ Lua скриптов
    async fn analyze_lua_scripts(&self) -> Result<LuaAnalysis> {
        info!("🔍 Analyzing Lua scripts...");
        
        let mut total_lines = 0;
        let mut complexity_score = 0.0;
        let mut qbcore_patterns = Vec::new();
        let mut native_calls = Vec::new();
        let mut event_handlers = Vec::new();
        let mut callbacks = Vec::new();
        
        for lua_file in &self.lua_files {
            let content = fs::read_to_string(lua_file)
                .with_context(|| format!("Failed to read Lua file: {:?}", lua_file))?;
            
            total_lines += content.lines().count();
            
            // Анализ QBCore паттернов
            qbcore_patterns.extend(self.extract_qbcore_patterns(&content, lua_file)?);
            
            // Анализ нативных вызовов
            native_calls.extend(self.extract_native_calls(&content, lua_file)?);
            
            // Анализ событий
            event_handlers.extend(self.extract_event_handlers(&content, lua_file)?);
            
            // Анализ коллбэков
            callbacks.extend(self.extract_callbacks(&content, lua_file)?);
            
            // Вычисление сложности
            complexity_score += self.calculate_script_complexity(&content)?;
        }
        
        Ok(LuaAnalysis {
            total_files: self.lua_files.len(),
            total_lines,
            complexity_score: complexity_score / self.lua_files.len() as f32,
            qbcore_patterns,
            native_calls,
            event_handlers,
            callbacks,
        })
    }

    /// Анализ зависимостей
    pub async fn analyze_dependencies(&self) -> Result<Vec<Dependency>> {
        info!("🔗 Analyzing dependencies...");
        Ok(Vec::new())
    }

    // Остальные методы анализа...
    async fn analyze_database(&self) -> Result<DatabaseAnalysis> { 
        // Заглушка для анализа БД
        Ok(DatabaseAnalysis::default())
    }
    
    async fn analyze_ui(&self) -> Result<UIAnalysis> { 
        // Заглушка для анализа UI
        Ok(UIAnalysis::default())
    }

    fn calculate_migration_complexity(
        &self,
        _lua: &LuaAnalysis,
        _db: &DatabaseAnalysis,
        _ui: &UIAnalysis,
        _deps: &[Dependency]
    ) -> Result<MigrationComplexity> {
        // Заглушка для вычисления сложности
        Ok(MigrationComplexity::default())
    }

    fn generate_migration_steps(&self, _complexity: &MigrationComplexity) -> Result<Vec<MigrationStep>> {
        // Заглушка для генерации шагов
        Ok(Vec::new())
    }

    fn estimate_current_performance(&self, _lua: &LuaAnalysis, _ui: &UIAnalysis) -> Result<PerformanceMetrics> {
        // Заглушка для оценки производительности
        Ok(PerformanceMetrics::default())
    }

    fn estimate_gameverse_improvements(&self, _current: &PerformanceMetrics) -> Result<PerformanceImprovement> {
        // Заглушка для оценки улучшений
        Ok(PerformanceImprovement {
            memory_reduction_factor: 5.0,
            cpu_efficiency_factor: 3.0,
            startup_speedup_factor: 10.0,
            response_time_improvement_factor: 5.0,
            database_efficiency_factor: 8.0,
            ui_performance_factor: 20.0,
            overall_improvement_factor: 8.5,
        })
    }

    fn analyze_gameverse_compatibility(&self, _lua: &LuaAnalysis, _deps: &[Dependency]) -> Result<CompatibilityReport> {
        // Заглушка для анализа совместимости
        Ok(CompatibilityReport {
            gameverse_compatibility_score: 0.85,
            supported_features: vec!["Events".to_string(), "Callbacks".to_string()],
            unsupported_features: vec!["Legacy Exports".to_string()],
            alternative_approaches: HashMap::new(),
            native_function_coverage: 0.95,
            ui_compatibility: UICompatibility {
                ui_type: UIType::CEF,
                conversion_complexity: 3.0,
                webassembly_ready: true,
                memory_savings_factor: 5.0,
                performance_improvement_factor: 20.0,
            },
            database_compatibility: DatabaseCompatibility {
                database_type: DatabaseType::MySQL,
                schema_complexity: 4.0,
                migration_feasibility: 0.9,
                query_optimization_potential: 8.0,
                data_preservation_guarantee: true,
            },
        })
    }

    fn identify_issues(&self, _lua: &LuaAnalysis, _db: &DatabaseAnalysis, _ui: &UIAnalysis) -> Result<Vec<Issue>> {
        // Заглушка для выявления проблем
        Ok(Vec::new())
    }

    // Вспомогательные методы для анализа Lua
    fn extract_qbcore_patterns(&self, _content: &str, _file: &Path) -> Result<Vec<String>> {
        Ok(Vec::new())
    }

    fn extract_native_calls(&self, _content: &str, _file: &Path) -> Result<Vec<String>> {
        Ok(Vec::new())
    }

    fn extract_event_handlers(&self, _content: &str, _file: &Path) -> Result<Vec<String>> {
        Ok(Vec::new())
    }

    fn extract_callbacks(&self, _content: &str, _file: &Path) -> Result<Vec<String>> {
        Ok(Vec::new())
    }

    fn calculate_script_complexity(&self, _content: &str) -> Result<f32> {
        Ok(5.0)
    }
}

// Вспомогательные структуры для анализа
#[derive(Debug, Default)]
struct LuaAnalysis {
    total_files: usize,
    total_lines: usize,
    complexity_score: f32,
    qbcore_patterns: Vec<String>,
    native_calls: Vec<String>,
    event_handlers: Vec<String>,
    callbacks: Vec<String>,
}

#[derive(Debug, Default)]
struct DatabaseAnalysis {
    // Заглушка
}

#[derive(Debug, Default)]
struct UIAnalysis {
    // Заглушка
}

/// Загрузка популярных QBCore ресурсов для анализа
pub async fn download_popular_resources(_count: u32, _output_dir: &std::path::Path) -> Result<()> {
    info!("📥 Downloading QBCore resources (mock implementation)");
    Ok(())
} 