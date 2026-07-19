-- Тестовые данные для локальной ручной проверки.
--
-- Запускать после миграций:
--   psql "$DATABASE_URL" -f scripts/seed_dev_data.sql
--
-- Тестовые пользователи:
--   alice / secret123
--   bob   / secret123

WITH seed_users(id, username, email, password_hash, created_at) AS (
    VALUES
        (
            '11111111-1111-4111-8111-111111111111'::uuid,
            'alice',
            'alice@example.com',
            '$argon2id$v=19$m=19456,t=2,p=1$16Gix3Be4DvMfOvBcP9fXg$PtNwd6BHWGoxPJkCF+l0JDAeIPc5bfY17T6QnmgR30o',
            now() - interval '3 days'
        ),
        (
            '22222222-2222-4222-8222-222222222222'::uuid,
            'bob',
            'bob@example.com',
            '$argon2id$v=19$m=19456,t=2,p=1$16Gix3Be4DvMfOvBcP9fXg$PtNwd6BHWGoxPJkCF+l0JDAeIPc5bfY17T6QnmgR30o',
            now() - interval '2 days'
        )
)
INSERT INTO users (id, username, email, password_hash, created_at)
SELECT id, username, email, password_hash, created_at
FROM seed_users
ON CONFLICT DO NOTHING;

WITH seed_posts(title, content, author_username, created_at, updated_at) AS (
    VALUES
        (
            'Первый пост Alice',
            'Небольшой тестовый пост для проверки списка, карточек и пагинации.',
            'alice',
            now() - interval '2 days 3 hours',
            NULL::timestamptz
        ),
        (
            'Заметка про Rust',
            'Rust хорошо подходит для backend-а: строгие типы помогают ловить ошибки до запуска.',
            'alice',
            now() - interval '1 day 8 hours',
            now() - interval '1 day 6 hours'
        ),
        (
            'Пост Bob про gRPC',
            'Этот пост нужен, чтобы проверить отображение автора и запрет редактирования чужих записей.',
            'bob',
            now() - interval '1 day 2 hours',
            NULL::timestamptz
        ),
        (
            'Пагинация работает',
            'Еще одна запись, чтобы список постов был похож на реальные данные.',
            'bob',
            now() - interval '5 hours',
            NULL::timestamptz
        )
), resolved_posts AS (
    SELECT
        seed_posts.title,
        seed_posts.content,
        users.id AS author_id,
        seed_posts.created_at,
        seed_posts.updated_at
    FROM seed_posts
    JOIN users ON users.username = seed_posts.author_username
)
INSERT INTO posts (title, content, author_id, created_at, updated_at)
SELECT title, content, author_id, created_at, updated_at
FROM resolved_posts
WHERE NOT EXISTS (
    SELECT 1
    FROM posts
    WHERE posts.title = resolved_posts.title
      AND posts.author_id = resolved_posts.author_id
);
