-- Initial schema

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";

-- Users
-- Roles: organizer | attendee | admin
CREATE TABLE IF NOT EXISTS users (
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
  full_name text NOT NULL,
  role text NOT NULL CHECK (role IN ('organizer', 'attendee', 'admin')),
  created_at timestamptz NOT NULL DEFAULT now()
);

-- Tickets (placeholder)
CREATE TABLE IF NOT EXISTS tickets (
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4(),
  title text NOT NULL,
  created_at timestamptz NOT NULL DEFAULT now()
);
