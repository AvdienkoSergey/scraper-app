use thiserror::Error;

#[derive(Debug, Error)]
pub enum AppError {
    #[error("Telegram error: {0}")]
    TelegramError(String),
    
    #[error("Configuration error: {0}")]
    ConfigError(String),
    
    #[error("Network error: {0}")]
    NetworkError(#[from] reqwest::Error),
    
    #[error("Telegram API error: {0}")]
    TelegramApiError(#[from] teloxide::RequestError),

    #[error("Config read error: {0}")]
    ConfigReadError(String, String),

    #[error("Config parse error: {0}")]
    ConfigParseError(String, String),

    #[error("Feed processing error: {0}")]
    FeedProcessingError(String)
}