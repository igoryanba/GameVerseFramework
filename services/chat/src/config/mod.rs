use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatConfig {
    pub database: DatabaseConfig,
    pub redis: RedisConfig,
    pub server: ServerConfig,
    pub telegram: TelegramConfig,
    pub channels: ChannelConfig,
    pub security: SecurityConfig,
    pub voice: VoiceConfig,
    pub integrations: IntegrationConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatabaseConfig {
    pub url: String,
    pub max_connections: u32,
    pub min_connections: u32,
    pub connection_timeout: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RedisConfig {
    pub url: String,
    pub max_connections: u32,
    pub pool_timeout: u64,
    pub command_timeout: u64,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerConfig {
    pub http_port: u16,
    pub grpc_port: u16,
    pub websocket_port: u16,
    pub host: String,
    pub cors_allowed_origins: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TelegramConfig {
    pub bot_token: String,
    pub webhook_url: Option<String>,
    pub enabled: bool,
    pub admin_chat_id: Option<String>,
    pub notifications_enabled: bool,
    pub commands_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelConfig {
    pub global: ChannelSettings,
    pub local: LocalChannelSettings,
    pub radio: RadioChannelSettings,
    pub ooc: ChannelSettings,
    pub admin: ChannelSettings,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChannelSettings {
    pub enabled: bool,
    pub rate_limit: u32,  // сообщений в минуту
    pub max_length: usize,
    pub roleplay_only: bool,
    pub prefix: Option<String>,
    pub suffix: Option<String>,
    pub color: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LocalChannelSettings {
    #[serde(flatten)]
    pub base: ChannelSettings,
    pub distance: f64,  // метры
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RadioChannelSettings {
    #[serde(flatten)]
    pub base: ChannelSettings,
    pub frequencies: Vec<String>,
    pub require_item: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SecurityConfig {
    pub jwt_secret: String,
    pub rate_limit_messages_per_minute: u32,
    pub max_message_length: usize,
    pub max_channel_participants: u32,
    pub message_history_days: u32,
    pub profanity_filter_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoiceConfig {
    pub enabled: bool,
    pub pma_voice_integration: bool,
    pub proximity_range_max: f64,
    pub voice_ranges: HashMap<String, f64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct IntegrationConfig {
    pub auth_service_url: String,
    pub player_service_url: String,
    pub logging_service_url: String,
}

impl Default for ChatConfig {
    fn default() -> Self {
        Self {
            database: DatabaseConfig {
                url: "postgresql://localhost:5432/gameverse_chat".to_string(),
                max_connections: 10,
                min_connections: 2,
                connection_timeout: 30,
            },
            redis: RedisConfig {
                url: "redis://localhost:6379".to_string(),
                max_connections: 10,
                pool_timeout: 30,
                command_timeout: 5,
            },
            server: ServerConfig {
                http_port: 8080,
                grpc_port: 50053,
                websocket_port: 8081,
                host: "0.0.0.0".to_string(),
                cors_allowed_origins: vec!["*".to_string()],
            },
            telegram: TelegramConfig {
                bot_token: "".to_string(),
                webhook_url: None,
                enabled: false,
                admin_chat_id: None,
                notifications_enabled: true,
                commands_enabled: true,
            },
            channels: ChannelConfig {
                global: ChannelSettings {
                    enabled: true,
                    rate_limit: 10,
                    max_length: 200,
                    roleplay_only: false,
                    prefix: None,
                    suffix: None,
                    color: Some("#FFFFFF".to_string()),
                },
                local: LocalChannelSettings {
                    base: ChannelSettings {
                        enabled: true,
                        rate_limit: 30,
                        max_length: 500,
                        roleplay_only: true,
                        prefix: None,
                        suffix: None,
                        color: Some("#FFFF00".to_string()),
                    },
                    distance: 20.0,
                },
                radio: RadioChannelSettings {
                    base: ChannelSettings {
                        enabled: true,
                        rate_limit: 60,
                        max_length: 300,
                        roleplay_only: true,
                        prefix: Some("[RADIO]".to_string()),
                        suffix: None,
                        color: Some("#00FF00".to_string()),
                    },
                    frequencies: vec!["100.1".to_string(), "200.5".to_string()],
                    require_item: Some("radio".to_string()),
                },
                ooc: ChannelSettings {
                    enabled: true,
                    rate_limit: 15,
                    max_length: 300,
                    roleplay_only: false,
                    prefix: Some("((".to_string()),
                    suffix: Some("))".to_string()),
                    color: Some("#888888".to_string()),
                },
                admin: ChannelSettings {
                    enabled: true,
                    rate_limit: 100,
                    max_length: 1000,
                    roleplay_only: false,
                    prefix: Some("[ADMIN]".to_string()),
                    suffix: None,
                    color: Some("#FF0000".to_string()),
                },
            },
            security: SecurityConfig {
                jwt_secret: "your-secret-key".to_string(),
                rate_limit_messages_per_minute: 30,
                max_message_length: 500,
                max_channel_participants: 100,
                message_history_days: 30,
                profanity_filter_enabled: true,
            },
            voice: VoiceConfig {
                enabled: true,
                pma_voice_integration: true,
                proximity_range_max: 50.0,
                voice_ranges: {
                    let mut ranges = HashMap::new();
                    ranges.insert("whisper".to_string(), 3.0);
                    ranges.insert("normal".to_string(), 10.0);
                    ranges.insert("shout".to_string(), 25.0);
                    ranges
                },
            },
            integrations: IntegrationConfig {
                auth_service_url: "http://localhost:8079".to_string(),
                player_service_url: "http://localhost:8082".to_string(),
                logging_service_url: "http://localhost:50051".to_string(),
            },
        }
    }
}

impl ChatConfig {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let _cfg = config::Config::builder()
            .add_source(config::Environment::with_prefix("CHAT"))
            .build()?;
        
        // Применяем значения по умолчанию и переопределяем из env
        let mut config = Self::default();
        
        if let Ok(database_url) = std::env::var("DATABASE_URL") {
            config.database.url = database_url;
        }
        
        if let Ok(redis_url) = std::env::var("REDIS_URL") {
            config.redis.url = redis_url;
        }
        
        if let Ok(telegram_token) = std::env::var("TELEGRAM_BOT_TOKEN") {
            config.telegram.bot_token = telegram_token;
            config.telegram.enabled = true;
        }
        
        if let Ok(jwt_secret) = std::env::var("JWT_SECRET") {
            config.security.jwt_secret = jwt_secret;
        }
        
        Ok(config)
    }
} 