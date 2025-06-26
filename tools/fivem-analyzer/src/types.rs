use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

/// Результат анализа FiveM ресурса
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AnalysisReport {
    pub resource_name: String,
    pub framework_type: FrameworkType,
    pub migration_complexity: MigrationComplexity,
    pub current_performance: PerformanceMetrics,
    pub estimated_improvement: PerformanceImprovement,
    pub migration_steps: Vec<MigrationStep>,
    pub dependencies: Vec<Dependency>,
    pub issues: Vec<Issue>,
    pub compatibility: CompatibilityReport,
    pub analysis_timestamp: chrono::DateTime<chrono::Utc>,
}

/// Тип FiveM фреймворка
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum FrameworkType {
    QBCore,
    ESX,
    VRP,
    ND,
    Qbox,
    VORP,
    Custom(String),
    Unknown,
}

/// Сложность миграции
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationComplexity {
    pub overall_score: f32, // 0.0 (простая) до 10.0 (очень сложная)
    pub lua_script_complexity: f32,
    pub database_complexity: f32,
    pub ui_complexity: f32,
    pub dependency_complexity: f32,
    pub custom_native_usage: f32,
    pub estimated_migration_time_hours: f32,
    pub automation_percentage: f32, // Какой % можно автоматизировать
}

/// Метрики производительности
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceMetrics {
    pub memory_usage_mb: f32,
    pub cpu_usage_percent: f32,
    pub startup_time_ms: u32,
    pub response_time_ms: u32,
    pub database_queries_per_second: f32,
    pub ui_render_time_ms: u32,
    pub network_latency_ms: u32,
}

/// Ожидаемые улучшения в GameVerse
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceImprovement {
    pub memory_reduction_factor: f32, // 5.0 = 5x меньше памяти
    pub cpu_efficiency_factor: f32,
    pub startup_speedup_factor: f32,
    pub response_time_improvement_factor: f32,
    pub database_efficiency_factor: f32,
    pub ui_performance_factor: f32,
    pub overall_improvement_factor: f32,
}

/// Шаг миграции
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationStep {
    pub step_number: u32,
    pub title: String,
    pub description: String,
    pub automation_level: AutomationLevel,
    pub estimated_time_minutes: u32,
    pub prerequisites: Vec<String>,
    pub commands: Vec<String>,
    pub files_affected: Vec<PathBuf>,
    pub validation_checks: Vec<String>,
}

/// Уровень автоматизации миграции
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AutomationLevel {
    FullyAutomated,
    SemiAutomated,
    ManualWithGuidance,
    ManualOnly,
}

/// Зависимость ресурса
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Dependency {
    pub name: String,
    pub dependency_type: DependencyType,
    pub version: Option<String>,
    pub required: bool,
    pub source: DependencySource,
    pub gameverse_equivalent: Option<String>,
    pub migration_status: MigrationStatus,
}

/// Тип зависимости
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DependencyType {
    Framework,
    Library,
    Database,
    NativeFunction,
    Resource,
    External,
}

/// Источник зависимости
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DependencySource {
    GitHub(String),
    Cfx(String),
    Local(PathBuf),
    Unknown,
}

/// Статус миграции зависимости
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MigrationStatus {
    DirectlySupported,
    NeedsConversion,
    HasAlternative(String),
    Unsupported,
    UnderDevelopment,
}

/// Проблема или предупреждение
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub severity: IssueSeverity,
    pub category: IssueCategory,
    pub title: String,
    pub description: String,
    pub file_path: Option<PathBuf>,
    pub line_number: Option<u32>,
    pub fix_suggestion: Option<String>,
    pub gameverse_solution: Option<String>,
}

/// Серьезность проблемы
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IssueSeverity {
    Critical,
    High,
    Medium,
    Low,
    Info,
}

/// Категория проблемы
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum IssueCategory {
    Security,
    Performance,
    Compatibility,
    BestPractice,
    Migration,
    TypeSafety,
    Memory,
    Database,
}

/// Отчет о совместимости
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CompatibilityReport {
    pub gameverse_compatibility_score: f32, // 0.0 до 1.0
    pub supported_features: Vec<String>,
    pub unsupported_features: Vec<String>,
    pub alternative_approaches: HashMap<String, String>,
    pub native_function_coverage: f32,
    pub ui_compatibility: UICompatibility,
    pub database_compatibility: DatabaseCompatibility,
}

/// Совместимость UI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UICompatibility {
    pub ui_type: UIType,
    pub conversion_complexity: f32,
    pub webassembly_ready: bool,
    pub memory_savings_factor: f32,
    pub performance_improvement_factor: f32,
}

