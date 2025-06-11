use crate::http::utils::*;
use crate::models::{InventoryType, CreateInventoryData, UpdateInventoryData};
use crate::models::{CreateItemData, UpdateItemData};
use crate::models::{CreateItemTemplateData, UpdateItemTemplateData};
use crate::repositories::{InventoryRepository, ItemRepository, ItemTemplateRepository};
use crate::repositories::{PostgresInventoryRepository, PostgresItemRepository, PostgresItemTemplateRepository};
use crate::services::{InventoryService, ItemService, ServiceError};
use axum::{
    extract::{Path, Query, State},
    http::StatusCode,
    response::{IntoResponse, Response},
    Json,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use uuid::Uuid;

/// Состояние приложения, содержащее все необходимые сервисы
#[derive(Clone)]
pub struct AppState<I, T, V>
where
    I: ItemRepository,
    T: ItemTemplateRepository, 
    V: InventoryRepository,
{
    pub inventory_service: Arc<InventoryService<V, I>>,
    pub item_service: Arc<ItemService<I, T, V>>,
}

/// Type alias для состояния приложения с PostgreSQL репозиториями
pub type AppStatePostgres = AppState<PostgresItemRepository, PostgresItemTemplateRepository, PostgresInventoryRepository>;

/// Параметры для пагинации
#[derive(Deserialize)]
pub struct PaginationParams {
    pub page: Option<i32>,
    pub page_size: Option<i32>,
}

/// Параметры для фильтрации по категории
#[derive(Deserialize)]
pub struct CategoryParams {
    pub category: Option<String>,
}

/// Ответ с ошибкой
#[derive(Serialize)]
pub struct ErrorResponse {
    pub error: String,
}

/// Преобразование ошибки сервиса в ответ HTTP
impl IntoResponse for ServiceError {
    fn into_response(self) -> Response {
        let (status, error_message) = match self {
            ServiceError::NotFound { entity_type, entity_id } => (
                StatusCode::NOT_FOUND,
                format!("{} с ID {} не найден", entity_type, entity_id),
            ),
            ServiceError::ValidationError(msg) => (StatusCode::BAD_REQUEST, msg),
            ServiceError::NotEnoughSpace => (
                StatusCode::PRECONDITION_FAILED,
                "Недостаточно места в инвентаре".to_string(),
            ),
            ServiceError::NotEnoughItems { required, available } => (
                StatusCode::PRECONDITION_FAILED,
                format!("Недостаточно предметов: требуется {}, доступно {}", required, available),
            ),
            ServiceError::ItemNotSuitable(msg) => (
                StatusCode::PRECONDITION_FAILED,
                format!("Предмет не подходит для этой операции: {}", msg),
            ),
            ServiceError::SlotOccupied(slot) => (
                StatusCode::PRECONDITION_FAILED,
                format!("Слот {} уже занят", slot),
            ),
            _ => (
                StatusCode::INTERNAL_SERVER_ERROR,
                "Внутренняя ошибка сервера".to_string(),
            ),
        };

        let body = Json(ErrorResponse {
            error: error_message,
        });

        (status, body).into_response()
    }
}

/// Преобразует строку в Uuid, возвращая ошибку в случае неудачи
fn parse_uuid(id: &str) -> Result<Uuid, Response> {
    Uuid::parse_str(id).map_err(|_| {
        (
            StatusCode::BAD_REQUEST,
            Json(ErrorResponse {
                error: format!("Некорректный UUID: {}", id),
            }),
        )
            .into_response()
    })
}

/// Преобразует строку в InventoryType, возвращая ошибку в случае неудачи
fn parse_inventory_type(inv_type: &str) -> Result<InventoryType, Response> {
    match inv_type.to_lowercase().as_str() {
        "player" => Ok(InventoryType::Player),
        "vehicle" => Ok(InventoryType::Vehicle),
        "container" => Ok(InventoryType::Container),
        "shop" => Ok(InventoryType::Shop),
        "temp" => Ok(InventoryType::Temp),
        _ => Err(
            (
                StatusCode::BAD_REQUEST,
                Json(ErrorResponse {
                    error: format!("Некорректный тип инвентаря: {}", inv_type),
                }),
            )
                .into_response(),
        ),
    }
}

// ===== Обработчики для инвентарей =====

/// Получение всех инвентарей с пагинацией
pub async fn get_all_inventories<I, T, V>(
    State(state): State<Arc<AppState<I, T, V>>>,
    Query(params): Query<PaginationParams>,
) -> Result<Json<Vec<InventoryJson>>, Response>
where
    I: ItemRepository,
    T: ItemTemplateRepository,
    V: InventoryRepository,
{
    let page = params.page.unwrap_or(1);
    let page_size = params.page_size.unwrap_or(20);

    let (inventories, _) = state
        .inventory_service
        .get_all_inventories(page, page_size)
        .await
        .map_err(|e| e.into_response())?;

    let inventory_jsons = inventories
        .iter()
        .map(|inventory| InventoryJson::from(inventory))
        .collect();

    Ok(Json(inventory_jsons))
}

/// Получение инвентаря по ID
pub async fn get_inventory<I, T, V>(
    State(state): State<Arc<AppState<I, T, V>>>,
    Path(inventory_id): Path<String>,
) -> Result<Json<InventoryJson>, Response>
where
    I: ItemRepository,
    T: ItemTemplateRepository,
    V: InventoryRepository,
{
    let uuid = parse_uuid(&inventory_id)?;

    let inventory = state
        .inventory_service
        .get_inventory(uuid)
        .await
        .map_err(|e| e.into_response())?;

    Ok(Json(InventoryJson::from(&inventory)))
}

/// Создание нового инвентаря
pub async fn create_inventory<I, T, V>(
    State(state): State<Arc<AppState<I, T, V>>>,
    Json(payload): Json<CreateInventoryJson>,
) -> Result<Json<InventoryJson>, Response>
where
    I: ItemRepository,
    T: ItemTemplateRepository,
    V: InventoryRepository,
{
    let inventory_type = parse_inventory_type(&payload.inventory_type)?;

    let data = CreateInventoryData {
        owner_id: payload.owner_id,
        owner_type: payload.owner_type,
        inventory_type,
        max_weight: payload.max_weight,
        max_slots: payload.max_slots,
        name: payload.name,
        metadata: payload.metadata.unwrap_or_default(),
    };

    let inventory = state
        .inventory_service
        .create_inventory(data)
        .await
        .map_err(|e| e.into_response())?;

    Ok(Json(InventoryJson::from(&inventory)))
}

/// Обновление инвентаря
pub async fn update_inventory<I, T, V>(
    State(state): State<Arc<AppState<I, T, V>>>,
    Path(inventory_id): Path<String>,
    Json(payload): Json<UpdateInventoryJson>,
) -> Result<Json<InventoryJson>, Response>
where
    I: ItemRepository,
    T: ItemTemplateRepository,
    V: InventoryRepository,
{
    let uuid = parse_uuid(&inventory_id)?;

    let data = UpdateInventoryData {
        max_weight: payload.max_weight,
        max_slots: payload.max_slots,
        name: payload.name,
        metadata: payload.metadata,
    };

    let inventory = state
        .inventory_service
        .update_inventory(uuid, data)
        .await
        .map_err(|e| e.into_response())?;

    Ok(Json(InventoryJson::from(&inventory)))
}

/// Удаление инвентаря
pub async fn delete_inventory<I, T, V>(
    State(state): State<Arc<AppState<I, T, V>>>,
    Path(inventory_id): Path<String>,
) -> Result<StatusCode, Response>
where
    I: ItemRepository,
    T: ItemTemplateRepository,
    V: InventoryRepository,
{
    let uuid = parse_uuid(&inventory_id)?;

    state
        .inventory_service
        .delete_inventory(uuid)
        .await
        .map_err(|e| e.into_response())?;

    Ok(StatusCode::NO_CONTENT)
}

/// Получение инвентарей по владельцу
pub async fn get_owner_inventories<I, T, V>(
    State(state): State<Arc<AppState<I, T, V>>>,
    Path((owner_type, owner_id)): Path<(String, String)>,
) -> Result<Json<Vec<InventoryJson>>, Response>
where
    I: ItemRepository,
    T: ItemTemplateRepository,
    V: InventoryRepository,
{
    let inventories = state
        .inventory_service
        .get_owner_inventories(&owner_id, &owner_type)
        .await
        .map_err(|e| e.into_response())?;

    let inventory_jsons = inventories
        .iter()
        .map(|inventory| InventoryJson::from(inventory))
        .collect();

    Ok(Json(inventory_jsons))
}

// ===== Обработчики для предметов =====

/// Получение всех предметов в инвентаре
pub async fn get_inventory_items<I, T, V>(
    State(state): State<Arc<AppState<I, T, V>>>,
    Path(inventory_id): Path<String>,
) -> Result<Json<Vec<ItemJson>>, Response>
where
    I: ItemRepository,
    T: ItemTemplateRepository,
    V: InventoryRepository,
{
    let uuid = parse_uuid(&inventory_id)?;

    let items = state
        .item_service
        .get_inventory_items(uuid)
        .await
        .map_err(|e| e.into_response())?;

    let item_jsons = items.iter().map(|item| ItemJson::from(item)).collect();

    Ok(Json(item_jsons))
}

/// Получение предмета по ID
pub async fn get_item<I, T, V>(
    State(state): State<Arc<AppState<I, T, V>>>,
    Path(item_id): Path<String>,
) -> Result<Json<ItemJson>, Response>
where
    I: ItemRepository,
    T: ItemTemplateRepository,
    V: InventoryRepository,
{
    let uuid = parse_uuid(&item_id)?;

    let item = state
        .item_service
        .get_item(uuid)
        .await
        .map_err(|e| e.into_response())?;

    Ok(Json(ItemJson::from(&item)))
}

/// Добавление предмета в инвентарь
pub async fn add_item<I, T, V>(
    State(state): State<Arc<AppState<I, T, V>>>,
    Path(inventory_id): Path<String>,
    Json(payload): Json<CreateItemJson>,
) -> Result<Json<ItemJson>, Response>
where
    I: ItemRepository,
    T: ItemTemplateRepository,
    V: InventoryRepository,
{
    let inventory_uuid = parse_uuid(&inventory_id)?;
    let template_uuid = parse_uuid(&payload.template_id)?;

    let data = CreateItemData {
        inventory_id: inventory_uuid,
        template_id: template_uuid,
        position: payload.position,
        quantity: payload.quantity,
        durability: 100.0, // Максимальная прочность для нового предмета
        metadata: payload.metadata.unwrap_or_default(),
    };

    let item = state
        .item_service
        .create_item(data)
        .await
        .map_err(|e| e.into_response())?;

    Ok(Json(ItemJson::from(&item)))
}

/// Обновление предмета
pub async fn update_item<I, T, V>(
    State(state): State<Arc<AppState<I, T, V>>>,
    Path(item_id): Path<String>,
    Json(payload): Json<UpdateItemJson>,
) -> Result<Json<ItemJson>, Response>
where
    I: ItemRepository,
    T: ItemTemplateRepository,
    V: InventoryRepository,
{
    let uuid = parse_uuid(&item_id)?;

    let data = UpdateItemData {
        position: payload.position,
        quantity: payload.quantity,
        durability: payload.durability,
        metadata: payload.metadata,
    };

    let item = state
        .item_service
        .update_item(uuid, data)
        .await
        .map_err(|e| e.into_response())?;

    Ok(Json(ItemJson::from(&item)))
}

/// Удаление предмета
pub async fn delete_item<I, T, V>(
    State(state): State<Arc<AppState<I, T, V>>>,
    Path(item_id): Path<String>,
) -> Result<StatusCode, Response>
where
    I: ItemRepository,
    T: ItemTemplateRepository,
    V: InventoryRepository,
{
    let uuid = parse_uuid(&item_id)?;

    state
        .item_service
        .delete_item(uuid)
        .await
        .map_err(|e| e.into_response())?;

    Ok(StatusCode::NO_CONTENT)
}

/// Перемещение предмета в другой инвентарь
pub async fn move_item<I, T, V>(
    State(state): State<Arc<AppState<I, T, V>>>,
    Path((item_id, target_inventory_id)): Path<(String, String)>,
    Json(payload): Json<MoveItemJson>,
) -> Result<Json<ItemJson>, Response>
where
    I: ItemRepository,
    T: ItemTemplateRepository,
    V: InventoryRepository,
{
    let item_uuid = parse_uuid(&item_id)?;
    let target_inventory_uuid = parse_uuid(&target_inventory_id)?;

    let item = state
        .item_service
        .move_item(item_uuid, target_inventory_uuid, payload.target_position)
        .await
        .map_err(|e| e.into_response())?;

    Ok(Json(ItemJson::from(&item)))
}

/// Разделение стака предметов
pub async fn split_stack<I, T, V>(
    State(state): State<Arc<AppState<I, T, V>>>,
    Path(item_id): Path<String>,
    Json(payload): Json<SplitStackJson>,
) -> Result<Json<ItemJson>, Response>
where
    I: ItemRepository,
    T: ItemTemplateRepository,
    V: InventoryRepository,
{
    let item_uuid = parse_uuid(&item_id)?;

    let new_item = state
        .item_service
        .split_stack(item_uuid, payload.quantity, payload.new_position)
        .await
        .map_err(|e| e.into_response())?;

    Ok(Json(ItemJson::from(&new_item)))
}

/// Стакирование предметов
pub async fn stack_items<I, T, V>(
    State(state): State<Arc<AppState<I, T, V>>>,
    Path((source_item_id, target_item_id)): Path<(String, String)>,
) -> Result<StatusCode, Response>
where
    I: ItemRepository,
    T: ItemTemplateRepository,
    V: InventoryRepository,
{
    let source_uuid = parse_uuid(&source_item_id)?;
    let target_uuid = parse_uuid(&target_item_id)?;

    state
        .item_service
        .stack_items(source_uuid, target_uuid)
        .await
        .map_err(|e| e.into_response())?;

    Ok(StatusCode::NO_CONTENT)
}

// ===== Обработчики для шаблонов предметов =====

/// Получение всех шаблонов предметов
pub async fn get_all_item_templates<I, T, V>(
    State(state): State<Arc<AppState<I, T, V>>>,
    Query(pagination): Query<PaginationParams>,
    Query(category): Query<CategoryParams>,
) -> Result<Json<Vec<ItemTemplateJson>>, Response>
where
    I: ItemRepository,
    T: ItemTemplateRepository,
    V: InventoryRepository,
{
    // Если указана категория, получаем шаблоны по категории
    if let Some(category) = category.category {
        let templates = state
            .item_service
            .get_item_templates_by_category(&category)
            .await
            .map_err(|e| e.into_response())?;

        let template_jsons = templates
            .iter()
            .map(|template| ItemTemplateJson::from(template))
            .collect();

        return Ok(Json(template_jsons));
    }

    // Иначе получаем все шаблоны с пагинацией
    let page = pagination.page.unwrap_or(1);
    let page_size = pagination.page_size.unwrap_or(20);

    let (templates, _) = state
        .item_service
        .get_all_item_templates(page, page_size)
        .await
        .map_err(|e| e.into_response())?;

    let template_jsons = templates
        .iter()
        .map(|template| ItemTemplateJson::from(template))
        .collect();

    Ok(Json(template_jsons))
}

/// Получение шаблона предмета по ID
pub async fn get_item_template<I, T, V>(
    State(state): State<Arc<AppState<I, T, V>>>,
    Path(template_id): Path<String>,
) -> Result<Json<ItemTemplateJson>, Response>
where
    I: ItemRepository,
    T: ItemTemplateRepository,
    V: InventoryRepository,
{
    let uuid = parse_uuid(&template_id)?;

    let template = state
        .item_service
        .get_item_template(uuid)
        .await
        .map_err(|e| e.into_response())?;

    Ok(Json(ItemTemplateJson::from(&template)))
}

/// Создание нового шаблона предмета
pub async fn create_item_template<I, T, V>(
    State(state): State<Arc<AppState<I, T, V>>>,
    Json(payload): Json<CreateItemTemplateJson>,
) -> Result<Json<ItemTemplateJson>, Response>
where
    I: ItemRepository,
    T: ItemTemplateRepository,
    V: InventoryRepository,
{
    let data = CreateItemTemplateData {
        name: payload.name,
        description: payload.description,
        weight: payload.weight,
        stackable: payload.stackable,
        max_stack: payload.max_stack,
        max_durability: payload.max_durability,
        icon: payload.icon,
        category: payload.category,
        properties: payload.properties.unwrap_or_default(),
    };

    let template = state
        .item_service
        .create_item_template(data)
        .await
        .map_err(|e| e.into_response())?;

    Ok(Json(ItemTemplateJson::from(&template)))
}

/// Обновление шаблона предмета
pub async fn update_item_template<I, T, V>(
    State(state): State<Arc<AppState<I, T, V>>>,
    Path(template_id): Path<String>,
    Json(payload): Json<UpdateItemTemplateJson>,
) -> Result<Json<ItemTemplateJson>, Response>
where
    I: ItemRepository,
    T: ItemTemplateRepository,
    V: InventoryRepository,
{
    let uuid = parse_uuid(&template_id)?;

    let data = UpdateItemTemplateData {
        name: payload.name,
        description: payload.description,
        weight: payload.weight,
        stackable: payload.stackable,
        max_stack: payload.max_stack,
        max_durability: payload.max_durability,
        icon: payload.icon,
        category: payload.category,
        properties: payload.properties,
    };

    let template = state
        .item_service
        .update_item_template(uuid, data)
        .await
        .map_err(|e| e.into_response())?;

    Ok(Json(ItemTemplateJson::from(&template)))
}

/// Удаление шаблона предмета
pub async fn delete_item_template<I, T, V>(
    State(state): State<Arc<AppState<I, T, V>>>,
    Path(template_id): Path<String>,
) -> Result<StatusCode, Response>
where
    I: ItemRepository,
    T: ItemTemplateRepository,
    V: InventoryRepository,
{
    let uuid = parse_uuid(&template_id)?;

    state
        .item_service
        .delete_item_template(uuid)
        .await
        .map_err(|e| e.into_response())?;

    Ok(StatusCode::NO_CONTENT)
}

/// JSON для перемещения предмета
#[derive(Deserialize)]
pub struct MoveItemJson {
    pub target_position: i32,
}

/// JSON для разделения стака предметов
#[derive(Deserialize)]
pub struct SplitStackJson {
    pub quantity: i32,
    pub new_position: i32,
} 