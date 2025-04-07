use crate::config::Config;
use crate::processor::NewsItem;
use crate::bot::send_telegram_message;

/// Tests Telegram messaging
pub async fn test_telegram_messaging(config: &Config) {
    println!("\nТестирование отправки сообщений в Telegram...");

    if config.telegram_bot_token == "YOUR_TELEGRAM_BOT_TOKEN" || config.telegram_chat_id == 0 {
        println!("Учетные данные Telegram не настроены в файле конфигурации");
        return;
    }

    println!("Отправка тестового сообщения в чат с ID: {}", config.telegram_chat_id);

    let messages: Vec<NewsItem> = vec![
        NewsItem {
            title: "Тестовое сообщение".to_string(),
            url: "https://example.com".to_string(),
            description: "Это тестовое сообщение".to_string(),
        }
    ];

    // Use the actual send_telegram_message function
    match send_telegram_message(config, &messages).await {
        Ok(_) => {
            println!("Тестовое сообщение успешно отправлено в Telegram");
        },
        Err(e) => {
            println!("Не удалось отправить тестовое сообщение: {}", e);
        }
    }
} 