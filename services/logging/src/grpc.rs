use crate::{LogEntry, LogLevel, Logger};
use std::sync::Arc;
use chrono::{DateTime, Utc};
use tokio::sync::mpsc;

use futures_util::StreamExt;
use tonic::{Request, Response, Status};
use uuid::Uuid;

// Импортируем сгенерированные protobuf модули
pub mod proto {
    tonic::include_proto!("gameverse.logging.v1");
}

use proto::logging_service_server::{LoggingService, LoggingServiceServer};
use proto::{
    LogRequest, LogResponse, LogBatchRequest, LogBatchResponse,
    LogLevel as ProtoLogLevel,
};

// Конвертация между protobuf LogLevel и нашим внутренним LogLevel
impl From<ProtoLogLevel> for LogLevel {
    fn from(level: ProtoLogLevel) -> Self {
        match level {
            ProtoLogLevel::Trace => LogLevel::Trace,
            ProtoLogLevel::Debug => LogLevel::Debug,
            ProtoLogLevel::Info => LogLevel::Info,
            ProtoLogLevel::Warn => LogLevel::Warn,
            ProtoLogLevel::Error => LogLevel::Error,
            ProtoLogLevel::Fatal => LogLevel::Fatal,
            _ => LogLevel::Info,
        }
    }
}

// Конвертация из LogRequest в наш внутренний LogEntry
fn convert_log_request(req: &LogRequest) -> Result<LogEntry, Status> {
    // Парсим timestamp
    let timestamp = match req.timestamp.parse::<DateTime<Utc>>() {
        Ok(ts) => ts,
        Err(_) => {
            return Err(Status::invalid_argument("Invalid timestamp format"));
        }
    };
    
    // Создаем базовый LogEntry
    let mut entry = LogEntry {
        timestamp,
        level: ProtoLogLevel::from_i32(req.level).unwrap_or(ProtoLogLevel::Info).into(),
        service: req.service.clone(),
        component: req.component.clone(),
        message: req.message.clone(),
        context: crate::LogContext::default(),
        metrics: crate::LogMetrics::default(),
    };
    
    // Добавляем контекст если он есть
    if let Some(ctx) = &req.context {
        if !ctx.request_id.is_empty() {
            if let Ok(uuid) = Uuid::parse_str(&ctx.request_id) {
                entry.context.request_id = Some(uuid);
            }
        }
        
        if !ctx.user_id.is_empty() {
            entry.context.user_id = Some(ctx.user_id.clone());
        }
        
        if !ctx.entity_id.is_empty() {
            entry.context.entity_id = Some(ctx.entity_id.clone());
        }
        
        // Конвертируем дополнительные данные
        for (key, value) in &ctx.additional_data {
            entry = entry.with_data(key, value);
        }
    }
    
    // Добавляем метрики если они есть
    if let Some(metrics) = &req.metrics {
        if metrics.duration_ms > 0 {
            entry = entry.with_duration(metrics.duration_ms);
        }
        
        if metrics.memory_usage > 0 {
            entry = entry.with_memory_usage(metrics.memory_usage);
        }
    }
    
    Ok(entry)
}

// Имплементация gRPC сервиса логирования
pub struct LoggingServiceImpl {
    logger: Arc<Logger>,
}

impl LoggingServiceImpl {
    pub fn new(logger: Arc<Logger>) -> Self {
        Self { logger }
    }
    
    pub fn create_server(logger: Arc<Logger>) -> LoggingServiceServer<Self> {
        LoggingServiceServer::new(Self::new(logger))
    }
}

#[tonic::async_trait]
impl LoggingService for LoggingServiceImpl {
    // Обработка одного лога
    async fn send_log(
        &self,
        request: Request<LogRequest>,
    ) -> Result<Response<LogResponse>, Status> {
        let req = request.into_inner();
        
        // Конвертируем запрос в LogEntry
        let entry = match convert_log_request(&req) {
            Ok(entry) => entry,
            Err(e) => {
                return Ok(Response::new(LogResponse {
                    success: false,
                    error_message: e.to_string(),
                }));
            }
        };
        
        // Логируем запись
        match self.logger.log_entry(entry).await {
            Ok(_) => Ok(Response::new(LogResponse {
                success: true,
                error_message: String::new(),
            })),
            Err(e) => Ok(Response::new(LogResponse {
                success: false,
                error_message: e.to_string(),
            })),
        }
    }
    
    // Обработка пакета логов
    async fn send_log_batch(
        &self,
        request: Request<LogBatchRequest>,
    ) -> Result<Response<LogBatchResponse>, Status> {
        let req = request.into_inner();
        let mut processed_count = 0;
        let mut failed_count = 0;
        let mut last_error = String::new();
        
        // Обрабатываем каждый лог в пакете
        for log_request in req.logs {
            match convert_log_request(&log_request) {
                Ok(entry) => {
                    match self.logger.log_entry(entry).await {
                        Ok(_) => processed_count += 1,
                        Err(e) => {
                            failed_count += 1;
                            last_error = e.to_string();
                        }
                    }
                },
                Err(e) => {
                    failed_count += 1;
                    last_error = e.to_string();
                }
            }
        }
        
        Ok(Response::new(LogBatchResponse {
            processed_count,
            failed_count,
            success: failed_count == 0,
            error_message: last_error,
        }))
    }
    
