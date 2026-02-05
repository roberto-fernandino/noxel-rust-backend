-- Add additional user fields
ALTER TABLE users
ADD COLUMN IF NOT EXISTS email text NOT NULL;

-- Email should be unique when present
CREATE UNIQUE INDEX IF NOT EXISTS users_email_unique ON users (lower(email))
WHERE
  email IS NOT NULL;