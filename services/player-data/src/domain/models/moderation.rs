use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;
use validator::Validate;

/// Система наказаний
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerPunishment {
    pub id: Uuid,
    pub player_id: Uuid,
    pub punishment_type: PunishmentType,
    pub reason: String,
    pub moderator_id: Uuid,
    pub duration: Option<u64>,  // Длительность в секундах (None = перманентно)
    pub start_time: DateTime<Utc>,
    pub end_time: Option<DateTime<Utc>>,
    pub is_active: bool,
    pub appeal_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
    pub metadata: PunishmentMetadata,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "punishment_type")]
pub enum PunishmentType {
    Warning,         // Предупреждение
    Mute,           // Мут в чате
    VoiceMute,      // Мут в голосовом чате
    Kick,           // Кик с сервера
    TempBan,        // Временный бан
    PermBan,        // Перманентный бан
    GamemodeBan,    // Бан в определенном режиме игры
    FeatureBan,     // Бан конкретных функций (торговля, чат и т.д.)
    Suspension,     // Временная заморозка аккаунта
}

/// Метаданные наказания
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PunishmentMetadata {
    pub ip_address: Option<String>,
    pub hardware_id: Option<String>,
    pub server_id: Option<String>,
    pub evidence_urls: Vec<String>,  // Ссылки на доказательства
    pub witness_ids: Vec<Uuid>,      // Свидетели нарушения
    pub automatic: bool,             // Автоматическое наказание
    pub anti_cheat_detection: Option<AntiCheatInfo>,
}

/// Информация об анти-чите
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AntiCheatInfo {
    pub detection_type: CheatType,
    pub confidence_level: f32,
    pub detection_time: DateTime<Utc>,
    pub system_version: String,
    pub additional_data: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum CheatType {
    Aimbot,          // Аимбот
    Wallhack,        // Вх (видение сквозь стены)
    SpeedHack,       // Спидхак
    NoClip,          // Прохождение сквозь стены
    GodMode,         // Режим бога
    MoneyHack,       // Накрутка денег
    StatHack,        // Накрутка статистики
    Exploiting,      // Использование багов
    Scripting,       // Запрещенные скрипты
    ModMenu,         // Мод-меню
    Other,           // Другое
}

/// Система отчетов о нарушениях
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerReport {
    pub id: Uuid,
    pub reporter_id: Uuid,
    pub reported_id: Uuid,
    pub report_type: ReportType,
    pub reason: String,
    pub description: String,
    pub evidence_urls: Vec<String>,
    pub server_id: Option<String>,
    pub match_id: Option<Uuid>,
    pub timestamp: DateTime<Utc>,
    pub status: ReportStatus,
    pub assigned_moderator: Option<Uuid>,
    pub resolution: Option<ReportResolution>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "report_type")]
pub enum ReportType {
    Cheating,        // Читерство
    Toxicity,        // Токсичное поведение
    Harassment,      // Домогательства
    Griefing,        // Грифинг
    Exploiting,      // Эксплойты
    RealMoneyTrading, // Торговля за реальные деньги
    Scamming,        // Мошенничество
    NameViolation,   // Неподобающее имя
    Other,           // Другое
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "report_status")]
pub enum ReportStatus {
    Open,            // Открыт
    InReview,        // На рассмотрении
    Resolved,        // Решен
    Dismissed,       // Отклонен
    Duplicate,       // Дубликат
}

/// Результат рассмотрения жалобы
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReportResolution {
    pub action_taken: ResolutionAction,
    pub punishment_id: Option<Uuid>,
    pub moderator_notes: String,
    pub resolved_at: DateTime<Utc>,
    pub resolved_by: Uuid,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum ResolutionAction {
    NoAction,        // Никаких действий
    WarningIssued,   // Выдано предупреждение
    Punished,        // Наказан
    Escalated,       // Передано выше
    NeedsMoreInfo,   // Нужна дополнительная информация
}

/// Апелляции на наказания
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PunishmentAppeal {
    pub id: Uuid,
    pub punishment_id: Uuid,
    pub player_id: Uuid,
    pub reason: String,
    pub description: String,
    pub evidence_urls: Vec<String>,
    pub status: AppealStatus,
    pub assigned_admin: Option<Uuid>,
    pub admin_response: Option<String>,
    pub decision: Option<AppealDecision>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "appeal_status")]
pub enum AppealStatus {
    Pending,         // В ожидании
    InReview,        // На рассмотрении
    Approved,        // Одобрена
    Denied,          // Отклонена
    Closed,          // Закрыта
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum AppealDecision {
    Overturn,        // Отменить наказание
    Reduce,          // Снизить наказание
    Uphold,          // Подтвердить наказание
    RequiresReview,  // Требует дополнительного рассмотрения
}

/// Система доверия игроков
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerTrustScore {
    pub player_id: Uuid,
    pub trust_score: f32,        // 0.0 - 100.0
    pub reports_against: u32,    // Количество жалоб на игрока
    pub reports_by: u32,         // Количество жалоб от игрока
    pub verified_reports: u32,   // Подтвержденные жалобы от игрока
    pub false_reports: u32,      // Ложные жалобы от игрока
    pub commendations: u32,      // Похвалы от других игроков
    pub playtime_hours: f32,     // Время игры в часах
    pub last_violation: Option<DateTime<Utc>>,
    pub account_age_days: u32,
    pub updated_at: DateTime<Utc>,
}

/// Похвалы игроков
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerCommendation {
    pub id: Uuid,
    pub from_player_id: Uuid,
    pub to_player_id: Uuid,
    pub commendation_type: CommendationType,
    pub message: Option<String>,
    pub match_id: Option<Uuid>,
    pub created_at: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, sqlx::Type)]
