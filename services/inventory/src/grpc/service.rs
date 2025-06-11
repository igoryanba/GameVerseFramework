use crate::proto::inventory::inventory_service_server::{InventoryService, InventoryServiceServer};
use crate::proto::inventory::*;
use crate::services::{InventoryService as InvService, ItemService as ItemSvc, ServiceError};
use crate::repositories::{InventoryRepository, ItemRepository, ItemTemplateRepository};
use crate::grpc::utils::*;
use std::sync::Arc;
use tonic::{Request, Response, Status};
use uuid::Uuid;

/// Реализация gRPC сервера для управления инвентарем
pub struct GrpcInventoryService<I, T, V>
where
    I: ItemRepository + 'static,
    T: ItemTemplateRepository + 'static,
    V: InventoryRepository + 'static,
{
    inventory_service: Arc<InvService<V, I>>,
    item_service: Arc<ItemSvc<I, T, V>>,
}

impl<I, T, V> GrpcInventoryService<I, T, V>
where
    I: ItemRepository + 'static,
    T: ItemTemplateRepository + 'static,
    V: InventoryRepository + 'static,
{
    /// Создает новый gRPC сервер для инвентаря
    pub fn new(
        inventory_service: Arc<InvService<V, I>>,
        item_service: Arc<ItemSvc<I, T, V>>,
    ) -> Self {
        Self {
            inventory_service,
            item_service,
        }
    }
    
    /// Создает серверную часть gRPC
    pub fn create_server(
        inventory_service: Arc<InvService<V, I>>,
        item_service: Arc<ItemSvc<I, T, V>>,
    ) -> InventoryServiceServer<Self> {
        let service = Self::new(inventory_service, item_service);
        InventoryServiceServer::new(service)
    }
    
    /// Преобразует ошибку сервиса в ошибку gRPC
    fn service_error_to_status(err: ServiceError) -> Status {
        match err {
            ServiceError::NotFound { entity_type, entity_id } => {
                Status::not_found(format!("{} с ID {} не найден", entity_type, entity_id))
            },
            ServiceError::ValidationError(msg) => {
                Status::invalid_argument(msg)
            },
            ServiceError::NotEnoughSpace => {
                Status::failed_precondition("Недостаточно места в инвентаре")
            },
            ServiceError::NotEnoughItems { required, available } => {
                Status::failed_precondition(format!("Недостаточно предметов: требуется {}, доступно {}", required, available))
            },
            ServiceError::ItemNotSuitable(msg) => {
                Status::failed_precondition(format!("Предмет не подходит для этой операции: {}", msg))
            },
            ServiceError::SlotOccupied(slot) => {
                Status::failed_precondition(format!("Слот {} уже занят", slot))
            },
            _ => Status::internal("Внутренняя ошибка сервера"),
        }
    }
}

