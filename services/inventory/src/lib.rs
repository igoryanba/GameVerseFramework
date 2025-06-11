pub mod models;
pub mod repositories;
pub mod services;
pub mod grpc; // Включаем обратно для исправления
pub mod http;
pub mod config;
pub mod proto;

// Реэкспорт основных компонентов для удобства
pub use config::Config;
pub use models::{Inventory, Item, ItemTemplate};
pub use repositories::{InventoryRepository, ItemRepository, ItemTemplateRepository};
pub use services::{InventoryService, ItemService};

// Re-export протогенерированный код
// Временно отключаем для быстрого тестирования
/*
pub mod proto {
    pub mod inventory {
        tonic::include_proto!("inventory");
    }
}
*/ 