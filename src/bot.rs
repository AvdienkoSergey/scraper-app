use std::error::Error;
use teloxide::prelude::*;
use crate::config::Config;
use crate::error::AppError;
use crate::processor::NewsItem;

/// Structure for working with Telegram bot
pub struct TelegramBot {
    bot: Bot,
}

impl TelegramBot {
    /// Creates a new bot instance with the specified token
    pub fn new(token: &str) -> Self {
        TelegramBot {
            bot: Bot::new(token),
        }
    }

    /// Creates a bot instance from configuration
    pub fn from_config(config: &Config) -> Self {
        Self::new(&config.telegram_bot_token)
    }

    /// Gets the bot instance
    pub fn get_bot(&self) -> Bot {
        self.bot.clone()
    }
}

/// Sends a message to the specified chat with HTML markup
pub async fn send_telegram_message(config: &Config, messages: &Vec<NewsItem>) -> Result<(), Box<dyn Error>> {
    // Check if token is set
    if config.telegram_bot_token == "YOUR_TELEGRAM_BOT_TOKEN" || config.telegram_bot_token.is_empty() {
        return Err(AppError::TelegramError("Telegram bot token is not set".to_string()).into());
    }

    // Initialize Telegram bot
    let telegram_bot = TelegramBot::from_config(&config);
    let bot = telegram_bot.get_bot();
    let chat_id = ChatId(config.telegram_chat_id);

    for item in messages {
        let message = render_template(&config.posted_template, &[
            ("title", &item.title),
            ("url", &item.url),
        ]);
        println!("Отправка сообщения: {}", message);
        // Send message to the chat
        bot.send_message(
            chat_id,
            message
        )
            .parse_mode(teloxide::types::ParseMode::Html)
            .disable_web_page_preview(false)
            .send()
            .await
            .map_err(|e| AppError::TelegramError(format!("Failed to send message: {}", e)))?;
    }

    Ok(())
}

fn render_template(template: &str, values: &[(&str, &str)]) -> String {
    let mut result = template.to_string();
    for (key, value) in values {
        let placeholder = format!("{{{{{}}}}}", key);
        result = result.replace(&placeholder, value);
    }
    result
}