//! Точка входа для серверного приложения GameVerse Framework
//!
//! Этот файл содержит main функцию для запуска сервера GameVerse.

use std::env;
use std::process;
use gameverse_core::{initialize, VERSION, NAME};

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Получаем путь к конфигурации из аргументов командной строки
    let config_path = env::args().nth(1);
    let config_path_ref = config_path.as_deref();

    // Выводим информацию о версии
    println!("{} v{}", NAME, VERSION);
    println!("Запуск сервера...");

    // Инициализируем ядро
    let core_context = match initialize(config_path_ref).await {
        Ok(context) => context,
        Err(err) => {
            eprintln!("Ошибка инициализации ядра: {}", err);
            process::exit(1);
        }
    };

    // Выводим информацию о конфигурации
    println!(
        "Сервер '{}' запущен на {}:{}",
        core_context.config.server.name,
        core_context.config.server.bind_address,
        core_context.config.server.port
    );

    // Здесь будет основной цикл сервера
    // В будущем он будет заменен на правильную обработку сигналов и т.д.
    println!("Нажмите Ctrl+C для завершения работы");

    // Создаем канал для сигнала завершения
    let (tx, rx) = tokio::sync::oneshot::channel::<()>();

    // Обработка Ctrl+C
    let ctrl_c_fut = async {
        tokio::signal::ctrl_c().await.unwrap();
        println!("Получен сигнал завершения работы");
        let _ = tx.send(());
    };

    // Ждем сигнала завершения
    tokio::select! {
        _ = ctrl_c_fut => {},
        _ = rx => {},
    }

    println!("Завершение работы сервера");

    // Здесь будет код для корректного завершения работы

    Ok(())
} 