use crate::bot::TelegramBot;
use crate::config::Config;
use std::error::Error;

pub async fn test_telegram_chat_id(config: &Config) -> Result<(), Box<dyn Error>> {
    let telegram_bot = TelegramBot::from_config(&config);
    telegram_bot.get_chat_id_once().await?;
    Ok(())
}

