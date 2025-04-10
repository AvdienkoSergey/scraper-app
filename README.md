# Scraper-App

Приложение для сбора новостей с веб-сайтов и отправки их в Telegram.

## Описание

Scraper-App - это утилита командной строки, написанная на Rust, которая:

- Собирает новости с указанного веб-сайта
- Фильтрует их по заданному регулярному выражению
- Отправляет отфильтрованные новости в Telegram-чат
- Отслеживает уже отправленные новости, чтобы избежать дублирования

## Установка

### Предварительные требования

- Rust и Cargo (установите с помощью [rustup](https://rustup.rs/))
- Токен Telegram-бота (получите у [@BotFather](https://t.me/BotFather))
- ID чата Telegram, куда будут отправляться сообщения

### Сборка из исходного кода

```bash
git clone https://github.com/yourusername/scraper-app.git
cd scraper-app
cargo build --release
```

Исполняемый файл будет доступен в директории `target/release/`.

## Настройка

При первом запуске приложение создаст файл конфигурации `scraper-app.json` с настройками по умолчанию. Вам необходимо отредактировать этот файл, указав:

- URL новостного сайта
- CSS-селектор для поиска новостных элементов
- Регулярное выражение для фильтрации новостей
- Токен Telegram-бота
- ID чата Telegram
- Интервал проверки новостей (в секундах)
- Шаблон сообщения для отправки в Telegram

### Пример файла конфигурации

```json
{
  "news_feed_url": "https://giookn.nso.ru/page/198?page=2",
  "news_item_selector": "td > a",
  "title_regex_pattern": "(?i).*Новосибирский\\s+Академгородок.*",
  "telegram_bot_token": "YOUR_TELEGRAM_BOT_TOKEN",
  "telegram_chat_id": 0,
  "posted_file": "posted_news.txt",
  "posted_interval": 3600,
  "posted_template": "<b>{{title}}</b>\n\n<a href=\"{{url}}\">Читать дальше</a>"
}
```

| Параметр              | Описание                                                  |
| --------------------- | --------------------------------------------------------- |
| `news_feed_url`       | URL страницы с новостями                                  |
| `news_item_selector`  | CSS-селектор для поиска новостных элементов               |
| `title_regex_pattern` | Регулярное выражение для фильтрации новостей по заголовку |
| `telegram_bot_token`  | Токен вашего Telegram-бота                                |
| `telegram_chat_id`    | ID чата, куда будут отправляться сообщения                |
| `posted_file`         | Файл для хранения уже отправленных новостей               |
| `posted_interval`     | Интервал проверки новостей в секундах                     |
| `posted_template`     | Шаблон сообщения для отправки в Telegram                  |

## Использование

### Запуск приложения в рабочем режиме

```bash
./scraper-app.exe [путь_к_файлу_конфигурации (опционально)]
```

Если путь к файлу конфигурации не указан, используется `scraper-app.json` в текущей директории.

При запуске приложение:

1. Загружает конфигурацию
2. Проверяет новости на указанном сайте (фиксирует результат в файл `posted_news.txt`)
3. Отправляет новые новости в Telegram
4. Ожидает указанный в конфигурации интервал времени
5. Повторяет процесс

Вы можете менять конфигурацию в любой момент между запусками рассылки, не перезапуская приложение. Перед каждой рассылкой приложение проверяет конфигурацию на корректность и подгружает её заново.

### Запуск приложения в тестовом режиме

```bash
./scraper-app.exe --test [тестируемый_модуль]
```

Доступные типы тестов:

- `config` - проверка загрузки конфигурации
- `fetch` - проверка получения HTML с сайта
- `parse` - проверка парсинга HTML и извлечения новостей
- `telegram` - проверка отправки сообщений в Telegram
- `all` - запуск всех тестов

Пример:

```bash
./scraper-app.exe --test config
```

## Структура проекта

- `src/main.rs` - основная точка входа в приложение
- `src/config.rs` - работа с конфигурацией
- `src/processor.rs` - обработка новостной ленты
- `src/bot.rs` - взаимодействие с Telegram API
- `src/error.rs` - обработка ошибок
- `src/utils.rs` - вспомогательные функции
- `src/test/` - модули для тестирования компонентов

## Логика работы

1. Приложение загружает конфигурацию из файла
2. Получает HTML-страницу с новостями
3. Парсит HTML, используя указанный CSS-селектор
4. Фильтрует новости по регулярному выражению
5. Проверяет, какие новости еще не были отправлены
6. Отправляет новые новости в Telegram
7. Сохраняет информацию об отправленных новостях
8. Ожидает указанный интервал времени и повторяет процесс

## Устранение неполадок

- email: avdienko.s@gmail.com
- telegram: @avdienkosa

### Не создается файл конфигурации

Убедитесь, что у приложения есть права на запись в текущую директорию.

### Не отправляются сообщения в Telegram

Убедитесь, что у вашего бота есть права на отправку сообщений в указанный чат.

### Ошибки при получении HTML

Проверьте доступность указанного URL и корректность сетевого подключения.

### Не находятся новости

Проверьте правильность CSS-селектора. Используйте тестовый режим `--test parse` для отладки.

### Ошибки при отправке в Telegram

Убедитесь, что:

- Токен бота указан правильно
- ID чата указан правильно
- Бот добавлен в указанный чат
- У бота есть права на отправку сообщений
- Вы не перестарались с темплейтом сообщения в конфигурации
