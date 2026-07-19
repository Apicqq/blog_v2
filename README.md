# Blog Project

Учебный backend/fullstack-проект блога на Rust. В репозитории собран Cargo workspace с HTTP API, gRPC API, клиентской библиотекой, CLI-клиентом, WASM-фронтендом и отдельным крейтом миграций.

## Что внутри

Workspace состоит из шести крейтов:

| Крейт | Назначение |
| --- | --- |
| `blog-server` | Backend блога: Actix HTTP API, Tonic gRPC API, JWT, Argon2, CORS, tracing, подключение к PostgreSQL через SeaORM. |
| `blog-client` | Библиотека клиента с единым фасадом `BlogClient` поверх HTTP (`reqwest`) и gRPC (`tonic`). Используется CLI-клиентом. |
| `blog-cli` | CLI-инструмент для проверки backend-сценариев: регистрация, вход, CRUD постов, HTTP/gRPC transport. |
| `blog-wasm` | Dioxus WASM-фронтенд, который работает в браузере и обращается к HTTP API напрямую. |
| `blog-proto` | Единый источник protobuf-схемы и сгенерированного Rust-кода для gRPC. |
| `blog-migration` | SeaORM migration crate для создания таблиц `users` и `posts`. |

## Архитектура backend

`blog-server` разделен по слоям:

- `domain/` — доменные модели, value objects и ошибки: `User`, `Post`, `RegistrationData`, `LoginCredentials`, `PostAttributes`, `UpdatePost`, `DomainError`.
- `application/` — прикладные сценарии: `AuthService`, `BlogService`, а также repository/token/password ports.
- `infrastructure/` — конкретные реализации: SeaORM repositories/entities, подключение к БД, JWT, Argon2, CORS, security headers, logging, запуск серверов.
- `presentation/` — HTTP handlers, gRPC service, DTO, middleware и error mapping.

HTTP и gRPC handlers не содержат бизнес-логику напрямую. Они преобразуют транспортные DTO в доменные типы и вызывают общие `AuthService` / `BlogService`.

## Осознанные отличия от ТЗ

В проекте есть несколько решений, которые отличаются от буквального учебного шаблона:

- **SeaORM вместо sqlx.** В ТЗ упоминается `sqlx`, но выбор был сделан в пользу ORM-подхода.
- **Миграции отдельным крейтом `blog-migration`.** Они не лежат в `blog-server/migrations/` и не применяются автоматически при старте сервера. Это сделано намеренно: миграции лучше запускать отдельной командой, а не скрыто на запуске бэкенда.
- **`blog-proto` как отдельный крейт.** Protobuf-схема хранится в одном месте: `blog-proto/proto/blog.v1.proto`. Сервер и клиент используют сгенерированный код из этого крейта без копирования `.proto`.
- **JWT без `username` в claims.** Токен содержит стабильный `sub` (`user_id`), `iat` и `exp`. `username` не кладется в JWT, потому что это изменяемое поле. Актуальный пользователь восстанавливается через `/api/me`.
- `Dioxus` вместо "голого" `wasm-bindgen`.
- **CLI-команды сгруппированы.** Вместо плоских `blog-cli register` / `blog-cli create` используется более явная структура `blog-cli auth register` и `blog-cli posts create`.

## Требования к окружению

Для запуска требуется иметь в окружении:

- Rust stable с edition 2024;
- PostgreSQL;
- `protoc` для генерации gRPC-кода;
- Dioxus CLI для WASM-фронтенда;
- Опционально: `grpcurl` для ручной проверки gRPC API.

Примеры установки инструментов:

```bash
rustup target add wasm32-unknown-unknown
curl -sSL https://dioxus.dev/install.sh | bash
```

Если не хочется использовать install script, Dioxus CLI можно поставить через Cargo:

```bash
cargo install dioxus-cli
```

Ubuntu/Debian:

```bash
sudo apt-get install protobuf-compiler postgresql
```

macOS через Homebrew:

```bash
brew install protobuf postgresql
```

## Настройка PostgreSQL

Создайте базу данных любым удобным способом. Например:

