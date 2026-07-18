use std::path::PathBuf;

use clap::{Parser, Subcommand};

#[derive(Debug, Parser)]
#[command(name = "blog-cli")]
#[command(about = "CLI-клиент для API блога")]
pub(crate) struct Cli {
    /// Использовать gRPC вместо HTTP.
    #[arg(long)]
    pub grpc: bool,

    /// Адрес сервера.
    ///
    /// Если не задан, для HTTP используется `http://127.0.0.1:8080`,
    /// а для gRPC — `http://127.0.0.1:50051`.
    #[arg(long)]
    pub server: Option<String>,

    /// Путь к файлу JWT-токена.
    #[arg(long, default_value = ".blog_token")]
    pub token_file: PathBuf,

    /// Выводить ответ в форматированном JSON.
    #[arg(long)]
    pub json: bool,

    /// Команда CLI.
    #[command(subcommand)]
    pub command: Command,
}

#[derive(Debug, Subcommand)]
pub(crate) enum Command {
    /// Регистрация и вход.
    Auth {
        /// Команда аутентификации.
        #[command(subcommand)]
        command: AuthCommand,
    },

    /// Операции с постами.
    Posts {
        /// Команда работы с постами.
        #[command(subcommand)]
        command: PostsCommand,
    },
}

#[derive(Debug, Subcommand)]
pub(crate) enum AuthCommand {
    /// Зарегистрировать нового пользователя.
    Register {
        /// Имя пользователя.
        #[arg(short, long)]
        username: String,

        /// Электронная почта.
        #[arg(short, long)]
        email: String,

        /// Пароль.
        #[arg(short, long)]
        password: String,
    },

    /// Войти и сохранить JWT-токен.
    Login {
        /// Имя пользователя.
        #[arg(short, long)]
        username: String,

        /// Пароль.
        #[arg(short, long)]
        password: String,
    },
}

#[derive(Debug, Subcommand)]
pub(crate) enum PostsCommand {
    /// Создать пост.
    Create {
        /// Заголовок.
        #[arg(short, long)]
        title: String,

        /// Содержимое.
        #[arg(short, long)]
        content: String,
    },

    /// Получить пост по ID.
    Get {
        /// ID поста.
        id: i64,
    },

    /// Обновить пост.
    Update {
        /// ID поста.
        id: i64,

        /// Новый заголовок.
        #[arg(short, long)]
        title: String,

        /// Новое содержимое.
        #[arg(short, long)]
        content: String,
    },

    /// Удалить пост.
    Delete {
        /// ID поста.
        id: i64,
    },

    /// Получить список постов.
    List {
        /// Размер страницы.
        #[arg(long, default_value_t = 10)]
        limit: u64,

        /// Смещение.
        #[arg(long, default_value_t = 0)]
        offset: u64,
    },
}
