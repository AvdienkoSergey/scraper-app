mod config;
mod utils;
mod error;
mod processor;
mod bot;
mod diagnostics;
mod library;

use std::env;
use std::thread;
use std::time::Duration;
use crate::config::{load_config, load_config_with_retry, config_validate};
use crate::processor::process_feed_async;
use crate::diagnostics::run_test_mode;

/// Main function.
/// On first run, all news matching the regex pattern are reposted.
/// Then it checks for new entries every hour in a loop.
#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Запуск анализа конфигурации...");

    let args: Vec<String> = env::args().collect();

    if args.len() > 1 && args[1] == "--test" {
        run_test_mode(&args).await;
        return Ok(());
    }

    let default_config_name = "scraper-app.json".to_string();
    let config_filename = env::args().nth(1).unwrap_or_else(|| default_config_name);
    // 6 attempts, ~30 seconds
    let max_attempts_retry = 6;

    // Load configuration
    let mut config = match load_config(&config_filename) {
        Ok(value) => value,
        Err(err) => {
            utils::handle_config_error(&config_filename, err);
            match load_config_with_retry(&config_filename, max_attempts_retry) {
                Ok(config) => config,
                Err(retry_err) => {
                    println!("Не удалось загрузить конфигурацию после нескольких попыток: {}", retry_err);
                    std::process::exit(1);
                }
            }
        }
    };

    // Validate configuration
    if !config_validate(&config) {
        println!("Ошибка: некорректная конфигурация. Исправьте ошибки и перезапустите программу.");
        std::process::exit(1);
    }

    // First run: process the feed immediately
    match process_feed_async(&config).await {
        Ok(count) => println!("Изначально опубликовано новостей: {}", count),
        Err(e) => eprintln!("Ошибка обработки ленты: {}", e),
    }

    // After first run - check the feed every hour. Reload config
    loop {
        println!("Ожидание следующей проверки ({} секунд)...", config.posted_interval);

        thread::sleep(Duration::from_secs(config.posted_interval));

        match load_config(&config_filename) {
            Ok(new_config) => config = new_config,
            Err(e) => {
                eprintln!("Ошибка чтения конфига, использую предыдущую версию: {}", e);
            }
        }

        match process_feed_async(&config).await {
            Ok(count) => println!("Новых новостей за последние {} секунд: {}", config.posted_interval, count),
            Err(e) => eprintln!("Ошибка при периодической проверке ленты: {}", e),
        }
    }
}