```bash
createdb blog_db
```

Пример строки подключения:

```bash
postgres://postgres:postgres@127.0.0.1:5432/blog_db
```

## Переменные окружения

Скопируйте пример окружения и подставьте свои значения. `.env` не коммитится.

```bash
cp .env.example .env
```

Минимальный набор переменных:

```env
DATABASE_URL=postgres://postgres:postgres@127.0.0.1:5432/blog_db
JWT_SECRET=replace-with-at-least-32-random-characters
HOST=127.0.0.1
PORT=8080
GRPC_PORT=50051
JWT_TTL_SECONDS=3600
CORS_ORIGINS=*
BLOG_API_BASE_URL=http://127.0.0.1:8080/api
```

## Миграции

Миграции запускаются отдельным крейтом:

```bash
cargo run -p blog-migration -- up
```

Проверить статус:

```bash
cargo run -p blog-migration -- status
```

Откатить последнюю миграцию:

```bash
cargo run -p blog-migration -- down
```

Полностью пересоздать схему в учебной/тестовой БД:

```bash
cargo run -p blog-migration -- fresh
```

Миграция создает:

- `users`: `id`, `username`, `email`, `password_hash`, `created_at`.
- `posts`: `id`, `title`, `content`, `author_id`, `created_at`, `updated_at`.
- внешний ключ `posts.author_id -> users.id` с `ON DELETE CASCADE`.
- индексы на `posts.created_at` и `posts.author_id`.
- unique constraints для `users.username` и `users.email`.

Наполнить БД тестовыми пользователями и постами:

```bash
psql "$DATABASE_URL" -f scripts/seed_dev_data.sql
```

Seed-скрипт можно запускать повторно. Он добавляет пользователей `alice` / `bob` с паролем `secret123` и несколько постов для ручной проверки UI, HTTP API и CLI.

## Сборка и проверки

Полная проверка workspace:

```bash
cargo test --workspace
cargo clippy --workspace -- -D warnings
```

Сборка всех крейтов:

```bash
cargo build --workspace
```

Сборка бэкенда:

```bash
cargo build --bin blog-server
```

Сборка клиента:

```bash
cargo build --lib -p blog-client
```

Сборка CLI:

```bash
cargo build --bin blog-cli
```

Проверка Dioxus/WASM-крейта:

```bash
cargo check -p blog-wasm --target wasm32-unknown-unknown
```

## Запуск Backend

Перед запуском убедитесь, что PostgreSQL работает, `.env` настроен, а миграции применены.

```bash
cargo run --bin blog-server
```

По умолчанию поднимаются два сервера:

- HTTP API: `http://127.0.0.1:8080/api`
- gRPC API: `http://127.0.0.1:50051`

Оба сервера запускаются одновременно и завершаются через graceful shutdown по `Ctrl+C`.

## HTTP API

Публичные endpoints:

- `POST /api/auth/register`
- `POST /api/auth/login`
- `GET /api/posts`
- `GET /api/posts/{id}`

Защищенные endpoints:

- `GET /api/me`
- `POST /api/posts`
- `PUT /api/posts/{id}`
- `DELETE /api/posts/{id}`

Для защищенных endpoints нужен заголовок:

```http
Authorization: Bearer <token>
```

### Smoke Test Через curl

Регистрация:

```bash
curl -i -X POST http://127.0.0.1:8080/api/auth/register \
  -H 'Content-Type: application/json' \
  -d '{
    "username": "alice",
    "email": "alice@example.com",
    "password": "secret123"
  }'
```

Вход:

```bash
curl -i -X POST http://127.0.0.1:8080/api/auth/login \
  -H 'Content-Type: application/json' \
  -d '{
    "username": "alice",
    "password": "secret123"
  }'
```

Сохранить токен в shell-переменную, если установлен `jq`:

```bash
TOKEN=$(curl -s -X POST http://127.0.0.1:8080/api/auth/login \
  -H 'Content-Type: application/json' \
  -d '{"username":"alice","password":"secret123"}' | jq -r .token)
```

Проверить текущего пользователя:

