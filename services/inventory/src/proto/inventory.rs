/// Модель инвентаря
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Inventory {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub owner_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub owner_type: ::prost::alloc::string::String,
    #[prost(int32, tag = "4")]
    pub inventory_type: i32,
    #[prost(float, tag = "5")]
    pub max_weight: f32,
    #[prost(int32, tag = "6")]
    pub max_slots: i32,
    #[prost(string, tag = "7")]
    pub name: ::prost::alloc::string::String,
    #[prost(map = "string, string", tag = "8")]
    pub metadata: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Модель предмета
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct Item {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub inventory_id: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub template_id: ::prost::alloc::string::String,
    #[prost(int32, tag = "4")]
    pub position: i32,
    #[prost(int32, tag = "5")]
    pub quantity: i32,
    #[prost(float, tag = "6")]
    pub durability: f32,
    #[prost(map = "string, string", tag = "7")]
    pub metadata: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Модель шаблона предмета
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct ItemTemplate {
    #[prost(string, tag = "1")]
    pub id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub name: ::prost::alloc::string::String,
    #[prost(string, tag = "3")]
    pub description: ::prost::alloc::string::String,
    #[prost(float, tag = "4")]
    pub weight: f32,
    #[prost(bool, tag = "5")]
    pub stackable: bool,
    #[prost(int32, tag = "6")]
    pub max_stack: i32,
    #[prost(float, tag = "7")]
    pub max_durability: f32,
    #[prost(string, tag = "8")]
    pub icon: ::prost::alloc::string::String,
    #[prost(string, tag = "9")]
    pub category: ::prost::alloc::string::String,
    #[prost(map = "string, string", tag = "10")]
    pub properties: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Запрос на получение инвентаря
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInventoryRequest {
    #[prost(string, tag = "1")]
    pub inventory_id: ::prost::alloc::string::String,
}
/// Ответ с информацией об инвентаре
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInventoryResponse {
    #[prost(message, optional, tag = "1")]
    pub inventory: ::core::option::Option<Inventory>,
}
/// Запрос на получение предметов в инвентаре
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInventoryItemsRequest {
    #[prost(string, tag = "1")]
    pub inventory_id: ::prost::alloc::string::String,
}
/// Ответ со списком предметов в инвентаре
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetInventoryItemsResponse {
    #[prost(message, repeated, tag = "1")]
    pub items: ::prost::alloc::vec::Vec<Item>,
    #[prost(int32, tag = "2")]
    pub total_count: i32,
}
/// Запрос на добавление предмета в инвентарь
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddItemRequest {
    #[prost(string, tag = "1")]
    pub inventory_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub template_id: ::prost::alloc::string::String,
    #[prost(int32, tag = "3")]
    pub position: i32,
    #[prost(int32, tag = "4")]
    pub quantity: i32,
    #[prost(map = "string, string", tag = "5")]
    pub metadata: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Ответ на добавление предмета
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct AddItemResponse {
    #[prost(message, optional, tag = "1")]
    pub item: ::core::option::Option<Item>,
    #[prost(bool, tag = "2")]
    pub success: bool,
    #[prost(string, tag = "3")]
    pub error_message: ::prost::alloc::string::String,
}
/// Запрос на удаление предмета из инвентаря
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveItemRequest {
    #[prost(string, tag = "1")]
    pub item_id: ::prost::alloc::string::String,
    /// 0 - удалить весь стак
    #[prost(int32, tag = "2")]
    pub quantity: i32,
}
/// Ответ на удаление предмета
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct RemoveItemResponse {
    #[prost(bool, tag = "1")]
    pub success: bool,
    #[prost(int32, tag = "2")]
    pub remaining_quantity: i32,
    #[prost(string, tag = "3")]
    pub error_message: ::prost::alloc::string::String,
}
/// Запрос на перемещение предмета
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveItemRequest {
    #[prost(string, tag = "1")]
    pub item_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub target_inventory_id: ::prost::alloc::string::String,
    #[prost(int32, tag = "3")]
    pub target_position: i32,
    /// 0 - весь стак
    #[prost(int32, tag = "4")]
    pub quantity: i32,
}
/// Ответ на перемещение предмета
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct MoveItemResponse {
    #[prost(bool, tag = "1")]
    pub success: bool,
    /// Может быть null, если весь стак перемещен
    #[prost(message, optional, tag = "2")]
    pub source_item: ::core::option::Option<Item>,
    /// Новый предмет в целевом инвентаре
    #[prost(message, optional, tag = "3")]
    pub target_item: ::core::option::Option<Item>,
    #[prost(string, tag = "4")]
    pub error_message: ::prost::alloc::string::String,
}
/// Запрос на обновление предмета
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateItemRequest {
    #[prost(string, tag = "1")]
    pub item_id: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub quantity: i32,
    #[prost(float, tag = "3")]
    pub durability: f32,
    #[prost(map = "string, string", tag = "4")]
    pub metadata: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Ответ на обновление предмета
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UpdateItemResponse {
    #[prost(bool, tag = "1")]
    pub success: bool,
    #[prost(message, optional, tag = "2")]
    pub item: ::core::option::Option<Item>,
    #[prost(string, tag = "3")]
    pub error_message: ::prost::alloc::string::String,
}
/// Запрос на крафт предмета
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CraftItemRequest {
    #[prost(string, tag = "1")]
    pub inventory_id: ::prost::alloc::string::String,
    #[prost(string, tag = "2")]
    pub recipe_id: ::prost::alloc::string::String,
    #[prost(string, repeated, tag = "3")]
    pub source_item_ids: ::prost::alloc::vec::Vec<::prost::alloc::string::String>,
    #[prost(int32, tag = "4")]
    pub target_position: i32,
}
/// Ответ на крафт предмета
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct CraftItemResponse {
    #[prost(bool, tag = "1")]
    pub success: bool,
    #[prost(message, optional, tag = "2")]
    pub crafted_item: ::core::option::Option<Item>,
    #[prost(string, tag = "3")]
    pub error_message: ::prost::alloc::string::String,
}
/// Запрос на использование предмета
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UseItemRequest {
    #[prost(string, tag = "1")]
    pub item_id: ::prost::alloc::string::String,
    /// ID цели (игрок, объект и т.д.)
    #[prost(string, tag = "2")]
    pub target_id: ::prost::alloc::string::String,
    #[prost(map = "string, string", tag = "3")]
    pub parameters: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Ответ на использование предмета
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct UseItemResponse {
    #[prost(bool, tag = "1")]
    pub success: bool,
    #[prost(message, optional, tag = "2")]
    pub updated_item: ::core::option::Option<Item>,
    #[prost(string, tag = "3")]
    pub error_message: ::prost::alloc::string::String,
    #[prost(map = "string, string", tag = "4")]
    pub result_data: ::std::collections::HashMap<
        ::prost::alloc::string::String,
        ::prost::alloc::string::String,
    >,
}
/// Запрос на получение шаблонов предметов
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetItemTemplatesRequest {
    /// Пустая строка - все категории
    #[prost(string, tag = "1")]
    pub category: ::prost::alloc::string::String,
    #[prost(int32, tag = "2")]
    pub page: i32,
    #[prost(int32, tag = "3")]
    pub page_size: i32,
}
/// Ответ со списком шаблонов предметов
#[allow(clippy::derive_partial_eq_without_eq)]
#[derive(Clone, PartialEq, ::prost::Message)]
pub struct GetItemTemplatesResponse {
    #[prost(message, repeated, tag = "1")]
    pub templates: ::prost::alloc::vec::Vec<ItemTemplate>,
    #[prost(int32, tag = "2")]
    pub total_count: i32,
}
/// Типы инвентарей
#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash, PartialOrd, Ord, ::prost::Enumeration)]
#[repr(i32)]
pub enum InventoryType {
    Unspecified = 0,
    Player = 1,
    Vehicle = 2,
    Container = 3,
    Shop = 4,
    Temp = 5,
}
impl InventoryType {
    /// String value of the enum field names used in the ProtoBuf definition.
    ///
    /// The values are not transformed in any way and thus are considered stable
    /// (if the ProtoBuf definition does not change) and safe for programmatic use.
    pub fn as_str_name(&self) -> &'static str {
        match self {
            InventoryType::Unspecified => "INVENTORY_TYPE_UNSPECIFIED",
            InventoryType::Player => "INVENTORY_TYPE_PLAYER",
            InventoryType::Vehicle => "INVENTORY_TYPE_VEHICLE",
            InventoryType::Container => "INVENTORY_TYPE_CONTAINER",
            InventoryType::Shop => "INVENTORY_TYPE_SHOP",
            InventoryType::Temp => "INVENTORY_TYPE_TEMP",
        }
    }
    /// Creates an enum from field names used in the ProtoBuf definition.
    pub fn from_str_name(value: &str) -> ::core::option::Option<Self> {
        match value {
            "INVENTORY_TYPE_UNSPECIFIED" => Some(Self::Unspecified),
            "INVENTORY_TYPE_PLAYER" => Some(Self::Player),
            "INVENTORY_TYPE_VEHICLE" => Some(Self::Vehicle),
            "INVENTORY_TYPE_CONTAINER" => Some(Self::Container),
            "INVENTORY_TYPE_SHOP" => Some(Self::Shop),
            "INVENTORY_TYPE_TEMP" => Some(Self::Temp),
            _ => None,
        }
    }
}
/// Generated client implementations.
pub mod inventory_service_client {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    use tonic::codegen::http::Uri;
    /// Сервис управления инвентарем
    #[derive(Debug, Clone)]
    pub struct InventoryServiceClient<T> {
        inner: tonic::client::Grpc<T>,
    }
    impl InventoryServiceClient<tonic::transport::Channel> {
        /// Attempt to create a new client by connecting to a given endpoint.
        pub async fn connect<D>(dst: D) -> Result<Self, tonic::transport::Error>
        where
            D: TryInto<tonic::transport::Endpoint>,
            D::Error: Into<StdError>,
        {
            let conn = tonic::transport::Endpoint::new(dst)?.connect().await?;
            Ok(Self::new(conn))
        }
    }
    impl<T> InventoryServiceClient<T>
    where
        T: tonic::client::GrpcService<tonic::body::BoxBody>,
        T::Error: Into<StdError>,
        T::ResponseBody: Body<Data = Bytes> + Send + 'static,
        <T::ResponseBody as Body>::Error: Into<StdError> + Send,
    {
        pub fn new(inner: T) -> Self {
            let inner = tonic::client::Grpc::new(inner);
            Self { inner }
        }
        pub fn with_origin(inner: T, origin: Uri) -> Self {
            let inner = tonic::client::Grpc::with_origin(inner, origin);
            Self { inner }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InventoryServiceClient<InterceptedService<T, F>>
        where
            F: tonic::service::Interceptor,
            T::ResponseBody: Default,
            T: tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
                Response = http::Response<
                    <T as tonic::client::GrpcService<tonic::body::BoxBody>>::ResponseBody,
                >,
            >,
            <T as tonic::codegen::Service<
                http::Request<tonic::body::BoxBody>,
            >>::Error: Into<StdError> + Send + Sync,
        {
            InventoryServiceClient::new(InterceptedService::new(inner, interceptor))
        }
        /// Compress requests with the given encoding.
        ///
        /// This requires the server to support it otherwise it might respond with an
        /// error.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.send_compressed(encoding);
            self
        }
        /// Enable decompressing responses.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.inner = self.inner.accept_compressed(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_decoding_message_size(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.inner = self.inner.max_encoding_message_size(limit);
            self
        }
        /// Получение информации об инвентаре
        pub async fn get_inventory(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInventoryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetInventoryResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/inventory.InventoryService/GetInventory",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("inventory.InventoryService", "GetInventory"));
            self.inner.unary(req, path, codec).await
        }
        /// Получение предметов в инвентаре
        pub async fn get_inventory_items(
            &mut self,
            request: impl tonic::IntoRequest<super::GetInventoryItemsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetInventoryItemsResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/inventory.InventoryService/GetInventoryItems",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("inventory.InventoryService", "GetInventoryItems"),
                );
            self.inner.unary(req, path, codec).await
        }
        /// Добавление предмета в инвентарь
        pub async fn add_item(
            &mut self,
            request: impl tonic::IntoRequest<super::AddItemRequest>,
        ) -> std::result::Result<
            tonic::Response<super::AddItemResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/inventory.InventoryService/AddItem",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("inventory.InventoryService", "AddItem"));
            self.inner.unary(req, path, codec).await
        }
        /// Удаление предмета из инвентаря
        pub async fn remove_item(
            &mut self,
            request: impl tonic::IntoRequest<super::RemoveItemRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveItemResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/inventory.InventoryService/RemoveItem",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("inventory.InventoryService", "RemoveItem"));
            self.inner.unary(req, path, codec).await
        }
        /// Перемещение предмета между инвентарями
        pub async fn move_item(
            &mut self,
            request: impl tonic::IntoRequest<super::MoveItemRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MoveItemResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/inventory.InventoryService/MoveItem",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("inventory.InventoryService", "MoveItem"));
            self.inner.unary(req, path, codec).await
        }
        /// Обновление предмета
        pub async fn update_item(
            &mut self,
            request: impl tonic::IntoRequest<super::UpdateItemRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateItemResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/inventory.InventoryService/UpdateItem",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("inventory.InventoryService", "UpdateItem"));
            self.inner.unary(req, path, codec).await
        }
        /// Крафт предмета
        pub async fn craft_item(
            &mut self,
            request: impl tonic::IntoRequest<super::CraftItemRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CraftItemResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/inventory.InventoryService/CraftItem",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("inventory.InventoryService", "CraftItem"));
            self.inner.unary(req, path, codec).await
        }
        /// Использование предмета
        pub async fn use_item(
            &mut self,
            request: impl tonic::IntoRequest<super::UseItemRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UseItemResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/inventory.InventoryService/UseItem",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(GrpcMethod::new("inventory.InventoryService", "UseItem"));
            self.inner.unary(req, path, codec).await
        }
        /// Получение шаблонов предметов
        pub async fn get_item_templates(
            &mut self,
            request: impl tonic::IntoRequest<super::GetItemTemplatesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetItemTemplatesResponse>,
            tonic::Status,
        > {
            self.inner
                .ready()
                .await
                .map_err(|e| {
                    tonic::Status::new(
                        tonic::Code::Unknown,
                        format!("Service was not ready: {}", e.into()),
                    )
                })?;
            let codec = tonic::codec::ProstCodec::default();
            let path = http::uri::PathAndQuery::from_static(
                "/inventory.InventoryService/GetItemTemplates",
            );
            let mut req = request.into_request();
            req.extensions_mut()
                .insert(
                    GrpcMethod::new("inventory.InventoryService", "GetItemTemplates"),
                );
            self.inner.unary(req, path, codec).await
        }
    }
}
/// Generated server implementations.
pub mod inventory_service_server {
    #![allow(unused_variables, dead_code, missing_docs, clippy::let_unit_value)]
    use tonic::codegen::*;
    /// Generated trait containing gRPC methods that should be implemented for use with InventoryServiceServer.
    #[async_trait]
    pub trait InventoryService: Send + Sync + 'static {
        /// Получение информации об инвентаре
        async fn get_inventory(
            &self,
            request: tonic::Request<super::GetInventoryRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetInventoryResponse>,
            tonic::Status,
        >;
        /// Получение предметов в инвентаре
        async fn get_inventory_items(
            &self,
            request: tonic::Request<super::GetInventoryItemsRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetInventoryItemsResponse>,
            tonic::Status,
        >;
        /// Добавление предмета в инвентарь
        async fn add_item(
            &self,
            request: tonic::Request<super::AddItemRequest>,
        ) -> std::result::Result<tonic::Response<super::AddItemResponse>, tonic::Status>;
        /// Удаление предмета из инвентаря
        async fn remove_item(
            &self,
            request: tonic::Request<super::RemoveItemRequest>,
        ) -> std::result::Result<
            tonic::Response<super::RemoveItemResponse>,
            tonic::Status,
        >;
        /// Перемещение предмета между инвентарями
        async fn move_item(
            &self,
            request: tonic::Request<super::MoveItemRequest>,
        ) -> std::result::Result<
            tonic::Response<super::MoveItemResponse>,
            tonic::Status,
        >;
        /// Обновление предмета
        async fn update_item(
            &self,
            request: tonic::Request<super::UpdateItemRequest>,
        ) -> std::result::Result<
            tonic::Response<super::UpdateItemResponse>,
            tonic::Status,
        >;
        /// Крафт предмета
        async fn craft_item(
            &self,
            request: tonic::Request<super::CraftItemRequest>,
        ) -> std::result::Result<
            tonic::Response<super::CraftItemResponse>,
            tonic::Status,
        >;
        /// Использование предмета
        async fn use_item(
            &self,
            request: tonic::Request<super::UseItemRequest>,
        ) -> std::result::Result<tonic::Response<super::UseItemResponse>, tonic::Status>;
        /// Получение шаблонов предметов
        async fn get_item_templates(
            &self,
            request: tonic::Request<super::GetItemTemplatesRequest>,
        ) -> std::result::Result<
            tonic::Response<super::GetItemTemplatesResponse>,
            tonic::Status,
        >;
    }
    /// Сервис управления инвентарем
    #[derive(Debug)]
    pub struct InventoryServiceServer<T: InventoryService> {
        inner: _Inner<T>,
        accept_compression_encodings: EnabledCompressionEncodings,
        send_compression_encodings: EnabledCompressionEncodings,
        max_decoding_message_size: Option<usize>,
        max_encoding_message_size: Option<usize>,
    }
    struct _Inner<T>(Arc<T>);
    impl<T: InventoryService> InventoryServiceServer<T> {
        pub fn new(inner: T) -> Self {
            Self::from_arc(Arc::new(inner))
        }
        pub fn from_arc(inner: Arc<T>) -> Self {
            let inner = _Inner(inner);
            Self {
                inner,
                accept_compression_encodings: Default::default(),
                send_compression_encodings: Default::default(),
                max_decoding_message_size: None,
                max_encoding_message_size: None,
            }
        }
        pub fn with_interceptor<F>(
            inner: T,
            interceptor: F,
        ) -> InterceptedService<Self, F>
        where
            F: tonic::service::Interceptor,
        {
            InterceptedService::new(Self::new(inner), interceptor)
        }
        /// Enable decompressing requests with the given encoding.
        #[must_use]
        pub fn accept_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.accept_compression_encodings.enable(encoding);
            self
        }
        /// Compress responses with the given encoding, if the client supports it.
        #[must_use]
        pub fn send_compressed(mut self, encoding: CompressionEncoding) -> Self {
            self.send_compression_encodings.enable(encoding);
            self
        }
        /// Limits the maximum size of a decoded message.
        ///
        /// Default: `4MB`
        #[must_use]
        pub fn max_decoding_message_size(mut self, limit: usize) -> Self {
            self.max_decoding_message_size = Some(limit);
            self
        }
        /// Limits the maximum size of an encoded message.
        ///
        /// Default: `usize::MAX`
        #[must_use]
        pub fn max_encoding_message_size(mut self, limit: usize) -> Self {
            self.max_encoding_message_size = Some(limit);
            self
        }
    }
    impl<T, B> tonic::codegen::Service<http::Request<B>> for InventoryServiceServer<T>
    where
        T: InventoryService,
        B: Body + Send + 'static,
        B::Error: Into<StdError> + Send + 'static,
    {
        type Response = http::Response<tonic::body::BoxBody>;
        type Error = std::convert::Infallible;
        type Future = BoxFuture<Self::Response, Self::Error>;
        fn poll_ready(
            &mut self,
            _cx: &mut Context<'_>,
        ) -> Poll<std::result::Result<(), Self::Error>> {
            Poll::Ready(Ok(()))
        }
        fn call(&mut self, req: http::Request<B>) -> Self::Future {
            let inner = self.inner.clone();
            match req.uri().path() {
                "/inventory.InventoryService/GetInventory" => {
                    #[allow(non_camel_case_types)]
                    struct GetInventorySvc<T: InventoryService>(pub Arc<T>);
                    impl<
                        T: InventoryService,
                    > tonic::server::UnaryService<super::GetInventoryRequest>
                    for GetInventorySvc<T> {
                        type Response = super::GetInventoryResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetInventoryRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_inventory(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetInventorySvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/inventory.InventoryService/GetInventoryItems" => {
                    #[allow(non_camel_case_types)]
                    struct GetInventoryItemsSvc<T: InventoryService>(pub Arc<T>);
                    impl<
                        T: InventoryService,
                    > tonic::server::UnaryService<super::GetInventoryItemsRequest>
                    for GetInventoryItemsSvc<T> {
                        type Response = super::GetInventoryItemsResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetInventoryItemsRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_inventory_items(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetInventoryItemsSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/inventory.InventoryService/AddItem" => {
                    #[allow(non_camel_case_types)]
                    struct AddItemSvc<T: InventoryService>(pub Arc<T>);
                    impl<
                        T: InventoryService,
                    > tonic::server::UnaryService<super::AddItemRequest>
                    for AddItemSvc<T> {
                        type Response = super::AddItemResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::AddItemRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).add_item(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = AddItemSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/inventory.InventoryService/RemoveItem" => {
                    #[allow(non_camel_case_types)]
                    struct RemoveItemSvc<T: InventoryService>(pub Arc<T>);
                    impl<
                        T: InventoryService,
                    > tonic::server::UnaryService<super::RemoveItemRequest>
                    for RemoveItemSvc<T> {
                        type Response = super::RemoveItemResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::RemoveItemRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).remove_item(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = RemoveItemSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/inventory.InventoryService/MoveItem" => {
                    #[allow(non_camel_case_types)]
                    struct MoveItemSvc<T: InventoryService>(pub Arc<T>);
                    impl<
                        T: InventoryService,
                    > tonic::server::UnaryService<super::MoveItemRequest>
                    for MoveItemSvc<T> {
                        type Response = super::MoveItemResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::MoveItemRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).move_item(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = MoveItemSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/inventory.InventoryService/UpdateItem" => {
                    #[allow(non_camel_case_types)]
                    struct UpdateItemSvc<T: InventoryService>(pub Arc<T>);
                    impl<
                        T: InventoryService,
                    > tonic::server::UnaryService<super::UpdateItemRequest>
                    for UpdateItemSvc<T> {
                        type Response = super::UpdateItemResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UpdateItemRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).update_item(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UpdateItemSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/inventory.InventoryService/CraftItem" => {
                    #[allow(non_camel_case_types)]
                    struct CraftItemSvc<T: InventoryService>(pub Arc<T>);
                    impl<
                        T: InventoryService,
                    > tonic::server::UnaryService<super::CraftItemRequest>
                    for CraftItemSvc<T> {
                        type Response = super::CraftItemResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::CraftItemRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).craft_item(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = CraftItemSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/inventory.InventoryService/UseItem" => {
                    #[allow(non_camel_case_types)]
                    struct UseItemSvc<T: InventoryService>(pub Arc<T>);
                    impl<
                        T: InventoryService,
                    > tonic::server::UnaryService<super::UseItemRequest>
                    for UseItemSvc<T> {
                        type Response = super::UseItemResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::UseItemRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move { (*inner).use_item(request).await };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = UseItemSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                "/inventory.InventoryService/GetItemTemplates" => {
                    #[allow(non_camel_case_types)]
                    struct GetItemTemplatesSvc<T: InventoryService>(pub Arc<T>);
                    impl<
                        T: InventoryService,
                    > tonic::server::UnaryService<super::GetItemTemplatesRequest>
                    for GetItemTemplatesSvc<T> {
                        type Response = super::GetItemTemplatesResponse;
                        type Future = BoxFuture<
                            tonic::Response<Self::Response>,
                            tonic::Status,
                        >;
                        fn call(
                            &mut self,
                            request: tonic::Request<super::GetItemTemplatesRequest>,
                        ) -> Self::Future {
                            let inner = Arc::clone(&self.0);
                            let fut = async move {
                                (*inner).get_item_templates(request).await
                            };
                            Box::pin(fut)
                        }
                    }
                    let accept_compression_encodings = self.accept_compression_encodings;
                    let send_compression_encodings = self.send_compression_encodings;
                    let max_decoding_message_size = self.max_decoding_message_size;
                    let max_encoding_message_size = self.max_encoding_message_size;
                    let inner = self.inner.clone();
                    let fut = async move {
                        let inner = inner.0;
                        let method = GetItemTemplatesSvc(inner);
                        let codec = tonic::codec::ProstCodec::default();
                        let mut grpc = tonic::server::Grpc::new(codec)
                            .apply_compression_config(
                                accept_compression_encodings,
                                send_compression_encodings,
                            )
                            .apply_max_message_size_config(
                                max_decoding_message_size,
                                max_encoding_message_size,
                            );
                        let res = grpc.unary(method, req).await;
                        Ok(res)
                    };
                    Box::pin(fut)
                }
                _ => {
                    Box::pin(async move {
                        Ok(
                            http::Response::builder()
                                .status(200)
                                .header("grpc-status", "12")
                                .header("content-type", "application/grpc")
                                .body(empty_body())
                                .unwrap(),
                        )
                    })
                }
            }
        }
    }
    impl<T: InventoryService> Clone for InventoryServiceServer<T> {
        fn clone(&self) -> Self {
            let inner = self.inner.clone();
            Self {
                inner,
                accept_compression_encodings: self.accept_compression_encodings,
                send_compression_encodings: self.send_compression_encodings,
                max_decoding_message_size: self.max_decoding_message_size,
                max_encoding_message_size: self.max_encoding_message_size,
            }
        }
    }
    impl<T: InventoryService> Clone for _Inner<T> {
        fn clone(&self) -> Self {
            Self(Arc::clone(&self.0))
        }
    }
    impl<T: std::fmt::Debug> std::fmt::Debug for _Inner<T> {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{:?}", self.0)
        }
    }
    impl<T: InventoryService> tonic::server::NamedService for InventoryServiceServer<T> {
        const NAME: &'static str = "inventory.InventoryService";
    }
}
