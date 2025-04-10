use console::style;
use dialoguer::{theme::ColorfulTheme, Select, Input, Confirm, MultiSelect};

pub struct DialogBuilder {
    theme: ColorfulTheme,
}

impl Default for DialogBuilder {
    fn default() -> Self {
        Self {
            theme: ColorfulTheme::default(),
        }
    }
}

impl DialogBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    /// Показывает заголовок диалога
    pub fn title(&self, title: &str) {
        println!("\n{}\n", style(title).bold().cyan());
    }

    /// Выбор одного варианта из списка
    pub fn select(&self, prompt: &str, options: &[&str]) -> Result<usize, std::io::Error> {
        Select::with_theme(&self.theme)
            .with_prompt(prompt)
            .items(options)
            .default(0)
            .interact()
    }

    /// Выбор нескольких вариантов из списка
    pub fn multi_select(&self, prompt: &str, options: &[&str]) -> Result<Vec<usize>, std::io::Error> {
        MultiSelect::with_theme(&self.theme)
            .with_prompt(prompt)
            .items(options)
            .interact()
    }

    /// Ввод текста
    pub fn input(&self, prompt: &str, default: Option<&str>) -> Result<String, std::io::Error> {
        let mut input = Input::with_theme(&self.theme);
        let input = input.with_prompt(prompt);
        
        let input = if let Some(default_value) = default {
            input.default(default_value.to_string())
        } else {
            input
        };
        
        input.interact()
    }

    /// Подтверждение (да/нет)
    pub fn confirm(&self, prompt: &str, default: bool) -> Result<bool, std::io::Error> {
        Confirm::with_theme(&self.theme)
            .with_prompt(prompt)
            .default(default)
            .interact()
    }

    /// Показывает сообщение об успехе
    pub fn success(&self, message: &str) {
        println!("\n{} {}\n", style("✓").bold().green(), style(message).green());
    }

    /// Показывает сообщение об ошибке
    pub fn error(&self, message: &str) {
        println!("\n{} {}\n", style("✗").bold().red(), style(message).red());
    }

    /// Показывает информационное сообщение
    pub fn info(&self, message: &str) {
        println!("\n{} {}\n", style("ℹ").bold().blue(), style(message).blue());
    }
} 