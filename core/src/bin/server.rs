//! Точка входа для серверного приложения GameVerse Framework
//!
//! Этот файл содержит main функцию для запуска сервера GameVerse.

use std::env;
use std::path::Path;
use std::process;
use gameverse_core::{VERSION, NAME};
use gameverse_core::server::ServerRuntime;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Получаем путь к конфигурации из аргументов командной строки
    let config_path = env::args().nth(1);
    
    // Проверяем наличие флага --dev или -d
    let dev_mode = env::args().any(|arg| arg == "--dev" || arg == "-d");
    
    // Выводим информацию о версии
    println!("{} v{}", NAME, VERSION);
    println!("Запуск сервера...");
    
    // Создаем и запускаем серверный рантайм
    let config_path = config_path.as_deref().map(Path::new);
    let mut runtime = match ServerRuntime::new(config_path, dev_mode) {
        Ok(runtime) => runtime,
        Err(err) => {
            eprintln!("Ошибка инициализации сервера: {}", err);
            process::exit(1);
        }
    };
    
    // Запускаем сервер (блокирующий вызов до завершения)
    if let Err(err) = runtime.start().await {
        eprintln!("Ошибка в работе сервера: {}", err);
        process::exit(1);
    }
    
    println!("Сервер завершил работу");
    Ok(())
} 