```bash
curl -i http://127.0.0.1:8080/api/me \
  -H "Authorization: Bearer $TOKEN"
```

Создать пост:

```bash
curl -i -X POST http://127.0.0.1:8080/api/posts \
  -H "Authorization: Bearer $TOKEN" \
  -H 'Content-Type: application/json' \
  -d '{
    "title": "Первый пост",
    "content": "Текст первого поста"
  }'
```

Список постов с пагинацией:

```bash
curl -i 'http://127.0.0.1:8080/api/posts?limit=10&offset=0'
```

Получить пост:

```bash
curl -i http://127.0.0.1:8080/api/posts/1
```

Обновить пост:

```bash
curl -i -X PUT http://127.0.0.1:8080/api/posts/1 \
  -H "Authorization: Bearer $TOKEN" \
  -H 'Content-Type: application/json' \
  -d '{
    "title": "Обновленный пост",
    "content": "Новый текст"
  }'
```

Удалить пост:

```bash
curl -i -X DELETE http://127.0.0.1:8080/api/posts/1 \
  -H "Authorization: Bearer $TOKEN"
```

## gRPC API

Схема находится в `blog-proto/proto/blog.v1.proto`.

Методы сервиса `blog.v1.BlogService`:

- `Register`
- `Login`
- `CreatePost`
- `GetPost`
- `UpdatePost`
- `DeletePost`
- `ListPosts`

`Register`, `Login`, `GetPost`, `ListPosts` публичные. `CreatePost`, `UpdatePost`, `DeletePost` требуют metadata:

```text
authorization: Bearer <token>
```

Если установлен `grpcurl`, проверить reflection можно так:

```bash
grpcurl -plaintext 127.0.0.1:50051 list
```

Регистрация через gRPC:

```bash
grpcurl -plaintext \
  -d '{"username":"bob","email":"bob@example.com","password":"secret123"}' \
  127.0.0.1:50051 blog.v1.BlogService/Register
```

Вход через gRPC:

```bash
grpcurl -plaintext \
  -d '{"username":"bob","password":"secret123"}' \
  127.0.0.1:50051 blog.v1.BlogService/Login
```

Создание поста через gRPC:

```bash
grpcurl -plaintext \
  -H "authorization: Bearer $TOKEN" \
  -d '{"title":"gRPC пост","content":"Создано через grpcurl"}' \
  127.0.0.1:50051 blog.v1.BlogService/CreatePost
```

Список постов:

```bash
grpcurl -plaintext \
  -d '{"limit":10,"offset":0}' \
  127.0.0.1:50051 blog.v1.BlogService/ListPosts
```

## Клиентская библиотека

`blog-client` предоставляет единый API независимо от транспорта:

```rust
use blog_client::{BlogClient, Transport};

let mut client = BlogClient::new(Transport::Http("http://127.0.0.1:8080".to_string())).await?;
let _auth = client.login("alice", "secret123").await?;
let post = client.create_post("Заголовок", "Текст").await?;
```

Для gRPC:

```rust
let mut client = BlogClient::new(Transport::Grpc("http://127.0.0.1:50051".to_string())).await?;
```

`BlogClient` хранит JWT-токен внутри структуры после `register`/`login` и автоматически использует его в защищенных операциях.

## CLI

CLI использует `blog-client`, не дублируя HTTP/gRPC-код.

По умолчанию используется HTTP:

```bash
cargo run --bin blog-cli -- auth register \
  --username alice \
  --email alice@example.com \
  --password secret123
```

Вход:

```bash
cargo run --bin blog-cli -- auth login \
  --username alice \
  --password secret123
```

После регистрации или входа токен сохраняется в `.blog_token`.

Создание поста:

```bash
cargo run --bin blog-cli -- posts create \
  --title "CLI пост" \
  --content "Текст из CLI"
```

Получение поста:

```bash
cargo run --bin blog-cli -- posts get 1
```

Обновление поста:

```bash
cargo run --bin blog-cli -- posts update 1 \
  --title "Обновленный CLI пост" \
  --content "Новый текст"
```

Удаление поста:

```bash
cargo run --bin blog-cli -- posts delete 1
```

