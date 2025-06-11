use gameverse_core::plugins::{GameVersePlugin, PluginContext, PluginInfo, PluginResult, PluginError};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};
use chrono::{DateTime, Utc};

/// Структура игрока в экономической системе
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomyPlayer {
    pub id: u64,
    pub name: String,
    pub cash: f64,
    pub bank_balance: f64,
    pub last_login: DateTime<Utc>,
    pub transaction_history: Vec<Transaction>,
}

/// Транзакция в экономической системе
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Transaction {
    pub id: String,
    pub from_player: Option<u64>,
    pub to_player: Option<u64>,
    pub amount: f64,
    pub transaction_type: TransactionType,
    pub timestamp: DateTime<Utc>,
    pub description: String,
}

/// Типы транзакций
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TransactionType {
    Deposit,    // Внесение в банк
    Withdraw,   // Снятие из банка
    Transfer,   // Перевод между игроками
    Purchase,   // Покупка
    Salary,     // Зарплата
    AdminGive,  // Выдача администратором
}

/// Основной плагин экономики
pub struct EconomyPlugin {
    info: PluginInfo,
    players: Arc<RwLock<HashMap<u64, EconomyPlayer>>>,
    config: EconomyConfig,
    total_money_in_circulation: Arc<RwLock<f64>>,
}

/// Конфигурация экономики
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EconomyConfig {
    pub starting_balance: f64,
    pub max_cash: f64,
    pub transaction_tax: f64,
    pub deposit_interest_rate: f64,
    pub max_loan_amount: f64,
    pub transfer_fee: f64,
}

impl Default for EconomyConfig {
    fn default() -> Self {
        Self {
            starting_balance: 5000.0,
            max_cash: 100000.0,
            transaction_tax: 0.05,
            deposit_interest_rate: 0.02,
            max_loan_amount: 500000.0,
            transfer_fee: 50.0,
        }
    }
}

impl EconomyPlugin {
    pub fn new() -> Self {
        let info = PluginInfo {
            id: "economy-example".to_string(),
            name: "Economy Example".to_string(),
            version: semver::Version::parse("1.0.0").unwrap(),
            author: "GameVerse Team".to_string(),
            description: Some("Advanced economy system with currency management".to_string()),
            dependencies: vec![
                gameverse_core::plugins::PluginDependency {
                    name: "core".to_string(),
                    version_req: semver::VersionReq::parse(">=0.1.0").unwrap(),
                    optional: false,
                },
            ],
            permissions: vec![
                "economy.balance".to_string(),
                "economy.transfer".to_string(),
                "economy.deposit".to_string(),
                "economy.withdraw".to_string(),
            ],
            website: Some("https://gameverse.dev/plugins/economy".to_string()),
        };

        Self {
            info,
            players: Arc::new(RwLock::new(HashMap::new())),
            config: EconomyConfig::default(),
            total_money_in_circulation: Arc::new(RwLock::new(0.0)),
        }
    }

    /// Получить баланс игрока
    pub async fn get_balance(&self, player_id: u64) -> Option<(f64, f64)> {
        let players = self.players.read().await;
        players.get(&player_id).map(|p| (p.cash, p.bank_balance))
    }

    /// Перевести деньги между игроками
    pub async fn transfer_money(
        &self,
        from_player: u64,
        to_player: u64,
        amount: f64,
    ) -> PluginResult<String> {
        if amount <= 0.0 {
            return Err(PluginError::ApiError {
                reason: "Amount must be positive".to_string(),
            });
        }

        let mut players = self.players.write().await;

        // Проверить отправителя
        let from_player_data = players.get_mut(&from_player)
            .ok_or_else(|| PluginError::ApiError {
                reason: "From player not found".to_string(),
            })?;

        if from_player_data.cash < amount + self.config.transfer_fee {
            return Err(PluginError::ApiError {
                reason: "Insufficient funds (including transfer fee)".to_string(),
            });
        }

        // Проверить получателя
        if !players.contains_key(&to_player) {
            return Err(PluginError::ApiError {
                reason: "To player not found".to_string(),
            });
        }

        // Выполнить транзакцию
        let transaction_id = uuid::Uuid::new_v4().to_string();
        let transaction = Transaction {
            id: transaction_id.clone(),
            from_player: Some(from_player),
            to_player: Some(to_player),
            amount,
            transaction_type: TransactionType::Transfer,
            timestamp: Utc::now(),
            description: format!("Transfer from {} to {}", from_player, to_player),
        };

        // Списать у отправителя (с комиссией)
        if let Some(from) = players.get_mut(&from_player) {
            from.cash -= amount + self.config.transfer_fee;
            from.transaction_history.push(transaction.clone());
        }

        // Зачислить получателю
        if let Some(to) = players.get_mut(&to_player) {
            to.cash += amount;
            to.transaction_history.push(transaction);
        }

        println!("💰 Transfer completed: {} sent ${:.2} to {} (fee: ${:.2})", 
                from_player, amount, to_player, self.config.transfer_fee);

        Ok(transaction_id)
    }

