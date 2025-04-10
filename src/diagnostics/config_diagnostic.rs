use crate::config::{Config, config_validate};
use regex;

/// Tests configuration loading and validation
pub fn test_configuration(config: &Config) {
    println!("\n📋 Тестирование конфигурации...");

    println!("Конфигурация успешно загружена:");
    println!("  URL новостей: {}", config.news_feed_url);
    println!("  Селектор: {}", config.news_item_selector);
    println!("  Регулярное выражение: {}", config.title_regex_pattern);
    println!("  Токен Telegram бота: {}", if config.telegram_bot_token.len() > 10 {
        format!("{}...{}", &config.telegram_bot_token[..5], &config.telegram_bot_token[config.telegram_bot_token.len()-5..])
    } else {
        "Не установлен".to_string()
    });
    println!("  ID чата Telegram: {}", config.telegram_chat_id);

    // Test regex compilation
    match regex::Regex::new(&config.title_regex_pattern) {
        Ok(_) => println!("Регулярное выражение корректно"),
        Err(e) => println!("Регулярное выражение некорректно: {}", e)
    }

    // Validate the entire configuration
    if config_validate(config) {
        println!("Конфигурация корректна");
    } else {
        println!("Конфигурация некорректна");
    }
} 