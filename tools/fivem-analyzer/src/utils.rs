use anyhow::Result;
use serde_json;
use tracing::info;

use crate::types::*;

/// Выводит отчет анализа в указанном формате
pub fn output_report(report: &AnalysisReport, format: &str) -> Result<()> {
    match format {
        "json" => {
            let json = serde_json::to_string_pretty(report)?;
            println!("{}", json);
        },
        "yaml" => {
            let yaml = serde_yaml::to_string(report)?;
            println!("{}", yaml);
        },
        "table" | _ => {
            print_report_table(report);
        }
    }
    Ok(())
}

/// Выводит план миграции
pub fn output_migration_plan(plan: &MigrationPlan, format: &str) -> Result<()> {
    match format {
        "json" => {
            let json = serde_json::to_string_pretty(plan)?;
            println!("{}", json);
        },
        "yaml" => {
            let yaml = serde_yaml::to_string(plan)?;
            println!("{}", yaml);
        },
        "table" | _ => {
            print_migration_plan_table(plan);
        }
    }
    Ok(())
}

/// Выводит зависимости
pub fn output_dependencies(deps: &[Dependency], format: &str) -> Result<()> {
    match format {
        "json" => {
            let json = serde_json::to_string_pretty(deps)?;
            println!("{}", json);
        },
        "yaml" => {
            let yaml = serde_yaml::to_string(deps)?;
            println!("{}", yaml);
        },
        "table" | _ => {
            print_dependencies_table(deps);
        }
    }
    Ok(())
}

/// Выводит результаты бенчмарка
pub fn output_benchmark_results(results: &BenchmarkResults, format: &str) -> Result<()> {
    match format {
        "json" => {
            let json = serde_json::to_string_pretty(results)?;
            println!("{}", json);
        },
        "yaml" => {
            let yaml = serde_yaml::to_string(results)?;
            println!("{}", yaml);
        },
        "table" | _ => {
            print_benchmark_table(results);
        }
    }
    Ok(())
}

fn print_report_table(report: &AnalysisReport) {
    println!("\n📊 ОТЧЕТ АНАЛИЗА РЕСУРСА");
    println!("═══════════════════════════════════════════════");
    println!("📦 Ресурс: {}", report.resource_name);
    println!("🏗️  Фреймворк: {:?}", report.framework_type);
    println!("📈 Совместимость с GameVerse: {:.1}%", 
             report.compatibility.gameverse_compatibility_score * 100.0);
    
    println!("\n⚡ ОЖИДАЕМЫЕ УЛУЧШЕНИЯ ПРОИЗВОДИТЕЛЬНОСТИ");
    println!("─────────────────────────────────────────────");
    println!("🧠 Память: {}x меньше", report.estimated_improvement.memory_reduction_factor);
    println!("⚙️  CPU: {}x эффективнее", report.estimated_improvement.cpu_efficiency_factor);
    println!("🚀 Запуск: {}x быстрее", report.estimated_improvement.startup_speedup_factor);
    println!("🎨 UI: {}x отзывчивее", report.estimated_improvement.ui_performance_factor);
    println!("💽 База данных: {}x быстрее", report.estimated_improvement.database_efficiency_factor);
    println!("🎯 Общее улучшение: {}x", report.estimated_improvement.overall_improvement_factor);
    
    println!("\n🔧 СЛОЖНОСТЬ МИГРАЦИИ");
    println!("─────────────────────────────────────────────");
    println!("📊 Общая оценка: {:.1}/10", report.migration_complexity.overall_score);
    println!("⏱️  Время миграции: {:.1} часов", report.migration_complexity.estimated_migration_time_hours);
    println!("🤖 Автоматизация: {:.1}%", report.migration_complexity.automation_percentage);
    
    if !report.issues.is_empty() {
        println!("\n⚠️  ОБНАРУЖЕННЫЕ ПРОБЛЕМЫ");
        println!("─────────────────────────────────────────────");
        for issue in &report.issues {
            println!("  {:?}: {}", issue.severity, issue.title);
        }
    }
    
    println!("\n✅ Анализ завершен: {}", report.analysis_timestamp.format("%Y-%m-%d %H:%M:%S UTC"));
}