    /// Внести деньги в банк
    pub async fn deposit_to_bank(&self, player_id: u64, amount: f64) -> PluginResult<()> {
        if amount <= 0.0 {
            return Err(PluginError::ApiError {
                reason: "Amount must be positive".to_string(),
            });
        }

        let mut players = self.players.write().await;
        let player = players.get_mut(&player_id)
            .ok_or_else(|| PluginError::ApiError {
                reason: "Player not found".to_string(),
            })?;

        if player.cash < amount {
            return Err(PluginError::ApiError {
                reason: "Insufficient cash".to_string(),
            });
        }

        // Выполнить депозит
        player.cash -= amount;
        player.bank_balance += amount;

        let transaction = Transaction {
            id: uuid::Uuid::new_v4().to_string(),
            from_player: Some(player_id),
            to_player: None,
            amount,
            transaction_type: TransactionType::Deposit,
            timestamp: Utc::now(),
            description: "Bank deposit".to_string(),
        };
        player.transaction_history.push(transaction);

        println!("🏦 Bank deposit: Player {} deposited ${:.2}", player_id, amount);

        Ok(())
    }

    /// Создать нового игрока в экономической системе
    pub async fn create_player(&self, player_id: u64, player_name: String) -> PluginResult<()> {
        let mut players = self.players.write().await;
        
        if players.contains_key(&player_id) {
            return Err(PluginError::ApiError {
                reason: "Player already exists".to_string(),
            });
        }

        let new_player = EconomyPlayer {
            id: player_id,
            name: player_name.clone(),
            cash: self.config.starting_balance,
            bank_balance: 0.0,
            last_login: Utc::now(),
            transaction_history: vec![],
        };

        players.insert(player_id, new_player);

        // Обновить общую сумму денег в обращении
        {
            let mut total = self.total_money_in_circulation.write().await;
            *total += self.config.starting_balance;
        }

        println!("👤 New player created: {} (ID: {}) with starting balance ${:.2}", 
                player_name, player_id, self.config.starting_balance);

        Ok(())
    }

    /// Получить статистику экономики
    pub async fn get_economy_stats(&self) -> EconomyStats {
        let players = self.players.read().await;
        let total_circulation = *self.total_money_in_circulation.read().await;

        let total_players = players.len();
        let total_cash: f64 = players.values().map(|p| p.cash).sum();
        let total_bank: f64 = players.values().map(|p| p.bank_balance).sum();

        EconomyStats {
            total_players,
            total_cash,
            total_bank_balance: total_bank,
            total_money_in_circulation: total_circulation,
            average_player_wealth: if total_players > 0 { 
                (total_cash + total_bank) / total_players as f64 
            } else { 
                0.0 
            },
        }
    }
}

/// Статистика экономической системы
#[derive(Debug, Serialize, Deserialize)]
pub struct EconomyStats {
    pub total_players: usize,
    pub total_cash: f64,
    pub total_bank_balance: f64,
    pub total_money_in_circulation: f64,
    pub average_player_wealth: f64,
}

impl GameVersePlugin for EconomyPlugin {
    fn info(&self) -> PluginInfo {
        self.info.clone()
    }

    fn initialize(&mut self, _context: &PluginContext) -> PluginResult<()> {
        println!("🚀 Economy plugin initializing...");
        
        // Загрузка конфигурации, подключение к БД, etc.
        println!("💼 Economy config loaded:");
        println!("   Starting balance: ${:.2}", self.config.starting_balance);
        println!("   Max cash: ${:.2}", self.config.max_cash);
        println!("   Transfer fee: ${:.2}", self.config.transfer_fee);
        
        println!("✅ Economy plugin initialized successfully!");
        Ok(())
    }

    fn finalize(&mut self) -> PluginResult<()> {
        println!("🛑 Economy plugin shutting down...");
        
        // Сохранение данных, закрытие соединений
        let stats = tokio::runtime::Handle::current().block_on(self.get_economy_stats());
        println!("📊 Final economy stats:");
        println!("   Total players: {}", stats.total_players);
        println!("   Total money: ${:.2}", stats.total_money_in_circulation);
        
        println!("✅ Economy plugin finalized!");
        Ok(())
    }

    fn on_player_connect(&self, player_id: u64) -> PluginResult<()> {
        println!("👋 Player {} connected to economy system", player_id);
        
        // Создать игрока если не существует
        let player_name = format!("Player_{}", player_id);
        tokio::runtime::Handle::current().block_on(async {
            if let Err(e) = self.create_player(player_id, player_name).await {
                // Игрок уже существует - это нормально
                if !e.to_string().contains("already exists") {
                    println!("⚠️ Failed to create player {}: {}", player_id, e);
                }
            }
        });

        Ok(())
    }

    fn on_player_disconnect(&self, player_id: u64) -> PluginResult<()> {
        println!("👋 Player {} disconnected from economy system", player_id);
        
        // Сохранить данные игрока в БД
        tokio::runtime::Handle::current().block_on(async {
            let mut players = self.players.write().await;
            if let Some(player) = players.get_mut(&player_id) {
                player.last_login = Utc::now();
                println!("💾 Saved player {} data", player_id);
            }
        });

        Ok(())
    }
} 