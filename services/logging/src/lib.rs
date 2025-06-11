use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;
use uuid::Uuid;

// Добавляем и экспортируем модуль gRPC
pub mod grpc;

// Добавляем и экспортируем модуль ElasticSearch
pub mod elasticsearch;

/// Уровни логирования поддерживаемые системой
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "UPPERCASE")]
pub enum LogLevel {
    Trace,
    Debug,
    Info,
    Warn,
    Error,
    Fatal,
}

impl std::fmt::Display for LogLevel {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LogLevel::Trace => write!(f, "TRACE"),
            LogLevel::Debug => write!(f, "DEBUG"),
            LogLevel::Info => write!(f, "INFO"),
            LogLevel::Warn => write!(f, "WARN"),
            LogLevel::Error => write!(f, "ERROR"),
            LogLevel::Fatal => write!(f, "FATAL"),
        }
    }
}

/// Контекст лога, содержащий дополнительную информацию
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogContext {
    pub request_id: Option<Uuid>,
    pub user_id: Option<String>,
    pub entity_id: Option<String>,
    #[serde(flatten)]
    pub additional_data: HashMap<String, serde_json::Value>,
}

impl Default for LogContext {
    fn default() -> Self {
        Self {
            request_id: None,
            user_id: None,
            entity_id: None,
            additional_data: HashMap::new(),
        }
    }
}

/// Метрики для лога
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogMetrics {
    pub duration_ms: Option<u64>,
    pub memory_usage: Option<u64>,
}

impl Default for LogMetrics {
    fn default() -> Self {
        Self {
            duration_ms: None,
            memory_usage: None,
        }
    }
}

/// Основная структура лога
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LogEntry {
    pub timestamp: DateTime<Utc>,
    pub level: LogLevel,
    pub service: String,
    pub component: String,
    pub message: String,
    pub context: LogContext,
    pub metrics: LogMetrics,
}

impl LogEntry {
    pub fn new(
        level: LogLevel,
        service: impl Into<String>,
        component: impl Into<String>,
        message: impl Into<String>,
    ) -> Self {
        Self {
            timestamp: Utc::now(),
            level,
            service: service.into(),
            component: component.into(),
            message: message.into(),
            context: LogContext::default(),
            metrics: LogMetrics::default(),
        }
    }

    /// Установить request_id в контексте
    pub fn with_request_id(mut self, request_id: Uuid) -> Self {
        self.context.request_id = Some(request_id);
        self
    }

    /// Установить user_id в контексте
    pub fn with_user_id(mut self, user_id: impl Into<String>) -> Self {
        self.context.user_id = Some(user_id.into());
        self
    }

    /// Установить entity_id в контексте
    pub fn with_entity_id(mut self, entity_id: impl Into<String>) -> Self {
        self.context.entity_id = Some(entity_id.into());
        self
    }

    /// Добавить дополнительные данные в контекст
    pub fn with_data(mut self, key: impl Into<String>, value: impl Serialize) -> Self {
        if let Ok(value) = serde_json::to_value(value) {
            self.context.additional_data.insert(key.into(), value);
        }
        self
    }

    /// Установить метрику продолжительности
    pub fn with_duration(mut self, duration_ms: u64) -> Self {
        self.metrics.duration_ms = Some(duration_ms);
        self
    }

    /// Установить метрику использования памяти
    pub fn with_memory_usage(mut self, memory_usage: u64) -> Self {
        self.metrics.memory_usage = Some(memory_usage);
        self
    }

    /// Преобразовать лог в JSON строку
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string(&self)
    }
}

/// Интерфейс для обработчиков логов
#[async_trait::async_trait]
pub trait LogHandler: Send + Sync {
    async fn handle(&self, entry: LogEntry) -> Result<(), Box<dyn std::error::Error + Send + Sync>>;
}

/// Обработчик логов в файл
pub struct FileLogHandler {
    file_path: String,
}

