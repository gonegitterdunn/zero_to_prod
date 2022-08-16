-- Add salt column to table users
ALTER TABLE users ADD COLUMN salt TEXT NOT NULL;