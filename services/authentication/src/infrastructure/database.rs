use anyhow::Result;
use sqlx::{postgres::PgPoolOptions, Pool, Postgres};
use std::time::Duration;
use tracing::{info, debug, warn, error};

use crate::config::DatabaseConfig;

/// Создает пул соединений с базой данных
pub async fn create_pool(config: &DatabaseConfig) -> Result<Pool<Postgres>> {
    debug!("Создание пула соединений с базой данных: {}", config.postgres_url);
    
    let pool = PgPoolOptions::new()
        .max_connections(config.max_connections)
        .acquire_timeout(config.connection_timeout())
        .idle_timeout(Some(Duration::from_secs(300)))
        .connect(&config.postgres_url)
        .await?;
    
    // Проверяем соединение с базой данных
    debug!("Проверка соединения с базой данных");
    let result = sqlx::query("SELECT 1").execute(&pool).await?;
    debug!("Соединение с базой данных установлено, результат: {:?}", result);
    
    // Применяем миграции
    apply_migrations(&pool).await?;
    
    info!("Пул соединений с базой данных успешно создан");
    Ok(pool)
}

/// Применяет миграции к базе данных
async fn apply_migrations(pool: &Pool<Postgres>) -> Result<()> {
    info!("Применение миграций к базе данных");
    
    // Применяем миграции из директории migrations
    match sqlx::migrate!("./migrations").run(pool).await {
        Ok(_) => {
            info!("Миграции успешно применены");
            Ok(())
        },
        Err(e) => {
            error!("Ошибка применения миграций: {}", e);
            Err(anyhow::anyhow!("Ошибка применения миграций: {}", e))
        }
    }
}

/// Очищает базу данных (только для тестов)
#[cfg(test)]
pub async fn clear_database(pool: &Pool<Postgres>) -> Result<()> {
    warn!("Очистка базы данных");
    
    sqlx::query("TRUNCATE TABLE tokens CASCADE").execute(pool).await?;
    sqlx::query("TRUNCATE TABLE users CASCADE").execute(pool).await?;
    
    info!("База данных очищена");
    Ok(())
} 