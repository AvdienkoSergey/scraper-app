use crate::error::AppError;
use serde::{Deserialize, Serialize};
use std::error::Error;
use std::fs;

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Config {
    pub(crate) news_feed_url: String,
    pub(crate) news_item_selector: String,
    pub(crate) title_regex_pattern: String,
    pub(crate) telegram_bot_token: String,
    pub(crate) telegram_chat_id: i64,
    pub(crate) posted_file: String,
    pub(crate) posted_interval: u64,
    pub(crate) posted_template: String,
}

impl Config {
    pub fn default() -> Self {
        Config {
            news_feed_url: "https://giookn.nso.ru/page/198?page=2".to_string(),
            news_item_selector: "td > a".to_string(),
            title_regex_pattern: "(?i).*Новосибирский\\s+Академгородок.*".to_string(),
            telegram_bot_token: "YOUR_TELEGRAM_BOT_TOKEN".to_string(),
            telegram_chat_id: 0,
            posted_file: "posted_news.txt".to_string(),
            posted_interval: 3600,
            posted_template: "<b>{{title}}</b><br/><a href=\"{{url}}\">Читать дальше</a>".to_string()
        }
    }

    pub fn save_to_file(&self, filename: &str) -> Result<(), Box<dyn Error>> {
        let json = serde_json::to_string_pretty(self)?;
        fs::write(filename, json)?;
        Ok(())
    }
}

pub fn config_validate(config: &Config) -> bool {
    // Check if news feed URL is valid
    if config.news_feed_url.is_empty() || !config.news_feed_url.starts_with("http") {
        println!("Ошибка: некорректный URL новостной ленты: '{}'", config.news_feed_url);
        return false;
    }

    // Check if news item selector is set
    if config.news_item_selector.is_empty() {
        println!("Ошибка: не указан CSS-селектор для новостных элементов");
        return false;
    }

    // Check if regex pattern is valid
    match regex::Regex::new(&config.title_regex_pattern) {
        Ok(_) => {},
        Err(e) => {
            println!("Ошибка: некорректное регулярное выражение: {}", e);
            return false;
        }
    }

    // Check Telegram credentials if they're required
    if config.telegram_bot_token == "YOUR_TELEGRAM_BOT_TOKEN" ||
        config.telegram_chat_id == 0 {
        println!("Предупреждение: не настроены данные для Telegram (токен бота и ID чата)");
        // This might not be a critical error if Telegram posting is optional
        // return false;
    }

    // Check if posted_file is set
    if config.posted_file.is_empty() {
        println!("Ошибка: не указан файл для хранения опубликованных новостей");
        return false;
    }

    // Check if posted_interval is reasonable (at least 60 seconds)
    if config.posted_interval < 60 {
        println!("Предупреждение: интервал проверки новостей слишком мал ({}с), рекомендуется не менее 60с",
                 config.posted_interval);
        // This might be just a warning, not a critical error
        // return false;
    }

    // All checks passed
    true
}

pub fn load_config(filename: &str) -> Result<Config, Box<dyn Error>> {
    let config_str = fs::read_to_string(filename)
        .map_err(|e| AppError::ConfigReadError(filename.to_string(), e.to_string()))?;

    let config: Config = serde_json::from_str(&config_str)
        .map_err(|e| AppError::ConfigParseError(filename.to_string(), e.to_string()))?;

    Ok(config)
}

pub fn load_config_with_retry(filename: &str, max_attempts: u32) -> Result<Config, Box<dyn Error>> {
    let mut attempt = 0;

    loop {
        match load_config(filename) {
            Ok(config) => return Ok(config),
            Err(err) => {
                attempt += 1;

                // Check if we've reached the maximum number of attempts
                if max_attempts > 0 && attempt >= max_attempts {
                    println!("Превышено максимальное время ожидания ({} попыток).", max_attempts);
                    return Err(err);
                }

                println!("Ошибка загрузки файла конфигурации: {} (попытка {}/{})",
                         err, attempt, if max_attempts > 0 { max_attempts } else { attempt });
                println!("Ожидание создания файла '{}'. Нажмите Ctrl+C для выхода...", filename);

                // Wait for 5 seconds before retrying
                std::thread::sleep(std::time::Duration::from_secs(5));

                // Check if file exists now
                if std::path::Path::new(filename).exists() {
                    println!("Файл конфигурации обнаружен, пробуем загрузить снова...");
                } else {
                    println!("Файл конфигурации всё ещё отсутствует, ожидание...");
                }
            }
        }
    }
}