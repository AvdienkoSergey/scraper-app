use std::error::Error;
use teloxide::prelude::*;
use teloxide::types::UpdateKind;
use crate::config::Config;
use crate::error::AppError;
use crate::processor::NewsItem;

const DEFAULT_BOT_TOKEN: &str = "YOUR_TELEGRAM_BOT_TOKEN";
const UPDATE_TIMEOUT_SECONDS: u32 = 60;
const REQUEST_LIMIT: u8 = 1;
const POLLING_DELAY_SECONDS: u64 = 2;

pub struct TelegramBot {
    bot: Bot,
}

impl TelegramBot {
    pub fn new(token: &str) -> Self {
        TelegramBot {
            bot: Bot::new(token),
        }
    }

    pub fn from_config(config: &Config) -> Self {
        Self::new(&config.telegram_bot_token)
    }

    pub fn get_bot(&self) -> Bot {
        self.bot.clone()
    }

    pub async fn get_chat_id_once(&self) -> Result<(), AppError> {
        println!("Ожидание сообщения от пользователя...");
        println!("Пожалуйста, отправьте любое сообщение в чат с ботом в течение {} секунд...", UPDATE_TIMEOUT_SECONDS);
        let bot = self.get_bot();
        
        // Loop until we get a message
        loop {
            let updates = bot.get_updates()
                .timeout(UPDATE_TIMEOUT_SECONDS)
                .limit(REQUEST_LIMIT)
                .await?;
            
            if let Some(update) = updates.into_iter().next() {
                if let UpdateKind::Message(message) = update.kind {
                    let chat_id = message.chat.id;
                    println!("Получен chat_id: {}", chat_id);
                    println!("Используйте этот ID в вашей конфигурации.");
                    
                    let _ = bot.send_message(
                        message.chat.id,
                        format!("Ваш chat ID: {}. Вы можете использовать этот ID в вашей конфигурации.", chat_id)
                    ).await;
                    
                    return Ok(());
                }
            }
            
            // Short delay before trying again
            tokio::time::sleep(tokio::time::Duration::from_secs(POLLING_DELAY_SECONDS)).await;
        }
    }
}



pub async fn send_telegram_message(config: &Config, messages: &[NewsItem]) -> Result<(), AppError> {
    if config.telegram_bot_token == DEFAULT_BOT_TOKEN || config.telegram_bot_token.is_empty() {
        println!("Токен бота не установлен. Пожалуйста, установите его в конфигурации.");
        return Err(AppError::ConfigError("Токен бота не установлен".to_string()).into());
    }

    let telegram_bot = TelegramBot::from_config(&config);
    let bot = telegram_bot.get_bot();
    let chat_id = ChatId(config.telegram_chat_id);

    for item in messages {
        let message = render_template(&config.posted_template, &[
            ("title", &item.title),
            ("url", &item.url),
        ]);
        println!("Отправка сообщения: {}", message);
        bot.send_message(
            chat_id,
            message
        )
            .parse_mode(teloxide::types::ParseMode::Html)
            .disable_web_page_preview(false)
            .send()
            .await
            .map_err(|e| AppError::TelegramError(format!("Не удалось отправить сообщение: {}", e)))?;
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