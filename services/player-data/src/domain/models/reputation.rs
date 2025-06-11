use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use std::collections::HashMap;

/// Система репутации игрока
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerReputation {
    pub player_id: Uuid,
    pub overall_karma: i32,          // Общая карма (-1000 до +1000)
    pub reputation_points: u32,      // Очки репутации (только положительные)
    pub honor_level: u32,           // Уровень чести (1-100)
    pub categories: HashMap<ReputationCategory, i32>, // Репутация по категориям
    pub titles: Vec<ReputationTitle>, // Заработанные титулы
    pub badges: Vec<ReputationBadge>, // Заработанные значки
    pub last_updated: DateTime<Utc>,
    pub created_at: DateTime<Utc>,
}

/// Категории репутации
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum ReputationCategory {
    Combat,          // Боевая репутация
    Racing,          // Гоночная репутация
    Business,        // Бизнес репутация
    Criminal,        // Криминальная репутация
    Helpful,         // Помощь другим игрокам
    Leadership,      // Лидерские качества
    Roleplay,        // Ролевая игра
    Competitive,     // Соревновательная игра
    Community,       // Вклад в сообщество
    Mentorship,      // Наставничество новичков
}

/// Титулы игроков
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationTitle {
    pub id: Uuid,
    pub title_type: TitleType,
    pub name: String,
    pub description: String,
    pub icon_url: Option<String>,
    pub rarity: TitleRarity,
    pub requirements: TitleRequirements,
    pub earned_at: Option<DateTime<Utc>>,
    pub is_active: bool,            // Активен ли титул у игрока
    pub is_visible: bool,           // Виден ли другим игрокам
    pub expires_at: Option<DateTime<Utc>>, // Временные титулы
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TitleType {
    Achievement,     // За достижения
    Seasonal,        // Сезонный титул
    Event,          // За участие в событиях
    Rank,           // Ранговый титул
    Special,        // Специальный (от администрации)
    Legacy,         // Наследственный (старые игроки)
    Community,      // За вклад в сообщество
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum TitleRarity {
    Common,         // Обычный
    Uncommon,       // Необычный
    Rare,           // Редкий
    Epic,           // Эпический
    Legendary,      // Легендарный
    Mythic,         // Мифический
}

/// Требования для получения титула
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TitleRequirements {
    pub min_level: Option<u32>,
    pub min_playtime_hours: Option<f32>,
    pub min_karma: Option<i32>,
    pub min_honor: Option<u32>,
    pub required_achievements: Vec<String>,
    pub category_requirements: HashMap<ReputationCategory, i32>,
    pub special_conditions: Vec<String>,
}

/// Значки репутации
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationBadge {
    pub id: Uuid,
    pub badge_type: BadgeType,
    pub name: String,
    pub description: String,
    pub icon_url: String,
    pub progress: u32,              // Текущий прогресс
    pub max_progress: u32,          // Максимальный прогресс для получения
    pub is_completed: bool,
    pub earned_at: Option<DateTime<Utc>>,
    pub category: ReputationCategory,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum BadgeType {
    Progress,       // Прогрессивный (можно улучшать)
    Binary,         // Бинарный (есть/нет)
    Counter,        // Счетчик (количественный)
    Streak,         // Серия (подряд)
    Milestone,      // Веха (определенное достижение)
}

/// История изменений репутации
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationHistory {
    pub id: Uuid,
    pub player_id: Uuid,
    pub category: ReputationCategory,
    pub change_type: ReputationChangeType,
    pub amount: i32,
    pub reason: String,
    pub source_player_id: Option<Uuid>,  // Кто дал/убавил репутацию
    pub match_id: Option<Uuid>,          // Связанный матч
    pub event_id: Option<Uuid>,          // Связанное событие
    pub timestamp: DateTime<Utc>,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ReputationChangeType {
    Earned,         // Заработано
    Lost,           // Потеряно
    Bonus,          // Бонус
    Penalty,        // Штраф
    Transfer,       // Передача от другого игрока
    Admin,          // Изменение администратором
    Decay,          // Естественное убывание
    Reset,          // Сброс
}

/// Система голосования за игроков
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerVoting {
    pub id: Uuid,
    pub voter_id: Uuid,
    pub target_id: Uuid,
    pub vote_type: VoteType,
    pub category: ReputationCategory,
    pub reason: Option<String>,
    pub match_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub is_valid: bool,              // Валидно ли голосование
    pub weight: f32,                 // Вес голоса (зависит от репутации голосующего)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum VoteType {
    Positive,       // Положительный голос (+1)
    Negative,       // Отрицательный голос (-1)
    Neutral,        // Нейтральный (0)
}

/// Система влияния игроков
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerInfluence {
    pub player_id: Uuid,
    pub influence_score: f32,        // Общий показатель влияния
    pub categories: HashMap<ReputationCategory, f32>, // Влияние по категориям
    pub followers: u32,              // Количество подписчиков
    pub following: u32,              // На кого подписан
    pub endorsements: u32,           // Количество одобрений
    pub content_score: f32,          // Очки за контент (видео, гайды и т.д.)
    pub last_calculated: DateTime<Utc>,
}

/// Лидеры сообщества
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CommunityLeader {
    pub player_id: Uuid,
    pub category: ReputationCategory,
    pub rank: u32,                   // Позиция в рейтинге
    pub score: f32,                  // Общий балл лидерства
    pub period: LeaderboardPeriod,   // Период (месяц, сезон, все время)
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum LeaderboardPeriod {
    Weekly,
    Monthly,
    Seasonal,
    AllTime,
}

/// События репутации
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReputationEvent {
    pub id: Uuid,
    pub event_type: EventType,
    pub name: String,
    pub description: String,
    pub start_time: DateTime<Utc>,
    pub end_time: DateTime<Utc>,
    pub multiplier: f32,             // Множитель репутации во время события
    pub categories: Vec<ReputationCategory>, // Затронутые категории
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum EventType {
    DoubleReputation,    // Двойная репутация
    KarmaBoost,         // Усиление кармы
    CommunityDay,       // День сообщества
    SpecialEvent,       // Специальное событие
}

impl PlayerReputation {
    pub fn new(player_id: Uuid) -> Self {
        let now = Utc::now();
        
        Self {
            player_id,
            overall_karma: 0,
            reputation_points: 0,
            honor_level: 1,
            categories: HashMap::new(),
            titles: Vec::new(),
            badges: Vec::new(),
            last_updated: now,
            created_at: now,
        }
    }
    
    pub fn add_reputation(&mut self, category: ReputationCategory, amount: i32) {
        let current = self.categories.get(&category).copied().unwrap_or(0);
        self.categories.insert(category, current + amount);
        
        // Обновляем общую карму
        self.overall_karma += amount;
        
        // Клампим карму в пределах -1000 до +1000
        self.overall_karma = self.overall_karma.max(-1000).min(1000);
        
        // Обновляем очки репутации (только положительные изменения)
        if amount > 0 {
            self.reputation_points += amount as u32;
        }
        
        self.update_honor_level();
        self.last_updated = Utc::now();
    }
    
    fn update_honor_level(&mut self) {
        // Уровень чести основан на общей карме и очках репутации
        let karma_factor = ((self.overall_karma + 1000) as f32 / 2000.0).max(0.0).min(1.0);
        let rep_factor = (self.reputation_points as f32 / 10000.0).min(1.0);
        
        let combined_score = (karma_factor * 0.6 + rep_factor * 0.4) * 100.0;
        self.honor_level = (combined_score as u32).max(1).min(100);
    }
    
    pub fn get_karma_level(&self) -> KarmaLevel {
        match self.overall_karma {
            -1000..=-500 => KarmaLevel::VeryBad,
            -499..=-100 => KarmaLevel::Bad,
            -99..=99 => KarmaLevel::Neutral,
            100..=499 => KarmaLevel::Good,
            _ => KarmaLevel::VeryGood,
        }
    }
    
    pub fn get_reputation_rank(&self) -> ReputationRank {
        match self.honor_level {
            1..=10 => ReputationRank::Newcomer,
            11..=25 => ReputationRank::Citizen,
            26..=40 => ReputationRank::Respected,
            41..=60 => ReputationRank::Honored,
            61..=80 => ReputationRank::Distinguished,
            81..=95 => ReputationRank::Legendary,
            _ => ReputationRank::Mythical,
        }
    }
    
    pub fn can_vote(&self, category: &ReputationCategory) -> bool {
        // Игрок может голосовать, если у него достаточно репутации в категории
        let min_reputation = 50;
        self.categories.get(category).copied().unwrap_or(0) >= min_reputation
    }
    
    pub fn get_vote_weight(&self, category: &ReputationCategory) -> f32 {
        // Вес голоса зависит от репутации в категории
        let rep = self.categories.get(category).copied().unwrap_or(0) as f32;
        (rep / 1000.0).min(2.0).max(0.1) // От 0.1 до 2.0
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum KarmaLevel {
    VeryBad,        // -1000 до -500
    Bad,            // -499 до -100
    Neutral,        // -99 до +99
    Good,           // +100 до +499
    VeryGood,       // +500 до +1000
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum ReputationRank {
    Newcomer,       // 1-10
    Citizen,        // 11-25
    Respected,      // 26-40
    Honored,        // 41-60
    Distinguished,  // 61-80
    Legendary,      // 81-95
    Mythical,       // 96-100
}

impl PlayerInfluence {
    pub fn new(player_id: Uuid) -> Self {
        Self {
            player_id,
            influence_score: 0.0,
            categories: HashMap::new(),
            followers: 0,
            following: 0,
            endorsements: 0,
            content_score: 0.0,
            last_calculated: Utc::now(),
        }
    }
    
    pub fn calculate_influence(&mut self, reputation: &PlayerReputation) {
        let mut score = 0.0;
        
        // Базовый балл от уровня чести
        score += reputation.honor_level as f32 * 0.5;
        
        // Бонус от подписчиков
        score += (self.followers as f32).sqrt() * 2.0;
        
        // Бонус от одобрений
        score += self.endorsements as f32 * 0.1;
        
        // Бонус от контента
        score += self.content_score;
        
        // Штраф за негативную карму
        if reputation.overall_karma < 0 {
            score *= 1.0 + (reputation.overall_karma as f32 / 1000.0);
        }
        
        self.influence_score = score.max(0.0);
        self.last_calculated = Utc::now();
    }
} 