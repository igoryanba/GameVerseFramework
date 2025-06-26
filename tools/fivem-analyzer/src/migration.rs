use anyhow::Result;
use std::path::PathBuf;
use tracing::info;

use crate::types::*;

/// Генерирует план миграции на основе отчета анализа
pub fn generate_migration_plan(report: &AnalysisReport) -> Result<MigrationPlan> {
    info!("📋 Generating migration plan for: {}", report.resource_name);

    let migration_plan = MigrationPlan {
        server_name: report.resource_name.clone(),
        total_resources: 1,
        migration_phases: vec![
            MigrationPhase {
                phase_number: 1,
                title: "Анализ и подготовка".to_string(),
                description: format!("Подготовка к миграции ресурса {}", report.resource_name),
                resources: vec![report.resource_name.clone()],
                steps: generate_preparation_steps(report)?,
                estimated_time_hours: report.migration_complexity.estimated_migration_time_hours * 0.3,
                dependencies: vec![],
                validation_criteria: vec![
                    "Резервная копия создана".to_string(),
                    "Зависимости проанализированы".to_string(),
                ],
            },
            MigrationPhase {
                phase_number: 2,
                title: "Основная миграция".to_string(),
                description: "Конвертация основных компонентов".to_string(),
                resources: vec![report.resource_name.clone()],
                steps: generate_migration_steps(report)?,
                estimated_time_hours: report.migration_complexity.estimated_migration_time_hours * 0.6,
                dependencies: vec![1],
                validation_criteria: vec![
                    "Ресурс конвертирован".to_string(),
                    "Тесты пройдены".to_string(),
                ],
            },
            MigrationPhase {
                phase_number: 3,
                title: "Валидация и деплой".to_string(),
                description: "Финальная проверка и развертывание".to_string(),
                resources: vec![report.resource_name.clone()],
                steps: generate_validation_steps(report)?,
                estimated_time_hours: report.migration_complexity.estimated_migration_time_hours * 0.1,
                dependencies: vec![2],
                validation_criteria: vec![
                    "Все тесты пройдены".to_string(),
                    "Производительность соответствует ожиданиям".to_string(),
                ],
            },
        ],
        estimated_total_time_hours: report.migration_complexity.estimated_migration_time_hours,
        automation_percentage: report.migration_complexity.automation_percentage,
        required_tools: vec![
            "fivem-analyzer".to_string(),
            "gameverse-cli".to_string(),
        ],
        risk_assessment: assess_migration_risks(report),
        rollback_plan: create_rollback_plan(report),
        testing_strategy: create_testing_strategy(report),
    };

    Ok(migration_plan)
}

/// Генерирует автоматические скрипты миграции
pub fn generate_migration_scripts(plan: &MigrationPlan, _server_path: &PathBuf) -> Result<()> {
    info!("📜 Generating migration scripts for: {}", plan.server_name);
    
    // В реальной реализации здесь будет создание Bash/PowerShell скриптов
    // для автоматизации процесса миграции
    
    Ok(())
}

fn generate_preparation_steps(report: &AnalysisReport) -> Result<Vec<MigrationStep>> {
    Ok(vec![
        MigrationStep {
            step_number: 1,
            title: "Создание резервной копии".to_string(),
            description: format!("Резервное копирование ресурса {}", report.resource_name),
            automation_level: AutomationLevel::FullyAutomated,
            estimated_time_minutes: 10,
            prerequisites: vec![],
            commands: vec![
                format!("gameverse backup create --resource {}", report.resource_name)
            ],
            files_affected: vec![],
            validation_checks: vec!["Резервная копия создана".to_string()],
        },
        MigrationStep {
            step_number: 2,
            title: "Анализ зависимостей".to_string(),
            description: "Детальный анализ всех зависимостей ресурса".to_string(),
            automation_level: AutomationLevel::FullyAutomated,
            estimated_time_minutes: 5,
            prerequisites: vec!["Резервная копия".to_string()],
            commands: vec![
                format!("fivem-analyzer qbcore --path {} --dependencies", report.resource_name)
            ],
            files_affected: vec![],
            validation_checks: vec!["Все зависимости идентифицированы".to_string()],
        },
    ])
}