fn print_migration_plan_table(plan: &MigrationPlan) {
    println!("\n🔄 ПЛАН МИГРАЦИИ");
    println!("═══════════════════════════════════════════════");
    println!("🖥️  Сервер: {}", plan.server_name);
    println!("📦 Ресурсов: {}", plan.total_resources);
    println!("⏱️  Общее время: {:.1} часов", plan.estimated_total_time_hours);
    println!("🤖 Автоматизация: {:.1}%", plan.automation_percentage);
    
    println!("\n📋 ФАЗЫ МИГРАЦИИ");
    println!("─────────────────────────────────────────────");
    for phase in &plan.migration_phases {
        println!("\n{}. {}", phase.phase_number, phase.title);
        println!("   📝 {}", phase.description);
        println!("   ⏱️  Время: {:.1} часов", phase.estimated_time_hours);
        println!("   📦 Ресурсы: {}", phase.resources.join(", "));
        
        if !phase.steps.is_empty() {
            println!("   🔧 Шаги:");
            for step in &phase.steps {
                println!("     • {} ({:?})", step.title, step.automation_level);
            }
        }
    }
    
    println!("\n🛡️  ОЦЕНКА РИСКОВ");
    println!("─────────────────────────────────────────────");
    println!("🎯 Общий уровень: {:?}", plan.risk_assessment.overall_risk_level);
    println!("💾 Потеря данных: {:?}", plan.risk_assessment.data_loss_risk);
    println!("⏱️  Простой: {:?}", plan.risk_assessment.downtime_risk);
    
    println!("\n🔧 НЕОБХОДИМЫЕ ИНСТРУМЕНТЫ");
    println!("─────────────────────────────────────────────");
    for tool in &plan.required_tools {
        println!("  • {}", tool);
    }
}

fn print_dependencies_table(deps: &[Dependency]) {
    println!("\n🔗 АНАЛИЗ ЗАВИСИМОСТЕЙ");
    println!("═══════════════════════════════════════════════");
    
    for dep in deps {
        println!("\n📦 {}", dep.name);
        println!("   🏷️  Тип: {:?}", dep.dependency_type);
        println!("   📍 Источник: {:?}", dep.source);
        println!("   🔄 Статус миграции: {:?}", dep.migration_status);
        
        if let Some(ref equiv) = dep.gameverse_equivalent {
            println!("   ✨ GameVerse эквивалент: {}", equiv);
        }
    }
}

fn print_benchmark_table(results: &BenchmarkResults) {
    println!("\n⚡ РЕЗУЛЬТАТЫ БЕНЧМАРКА");
    println!("═══════════════════════════════════════════════");
    println!("📦 Ресурс: {}", results.resource_name);
    println!("⏱️  Продолжительность: {}s", results.test_duration_seconds);
    
    println!("\n📊 СРАВНЕНИЕ ПРОИЗВОДИТЕЛЬНОСТИ");
    println!("─────────────────────────────────────────────");
    println!("              FiveM      GameVerse   Улучшение");
    println!("🧠 Память:    {:.1}MB      {:.1}MB        {}x",
             results.fivem_metrics.memory_usage_mb,
             results.gameverse_estimated_metrics.memory_usage_mb,
             results.improvement_factors.memory_reduction_factor);
    println!("⚙️  CPU:       {:.1}%       {:.1}%         {}x",
             results.fivem_metrics.cpu_usage_percent,
             results.gameverse_estimated_metrics.cpu_usage_percent,
             results.improvement_factors.cpu_efficiency_factor);
    println!("🚀 Запуск:    {}ms      {}ms        {}x",
             results.fivem_metrics.startup_time_ms,
             results.gameverse_estimated_metrics.startup_time_ms,
             results.improvement_factors.startup_speedup_factor);
    println!("🎨 UI:        {}ms       {}ms        {}x",
             results.fivem_metrics.ui_render_time_ms,
             results.gameverse_estimated_metrics.ui_render_time_ms,
             results.improvement_factors.ui_performance_factor);
    
    if !results.bottlenecks.is_empty() {
        println!("\n🚫 УЗКИЕ МЕСТА");
        println!("─────────────────────────────────────────────");
        for bottleneck in &results.bottlenecks {
            println!("• {:?}: {}", bottleneck.category, bottleneck.description);
            println!("  💡 Решение: {}", bottleneck.gameverse_solution);
            println!("  📈 Улучшение: {}x", bottleneck.improvement_factor);
        }
    }
    
    if !results.optimization_suggestions.is_empty() {
        println!("\n💡 РЕКОМЕНДАЦИИ ПО ОПТИМИЗАЦИИ");
        println!("─────────────────────────────────────────────");
        for suggestion in &results.optimization_suggestions {
            println!("• {}", suggestion.title);
            println!("  📝 {}", suggestion.description);
            println!("  🔧 Сложность: {:?}", suggestion.implementation_effort);
            println!("  📈 Ожидаемое улучшение: {}x", suggestion.expected_improvement);
        }
    }
    
    println!("\n🎯 ОБЩЕЕ УЛУЧШЕНИЕ: {}x", results.improvement_factors.overall_improvement_factor);
} 