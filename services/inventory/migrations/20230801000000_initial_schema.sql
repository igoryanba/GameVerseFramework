-- Создание типа для инвентаря
CREATE TYPE inventory_type AS ENUM (
    'player',
    'vehicle',
    'container',
    'shop',
    'temp'
);

-- Таблица инвентарей
CREATE TABLE inventories (
    id UUID PRIMARY KEY,
    owner_id TEXT NOT NULL,
    owner_type TEXT NOT NULL,
    inventory_type inventory_type NOT NULL,
    max_weight FLOAT NOT NULL,
    max_slots INTEGER NOT NULL,
    name TEXT NOT NULL,
    metadata JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL
);

-- Индексы для таблицы инвентарей
CREATE INDEX idx_inventories_owner ON inventories(owner_id, owner_type);
CREATE INDEX idx_inventories_type ON inventories(inventory_type);

-- Таблица шаблонов предметов
CREATE TABLE item_templates (
    id UUID PRIMARY KEY,
    name TEXT NOT NULL,
    description TEXT NOT NULL,
    weight FLOAT NOT NULL,
    stackable BOOLEAN NOT NULL,
    max_stack INTEGER NOT NULL,
    max_durability FLOAT NOT NULL,
    icon TEXT NOT NULL,
    category TEXT NOT NULL,
    properties JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL
);

-- Индексы для таблицы шаблонов предметов
CREATE INDEX idx_item_templates_category ON item_templates(category);
CREATE UNIQUE INDEX idx_item_templates_name ON item_templates(name);

-- Таблица предметов
CREATE TABLE items (
    id UUID PRIMARY KEY,
    inventory_id UUID NOT NULL REFERENCES inventories(id) ON DELETE CASCADE,
    template_id UUID NOT NULL REFERENCES item_templates(id),
    position INTEGER NOT NULL,
    quantity INTEGER NOT NULL,
    durability FLOAT NOT NULL,
    metadata JSONB NOT NULL DEFAULT '{}',
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL,
    UNIQUE(inventory_id, position)
);

-- Индексы для таблицы предметов
CREATE INDEX idx_items_inventory_id ON items(inventory_id);
CREATE INDEX idx_items_template_id ON items(template_id); 