-- Migration: Create players table
-- Created: 2025-06-29

-- Create custom enum types
CREATE TYPE player_status AS ENUM ('active', 'suspended', 'banned', 'inactive');

-- Create players table
CREATE TABLE players (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    username VARCHAR(50) NOT NULL UNIQUE,
    display_name VARCHAR(100) NOT NULL,
    avatar_url TEXT,
    email VARCHAR(255) NOT NULL UNIQUE,
    level INTEGER NOT NULL DEFAULT 1 CHECK (level >= 1),
    experience BIGINT NOT NULL DEFAULT 0 CHECK (experience >= 0),
    experience_to_next_level BIGINT NOT NULL DEFAULT 1000,
    gold BIGINT NOT NULL DEFAULT 1000 CHECK (gold >= 0),
    gems BIGINT NOT NULL DEFAULT 100 CHECK (gems >= 0),
    settings JSONB NOT NULL DEFAULT '{}',
    status player_status NOT NULL DEFAULT 'active',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    last_login TIMESTAMPTZ,
    total_playtime BIGINT NOT NULL DEFAULT 0 CHECK (total_playtime >= 0),
    current_session_id UUID,
    current_region VARCHAR(100)
);

-- Create indexes for better query performance
CREATE INDEX idx_players_username ON players(username);
CREATE INDEX idx_players_email ON players(email);
CREATE INDEX idx_players_status ON players(status);
CREATE INDEX idx_players_level ON players(level);
CREATE INDEX idx_players_created_at ON players(created_at);
CREATE INDEX idx_players_last_login ON players(last_login);
CREATE INDEX idx_players_current_session ON players(current_session_id);
CREATE INDEX idx_players_region ON players(current_region);

-- Create updated_at trigger function
CREATE OR REPLACE FUNCTION update_updated_at_column()
RETURNS TRIGGER AS $$
BEGIN
    NEW.updated_at = NOW();
    RETURN NEW;
END;
$$ language 'plpgsql';

-- Apply trigger to players table
CREATE TRIGGER update_players_updated_at 
    BEFORE UPDATE ON players 
    FOR EACH ROW 
    EXECUTE FUNCTION update_updated_at_column();

-- Add comments for documentation
COMMENT ON TABLE players IS 'Main player data table storing user profiles and game progress';
COMMENT ON COLUMN players.username IS 'Unique username for login and identification';
COMMENT ON COLUMN players.display_name IS 'Display name shown to other players';
COMMENT ON COLUMN players.level IS 'Current player level (starts at 1)';
COMMENT ON COLUMN players.experience IS 'Current experience points';
COMMENT ON COLUMN players.experience_to_next_level IS 'Experience needed for next level';
COMMENT ON COLUMN players.gold IS 'In-game currency (gold coins)';
COMMENT ON COLUMN players.gems IS 'Premium currency (gems/crystals)';
COMMENT ON COLUMN players.settings IS 'Player preferences and game settings in JSON format';
COMMENT ON COLUMN players.total_playtime IS 'Total time played in seconds';
COMMENT ON COLUMN players.current_session_id IS 'ID of current active game session';
COMMENT ON COLUMN players.current_region IS 'Current game server/region';