#[serde(rename_all = "snake_case")]
#[sqlx(type_name = "commendation_type")]
pub enum CommendationType {
    Friendly,        // Дружелюбный
    Helpful,         // Полезный
    GoodLeader,      // Хороший лидер
    Skillful,        // Умелый
    Sportsmanship,   // Спортивное поведение
}

/// Список запрещенных слов и фильтрация
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatFilter {
    pub id: Uuid,
    pub pattern: String,         // Регулярное выражение или слово
    pub filter_type: FilterType,
    pub severity: FilterSeverity,
    pub action: FilterAction,
    pub created_by: Uuid,
    pub created_at: DateTime<Utc>,
    pub is_active: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FilterType {
    ExactMatch,      // Точное совпадение
    Contains,        // Содержит
    Regex,          // Регулярное выражение
    Whitelist,      // Белый список (разрешено)
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, PartialOrd, Ord)]
pub enum FilterSeverity {
    Low,            // Низкая
    Medium,         // Средняя
    High,           // Высокая
    Critical,       // Критическая
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum FilterAction {
    Block,          // Заблокировать сообщение
    Replace,        // Заменить на звездочки
    Warn,           // Предупредить игрока
    Mute,           // Замутить игрока
    Log,            // Только записать в лог
}

/// Запросы для создания отчетов
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateReportRequest {
    pub reported_player_id: Uuid,
    pub report_type: ReportType,
    
    #[validate(length(min = 10, max = 100, message = "Причина должна быть от 10 до 100 символов"))]
    pub reason: String,
    
    #[validate(length(min = 20, max = 1000, message = "Описание должно быть от 20 до 1000 символов"))]
    pub description: String,
    
    pub evidence_urls: Vec<String>,
    pub server_id: Option<String>,
    pub match_id: Option<Uuid>,
}

/// Запрос на создание апелляции
#[derive(Debug, Serialize, Deserialize, Validate)]
pub struct CreateAppealRequest {
    pub punishment_id: Uuid,
    
    #[validate(length(min = 10, max = 100, message = "Причина должна быть от 10 до 100 символов"))]
    pub reason: String,
    
    #[validate(length(min = 50, max = 2000, message = "Описание должно быть от 50 до 2000 символов"))]
    pub description: String,
    
    pub evidence_urls: Vec<String>,
}

impl PlayerPunishment {
    pub fn new(
        player_id: Uuid,
        punishment_type: PunishmentType,
        reason: String,
        moderator_id: Uuid,
        duration: Option<u64>,
    ) -> Self {
        let now = Utc::now();
        let end_time = duration.map(|d| now + chrono::Duration::seconds(d as i64));
        
        Self {
            id: Uuid::new_v4(),
            player_id,
            punishment_type,
            reason,
            moderator_id,
            duration,
            start_time: now,
            end_time,
            is_active: true,
            appeal_id: None,
            created_at: now,
            metadata: PunishmentMetadata {
                ip_address: None,
                hardware_id: None,
                server_id: None,
                evidence_urls: Vec::new(),
                witness_ids: Vec::new(),
                automatic: false,
                anti_cheat_detection: None,
            },
        }
    }
    
    pub fn is_expired(&self) -> bool {
        if let Some(end_time) = self.end_time {
            Utc::now() > end_time
        } else {
            false
        }
    }
    
    pub fn remaining_time(&self) -> Option<chrono::Duration> {
        if let Some(end_time) = self.end_time {
            let remaining = end_time - Utc::now();
            if remaining > chrono::Duration::zero() {
                Some(remaining)
            } else {
                None
            }
        } else {
            None
        }
    }
}

impl PlayerTrustScore {
    pub fn new(player_id: Uuid) -> Self {
        Self {
            player_id,
            trust_score: 50.0, // Начальный рейтинг доверия
            reports_against: 0,
            reports_by: 0,
            verified_reports: 0,
            false_reports: 0,
            commendations: 0,
            playtime_hours: 0.0,
            last_violation: None,
            account_age_days: 0,
            updated_at: Utc::now(),
        }
    }
    
    pub fn calculate_trust_score(&mut self) {
        let mut score = 50.0; // Базовый балл
        
        // Положительные факторы
        score += self.commendations as f32 * 2.0;
        score += (self.playtime_hours / 10.0).min(20.0); // Максимум 20 баллов за время игры
        score += (self.account_age_days as f32 / 30.0).min(10.0); // Максимум 10 баллов за возраст аккаунта
        
        if self.reports_by > 0 {
            let accuracy = self.verified_reports as f32 / self.reports_by as f32;
            score += accuracy * 10.0; // Бонус за точность жалоб
        }
        
        // Негативные факторы
        score -= self.reports_against as f32 * 5.0;
        score -= self.false_reports as f32 * 3.0;
        
        // Штраф за недавние нарушения
        if let Some(last_violation) = self.last_violation {
            let days_since = (Utc::now() - last_violation).num_days();
            if days_since < 30 {
                score -= (30 - days_since) as f32 * 0.5;
            }
        }
        
        // Ограничиваем в пределах 0-100
        self.trust_score = score.max(0.0).min(100.0);
        self.updated_at = Utc::now();
    }
    
    pub fn get_trust_level(&self) -> TrustLevel {
        match self.trust_score as u32 {
            0..=20 => TrustLevel::VeryLow,
            21..=40 => TrustLevel::Low,
            41..=60 => TrustLevel::Medium,
            61..=80 => TrustLevel::High,
            _ => TrustLevel::VeryHigh,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub enum TrustLevel {
    VeryLow,
    Low,
    Medium,
    High,
    VeryHigh,
} 