    // Потоковая обработка логов
    async fn stream_logs(
        &self,
        request: Request<tonic::Streaming<LogRequest>>,
    ) -> Result<Response<LogBatchResponse>, Status> {
        let mut stream = request.into_inner();
        let mut processed_count = 0;
        let mut failed_count = 0;
        let mut last_error = String::new();
        
        // Читаем и обрабатываем поток логов
        while let Some(log_request) = stream.next().await {
            match log_request {
                Ok(req) => {
                    match convert_log_request(&req) {
                        Ok(entry) => {
                            match self.logger.log_entry(entry).await {
                                Ok(_) => processed_count += 1,
                                Err(e) => {
                                    failed_count += 1;
                                    last_error = e.to_string();
                                }
                            }
                        },
                        Err(e) => {
                            failed_count += 1;
                            last_error = e.to_string();
                        }
                    }
                },
                Err(e) => {
                    failed_count += 1;
                    last_error = e.to_string();
                }
            }
        }
        
        Ok(Response::new(LogBatchResponse {
            processed_count,
            failed_count,
            success: failed_count == 0,
            error_message: last_error,
        }))
    }
}

// Обработчик логов, который отправляет логи через gRPC
pub struct GrpcLogHandler {
    client: proto::logging_service_client::LoggingServiceClient<tonic::transport::Channel>,
    service_name: String,
    pending_logs: mpsc::Sender<LogEntry>,
}

impl GrpcLogHandler {
    pub async fn new(service_name: impl Into<String>, server_addr: impl Into<String>) -> Result<Self, tonic::transport::Error> {
        let server_addr = server_addr.into();
        let service_name = service_name.into();
        
        // Создаем канал для буферизации логов
        let (tx, mut rx) = mpsc::channel::<LogEntry>(100);
        
        // Подключаемся к серверу логирования
        let client = proto::logging_service_client::LoggingServiceClient::connect(format!("http://{}", server_addr)).await?;
        let client_clone = client.clone();
        
        // Запускаем фоновую задачу для отправки логов пакетами
        tokio::spawn(async move {
            let mut batch = Vec::with_capacity(50);
            let mut interval = tokio::time::interval(std::time::Duration::from_secs(5));
            
            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        if !batch.is_empty() {
                            let mut client = client_clone.clone();
                            let logs = std::mem::take(&mut batch);
                            if let Err(e) = Self::send_batch(&mut client, logs).await {
                                eprintln!("Ошибка отправки пакета логов: {}", e);
                            }
                        }
                    }
                    Some(log) = rx.recv() => {
                        batch.push(log);
                        if batch.len() >= 50 {
                            let mut client = client_clone.clone();
                            let logs = std::mem::take(&mut batch);
                            if let Err(e) = Self::send_batch(&mut client, logs).await {
                                eprintln!("Ошибка отправки пакета логов: {}", e);
                            }
                        }
                    }
                }
            }
        });
        
        Ok(Self {
            client,
            service_name,
            pending_logs: tx,
        })
    }
    
    // Отправка пакета логов
    async fn send_batch(
        client: &mut proto::logging_service_client::LoggingServiceClient<tonic::transport::Channel>,
        logs: Vec<LogEntry>,
    ) -> Result<(), tonic::Status> {
        let proto_logs: Vec<LogRequest> = logs.iter().map(|entry| {
            // Конвертируем LogEntry в LogRequest
            LogRequest {
                timestamp: entry.timestamp.to_rfc3339(),
                level: match entry.level {
                    LogLevel::Trace => ProtoLogLevel::Trace,
                    LogLevel::Debug => ProtoLogLevel::Debug,
                    LogLevel::Info => ProtoLogLevel::Info,
                    LogLevel::Warn => ProtoLogLevel::Warn,
                    LogLevel::Error => ProtoLogLevel::Error,
                    LogLevel::Fatal => ProtoLogLevel::Fatal,
                } as i32,
                service: entry.service.clone(),
                component: entry.component.clone(),
                message: entry.message.clone(),
                context: Some(proto::LogContext {
                    request_id: entry.context.request_id.map_or(String::new(), |id| id.to_string()),
                    user_id: entry.context.user_id.clone().unwrap_or_default(),
                    entity_id: entry.context.entity_id.clone().unwrap_or_default(),
                    additional_data: entry.context.additional_data.iter()
                        .map(|(k, v)| (k.clone(), v.to_string()))
                        .collect(),
                }),
                metrics: Some(proto::LogMetrics {
                    duration_ms: entry.metrics.duration_ms.unwrap_or_default(),
                    memory_usage: entry.metrics.memory_usage.unwrap_or_default(),
                }),
            }
        }).collect();
        
        let request = LogBatchRequest {
            logs: proto_logs,
        };
        
        let response = client.send_log_batch(request).await?;
        let response = response.into_inner();
        
        if !response.success {
            return Err(tonic::Status::internal(response.error_message));
        }
        
        Ok(())
    }
}

#[async_trait::async_trait]
impl crate::LogHandler for GrpcLogHandler {
    async fn handle(&self, entry: LogEntry) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Отправляем лог в канал для буферизации
        if let Err(e) = self.pending_logs.send(entry).await {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Не удалось отправить лог в буфер: {}", e),
            )));
        }
        
        Ok(())
    }
} 