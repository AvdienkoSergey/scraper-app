mod config_diagnostic;
mod fetch_diagnostic;
mod parse_diagnostic;
mod telegram_diagnostic;
mod chat_diagnostic;
mod progress_diagnostic;
mod dialog_diagnostic;
use crate::config::{ load_config, load_config_with_retry};
use crate::utils;
use std::process;

pub use config_diagnostic::test_configuration;
pub use fetch_diagnostic::test_html_fetching;
pub use parse_diagnostic::test_html_parsing;
pub use telegram_diagnostic::test_telegram_messaging;
pub use chat_diagnostic::test_telegram_chat_id;
pub use progress_diagnostic::process_diagnostic;
pub use dialog_diagnostic::dialog_diagnostic;
pub async fn run_test_mode(args: &[String]) {
    println!("Запуск в режиме ДИАГНОСТИКА");

    // Check which test to run
    if args.len() < 3 {
        println!("Доступные тесты:");
        println!("  --test config    - Тест загрузки конфигурации");
        println!("  --test fetch     - Тест получения HTML");
        println!("  --test parse     - Тест парсинга HTML");
        println!("  --test telegram  - Тест отправки сообщений в Telegram");
        println!("  --test chat      - Тест получения chat_id от бота");
        println!("  --test all       - Запустить все тесты");
        return;
    }

    let test_type = &args[2];

    // Load config for tests
    let config_filename = args.get(3).map(|s| s.as_str()).unwrap_or("scraper-app.json");
    println!("Использование файла конфигурации: {}", config_filename);

    // 6 attempts, ~30 seconds
    let max_attempts_retry = 6;

    // Load configuration
    let config = match load_config(&config_filename) {
        Ok(value) => value,
        Err(err) => {
            utils::handle_config_error(&config_filename, err);
            match load_config_with_retry(&config_filename, max_attempts_retry) {
                Ok(config) => config,
                Err(retry_err) => {
                    println!("Не удалось загрузить конфигурацию после нескольких попыток: {}", retry_err);
                    process::exit(1);
                }
            }
        }
    };

    match test_type.as_str() {
        "config" => test_configuration(&config),
        "fetch" => test_html_fetching(&config).await,
        "parse" => test_html_parsing(&config).await,
        "telegram" => test_telegram_messaging(&config).await,
        "chat" => test_telegram_chat_id(&config).await.unwrap(),
        "progress" => process_diagnostic().await.unwrap(),
        "dialog" => dialog_diagnostic().unwrap(),
        "all" => {
            test_configuration(&config);
            test_html_fetching(&config).await;
            test_html_parsing(&config).await;
            test_telegram_messaging(&config).await;
        },
        _ => {
            println!("Неизвестный тип теста: {}", test_type);
            println!("Запустите --test без аргументов, чтобы увидеть доступные тесты.");
        }
    }
} 