#[tonic::async_trait]
impl<I, T, V> InventoryService for GrpcInventoryService<I, T, V>
where
    I: ItemRepository + 'static,
    T: ItemTemplateRepository + 'static,
    V: InventoryRepository + 'static,
{
    async fn get_inventory(
        &self,
        request: Request<GetInventoryRequest>,
    ) -> Result<Response<GetInventoryResponse>, Status> {
        let req = request.into_inner();
        
        // Получаем UUID инвентаря
        let inventory_id = Uuid::parse_str(&req.inventory_id)
            .map_err(|_| Status::invalid_argument(format!("Некорректный UUID инвентаря: {}", req.inventory_id)))?;
        
        // Получаем инвентарь
        let inventory = self.inventory_service.get_inventory(inventory_id)
            .await
            .map_err(Self::service_error_to_status)?;
        
        // Конвертируем в protobuf модель
        let proto_inventory = inventory_to_proto(&inventory);
        
        Ok(Response::new(GetInventoryResponse {
            inventory: Some(proto_inventory),
        }))
    }
    
    async fn get_inventory_items(
        &self,
        request: Request<GetInventoryItemsRequest>,
    ) -> Result<Response<GetInventoryItemsResponse>, Status> {
        let req = request.into_inner();
        
        // Получаем UUID инвентаря
        let inventory_id = Uuid::parse_str(&req.inventory_id)
            .map_err(|_| Status::invalid_argument(format!("Некорректный UUID инвентаря: {}", req.inventory_id)))?;
        
        // Получаем предметы в инвентаре
        let items = self.item_service.get_inventory_items(inventory_id)
            .await
            .map_err(Self::service_error_to_status)?;
        
        // Конвертируем в protobuf модели
        let proto_items = items.iter().map(item_to_proto).collect();
        
        Ok(Response::new(GetInventoryItemsResponse {
            items: proto_items,
            total_count: items.len() as i32,
        }))
    }
    
    async fn add_item(
        &self,
        request: Request<AddItemRequest>,
    ) -> Result<Response<AddItemResponse>, Status> {
        let req = request.into_inner();
        
        // Создаем данные для создания предмета
        let create_data = match create_item_data_from_request(
            req.inventory_id,
            req.template_id,
            req.position,
            req.quantity,
            req.metadata,
        ) {
            Ok(data) => data,
            Err(e) => return Ok(Response::new(AddItemResponse {
                item: None,
                success: false,
                error_message: e,
            })),
        };
        
        // Создаем предмет
        match self.item_service.create_item(create_data).await {
            Ok(item) => Ok(Response::new(AddItemResponse {
                item: Some(item_to_proto(&item)),
                success: true,
                error_message: String::new(),
            })),
            Err(e) => Ok(Response::new(AddItemResponse {
                item: None,
                success: false,
                error_message: format!("{}", e),
            })),
        }
    }
    
    async fn remove_item(
        &self,
        request: Request<RemoveItemRequest>,
    ) -> Result<Response<RemoveItemResponse>, Status> {
        let req = request.into_inner();
        
        // Получаем UUID предмета
        let item_id = Uuid::parse_str(&req.item_id)
            .map_err(|_| Status::invalid_argument(format!("Некорректный UUID предмета: {}", req.item_id)))?;
        
        // Если указано количество, уменьшаем количество предмета, иначе удаляем полностью
        if req.quantity > 0 {
            // Получаем текущий предмет
            let item = self.item_service.get_item(item_id)
                .await
                .map_err(Self::service_error_to_status)?;
            
            if req.quantity >= item.quantity {
                // Удаляем предмет полностью
                match self.item_service.delete_item(item_id).await {
                    Ok(_) => Ok(Response::new(RemoveItemResponse {
                        success: true,
                        remaining_quantity: 0,
                        error_message: String::new(),
                    })),
                    Err(e) => Ok(Response::new(RemoveItemResponse {
                        success: false,
                        remaining_quantity: item.quantity,
                        error_message: format!("{}", e),
                    })),
                }
            } else {
                // Уменьшаем количество предмета
                let update_data = update_item_data_from_request(
                    None,
                    Some(item.quantity - req.quantity),
                    None,
                    None,
                );
                
                match self.item_service.update_item(item_id, update_data).await {
                    Ok(updated_item) => Ok(Response::new(RemoveItemResponse {
                        success: true,
                        remaining_quantity: updated_item.quantity,
                        error_message: String::new(),
                    })),
                    Err(e) => Ok(Response::new(RemoveItemResponse {
                        success: false,
                        remaining_quantity: item.quantity,
                        error_message: format!("{}", e),
                    })),
                }
            }
        } else {
            // Удаляем предмет полностью
            match self.item_service.delete_item(item_id).await {
                Ok(_) => Ok(Response::new(RemoveItemResponse {
                    success: true,
                    remaining_quantity: 0,
                    error_message: String::new(),
                })),
                Err(e) => Ok(Response::new(RemoveItemResponse {
                    success: false,
                    remaining_quantity: 0,
                    error_message: format!("{}", e),
                })),
            }
        }
    }
    
    async fn move_item(
        &self,
        request: Request<MoveItemRequest>,
    ) -> Result<Response<MoveItemResponse>, Status> {
        let req = request.into_inner();
        
        // Получаем UUID предмета
        let item_id = Uuid::parse_str(&req.item_id)
            .map_err(|_| Status::invalid_argument(format!("Некорректный UUID предмета: {}", req.item_id)))?;
        
        // Получаем UUID целевого инвентаря
        let target_inventory_id = Uuid::parse_str(&req.target_inventory_id)
            .map_err(|_| Status::invalid_argument(format!("Некорректный UUID инвентаря: {}", req.target_inventory_id)))?;
        
        // Получаем текущий предмет (для ответа)
        let item = self.item_service.get_item(item_id)
            .await
            .map_err(Self::service_error_to_status)?;
        
        // Перемещаем предмет
        match self.item_service.move_item(item_id, target_inventory_id, req.target_position).await {
            Ok(target_item) => Ok(Response::new(MoveItemResponse {
                success: true,
                source_item: if req.quantity > 0 && req.quantity < item.quantity {
                    // Если перемещена часть стака, исходный предмет остается
                    Some(item_to_proto(&item))
                } else {
                    // Иначе исходный предмет удален
                    None
                },
                target_item: Some(item_to_proto(&target_item)),
                error_message: String::new(),
            })),
            Err(e) => Ok(Response::new(MoveItemResponse {
                success: false,
                source_item: Some(item_to_proto(&item)),
                target_item: None,
                error_message: format!("{}", e),
            })),
        }
    }
    
    async fn update_item(
        &self,
        request: Request<UpdateItemRequest>,
    ) -> Result<Response<UpdateItemResponse>, Status> {
        let req = request.into_inner();
        
        // Получаем UUID предмета
        let item_id = Uuid::parse_str(&req.item_id)
            .map_err(|_| Status::invalid_argument(format!("Некорректный UUID предмета: {}", req.item_id)))?;
        
        // Создаем данные для обновления
        let update_data = update_item_data_from_request(
            None, // Позиция не меняется через этот метод
            if req.quantity > 0 { Some(req.quantity) } else { None },
            if req.durability > 0.0 { Some(req.durability) } else { None },
            if !req.metadata.is_empty() { Some(req.metadata) } else { None },
        );
        
        // Обновляем предмет
        match self.item_service.update_item(item_id, update_data).await {
            Ok(item) => Ok(Response::new(UpdateItemResponse {
                item: Some(item_to_proto(&item)),
                success: true,
                error_message: String::new(),
            })),
            Err(e) => Ok(Response::new(UpdateItemResponse {
                item: None,
                success: false,
                error_message: format!("{}", e),
            })),
        }
    }
    
    async fn craft_item(
        &self,
        _request: Request<CraftItemRequest>,
    ) -> Result<Response<CraftItemResponse>, Status> {
        // TODO: Реализовать крафт предметов
        Err(Status::unimplemented("Крафт предметов пока не реализован"))
    }
    
    async fn use_item(
        &self,
        _request: Request<UseItemRequest>,
    ) -> Result<Response<UseItemResponse>, Status> {
        // TODO: Реализовать использование предметов
        Err(Status::unimplemented("Использование предметов пока не реализовано"))
    }
    
    async fn get_item_templates(
        &self,
        request: Request<GetItemTemplatesRequest>,
    ) -> Result<Response<GetItemTemplatesResponse>, Status> {
        let req = request.into_inner();
        
        // Если указана категория, получаем шаблоны по категории
        if !req.category.is_empty() {
            let templates = self.item_service.get_item_templates_by_category(&req.category)
                .await
                .map_err(Self::service_error_to_status)?;
            
            // Конвертируем в protobuf модели
            let proto_templates = templates.iter().map(item_template_to_proto).collect();
            
            Ok(Response::new(GetItemTemplatesResponse {
                templates: proto_templates,
                total_count: templates.len() as i32,
            }))
        } else {
            // Иначе получаем все шаблоны с пагинацией
            let page = if req.page > 0 { req.page } else { 1 };
            let page_size = if req.page_size > 0 { req.page_size } else { 20 };
            
            let (templates, total) = self.item_service.get_all_item_templates(page, page_size)
                .await
                .map_err(Self::service_error_to_status)?;
            
            // Конвертируем в protobuf модели
            let proto_templates = templates.iter().map(item_template_to_proto).collect();
            
            Ok(Response::new(GetItemTemplatesResponse {
                templates: proto_templates,
                total_count: total as i32,
            }))
        }
    }
} 