use std::io::{self, Write};
use crate::config::Config;

pub fn handle_config_error(filename: &str, err: Box<dyn std::error::Error>) {
    eprintln!("Ошибка при загрузке конфигурации из файла '{}': {}", filename, err);

    // Offer the user to create a default configuration file
    eprintln!("Хотите создать стандартный файл конфигурации? (y/n)");

    if prompt_yes_no() {
        let default_config = Config::default();

        match default_config.save_to_file(filename) {
            Ok(_) => println!("Стандартный файл конфигурации создан: '{}'", filename),
            Err(write_err) => {
                eprintln!("Не удалось записать файл конфигурации: {}", write_err);
            }
        }
    } else {
        eprintln!("Программа завершена. Пожалуйста, создайте файл конфигурации вручную.");
    }
}

fn prompt_yes_no() -> bool {
    let mut input = String::new();
    print!("> ");
    io::stdout().flush().unwrap_or(());

    match io::stdin().read_line(&mut input) {
        Ok(_) => input.trim().to_lowercase() == "y",
        Err(_) => false,
    }
}