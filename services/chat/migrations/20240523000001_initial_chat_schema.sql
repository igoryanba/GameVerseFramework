-- Создание типов для микросервиса чата
CREATE TYPE message_type AS ENUM (
    'text',
    'action',
    'ooc',
    'radio',
    'phone',
    'admin',
    'system',
    'dice'
);

CREATE TYPE channel_type AS ENUM (
    'global',
    'local',
    'radio',
    'phone',
    'group',
    'direct',
    'ooc',
    'admin',
    'telegram'
);

CREATE TYPE member_role AS ENUM (
    'owner',
    'admin',
    'moderator',
    'member'
);

CREATE TYPE voice_session_type AS ENUM (
    'proximity',
    'radio',
    'phone',
    'group'
);

CREATE TYPE voice_range AS ENUM (
    'whisper',
    'normal',
    'shout',
    'megaphone'
);

CREATE TYPE voice_quality AS ENUM (
    'low',
    'medium',
    'high',
    'crystal'
);

CREATE TYPE read_status AS ENUM (
    'sent',
    'delivered',
    'read'
);

CREATE TYPE telegram_chat_type AS ENUM (
    'private',
    'group',
    'supergroup',
    'channel'
);

-- Таблица каналов чата
CREATE TABLE chat_channels (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    channel_type channel_type NOT NULL,
    created_by UUID NOT NULL,
    is_private BOOLEAN NOT NULL DEFAULT FALSE,
    is_temporary BOOLEAN NOT NULL DEFAULT FALSE,
    max_participants INTEGER,
    settings JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    expires_at TIMESTAMP WITH TIME ZONE,
    telegram_chat_id BIGINT
);

-- Индексы для таблицы каналов
CREATE INDEX idx_chat_channels_type ON chat_channels(channel_type);
CREATE INDEX idx_chat_channels_created_by ON chat_channels(created_by);
CREATE INDEX idx_chat_channels_telegram ON chat_channels(telegram_chat_id);

-- Таблица сообщений чата
CREATE TABLE chat_messages (
    id UUID PRIMARY KEY,
    channel_id UUID NOT NULL REFERENCES chat_channels(id) ON DELETE CASCADE,
    sender_id UUID NOT NULL,
    content TEXT NOT NULL,
    message_type message_type NOT NULL DEFAULT 'text',
    reply_to UUID REFERENCES chat_messages(id) ON DELETE SET NULL,
    forwarded_from UUID REFERENCES chat_messages(id) ON DELETE SET NULL,
    edited_at TIMESTAMP WITH TIME ZONE,
    deleted_at TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    metadata JSONB NOT NULL DEFAULT '{}'
);

-- Индексы для таблицы сообщений
CREATE INDEX idx_chat_messages_channel ON chat_messages(channel_id, created_at DESC);
CREATE INDEX idx_chat_messages_sender ON chat_messages(sender_id, created_at DESC);
CREATE INDEX idx_chat_messages_type ON chat_messages(message_type);
CREATE INDEX idx_chat_messages_reply ON chat_messages(reply_to);

-- Таблица участников каналов
CREATE TABLE channel_members (
    channel_id UUID NOT NULL REFERENCES chat_channels(id) ON DELETE CASCADE,
    user_id UUID NOT NULL,
    role member_role NOT NULL DEFAULT 'member',
    joined_at TIMESTAMP WITH TIME ZONE NOT NULL,
    last_read_at TIMESTAMP WITH TIME ZONE,
    is_muted BOOLEAN NOT NULL DEFAULT FALSE,
    muted_until TIMESTAMP WITH TIME ZONE,
    notification_settings JSONB NOT NULL DEFAULT '{}',
    PRIMARY KEY (channel_id, user_id)
);

-- Индексы для участников каналов
CREATE INDEX idx_channel_members_user ON channel_members(user_id);
CREATE INDEX idx_channel_members_role ON channel_members(role);

-- Таблица голосовых сессий
CREATE TABLE voice_sessions (
    id UUID PRIMARY KEY,
    user_id UUID NOT NULL,
    session_type voice_session_type NOT NULL,
    channel_id UUID REFERENCES chat_channels(id) ON DELETE SET NULL,
    voice_range voice_range NOT NULL DEFAULT 'normal',
    is_muted BOOLEAN NOT NULL DEFAULT FALSE,
    is_deafened BOOLEAN NOT NULL DEFAULT FALSE,
    position JSONB NOT NULL DEFAULT '{}',
    quality voice_quality NOT NULL DEFAULT 'medium',
    started_at TIMESTAMP WITH TIME ZONE NOT NULL,
    ended_at TIMESTAMP WITH TIME ZONE
);

-- Индексы для голосовых сессий
CREATE INDEX idx_voice_sessions_user ON voice_sessions(user_id);
CREATE INDEX idx_voice_sessions_active ON voice_sessions(ended_at) WHERE ended_at IS NULL;
CREATE INDEX idx_voice_sessions_channel ON voice_sessions(channel_id);

-- Таблица интеграции с Telegram
CREATE TABLE telegram_users (
    user_id UUID NOT NULL,
    telegram_id BIGINT NOT NULL UNIQUE,
    username TEXT,
    first_name TEXT NOT NULL,
    last_name TEXT,
    is_verified BOOLEAN NOT NULL DEFAULT FALSE,
    notifications_enabled BOOLEAN NOT NULL DEFAULT TRUE,
    linked_at TIMESTAMP WITH TIME ZONE NOT NULL,
    last_activity TIMESTAMP WITH TIME ZONE,
    PRIMARY KEY (user_id, telegram_id)
);

-- Индексы для Telegram пользователей
CREATE INDEX idx_telegram_users_telegram_id ON telegram_users(telegram_id);
CREATE INDEX idx_telegram_users_verified ON telegram_users(is_verified);

-- Таблица связей каналов с Telegram
CREATE TABLE telegram_channel_links (
    channel_id UUID NOT NULL REFERENCES chat_channels(id) ON DELETE CASCADE,
    telegram_chat_id BIGINT NOT NULL,
    chat_type telegram_chat_type NOT NULL,
    is_bidirectional BOOLEAN NOT NULL DEFAULT TRUE,
    created_by UUID NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    settings JSONB NOT NULL DEFAULT '{}',
    PRIMARY KEY (channel_id, telegram_chat_id)
);

-- Индексы для связей Telegram
CREATE INDEX idx_telegram_links_chat_id ON telegram_channel_links(telegram_chat_id);
CREATE INDEX idx_telegram_links_type ON telegram_channel_links(chat_type);

-- Таблица статусов прочтения
CREATE TABLE message_statuses (
    message_id UUID NOT NULL REFERENCES chat_messages(id) ON DELETE CASCADE,
    user_id UUID NOT NULL,
    status read_status NOT NULL,
    timestamp TIMESTAMP WITH TIME ZONE NOT NULL,
    PRIMARY KEY (message_id, user_id)
);

-- Индексы для статусов сообщений
CREATE INDEX idx_message_statuses_user ON message_statuses(user_id, timestamp DESC);
CREATE INDEX idx_message_statuses_status ON message_statuses(status); 