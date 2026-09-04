BEGIN;

CREATE TABLE accounts (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    login TEXT NOT NULL UNIQUE CHECK (length(login) BETWEEN 3 AND 64),
    password_hash TEXT NOT NULL CHECK (password_hash LIKE '$argon2id$%'),
    role TEXT NOT NULL DEFAULT 'player' CHECK (role IN ('player','moderator','administrator')),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE TABLE invites (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    code_hash TEXT NOT NULL UNIQUE,
    created_by BIGINT REFERENCES accounts(id),
    expires_at TIMESTAMPTZ,
    redeemed_by BIGINT UNIQUE REFERENCES accounts(id),
    redeemed_at TIMESTAMPTZ
);
CREATE TABLE sessions (
    id UUID PRIMARY KEY,
    account_id BIGINT NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    refresh_token_hash TEXT NOT NULL UNIQUE,
    expires_at TIMESTAMPTZ NOT NULL,
    revoked_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE TABLE bans (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    account_id BIGINT NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    reason TEXT NOT NULL,
    created_by BIGINT NOT NULL REFERENCES accounts(id),
    expires_at TIMESTAMPTZ,
    revoked_at TIMESTAMPTZ,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE characters (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    account_id BIGINT NOT NULL REFERENCES accounts(id) ON DELETE CASCADE,
    first_name TEXT NOT NULL CHECK (length(first_name) BETWEEN 2 AND 32),
    last_name TEXT NOT NULL CHECK (length(last_name) BETWEEN 2 AND 32),
    model_hash BIGINT NOT NULL CHECK (model_hash BETWEEN 1 AND 4294967295),
    instance_id INTEGER NOT NULL DEFAULT 0 CHECK (instance_id >= 0),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (account_id, first_name, last_name)
);
CREATE TABLE appearances (
    character_id BIGINT PRIMARY KEY REFERENCES characters(id) ON DELETE CASCADE,
    face_preset SMALLINT NOT NULL DEFAULT 0 CHECK (face_preset BETWEEN 0 AND 20),
    components JSONB NOT NULL DEFAULT '{}'::jsonb
);
CREATE TABLE character_positions (
    character_id BIGINT PRIMARY KEY REFERENCES characters(id) ON DELETE CASCADE,
    x DOUBLE PRECISION NOT NULL CHECK (abs(x) <= 20000),
    y DOUBLE PRECISION NOT NULL CHECK (abs(y) <= 20000),
    z DOUBLE PRECISION NOT NULL CHECK (abs(z) <= 20000),
    heading DOUBLE PRECISION NOT NULL CHECK (heading >= 0 AND heading < 360),
    confirmed_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE wallets (
    character_id BIGINT PRIMARY KEY REFERENCES characters(id) ON DELETE CASCADE,
    cash BIGINT NOT NULL DEFAULT 0 CHECK (cash >= 0),
    bank BIGINT NOT NULL DEFAULT 0 CHECK (bank >= 0),
    revision BIGINT NOT NULL DEFAULT 0 CHECK (revision >= 0)
);
CREATE TABLE ledger_entries (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    character_id BIGINT NOT NULL REFERENCES characters(id),
    cash_delta BIGINT NOT NULL,
    bank_delta BIGINT NOT NULL,
    reason TEXT NOT NULL CHECK (length(reason) BETWEEN 1 AND 128),
    idempotency_key TEXT NOT NULL CHECK (length(idempotency_key) BETWEEN 1 AND 128),
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    UNIQUE (character_id, idempotency_key)
);
CREATE TABLE command_receipts (
    character_id BIGINT NOT NULL REFERENCES characters(id) ON DELETE CASCADE,
    idempotency_key TEXT NOT NULL CHECK (length(idempotency_key) BETWEEN 1 AND 128),
    request_fingerprint TEXT NOT NULL CHECK (length(request_fingerprint) BETWEEN 1 AND 512),
    response JSONB NOT NULL,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now(),
    PRIMARY KEY (character_id, idempotency_key)
);
CREATE TABLE item_definitions (
    id INTEGER PRIMARY KEY CHECK (id > 0),
    name TEXT NOT NULL UNIQUE,
    unit_weight_grams INTEGER NOT NULL CHECK (unit_weight_grams > 0),
    usable BOOLEAN NOT NULL DEFAULT false
);
CREATE TABLE inventories (
    character_id BIGINT PRIMARY KEY REFERENCES characters(id) ON DELETE CASCADE,
    max_weight_grams INTEGER NOT NULL DEFAULT 30000 CHECK (max_weight_grams > 0),
    revision BIGINT NOT NULL DEFAULT 0 CHECK (revision >= 0)
);
CREATE TABLE inventory_items (
    character_id BIGINT NOT NULL REFERENCES inventories(character_id) ON DELETE CASCADE,
    item_id INTEGER NOT NULL REFERENCES item_definitions(id),
    quantity INTEGER NOT NULL CHECK (quantity > 0),
    PRIMARY KEY (character_id, item_id)
);
CREATE TABLE shops (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    name TEXT NOT NULL UNIQUE,
    enabled BOOLEAN NOT NULL DEFAULT true
);
CREATE TABLE shop_items (
    shop_id INTEGER NOT NULL REFERENCES shops(id) ON DELETE CASCADE,
    item_id INTEGER NOT NULL REFERENCES item_definitions(id),
    price BIGINT NOT NULL CHECK (price > 0),
    PRIMARY KEY (shop_id, item_id)
);
CREATE TABLE jobs (
    id INTEGER GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    code TEXT NOT NULL UNIQUE,
    enabled BOOLEAN NOT NULL DEFAULT true
);
CREATE TABLE job_progress (
    character_id BIGINT PRIMARY KEY REFERENCES characters(id) ON DELETE CASCADE,
    job_id INTEGER NOT NULL REFERENCES jobs(id),
    state JSONB NOT NULL,
    revision BIGINT NOT NULL DEFAULT 0 CHECK (revision >= 0),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE TABLE audit_events (
    id BIGINT GENERATED ALWAYS AS IDENTITY PRIMARY KEY,
    actor_account_id BIGINT REFERENCES accounts(id),
    action TEXT NOT NULL,
    target_type TEXT,
    target_id TEXT,
    correlation_id UUID,
    details JSONB NOT NULL DEFAULT '{}'::jsonb,
    created_at TIMESTAMPTZ NOT NULL DEFAULT now()
);
CREATE TABLE resource_state (
    resource_name TEXT PRIMARY KEY,
    desired_state TEXT NOT NULL CHECK (desired_state IN ('stopped','started')),
    generation BIGINT NOT NULL DEFAULT 0 CHECK (generation >= 0),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT now()
);

CREATE INDEX sessions_account_active_idx ON sessions(account_id, expires_at) WHERE revoked_at IS NULL;
CREATE INDEX bans_account_active_idx ON bans(account_id, expires_at) WHERE revoked_at IS NULL;
CREATE INDEX characters_account_idx ON characters(account_id);
CREATE INDEX ledger_character_time_idx ON ledger_entries(character_id, created_at DESC);
CREATE INDEX audit_time_idx ON audit_events(created_at DESC);

COMMIT;
