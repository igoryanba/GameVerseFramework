use crate::{LogEntry, LogHandler};
use reqwest::{Client, header};
use serde_json::json;
use tokio::sync::mpsc;
use std::time::Duration;

/// Обработчик логов для отправки в ElasticSearch
pub struct ElasticsearchLogHandler {
    client: Client,
    elasticsearch_url: String,
    index_name: String,
    pending_logs: mpsc::Sender<LogEntry>,
    batch_size: usize,
    flush_interval: Duration,
}

impl ElasticsearchLogHandler {
    /// Создать новый обработчик для ElasticSearch
    /// 
    /// # Аргументы
    /// * `elasticsearch_url` - URL ElasticSearch (например, "http://localhost:9200")
    /// * `index_name` - Имя индекса (например, "gameverse-logs")
    /// * `batch_size` - Размер пакета логов для отправки (по умолчанию 50)
    /// * `flush_interval_secs` - Интервал принудительной отправки в секундах (по умолчанию 5)
    pub async fn new(
        elasticsearch_url: impl Into<String>,
        index_name: impl Into<String>,
        batch_size: Option<usize>,
        flush_interval_secs: Option<u64>,
    ) -> Result<Self, Box<dyn std::error::Error + Send + Sync>> {
        let elasticsearch_url = elasticsearch_url.into();
        let index_name = index_name.into();
        let batch_size = batch_size.unwrap_or(50);
        let flush_interval = Duration::from_secs(flush_interval_secs.unwrap_or(5));
        
        // Создаем HTTP клиент
        let client = Client::builder()
            .timeout(Duration::from_secs(10))
            .build()?;
        
        // Проверяем подключение к ElasticSearch
        let response = client.get(&format!("{}", elasticsearch_url))
            .send()
            .await?;
            
        if !response.status().is_success() {
            return Err(format!("Не удалось подключиться к ElasticSearch: {}", response.status()).into());
        }
        
        // Создаем канал для буферизации логов
        let (tx, mut rx) = mpsc::channel::<LogEntry>(100);
        
        // Клонируем необходимые данные для фоновой задачи
        let es_url = elasticsearch_url.clone();
        let idx_name = index_name.clone();
        let client_clone = client.clone();
        
        // Запускаем фоновую задачу для отправки логов пакетами
        tokio::spawn(async move {
            let mut batch = Vec::with_capacity(batch_size);
            let mut interval = tokio::time::interval(flush_interval);
            
            loop {
                tokio::select! {
                    _ = interval.tick() => {
                        if !batch.is_empty() {
                            let logs = std::mem::take(&mut batch);
                            if let Err(e) = Self::send_batch(&client_clone, &es_url, &idx_name, logs).await {
                                eprintln!("Ошибка отправки логов в ElasticSearch: {}", e);
                            }
                        }
                    }
                    Some(log) = rx.recv() => {
                        batch.push(log);
                        if batch.len() >= batch_size {
                            let logs = std::mem::take(&mut batch);
                            if let Err(e) = Self::send_batch(&client_clone, &es_url, &idx_name, logs).await {
                                eprintln!("Ошибка отправки логов в ElasticSearch: {}", e);
                            }
                        }
                    }
                }
            }
        });
        
        Ok(Self {
            client,
            elasticsearch_url,
            index_name,
            pending_logs: tx,
            batch_size,
            flush_interval,
        })
    }
    
    /// Отправить пакет логов в ElasticSearch через Bulk API
    async fn send_batch(
        client: &Client,
        elasticsearch_url: &str,
        index_name: &str,
        logs: Vec<LogEntry>,
    ) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        if logs.is_empty() {
            return Ok(());
        }
        
        // Формируем bulk запрос
        let mut bulk_body = String::new();
        
        for log in logs {
            // Для каждого лога добавляем метаданные и сам документ
            let action_and_metadata = json!({
                "index": {
                    "_index": index_name
                }
            });
            
            // Добавляем метаданные и сам документ в bulk запрос
            bulk_body.push_str(&serde_json::to_string(&action_and_metadata)?);
            bulk_body.push_str("\n");
            bulk_body.push_str(&serde_json::to_string(&log)?);
            bulk_body.push_str("\n");
        }
        
        // Отправляем запрос в ElasticSearch
        let response = client.post(&format!("{}/_bulk", elasticsearch_url))
            .header(header::CONTENT_TYPE, "application/x-ndjson")
            .body(bulk_body)
            .send()
            .await?;
            
        if !response.status().is_success() {
            let status = response.status();
            let error_text = response.text().await?;
            return Err(format!("Ошибка отправки логов в ElasticSearch: {} - {}", 
                status, error_text).into());
        }
        
        Ok(())
    }
}

#[async_trait::async_trait]
impl LogHandler for ElasticsearchLogHandler {
    async fn handle(&self, entry: LogEntry) -> Result<(), Box<dyn std::error::Error + Send + Sync>> {
        // Отправляем лог в канал для буферизации
        if let Err(e) = self.pending_logs.send(entry).await {
            return Err(Box::new(std::io::Error::new(
                std::io::ErrorKind::Other,
                format!("Не удалось отправить лог в буфер ElasticSearch: {}", e),
            )));
        }
        
        Ok(())
    }
} 