/// Тип UI
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum UIType {
    CEF,
    NUI,
    HTML,
    None,
    Custom(String),
}

/// Совместимость базы данных
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseCompatibility {
    pub database_type: DatabaseType,
    pub schema_complexity: f32,
    pub migration_feasibility: f32,
    pub query_optimization_potential: f32,
    pub data_preservation_guarantee: bool,
}

/// Тип базы данных
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DatabaseType {
    MySQL,
    MariaDB,
    SQLite,
    PostgreSQL,
    MongoDB,
    None,
    Custom(String),
}

/// Результаты бенчмарка
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BenchmarkResults {
    pub resource_name: String,
    pub test_duration_seconds: u64,
    pub fivem_metrics: PerformanceMetrics,
    pub gameverse_estimated_metrics: PerformanceMetrics,
    pub improvement_factors: PerformanceImprovement,
    pub memory_analysis: MemoryAnalysis,
    pub bottlenecks: Vec<PerformanceBottleneck>,
    pub optimization_suggestions: Vec<OptimizationSuggestion>,
}

/// Анализ памяти
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MemoryAnalysis {
    pub peak_memory_usage_mb: f32,
    pub average_memory_usage_mb: f32,
    pub memory_leaks_detected: bool,
    pub garbage_collection_pressure: f32,
    pub ui_memory_overhead_mb: f32,
    pub script_memory_usage_mb: f32,
}

/// Узкое место производительности
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PerformanceBottleneck {
    pub category: BottleneckCategory,
    pub impact_severity: f32,
    pub description: String,
    pub gameverse_solution: String,
    pub improvement_factor: f32,
}

/// Категория узкого места
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BottleneckCategory {
    UIRendering,
    DatabaseQuery,
    ScriptExecution,
    NetworkLatency,
    MemoryAllocation,
    NativeFunction,
    FileIO,
}

/// Предложение оптимизации
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OptimizationSuggestion {
    pub title: String,
    pub description: String,
    pub implementation_effort: ImplementationEffort,
    pub expected_improvement: f32,
    pub gameverse_advantage: String,
}

/// Усилие на реализацию
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ImplementationEffort {
    Minimal,
    Low,
    Medium,
    High,
    Significant,
}

/// План миграции
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationPlan {
    pub server_name: String,
    pub total_resources: u32,
    pub migration_phases: Vec<MigrationPhase>,
    pub estimated_total_time_hours: f32,
    pub automation_percentage: f32,
    pub required_tools: Vec<String>,
    pub risk_assessment: RiskAssessment,
    pub rollback_plan: RollbackPlan,
    pub testing_strategy: TestingStrategy,
}

/// Фаза миграции
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MigrationPhase {
    pub phase_number: u32,
    pub title: String,
    pub description: String,
    pub resources: Vec<String>,
    pub steps: Vec<MigrationStep>,
    pub estimated_time_hours: f32,
    pub dependencies: Vec<u32>, // Номера фаз-зависимостей
    pub validation_criteria: Vec<String>,
}

/// Оценка рисков
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RiskAssessment {
    pub overall_risk_level: RiskLevel,
    pub data_loss_risk: RiskLevel,
    pub downtime_risk: RiskLevel,
    pub performance_degradation_risk: RiskLevel,
    pub compatibility_risk: RiskLevel,
    pub mitigation_strategies: Vec<String>,
}

/// Уровень риска
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RiskLevel {
    Low,
    Medium,
    High,
    Critical,
}

/// План отката
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RollbackPlan {
    pub backup_strategy: String,
    pub rollback_steps: Vec<String>,
    pub estimated_rollback_time_hours: f32,
    pub data_recovery_plan: String,
}

/// Стратегия тестирования
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TestingStrategy {
    pub test_phases: Vec<String>,
    pub automated_tests: Vec<String>,
    pub manual_tests: Vec<String>,
    pub performance_benchmarks: Vec<String>,
    pub user_acceptance_criteria: Vec<String>,
}

impl Default for PerformanceMetrics {
    fn default() -> Self {
        Self {
            memory_usage_mb: 0.0,
            cpu_usage_percent: 0.0,
            startup_time_ms: 0,
            response_time_ms: 0,
            database_queries_per_second: 0.0,
            ui_render_time_ms: 0,
            network_latency_ms: 0,
        }
    }
}

impl Default for MigrationComplexity {
    fn default() -> Self {
        Self {
            overall_score: 5.0,
            lua_script_complexity: 5.0,
            database_complexity: 5.0,
            ui_complexity: 5.0,
            dependency_complexity: 5.0,
            custom_native_usage: 5.0,
            estimated_migration_time_hours: 8.0,
            automation_percentage: 70.0,
        }
    }
} 