use crate::config::Config;
use crate::processor::{fetch_html, parse_news};

/// Tests HTML parsing with the configured selectors
pub async fn test_html_parsing(config: &Config) {
    println!("\nТестирование парсинга HTML...");

    // First fetch the HTML
    match fetch_html(&config.news_feed_url).await {
        Ok(html) => {
            println!("HTML успешно получен, выполняется парсинг с селектором: {}", config.news_item_selector);

            // Use the actual parse_news function from your processor module
            match parse_news(&html, &config.news_item_selector, &config.title_regex_pattern) {
                Ok(news_items) => {
                    println!("Успешно обработано {} новостных элементов", news_items.len());

                    // Display the first few items
                    for (i, item) in news_items.iter().take(3).enumerate() {
                        println!("\nЭлемент {}:", i+1);
                        println!("  Заголовок: {}", item.title);
                        println!("  URL: {}", item.url);
                        println!("  Описание: {}", if item.description.len() > 50 {
                            format!("{}...", &item.description[..50])
                        } else {
                            item.description.clone()
                        });
                    }

                    if news_items.len() > 3 {
                        println!("\n... и ещё {} элементов", news_items.len() - 3);
                    }
                },
                Err(e) => {
                    println!("Не удалось выполнить парсинг HTML: {}", e);
                }
            }
        },
        Err(e) => {
            println!("Не удалось получить HTML для парсинга: {}", e);
        }
    }
} 