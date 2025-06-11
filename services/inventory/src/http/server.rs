use crate::http::handlers::*;
use crate::repositories::{InventoryRepository, ItemRepository, ItemTemplateRepository};
use crate::services::{InventoryService, ItemService};
use axum::{
    routing::{get, post, put, delete},
    Router,
};
use std::net::SocketAddr;
use std::sync::Arc;
use tower_http::trace::TraceLayer;

/// Создает и настраивает HTTP сервер для инвентаря
pub async fn create_http_server<I, T, V>(
    inventory_service: Arc<InventoryService<V, I>>,
    item_service: Arc<ItemService<I, T, V>>,
    addr: SocketAddr,
) -> std::io::Result<()>
where
    I: ItemRepository + 'static,
    T: ItemTemplateRepository + 'static,
    V: InventoryRepository + 'static,
{
    let state = Arc::new(AppState {
        inventory_service,
        item_service,
    });

    let app = Router::new()
        // Маршруты для инвентарей
        .route("/inventories", get(get_all_inventories::<I, T, V>))
        .route("/inventories", post(create_inventory::<I, T, V>))
        .route("/inventories/:inventory_id", get(get_inventory::<I, T, V>))
        .route("/inventories/:inventory_id", put(update_inventory::<I, T, V>))
        .route("/inventories/:inventory_id", delete(delete_inventory::<I, T, V>))
        .route("/owners/:owner_type/:owner_id/inventories", get(get_owner_inventories::<I, T, V>))
        
        // Маршруты для предметов
        .route("/inventories/:inventory_id/items", get(get_inventory_items::<I, T, V>))
        .route("/inventories/:inventory_id/items", post(add_item::<I, T, V>))
        .route("/items/:item_id", get(get_item::<I, T, V>))
        .route("/items/:item_id", put(update_item::<I, T, V>))
        .route("/items/:item_id", delete(delete_item::<I, T, V>))
        .route("/items/:item_id/move/:target_inventory_id", post(move_item::<I, T, V>))
        .route("/items/:item_id/split", post(split_stack::<I, T, V>))
        .route("/items/:source_item_id/stack/:target_item_id", post(stack_items::<I, T, V>))
        
        // Маршруты для шаблонов предметов
        .route("/templates", get(get_all_item_templates::<I, T, V>))
        .route("/templates", post(create_item_template::<I, T, V>))
        .route("/templates/:template_id", get(get_item_template::<I, T, V>))
        .route("/templates/:template_id", put(update_item_template::<I, T, V>))
        .route("/templates/:template_id", delete(delete_item_template::<I, T, V>))
        
        // Добавляем слой для трассировки запросов
        .layer(TraceLayer::new_for_http())
        .with_state(state);

    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))
} 