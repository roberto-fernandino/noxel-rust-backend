-- Enforce that every user must have a password.

-- If there are existing rows without a password_hash, this will fail.
-- Run a backfill or delete those rows before applying.
ALTER TABLE users
  ALTER COLUMN password_hash SET NOT NULL;
