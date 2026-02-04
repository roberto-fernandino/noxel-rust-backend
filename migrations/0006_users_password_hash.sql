-- Add password hash for local auth

ALTER TABLE users
  ADD COLUMN IF NOT EXISTS password_hash text;
