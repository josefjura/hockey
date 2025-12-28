-- Remove career totals columns as they don't make sense for cross-competition stats
-- Statistics should be calculated per season/event, not career-wide
ALTER TABLE player DROP COLUMN goals_total;
ALTER TABLE player DROP COLUMN assists_total;