fn generate_migration_steps(report: &AnalysisReport) -> Result<Vec<MigrationStep>> {
    let automation_level = if report.migration_complexity.automation_percentage > 80.0 {
        AutomationLevel::FullyAutomated
    } else if report.migration_complexity.automation_percentage > 50.0 {
        AutomationLevel::SemiAutomated
    } else {
        AutomationLevel::ManualWithGuidance
    };

    Ok(vec![
        MigrationStep {
            step_number: 3,
            title: "Конвертация fxmanifest.lua".to_string(),
            description: "Преобразование в gameverse.toml".to_string(),
            automation_level: AutomationLevel::FullyAutomated,
            estimated_time_minutes: 5,
            prerequisites: vec!["Анализ завершен".to_string()],
            commands: vec![
                "gameverse convert manifest".to_string()
            ],
            files_affected: vec![PathBuf::from("gameverse.toml")],
            validation_checks: vec!["gameverse.toml создан".to_string()],
        },
        MigrationStep {
            step_number: 4,
            title: "Миграция Lua скриптов".to_string(),
            description: "Адаптация скриптов для GameVerse".to_string(),
            automation_level: automation_level.clone(),
            estimated_time_minutes: (report.migration_complexity.lua_script_complexity * 10.0) as u32,
            prerequisites: vec!["Манифест конвертирован".to_string()],
            commands: vec![
                "gameverse migrate scripts".to_string()
            ],
            files_affected: vec![],
            validation_checks: vec!["Скрипты адаптированы".to_string()],
        },
        MigrationStep {
            step_number: 5,
            title: "Миграция UI".to_string(),
            description: "Конвертация CEF в WebAssembly".to_string(),
            automation_level: automation_level,
            estimated_time_minutes: (report.migration_complexity.ui_complexity * 15.0) as u32,
            prerequisites: vec!["Скрипты адаптированы".to_string()],
            commands: vec![
                "gameverse migrate ui".to_string()
            ],
            files_affected: vec![],
            validation_checks: vec!["UI конвертирован".to_string()],
        },
    ])
}

fn generate_validation_steps(report: &AnalysisReport) -> Result<Vec<MigrationStep>> {
    Ok(vec![
        MigrationStep {
            step_number: 6,
            title: "Запуск тестов".to_string(),
            description: "Проверка функциональности мигрированного ресурса".to_string(),
            automation_level: AutomationLevel::FullyAutomated,
            estimated_time_minutes: 15,
            prerequisites: vec!["Миграция завершена".to_string()],
            commands: vec![
                format!("gameverse test --resource {}", report.resource_name)
            ],
            files_affected: vec![],
            validation_checks: vec!["Все тесты пройдены".to_string()],
        },
        MigrationStep {
            step_number: 7,
            title: "Бенчмарк производительности".to_string(),
            description: "Проверка улучшений производительности".to_string(),
            automation_level: AutomationLevel::FullyAutomated,
            estimated_time_minutes: 10,
            prerequisites: vec!["Тесты пройдены".to_string()],
            commands: vec![
                format!("gameverse benchmark --resource {}", report.resource_name)
            ],
            files_affected: vec![],
            validation_checks: vec![
                format!("Производительность улучшена в {}x раз", 
                       report.estimated_improvement.overall_improvement_factor)
            ],
        },
    ])
}

fn assess_migration_risks(report: &AnalysisReport) -> RiskAssessment {
    let overall_risk = if report.migration_complexity.overall_score < 3.0 {
        RiskLevel::Low
    } else if report.migration_complexity.overall_score < 7.0 {
        RiskLevel::Medium
    } else {
        RiskLevel::High
    };

    RiskAssessment {
        overall_risk_level: overall_risk,
        data_loss_risk: RiskLevel::Low, // Благодаря резервным копиям
        downtime_risk: RiskLevel::Medium,
        performance_degradation_risk: RiskLevel::Low, // GameVerse быстрее
        compatibility_risk: if report.compatibility.gameverse_compatibility_score > 0.8 {
            RiskLevel::Low
        } else {
            RiskLevel::Medium
        },
        mitigation_strategies: vec![
            "Полное резервное копирование".to_string(),
            "Поэтапная миграция".to_string(),
            "Автоматическое тестирование".to_string(),
            "План отката".to_string(),
        ],
    }
}

fn create_rollback_plan(report: &AnalysisReport) -> RollbackPlan {
    RollbackPlan {
        backup_strategy: "Автоматическое резервное копирование перед каждым шагом".to_string(),
        rollback_steps: vec![
            "Остановка GameVerse".to_string(),
            format!("Восстановление {} из резервной копии", report.resource_name),
            "Перезапуск FiveM".to_string(),
            "Проверка работоспособности".to_string(),
        ],
        estimated_rollback_time_hours: 0.5,
        data_recovery_plan: "Восстановление из последней валидной резервной копии".to_string(),
    }
}

fn create_testing_strategy(report: &AnalysisReport) -> TestingStrategy {
    TestingStrategy {
        test_phases: vec![
            "Модульное тестирование".to_string(),
            "Интеграционное тестирование".to_string(),
            "Нагрузочное тестирование".to_string(),
        ],
        automated_tests: vec![
            "Проверка API endpoints".to_string(),
            "Тестирование событий".to_string(),
            "Валидация данных".to_string(),
        ],
        manual_tests: vec![
            "Проверка UI".to_string(),
            "Тестирование игровых сценариев".to_string(),
        ],
        performance_benchmarks: vec![
            format!("Память: улучшение в {}x", report.estimated_improvement.memory_reduction_factor),
            format!("CPU: улучшение в {}x", report.estimated_improvement.cpu_efficiency_factor),
            format!("UI: улучшение в {}x", report.estimated_improvement.ui_performance_factor),
        ],
        user_acceptance_criteria: vec![
            "Функциональность сохранена на 100%".to_string(),
            format!("Производительность улучшена минимум в {}x", 
                   report.estimated_improvement.overall_improvement_factor),
            "UI более отзывчив".to_string(),
        ],
    }
} 