//! # Утилиты GameVerse
//!
//! Общие утилиты и вспомогательные функции

use std::time::{SystemTime, UNIX_EPOCH};

/// Получить текущий timestamp в миллисекундах
pub fn current_timestamp_ms() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_millis() as u64
}

/// Получить текущий timestamp в секундах
pub fn current_timestamp_s() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}

/// Форматировать размер в байтах в человеко-читаемый формат
pub fn format_bytes(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB", "TB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;
    
    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }
    
    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.2} {}", size, UNITS[unit_index])
    }
}

/// Генерировать случайную строку заданной длины
pub fn generate_random_string(length: usize) -> String {
    use rand::Rng;
    const CHARSET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZ\
                            abcdefghijklmnopqrstuvwxyz\
                            0123456789";
    let mut rng = rand::thread_rng();
    
    (0..length)
        .map(|_| {
            let idx = rng.gen_range(0..CHARSET.len());
            CHARSET[idx] as char
        })
        .collect()
}

/// Валидация email адреса (простая)
pub fn is_valid_email(email: &str) -> bool {
    email.contains('@') && email.contains('.') && email.len() > 5
}

/// Валидация IP адреса (IPv4)
pub fn is_valid_ipv4(ip: &str) -> bool {
    ip.split('.')
        .filter_map(|part| part.parse::<u8>().ok())
        .count() == 4
}

/// Хеширование строки (SHA-256)
pub fn hash_string(input: &str) -> String {
    use sha2::{Sha256, Digest};
    let mut hasher = Sha256::new();
    hasher.update(input.as_bytes());
    format!("{:x}", hasher.finalize())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_format_bytes() {
        assert_eq!(format_bytes(1024), "1.00 KB");
        assert_eq!(format_bytes(1048576), "1.00 MB");
        assert_eq!(format_bytes(500), "500 B");
    }

    #[test]
    fn test_email_validation() {
        assert!(is_valid_email("test@example.com"));
        assert!(!is_valid_email("invalid-email"));
        assert!(!is_valid_email("@.com"));
    }

    #[test]
    fn test_ipv4_validation() {
        assert!(is_valid_ipv4("192.168.1.1"));
        assert!(is_valid_ipv4("127.0.0.1"));
        assert!(!is_valid_ipv4("256.1.1.1"));
        assert!(!is_valid_ipv4("192.168.1"));
    }

    #[test]
    fn test_random_string_generation() {
        let random_str = generate_random_string(10);
        assert_eq!(random_str.len(), 10);
        
        let another_str = generate_random_string(10);
        assert_ne!(random_str, another_str); // Вероятность совпадения крайне мала
    }
} 