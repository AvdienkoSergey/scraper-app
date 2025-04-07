use crate::config::Config;
use crate::processor::fetch_html;

/// Tests HTML fetching from the news source
pub async fn test_html_fetching(config: &Config) {
    println!("\nТестирование получения HTML...");

    println!("Получение HTML с: {}", config.news_feed_url);

    // Use the actual fetch_html function from your processor module
    match fetch_html(&config.news_feed_url).await {
        Ok(html) => {
            let len = html.len();
            let preview = if len > 100 { &html[..100] } else { &html };
            println!("Успешно получен HTML ({} байт)", len);
            println!("Предпросмотр: {}", preview);
        },
        Err(e) => {
            println!("Не удалось получить HTML: {}", e);
        }
    }
} 