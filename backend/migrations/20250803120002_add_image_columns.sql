-- Add image storage columns for MinIO

-- Add logo column to team table
ALTER TABLE team ADD logo_path TEXT;

-- Add photo column to player table  
ALTER TABLE player ADD photo_path TEXT;

-- Update audit columns for new fields
UPDATE team SET updated_at = CURRENT_TIMESTAMP WHERE logo_path IS NOT NULL;
UPDATE player SET updated_at = CURRENT_TIMESTAMP WHERE photo_path IS NOT NULL;
