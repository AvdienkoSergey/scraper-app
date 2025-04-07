use std::error::Error;
use std::fmt;

#[derive(Debug)]
pub enum AppError {
    ConfigReadError(String, String),
    ConfigParseError(String, String),
    FeedProcessingError(String),
    TelegramError(String),
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            AppError::ConfigReadError(filename, err) =>
                write!(f, "Failed to read config file '{}': {}", filename, err),
            AppError::ConfigParseError(filename, err) =>
                write!(f, "Error parsing config file '{}': {}", filename, err),
            AppError::FeedProcessingError(msg) =>
                write!(f, "Feed processing error: {}", msg),
            AppError::TelegramError(msg) =>
                write!(f, "Telegram API error: {}", msg),
        }
    }
}

impl Error for AppError {}