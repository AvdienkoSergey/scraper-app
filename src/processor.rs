use std::collections::HashSet;
use std::error::Error;
use std::fs::File;
use std::io::{BufRead, BufReader, Write};
use regex::Regex;
use reqwest;
use scraper::{Html, Selector};
use crate::bot::send_telegram_message;
use crate::config::Config;
use crate::error::AppError;

/// Struct to represent a news item
pub struct NewsItem {
    pub title: String,
    pub url: String,
    pub description: String,
}

/// Async implementation of feed processing
pub async fn process_feed_async(config: &Config) -> Result<usize, Box<dyn Error>> {
    // Load list of already posted news
    let posted = load_posted_news(&config.posted_file)?;
    // Fetch the news page
    let content = fetch_html(&config.news_feed_url).await?;
    // Parse the news page
    let news_items: Vec<NewsItem> = parse_news(&content, &config.news_item_selector, &config.title_regex_pattern)?;
    // Filter out already posted news
    let news_items_to_post: Vec<NewsItem> = news_items.into_iter().filter(|item| !posted.contains(&item.url)).collect();
    // Save newly posted news to file
    if !news_items_to_post.is_empty() {
        append_posted_news(&config.posted_file, &news_items_to_post)?;
        // Send message to Telegram
        send_telegram_message(&config, &news_items_to_post).await?;
    }
    Ok(news_items_to_post.len())
}

/// Load list of already processed news from the specified file.
/// Each line contains a unique news identifier (e.g., URL).
pub fn load_posted_news(filename: &str) -> Result<HashSet<String>, Box<dyn Error>> {
    let mut set = HashSet::new();
    if let Ok(file) = File::open(filename) {
        let reader = BufReader::new(file);
        for line in reader.lines() {
            let line = line?;
            set.insert(line);
        }
    }
    Ok(set)
}

/// Function to save news identifiers
pub fn append_posted_news(filename: &str, items: &[NewsItem]) -> Result<(), Box<dyn Error>> {
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(filename)?;
    
    for item in items {
        writeln!(file, "{}", item.url)?;
    }
    
    Ok(())
}

/// Fetch HTML content from a URL
pub async fn fetch_html(url: &str) -> Result<String, Box<dyn Error>> {
    let response = reqwest::get(url)
        .await
        .map_err(|e| AppError::FeedProcessingError(format!("Failed to fetch URL: {}", e)))?;

    let content = response
        .text()
        .await
        .map_err(|e| AppError::FeedProcessingError(format!("Failed to get response text: {}", e)))?;
    
    Ok(content)
}

/// Parse HTML content to extract news items
pub fn parse_news(html: &str, selector: &str, title_pattern: &str) -> Result<Vec<NewsItem>, Box<dyn Error>> {
    let document = Html::parse_document(html);

    let selector = Selector::parse(selector)
        .map_err(|_| AppError::FeedProcessingError("Invalid selector".to_string()))?;

    // Compile regex for title filtering
    let title_re = Regex::new(title_pattern)
        .map_err(|_| AppError::FeedProcessingError("Invalid regex pattern".to_string()))?;

    let mut news_items = Vec::new();

    // Process all elements matching the selector
    for element in document.select(&selector) {
        // Extract title from element text
        let title = element.text().collect::<Vec<_>>().join(" ").trim().to_string();
        
        // Extract URL from href attribute
        let url = match element.value().attr("href") {
            Some(url) => url.to_string(),
            None => continue, // skip if no link
        };

        // Filter title by regex pattern
        if !title_re.is_match(&title) {
            continue;
        }

        // Create a news item
        let news_item = NewsItem {
            title,
            url,
            description: "".to_string(), // No description in basic implementation
        };

        news_items.push(news_item);
    }

    Ok(news_items)
}