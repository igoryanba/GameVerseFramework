-- Создание таблицы токенов
CREATE TABLE IF NOT EXISTS tokens (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    token_type VARCHAR(20) NOT NULL,
    token TEXT NOT NULL UNIQUE,
    created_at TIMESTAMPTZ NOT NULL,
    expires_at TIMESTAMPTZ NOT NULL,
    revoked BOOLEAN NOT NULL DEFAULT FALSE,
    revoked_at TIMESTAMPTZ,
    ip_address VARCHAR(45),
    user_agent TEXT
);

-- Индексы для повышения производительности
CREATE INDEX idx_tokens_user_id ON tokens(user_id);
CREATE INDEX idx_tokens_token_type ON tokens(token_type);
CREATE INDEX idx_tokens_expires_at ON tokens(expires_at);
CREATE INDEX idx_tokens_revoked ON tokens(revoked);

-- Комментарии к таблице и полям
COMMENT ON TABLE tokens IS 'Таблица токенов аутентификации';
COMMENT ON COLUMN tokens.id IS 'Уникальный идентификатор токена';
COMMENT ON COLUMN tokens.user_id IS 'Идентификатор пользователя, которому принадлежит токен';
COMMENT ON COLUMN tokens.token_type IS 'Тип токена: access, refresh';
COMMENT ON COLUMN tokens.token IS 'Значение токена';
COMMENT ON COLUMN tokens.created_at IS 'Дата создания токена';
COMMENT ON COLUMN tokens.expires_at IS 'Дата истечения срока действия токена';
COMMENT ON COLUMN tokens.revoked IS 'Флаг отзыва токена';
COMMENT ON COLUMN tokens.revoked_at IS 'Дата отзыва токена';
COMMENT ON COLUMN tokens.ip_address IS 'IP-адрес, с которого был создан токен';
COMMENT ON COLUMN tokens.user_agent IS 'User-Agent, с которого был создан токен'; 