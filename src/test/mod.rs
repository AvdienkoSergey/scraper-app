mod config_test;
mod fetch_test;
mod parse_test;
mod telegram_test;

use crate::config::{ load_config, load_config_with_retry};
use crate::utils;
use std::process;

pub use config_test::test_configuration;
pub use fetch_test::test_html_fetching;
pub use parse_test::test_html_parsing;
pub use telegram_test::test_telegram_messaging;

pub async fn run_test_mode(args: &[String]) {
    println!("Запуск в ТЕСТОВОМ РЕЖИМЕ");

    // Check which test to run
    if args.len() < 3 {
        println!("Доступные тесты:");
        println!("  --test config    - Тест загрузки конфигурации");
        println!("  --test fetch     - Тест получения HTML");
        println!("  --test parse     - Тест парсинга HTML");
        println!("  --test telegram  - Тест отправки сообщений в Telegram");
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