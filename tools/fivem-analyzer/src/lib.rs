//! # FiveM Analyzer
//! 
//! Инструмент анализа и миграции FiveM ресурсов в GameVerse Framework.
//! 
//! ## Основные компоненты:
//! 
//! - **QBCore Analysis**: Анализ ресурсов QBCore фреймворка
//! - **ESX Analysis**: Анализ ресурсов ESX фреймворка  
//! - **Migration Planning**: Автоматическое планирование миграции
//! - **Performance Benchmarking**: Сравнение производительности
//! - **Compatibility Assessment**: Оценка совместимости с GameVerse
//! 
//! ## Пример использования:
//! 
//! ```rust,no_run
//! use fivem_analyzer::qbcore::QBCoreAnalyzer;
//! use std::path::PathBuf;
//! 
//! #[tokio::main]
//! async fn main() -> anyhow::Result<()> {
//!     let mut analyzer = QBCoreAnalyzer::new(PathBuf::from("./qb-banking"));
//!     let report = analyzer.analyze_resource().await?;
//!     
//!     println!("Ожидаемые улучшения: {}x", 
//!              report.estimated_improvement.overall_improvement_factor);
//!     
//!     Ok(())
//! }
//! ```

pub mod types;
pub mod qbcore;
pub mod esx;
pub mod analysis_engine;
pub mod migration;
pub mod benchmarks;
pub mod utils;

// Re-export основных типов для удобства
pub use types::{
    AnalysisReport, 
    MigrationPlan, 
    BenchmarkResults, 
    FrameworkType,
    PerformanceImprovement,
    MigrationComplexity,
};

pub use qbcore::QBCoreAnalyzer;
pub use esx::ESXAnalyzer;
pub use analysis_engine::AnalysisEngine; 