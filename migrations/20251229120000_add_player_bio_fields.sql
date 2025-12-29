-- Add biographical fields to player table (hockey card info)

ALTER TABLE player ADD COLUMN birth_date TEXT; -- ISO date format YYYY-MM-DD
ALTER TABLE player ADD COLUMN birth_place TEXT; -- City/town of birth
ALTER TABLE player ADD COLUMN height_cm INTEGER; -- Height in centimeters
ALTER TABLE player ADD COLUMN weight_kg INTEGER; -- Weight in kilograms
ALTER TABLE player ADD COLUMN position TEXT; -- Forward, Defense, Goalie
ALTER TABLE player ADD COLUMN shoots TEXT; -- Left, Right, or NULL
