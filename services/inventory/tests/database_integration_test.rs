use gameverse_inventory::{
    Config,
    models::{CreateInventoryData, InventoryType, CreateItemTemplateData},
    repositories::{InventoryRepository, PostgresInventoryRepository, ItemTemplateRepository, PostgresItemTemplateRepository}
};
use sqlx::{PgPool, Row};
use std::collections::HashMap;

#[tokio::test]
async fn test_database_connection() {
    // Загружаем конфигурацию
    let config = Config::from_env();
    
    // Подключаемся к базе данных
    let pool = PgPool::connect(&config.database.url)
        .await
        .expect("Не удалось подключиться к базе данных");
    
    // Проверяем, что подключение работает
    let result = sqlx::query("SELECT 1 as test_value")
        .fetch_one(&pool)
        .await
        .expect("Ошибка выполнения тестового запроса");
    
    let test_value: i32 = result.get("test_value");
    assert_eq!(test_value, 1);
    
    println!("✅ Подключение к базе данных работает!");
}

#[tokio::test]
async fn test_inventory_crud_operations() {
    // Загружаем конфигурацию и подключаемся к БД
    let config = Config::from_env();
    let pool = PgPool::connect(&config.database.url)
        .await
        .expect("Не удалось подключиться к базе данных");
    
    let inventory_repo = PostgresInventoryRepository::new(pool);
    
    // Создаем тестовый инвентарь
    let create_data = CreateInventoryData {
        owner_id: "test_player_123".to_string(),
        owner_type: "player".to_string(),
        inventory_type: InventoryType::Player,
        max_weight: 100.0,
        max_slots: 50,
        name: "Тестовый инвентарь".to_string(),
        metadata: HashMap::new(),
    };
    
    let created_inventory = inventory_repo.create(create_data)
        .await
        .expect("Ошибка создания инвентаря");
    
    println!("✅ Инвентарь создан: ID={}", created_inventory.id);
    
    // Получаем инвентарь по ID
    let found_inventory = inventory_repo.find_by_id(created_inventory.id)
        .await
        .expect("Ошибка получения инвентаря по ID");
    
    assert_eq!(found_inventory.id, created_inventory.id);
    assert_eq!(found_inventory.name, "Тестовый инвентарь");
    
    println!("✅ Инвентарь найден по ID");
    
    // Находим инвентари по владельцу
    let owner_inventories = inventory_repo.find_by_owner("test_player_123", "player")
        .await
        .expect("Ошибка поиска инвентарей по владельцу");
    
    assert!(!owner_inventories.is_empty());
    println!("✅ Найдено {} инвентарей для владельца", owner_inventories.len());
    
    // Удаляем тестовый инвентарь
    inventory_repo.delete(created_inventory.id)
        .await
        .expect("Ошибка удаления инвентаря");
    
    println!("✅ Инвентарь удален");
    
    // Проверяем, что инвентарь действительно удален
    let deleted_result = inventory_repo.find_by_id(created_inventory.id).await;
    assert!(deleted_result.is_err());
    
    println!("✅ Подтверждено: инвентарь удален из базы данных");
}

#[tokio::test]
async fn test_item_template_operations() {
    // Загружаем конфигурацию и подключаемся к БД
    let config = Config::from_env();
    let pool = PgPool::connect(&config.database.url)
        .await
        .expect("Не удалось подключиться к базе данных");
    
    let template_repo = PostgresItemTemplateRepository::new(pool);
    
    // Создаем тестовый шаблон предмета
    let create_data = CreateItemTemplateData {
        name: "Тестовое оружие".to_string(),
        description: "Описание тестового оружия".to_string(),
        weight: 2.5,
        stackable: false,
        max_stack: 1,
        max_durability: 100.0,
        icon: "weapon_icon.png".to_string(),
        category: "weapon".to_string(),
        properties: HashMap::new(),
    };
    
    let created_template = template_repo.create(create_data)
        .await
        .expect("Ошибка создания шаблона предмета");
    
    println!("✅ Шаблон предмета создан: ID={}", created_template.id);
    
    // Получаем шаблон по ID
    let found_template = template_repo.find_by_id(created_template.id)
        .await
        .expect("Ошибка получения шаблона по ID");
    
    assert_eq!(found_template.id, created_template.id);
    assert_eq!(found_template.name, "Тестовое оружие");
    assert_eq!(found_template.weight, 2.5);
    
    println!("✅ Шаблон найден по ID");
    
    // Удаляем тестовый шаблон
    template_repo.delete(created_template.id)
        .await
        .expect("Ошибка удаления шаблона");
    
    println!("✅ Шаблон предмета удален");
} 