Список постов:

```bash
cargo run --bin blog-cli -- posts list --limit 10 --offset 0
```

Переключение на gRPC:

```bash
cargo run --bin blog-cli -- --grpc auth login \
  --username alice \
  --password secret123

cargo run --bin blog-cli -- --grpc posts list --limit 10 --offset 0
```

Переопределить адрес сервера:

```bash
cargo run --bin blog-cli -- --server http://127.0.0.1:8080 posts list
cargo run --bin blog-cli -- --grpc --server http://127.0.0.1:50051 posts list
```

JSON-вывод:

```bash
cargo run --bin blog-cli -- --json posts list
```

## WASM-Фронтенд

Фронтенд реализован на Dioxus и обращается к HTTP API напрямую через `gloo-net`.

Функциональность:

- регистрация и вход;
- хранение JWT в `localStorage`;
- загрузка текущего пользователя через `/api/me` по сохраненному JWT;
- список постов при загрузке;
- пагинация списка;
- создание поста только после входа;
- редактирование и удаление только для автора;
- выход из аккаунта;
- toast-уведомления;
- локальная UX-валидация форм;

Запуск из корня репозитория:

```bash
dx serve -p blog-wasm
```

Если backend запущен на другом адресе, передайте URL HTTP API при запуске Dioxus:

```bash
BLOG_API_BASE_URL=http://127.0.0.1:8080/api dx serve -p blog-wasm
```

Dioxus использует настройки из `Dioxus.toml` и assets из `blog-wasm/public`.

Перед запуском фронтенда должен быть поднят backend. По умолчанию WASM-клиент обращается к `http://127.0.0.1:8080/api`;

Сценарий проверки в браузере:

1. Запустить PostgreSQL.
2. Применить миграции: `cargo run -p blog-migration -- up`.
3. Запустить backend: `cargo run --bin blog-server`.
4. Запустить frontend: `dx serve -p blog-wasm`.
5. Открыть адрес, который покажет Dioxus CLI.
6. Нажать `Регистрация`, создать пользователя.
7. Создать пост.
8. Обновить пост через кнопку `Редактировать`.
9. Удалить пост через кнопку `Удалить`.
10. Выйти и убедиться, что кнопки редактирования/удаления больше не показываются.

## Обработка ошибок

На сервере используется `DomainError` на базе `thiserror`. Ошибки преобразуются:

- в HTTP-статусы через `actix_web::ResponseError`;
- в gRPC-статусы через `tonic::Status`;
- в клиентские ошибки через `BlogClientError`;
- в русские сообщения во WASM-фронтенде.

Типовые соответствия:

| Ошибка | HTTP | gRPC |
| --- | --- | --- |
| Validation | `400 Bad Request` | `INVALID_ARGUMENT` |
| InvalidCredentials / Unauthorized | `401 Unauthorized` | `UNAUTHENTICATED` |
| Forbidden | `403 Forbidden` | `PERMISSION_DENIED` |
| NotFound | `404 Not Found` | `NOT_FOUND` |
| UsernameAlreadyTaken / EmailAlreadyTaken | `409 Conflict` | `ALREADY_EXISTS` |
| Internal | `500 Internal Server Error` | `INTERNAL` |

## Безопасность

- Пароли не хранятся в открытом виде, используется Argon2.
- JWT проверяется на сервере для защищенных HTTP/gRPC операций.
- Авторство постов проверяется в `BlogService`, а не только в UI.
- Запросы к БД строятся через SeaORM, ручной конкатенации SQL нет.
- CORS настраивается через `CORS_ORIGINS`.
- Для HTTP API добавлены базовые security headers.

## Полезные команды

```bash
# Проверить весь workspace
cargo test --workspace
cargo clippy --workspace -- -D warnings

# Запустить миграции
cargo run -p blog-migration -- up

# Запустить backend
cargo run --bin blog-server

# Запустить WASM frontend
dx serve -p blog-wasm

# Запустить CLI help
cargo run --bin blog-cli -- --help
cargo run --bin blog-cli -- auth --help
cargo run --bin blog-cli -- posts --help
```
