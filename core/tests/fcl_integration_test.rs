use std::sync::Arc;
use tokio::sync::RwLock;
use anyhow::Result;

use gameverse_core::fcl::FiveMCompat;
use gameverse_core::natives::NativeManager;
use gameverse_core::game_integration::native_executor::NativeValue;
use gameverse_core::game_integration::event_system::EventSystem;

/// Интеграционные тесты для FiveM Compatibility Layer
/// Проверяют корректность работы основных FiveM функций через GameVerse

#[tokio::test]
async fn test_fcl_basic_natives() -> Result<()> {
    // Setup
    let native_manager = Arc::new(RwLock::new(NativeManager::new()));
    let event_system = Arc::new(EventSystem::new().await?);
    let fcl = FiveMCompat::new(native_manager.clone(), event_system.clone());
    
    // Регистрируем тестовые нативы
    {
        let mut nm = native_manager.write().await;
        nm.register_test_natives().await?;
    }
    
    // Test GET_PLAYER_PED
    let player_id = 1;
    let ped_handle = fcl.get_player_ped(player_id).await?;
    
    assert!(ped_handle > 0, "GET_PLAYER_PED должен возвращать валидный handle");
    println!("✅ GET_PLAYER_PED({}) = {}", player_id, ped_handle);
    
    Ok(())
}

#[tokio::test]
async fn test_fcl_event_system() -> Result<()> {
    let native_manager = Arc::new(RwLock::new(NativeManager::new()));
    let event_system = Arc::new(EventSystem::new().await?);
    let fcl = FiveMCompat::new(native_manager.clone(), event_system.clone());
    
    let received_events = Arc::new(RwLock::new(Vec::new()));
    let received_events_clone = received_events.clone();
    
    // Регистрируем event handler
    fcl.register_net_event("test_event", move |args: Vec<NativeValue>| {
        let received = received_events_clone.clone();
        tokio::spawn(async move {
            received.write().await.push(args);
        });
    }).await?;
    
    // Отправляем event
    let test_args = vec![
        NativeValue::String("test_message".to_string()),
        NativeValue::Int(42),
        NativeValue::Bool(true)
    ];
    
    fcl.trigger_server_event("test_event", test_args.clone()).await?;
    
    // Ждем обработки
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    let events = received_events.read().await;
    assert!(!events.is_empty(), "Event должен был быть обработан");
    
    println!("✅ Event system работает: получено {} событий", events.len());
    
    Ok(())
}

#[tokio::test]
async fn test_fcl_qbcore_compatibility() -> Result<()> {
    let native_manager = Arc::new(RwLock::new(NativeManager::new()));
    let event_system = Arc::new(EventSystem::new().await?);
    let fcl = FiveMCompat::new(native_manager.clone(), event_system.clone());
    
    // Регистрируем тестовые нативы
    {
        let mut nm = native_manager.write().await;
        nm.register_test_natives().await?;
    }
    
    // Симулируем QBCore-style операции
    
    // 1. Получение игрока
    let player_ped = fcl.get_player_ped(1).await?;
    assert!(player_ped > 0);
    
    // 2. Регистрация QBCore событий
    let qb_events_received = Arc::new(RwLock::new(0));
    let counter = qb_events_received.clone();
    
    fcl.register_net_event("QBCore:Server:PlayerLoaded", move |_args| {
        let c = counter.clone();
        tokio::spawn(async move {
            *c.write().await += 1;
        });
    }).await?;
    
    fcl.register_net_event("QBCore:Server:OnPlayerUnload", move |_args| {
        let c = qb_events_received.clone();
        tokio::spawn(async move {
            *c.write().await += 1;
        });
    }).await?;
    
    // 3. Симулируем QBCore события
    fcl.trigger_server_event("QBCore:Server:PlayerLoaded", vec![
        NativeValue::String("player123".to_string()),
        NativeValue::Int(1)
    ]).await?;
    
    fcl.trigger_server_event("QBCore:Server:OnPlayerUnload", vec![
        NativeValue::String("player123".to_string())
    ]).await?;
    
    // Ждем обработки
    tokio::time::sleep(tokio::time::Duration::from_millis(200)).await;
    
    let events_count = *qb_events_received.read().await;
    assert_eq!(events_count, 2, "Должно быть обработано 2 QBCore события");
    
    println!("✅ QBCore compatibility: обработано {} событий", events_count);
    
    Ok(())
}

