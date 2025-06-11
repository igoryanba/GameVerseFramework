use gameverse_inventory::proto::inventory::{
    inventory_service_client::InventoryServiceClient,
    GetInventoryRequest, GetInventoryItemsRequest, AddItemRequest,
    UpdateItemRequest, RemoveItemRequest, MoveItemRequest, GetItemTemplatesRequest,
    inventory_type,
};
use std::collections::HashMap;
use uuid::Uuid;
use gameverse_logging::Logger;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Настраиваем логирование
    let logger = Logger::new("inventory-client-example");
    logger.add_handler(gameverse_logging::ConsoleLogHandler).await;
    
    // Адрес gRPC сервера
    let addr = std::env::var("INVENTORY_SERVICE_URL")
        .unwrap_or_else(|_| "http://localhost:50052".to_string());
    
    logger.info("startup", &format!("Подключение к сервису инвентаря по адресу {}", addr)).await?;
    
    // Создаем клиент
    let mut client = InventoryServiceClient::connect(addr).await?;
    
    // Создаем тестовые данные
    let owner_id = "player-123";
    let owner_type = "player";
    let inventory_id = Uuid::new_v4().to_string();
    let template_id = Uuid::new_v4().to_string();
    
    // Создаем инвентарь (используя GRPC)
    logger.info("create_inventory", &format!("Создаем инвентарь с ID {}", inventory_id)).await?;
    
    // TODO: Добавить метод создания инвентаря в GRPC API
    
    // Получаем информацию об инвентаре
    logger.info("get_inventory", "Получаем информацию об инвентаре").await?;
    let get_inventory_response = client.get_inventory(GetInventoryRequest {
        inventory_id: inventory_id.clone(),
    }).await?;
    
    if let Some(inventory) = get_inventory_response.into_inner().inventory {
        logger.info("inventory", &format!("Получен инвентарь: ID={}, Name={}, Max Slots={}", 
            inventory.id, inventory.name, inventory.max_slots)).await?;
    } else {
        logger.error("inventory", "Инвентарь не найден").await?;
        return Ok(());
    }
    
    // Добавляем предмет в инвентарь
    logger.info("add_item", "Добавляем предмет в инвентарь").await?;
    let add_item_response = client.add_item(AddItemRequest {
        inventory_id: inventory_id.clone(),
        template_id: template_id.clone(),
        position: 0,
        quantity: 10,
        metadata: HashMap::new(),
    }).await?;
    
    let add_result = add_item_response.into_inner();
    if add_result.success {
        if let Some(item) = add_result.item {
            logger.info("item_added", &format!("Предмет добавлен: ID={}, Position={}, Quantity={}", 
                item.id, item.position, item.quantity)).await?;
                
            // Теперь получаем все предметы в инвентаре
            logger.info("get_inventory_items", "Получаем все предметы в инвентаре").await?;
            let get_items_response = client.get_inventory_items(GetInventoryItemsRequest {
                inventory_id: inventory_id.clone(),
            }).await?;
            
            let items_result = get_items_response.into_inner();
            logger.info("items", &format!("Найдено {} предметов", items_result.total_count)).await?;
            
            for (i, item) in items_result.items.iter().enumerate() {
                logger.info("item", &format!("{}. ID={}, Position={}, Quantity={}", 
                    i+1, item.id, item.position, item.quantity)).await?;
            }
            
            // Обновляем предмет
            logger.info("update_item", "Обновляем количество предмета").await?;
            let update_response = client.update_item(UpdateItemRequest {
                item_id: item.id.clone(),
                quantity: 5,
                durability: 90.0,
                metadata: HashMap::new(),
            }).await?;
            
            let update_result = update_response.into_inner();
            if update_result.success {
                if let Some(updated_item) = update_result.item {
                    logger.info("item_updated", &format!("Предмет обновлен: ID={}, Quantity={}, Durability={}", 
                        updated_item.id, updated_item.quantity, updated_item.durability)).await?;
                }
            } else {
                logger.error("update_error", &format!("Ошибка при обновлении предмета: {}", update_result.error_message)).await?;
            }
            
            // Удаляем предмет
            logger.info("remove_item", "Удаляем предмет").await?;
            let remove_response = client.remove_item(RemoveItemRequest {
                item_id: item.id.clone(),
                quantity: 0, // Удаляем полностью
            }).await?;
            
            let remove_result = remove_response.into_inner();
            if remove_result.success {
                logger.info("item_removed", "Предмет успешно удален").await?;
            } else {
                logger.error("remove_error", &format!("Ошибка при удалении предмета: {}", remove_result.error_message)).await?;
            }
        }
    } else {
        logger.error("add_error", &format!("Ошибка при добавлении предмета: {}", add_result.error_message)).await?;
    }
    
    // Получаем шаблоны предметов
    logger.info("get_templates", "Получаем все шаблоны предметов").await?;
    let templates_response = client.get_item_templates(GetItemTemplatesRequest {
        category: String::new(),
        page: 1,
        page_size: 20,
    }).await?;
    
    let templates_result = templates_response.into_inner();
    logger.info("templates", &format!("Найдено {} шаблонов предметов", templates_result.total_count)).await?;
    
    logger.info("done", "Тестирование завершено").await?;
    
    Ok(())
}