use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::collections::HashMap;

/// Рейтинг игрока в различных категориях
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerRating {
    pub id: Uuid,
    pub player_id: Uuid,
    pub category: RatingCategory,
    pub current_rating: u32,
    pub peak_rating: u32,
    pub games_played: u32,
    pub wins: u32,
    pub losses: u32,
    pub draws: u32,
    pub season_id: Option<Uuid>,
    pub last_updated: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

/// Категории рейтингов
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "rating_category")]
pub enum RatingCategory {
    // PvP рейтинги
    Deathmatch,      // Дефматч
    Racing,          // Гонки
    LastTeamStanding, // Последняя команда
    Adversary,       // Противостояние
    
    // Кооперативные режимы
    Heists,          // Ограбления
    Missions,        // Миссии
    Survival,        // Выживание
    
    // Специальные режимы
    Arena,           // Арена
    GunRunning,      // Торговля оружием
    Import,          // Импорт/экспорт
    
    // Общие категории
    Overall,         // Общий рейтинг
    Criminal,        // Криминальная деятельность
    Business,        // Бизнес активность
}

/// Ранги в зависимости от рейтинга
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum PlayerRank {
    Newbie,          // 0-999
    Amateur,         // 1000-1499
    Intermediate,    // 1500-1999
    Advanced,        // 2000-2499  
    Expert,          // 2500-2999
    Master,          // 3000-3499
    GrandMaster,     // 3500-3999
    Legend,          // 4000+
}

/// Сезонная система
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Season {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub start_date: DateTime<Utc>,
    pub end_date: DateTime<Utc>,
    pub is_active: bool,
    pub rewards: HashMap<RatingCategory, SeasonRewards>,
}

/// Награды за сезон
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SeasonRewards {
    pub rank_rewards: HashMap<PlayerRank, Vec<RewardItem>>,
    pub top_player_rewards: Vec<RewardItem>,  // Топ-10 игроков
    pub participation_rewards: Vec<RewardItem>, // За участие
}

/// Предмет награды
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RewardItem {
    pub item_type: RewardType,
    pub item_id: String,
    pub quantity: u32,
    pub description: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RewardType {
    Currency,        // Игровая валюта
    Gems,           // Премиум валюта
    Title,          // Титул
    Badge,          // Значок
    Skin,           // Скин
    Vehicle,        // Транспорт
    Weapon,         // Оружие
    Experience,     // Опыт
}

/// Лидерборды
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Leaderboard {
    pub id: Uuid,
    pub category: RatingCategory,
    pub board_type: LeaderboardType,
    pub region: Option<String>,
    pub season_id: Option<Uuid>,
    pub last_updated: DateTime<Utc>,
    pub entries: Vec<LeaderboardEntry>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LeaderboardType {
    Global,          // Глобальный
    Regional,        // По региону
    Friends,         // Среди друзей
    Clan,           // В рамках клана
    Weekly,         // Недельный
    Monthly,        // Месячный
    AllTime,        // За все время
}

/// Запись в лидерборде
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardEntry {
    pub rank: u32,
    pub player_id: Uuid,
    pub username: String,
    pub display_name: String,
    pub clan_tag: Option<String>,
    pub rating: u32,
    pub stats: LeaderboardStats,
}

/// Статистика для лидерборда
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct LeaderboardStats {
    pub games_played: u32,
    pub win_rate: f32,
    pub peak_rating: u32,
    pub streak: i32,  // Текущая серия побед/поражений
    pub additional_stats: HashMap<String, f64>, // Дополнительные метрики
}

/// История изменений рейтинга
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RatingHistory {
    pub id: Uuid,
    pub player_id: Uuid,
    pub category: RatingCategory,
    pub old_rating: u32,
    pub new_rating: u32,
    pub change: i32,
    pub match_id: Option<Uuid>,
    pub reason: RatingChangeReason,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum RatingChangeReason {
    MatchWin,        // Победа в матче
    MatchLoss,       // Поражение в матче
    MatchDraw,       // Ничья
    SeasonReset,     // Сброс сезона
    AdminAdjustment, // Корректировка администратором
    Penalty,         // Штраф за нарушения
    Bonus,          // Бонус
}

/// Турниры и события
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tournament {
    pub id: Uuid,
    pub name: String,
    pub description: String,
    pub category: RatingCategory,
    pub tournament_type: TournamentType,
    pub max_participants: u32,
    pub entry_fee: u64,
    pub prize_pool: u64,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub registration_deadline: DateTime<Utc>,
    pub status: TournamentStatus,
    pub rules: String,
    pub organizer_id: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TournamentType {
    SingleElimination,  // На выбывание
    DoubleElimination,  // Двойное выбывание
    RoundRobin,        // Круговая система
    Swiss,             // Швейцарская система
    Ladder,            // Лестничная система
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TournamentStatus {
    Registration,      // Регистрация открыта
    Ready,            // Готов к началу
    InProgress,       // В процессе
    Completed,        // Завершен
    Cancelled,        // Отменен
}

/// Участие в турнире
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TournamentParticipant {
    pub tournament_id: Uuid,
    pub player_id: Uuid,
    pub registration_time: DateTime<Utc>,
    pub seed: Option<u32>,  // Позиция в сетке
    pub current_round: u32,
    pub wins: u32,
    pub losses: u32,
    pub status: ParticipantStatus,
    pub prize_won: Option<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ParticipantStatus {
    Registered,        // Зарегистрирован
    Active,           // Активно участвует
    Eliminated,       // Исключен
    Winner,           // Победитель
    Withdrawn,        // Снялся с турнира
}

impl PlayerRating {
    pub fn new(player_id: Uuid, category: RatingCategory) -> Self {
        let now = Utc::now();
        
        Self {
            id: Uuid::new_v4(),
            player_id,
            category,
            current_rating: 1000, // Начальный рейтинг
            peak_rating: 1000,
            games_played: 0,
            wins: 0,
            losses: 0,
            draws: 0,
            season_id: None,
            last_updated: now,
            created_at: now,
        }
    }
    
    pub fn update_rating(&mut self, new_rating: u32, result: MatchResult) {
        self.current_rating = new_rating;
        
        if new_rating > self.peak_rating {
            self.peak_rating = new_rating;
        }
        
        self.games_played += 1;
        
        match result {
            MatchResult::Win => self.wins += 1,
            MatchResult::Loss => self.losses += 1,
            MatchResult::Draw => self.draws += 1,
        }
        
        self.last_updated = Utc::now();
    }
    
    pub fn get_rank(&self) -> PlayerRank {
        match self.current_rating {
            0..=999 => PlayerRank::Newbie,
            1000..=1499 => PlayerRank::Amateur,
            1500..=1999 => PlayerRank::Intermediate,
            2000..=2499 => PlayerRank::Advanced,
            2500..=2999 => PlayerRank::Expert,
            3000..=3499 => PlayerRank::Master,
            3500..=3999 => PlayerRank::GrandMaster,
            _ => PlayerRank::Legend,
        }
    }
    
    pub fn win_rate(&self) -> f32 {
        if self.games_played == 0 {
            return 0.0;
        }
        self.wins as f32 / self.games_played as f32
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum MatchResult {
    Win,
    Loss,
    Draw,
} 