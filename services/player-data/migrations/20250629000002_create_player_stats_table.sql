-- Migration: Create player stats table
-- Created: 2025-06-29

-- Create player_stats table for detailed statistics
CREATE TABLE player_stats (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    player_id UUID NOT NULL REFERENCES players(id) ON DELETE CASCADE,
    stat_name VARCHAR(100) NOT NULL,
    stat_value BIGINT NOT NULL DEFAULT 0,
    stat_type VARCHAR(50) NOT NULL DEFAULT 'counter', -- counter, gauge, timer, etc.
    category VARCHAR(50) NOT NULL DEFAULT 'general', -- combat, racing, heists, etc.
    description TEXT,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    
    UNIQUE(player_id, stat_name)
);

-- Create indexes for player stats
CREATE INDEX idx_player_stats_player_id ON player_stats(player_id);
CREATE INDEX idx_player_stats_stat_name ON player_stats(stat_name);
CREATE INDEX idx_player_stats_category ON player_stats(category);
CREATE INDEX idx_player_stats_stat_type ON player_stats(stat_type);
CREATE INDEX idx_player_stats_updated_at ON player_stats(updated_at);

-- Apply updated_at trigger
CREATE TRIGGER update_player_stats_updated_at 
    BEFORE UPDATE ON player_stats 
    FOR EACH ROW 
    EXECUTE FUNCTION update_updated_at_column();

-- Create achievements table
CREATE TABLE achievements (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    achievement_key VARCHAR(100) NOT NULL UNIQUE,
    name VARCHAR(200) NOT NULL,
    description TEXT NOT NULL,
    icon_url TEXT,
    category VARCHAR(50) NOT NULL DEFAULT 'general',
    difficulty INTEGER NOT NULL DEFAULT 1 CHECK (difficulty BETWEEN 1 AND 5),
    points INTEGER NOT NULL DEFAULT 10 CHECK (points >= 0),
    requirements JSONB NOT NULL DEFAULT '{}',
    is_active BOOLEAN NOT NULL DEFAULT true,
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Create player achievements junction table
CREATE TABLE player_achievements (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    player_id UUID NOT NULL REFERENCES players(id) ON DELETE CASCADE,
    achievement_id UUID NOT NULL REFERENCES achievements(id) ON DELETE CASCADE,
    unlocked_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    progress JSONB NOT NULL DEFAULT '{}',
    
    UNIQUE(player_id, achievement_id)
);

-- Create indexes for achievements
CREATE INDEX idx_achievements_key ON achievements(achievement_key);
CREATE INDEX idx_achievements_category ON achievements(category);
CREATE INDEX idx_achievements_difficulty ON achievements(difficulty);
CREATE INDEX idx_achievements_active ON achievements(is_active);

CREATE INDEX idx_player_achievements_player_id ON player_achievements(player_id);
CREATE INDEX idx_player_achievements_achievement_id ON player_achievements(achievement_id);
CREATE INDEX idx_player_achievements_unlocked_at ON player_achievements(unlocked_at);

-- Apply triggers
CREATE TRIGGER update_achievements_updated_at 
    BEFORE UPDATE ON achievements 
    FOR EACH ROW 
    EXECUTE FUNCTION update_updated_at_column();

-- Create game sessions table
CREATE TABLE game_sessions (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),
    player_id UUID NOT NULL REFERENCES players(id) ON DELETE CASCADE,
    server_id VARCHAR(100),
    region VARCHAR(100),
    started_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    ended_at TIMESTAMPTZ,
    duration_seconds INTEGER,
    ip_address INET,
    user_agent TEXT,
    game_version VARCHAR(50),
    is_active BOOLEAN NOT NULL DEFAULT true,
    session_data JSONB NOT NULL DEFAULT '{}'
);

-- Create indexes for sessions
CREATE INDEX idx_game_sessions_player_id ON game_sessions(player_id);
CREATE INDEX idx_game_sessions_started_at ON game_sessions(started_at);
CREATE INDEX idx_game_sessions_ended_at ON game_sessions(ended_at);
CREATE INDEX idx_game_sessions_active ON game_sessions(is_active);
CREATE INDEX idx_game_sessions_server_id ON game_sessions(server_id);
CREATE INDEX idx_game_sessions_region ON game_sessions(region);

-- Add comments for documentation
COMMENT ON TABLE player_stats IS 'Detailed player statistics and metrics';
COMMENT ON TABLE achievements IS 'Available achievements and rewards';
COMMENT ON TABLE player_achievements IS 'Player achievement unlocks and progress';
COMMENT ON TABLE game_sessions IS 'Player game session tracking';