impl FileLogHandler {
    pub fn new(file_path: impl Into<String>) -> Self {
        Self {
            file_path: file_path.into(),
        }
    }
}

#[async_trait::async_trait]
impl LogHandler for FileLogHandler {
    async fn handle(&self, entry: LogEntry) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Преобразуем лог в JSON
        let json = entry.to_json()?;
        
        // Открываем файл в режиме добавления
        use tokio::fs::OpenOptions;
        use tokio::io::AsyncWriteExt;
        
        let mut file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(&self.file_path)
            .await?;
            
        // Добавляем лог и новую строку
        file.write_all(json.as_bytes()).await?;
        file.write_all(b"\n").await?;
        
        Ok(())
    }
}

/// Обработчик логов для вывода в консоль
pub struct ConsoleLogHandler;

#[async_trait::async_trait]
impl LogHandler for ConsoleLogHandler {
    async fn handle(&self, entry: LogEntry) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Преобразуем лог в JSON
        let json = entry.to_json()?;
        
        // Выводим в консоль с соответствующим цветом для уровня
        match entry.level {
            LogLevel::Trace => println!("\x1b[90m{}\x1b[0m", json), // Серый
            LogLevel::Debug => println!("\x1b[36m{}\x1b[0m", json), // Голубой
            LogLevel::Info => println!("\x1b[32m{}\x1b[0m", json),  // Зеленый
            LogLevel::Warn => println!("\x1b[33m{}\x1b[0m", json),  // Желтый
            LogLevel::Error => println!("\x1b[31m{}\x1b[0m", json), // Красный
            LogLevel::Fatal => println!("\x1b[41m\x1b[37m{}\x1b[0m", json), // Белый на красном
        }
        
        Ok(())
    }
}

/// Основной логгер
pub struct Logger {
    service: String,
    handlers: Arc<Mutex<Vec<Box<dyn LogHandler>>>>,
}

impl Logger {
    pub fn new(service: impl Into<String>) -> Self {
        Self {
            service: service.into(),
            handlers: Arc::new(Mutex::new(Vec::new())),
        }
    }
    
    /// Добавить обработчик логов
    pub async fn add_handler<H: LogHandler + 'static>(&self, handler: H) {
        let mut handlers = self.handlers.lock().await;
        handlers.push(Box::new(handler));
    }
    
    /// Создать запись лога и отправить всем обработчикам
    pub async fn log(
        &self,
        level: LogLevel,
        component: impl Into<String>,
        message: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let entry = LogEntry::new(level, self.service.clone(), component.into(), message.into());
        self.log_entry(entry).await
    }
    
    /// Отправить готовую запись лога всем обработчикам
    pub async fn log_entry(
        &self,
        entry: LogEntry,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        let handlers = self.handlers.lock().await;
        
        for handler in handlers.iter() {
            // Игнорируем ошибки отдельных обработчиков, чтобы не блокировать весь логгер
            let _ = handler.handle(entry.clone()).await;
        }
        
        Ok(())
    }
    
    // Вспомогательные методы для разных уровней логирования
    
    pub async fn trace(
        &self,
        component: impl Into<String>,
        message: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.log(LogLevel::Trace, component, message).await
    }
    
    pub async fn debug(
        &self,
        component: impl Into<String>,
        message: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.log(LogLevel::Debug, component, message).await
    }
    
    pub async fn info(
        &self,
        component: impl Into<String>,
        message: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.log(LogLevel::Info, component, message).await
    }
    
    pub async fn warn(
        &self,
        component: impl Into<String>,
        message: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.log(LogLevel::Warn, component, message).await
    }
    
    pub async fn error(
        &self,
        component: impl Into<String>,
        message: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.log(LogLevel::Error, component, message).await
    }
    
    pub async fn fatal(
        &self,
        component: impl Into<String>,
        message: impl Into<String>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        self.log(LogLevel::Fatal, component, message).await
    }
}

// TODO: Добавить gRPC обработчик для отправки логов в центральный сервис 