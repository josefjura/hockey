-- Add host country to season table
-- Seasons can have their own host country (e.g., World Championship 2024 in Sweden)
-- If NULL, the country should default to the event's country

ALTER TABLE season ADD COLUMN country_id INTEGER;

-- Add foreign key constraint
-- Note: SQLite requires recreating foreign key constraints on ALTER
-- But since we're just adding a nullable column, the foreign key will be enforced on INSERT/UPDATE

-- Add index for better query performance
CREATE INDEX IF NOT EXISTS idx_season_country_id ON season(country_id);