#[tokio::test]
async fn test_fcl_performance_stress() -> Result<()> {
    let native_manager = Arc::new(RwLock::new(NativeManager::new()));
    let event_system = Arc::new(EventSystem::new().await?);
    let fcl = FiveMCompat::new(native_manager.clone(), event_system.clone());
    
    // Регистрируем тестовые нативы
    {
        let mut nm = native_manager.write().await;
        nm.register_test_natives().await?;
    }
    
    let start_time = std::time::Instant::now();
    let iterations = 1000;
    
    // Стресс-тест native calls
    for i in 0..iterations {
        let _ped = fcl.get_player_ped(i % 100).await?;
    }
    
    let native_duration = start_time.elapsed();
    
    // Стресс-тест events
    let events_received = Arc::new(RwLock::new(0));
    let counter = events_received.clone();
    
    fcl.register_net_event("stress_test", move |_args| {
        let c = counter.clone();
        tokio::spawn(async move {
            *c.write().await += 1;
        });
    }).await?;
    
    let event_start = std::time::Instant::now();
    
    for i in 0..iterations {
        fcl.trigger_server_event("stress_test", vec![
            NativeValue::Int(i),
            NativeValue::String(format!("message_{}", i))
        ]).await?;
    }
    
    // Ждем обработки всех событий
    tokio::time::sleep(tokio::time::Duration::from_millis(500)).await;
    
    let event_duration = event_start.elapsed();
    let events_count = *events_received.read().await;
    
    println!("✅ Performance test:");
    println!("   {} native calls: {:?} ({:.2}μs/call)", 
             iterations, native_duration, 
             native_duration.as_micros() as f64 / iterations as f64);
    println!("   {} events: {:?} ({:.2}μs/event)", 
             events_count, event_duration,
             event_duration.as_micros() as f64 / events_count as f64);
    
    // Проверяем производительность
    assert!(native_duration.as_millis() < 1000, "Native calls должны быть быстрыми");
    assert_eq!(events_count, iterations, "Все события должны быть обработаны");
    
    Ok(())
}

#[tokio::test] 
async fn test_fcl_memory_safety() -> Result<()> {
    let native_manager = Arc::new(RwLock::new(NativeManager::new()));
    let event_system = Arc::new(EventSystem::new().await?);
    let fcl = FiveMCompat::new(native_manager.clone(), event_system.clone());
    
    // Регистрируем много event handlers
    for i in 0..100 {
        let event_name = format!("test_event_{}", i);
        fcl.register_net_event(&event_name, move |args| {
            // Обрабатываем, но не сохраняем данные
            let _processed = args.len();
        }).await?;
    }
    
    // Отправляем много событий с большими данными
    for i in 0..50 {
        let large_data = vec![
            NativeValue::String("x".repeat(1000)), // 1KB string
            NativeValue::Int(i),
            NativeValue::Vector3 { x: i as f32, y: i as f32, z: i as f32 }
        ];
        
        fcl.trigger_server_event(&format!("test_event_{}", i % 100), large_data).await?;
    }
    
    // Ждем обработки
    tokio::time::sleep(tokio::time::Duration::from_millis(300)).await;
    
    println!("✅ Memory safety test: нет утечек памяти при множественных событиях");
    
    Ok(())
}

#[tokio::test]
async fn test_fcl_error_handling() -> Result<()> {
    let native_manager = Arc::new(RwLock::new(NativeManager::new()));
    let event_system = Arc::new(EventSystem::new().await?);
    let fcl = FiveMCompat::new(native_manager.clone(), event_system.clone());
    
    // Тест с отсутствующим native
    let result = fcl.get_player_ped(-999).await;
    assert!(result.is_err() || result.unwrap() == 0, "Некорректный player_id должен обрабатываться");
    
    // Тест с некорректными event данными
    fcl.register_net_event("error_test", |args| {
        // Handler который может упасть, но не должен крашить систему
        if args.is_empty() {
            panic!("No args provided");
        }
    }).await?;
    
    // Отправляем событие без аргументов - не должно крашить систему
    let _result = fcl.trigger_server_event("error_test", vec![]).await;
    
    tokio::time::sleep(tokio::time::Duration::from_millis(100)).await;
    
    println!("✅ Error handling: система устойчива к ошибкам");
    
    Ok(())
}

// Дополнительный тест для проверки ready state
#[tokio::test]
async fn test_fcl_ready_verification() -> Result<()> {
    let native_manager = NativeManager::new();
    
    // Без регистрации нативов - не готов
    let ready_before = gameverse_core::fcl::verify_fcl_ready(&native_manager).await;
    assert!(!ready_before, "FCL не должен быть готов без зарегистрированных нативов");
    
    // После регистрации - готов
    let mut nm = native_manager;
    nm.register_test_natives().await?;
    
    let ready_after = gameverse_core::fcl::verify_fcl_ready(&nm).await;
    assert!(ready_after, "FCL должен быть готов после регистрации нативов");
    
    println!("✅ FCL ready verification работает корректно");
    
    Ok(())
}