-- Создание таблицы пользователей
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    username VARCHAR(32) NOT NULL UNIQUE,
    email VARCHAR(255) NOT NULL UNIQUE,
    password_hash VARCHAR(255) NOT NULL,
    status VARCHAR(20) NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ NOT NULL,
    updated_at TIMESTAMPTZ NOT NULL,
    last_login TIMESTAMPTZ,
    roles TEXT[] NOT NULL DEFAULT '{user}',
    permissions TEXT[] NOT NULL DEFAULT '{}',
    failed_login_attempts INTEGER NOT NULL DEFAULT 0,
    lockout_until TIMESTAMPTZ,
    totp_secret VARCHAR(255),
    totp_enabled BOOLEAN NOT NULL DEFAULT FALSE
);

-- Индексы для повышения производительности
CREATE INDEX idx_users_username ON users(username);
CREATE INDEX idx_users_email ON users(email);
CREATE INDEX idx_users_status ON users(status);
CREATE INDEX idx_users_last_login ON users(last_login);

-- Комментарии к таблице и полям
COMMENT ON TABLE users IS 'Таблица пользователей для аутентификации и авторизации';
COMMENT ON COLUMN users.id IS 'Уникальный идентификатор пользователя';
COMMENT ON COLUMN users.username IS 'Имя пользователя';
COMMENT ON COLUMN users.email IS 'Email пользователя';
COMMENT ON COLUMN users.password_hash IS 'Хеш пароля пользователя';
COMMENT ON COLUMN users.status IS 'Статус пользователя: active, inactive, banned, pending';
COMMENT ON COLUMN users.created_at IS 'Дата создания пользователя';
COMMENT ON COLUMN users.updated_at IS 'Дата последнего обновления пользователя';
COMMENT ON COLUMN users.last_login IS 'Дата последнего входа';
COMMENT ON COLUMN users.roles IS 'Роли пользователя';
COMMENT ON COLUMN users.permissions IS 'Разрешения пользователя';
COMMENT ON COLUMN users.failed_login_attempts IS 'Количество неудачных попыток входа';
COMMENT ON COLUMN users.lockout_until IS 'Время блокировки после превышения количества попыток';
COMMENT ON COLUMN users.totp_secret IS 'Секрет для двухфакторной аутентификации';
COMMENT ON COLUMN users.totp_enabled IS 'Флаг включения двухфакторной аутентификации'; 