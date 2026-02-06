-- Tickets (minimal MVP)
-- A user can own multiple tickets for the same event.

CREATE TABLE IF NOT EXISTS tickets (
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),

  event_id uuid NOT NULL REFERENCES events (id) ON DELETE CASCADE,
  owner_user_id uuid NOT NULL REFERENCES users (id) ON DELETE RESTRICT,

  -- Value encoded in the QR code (store token/string, not the image)
  qr_code text NOT NULL,

  is_active boolean NOT NULL DEFAULT true,

  created_at timestamptz NOT NULL DEFAULT now (),
  updated_at timestamptz NOT NULL DEFAULT now (),

  CONSTRAINT tickets_qr_code_unique UNIQUE (qr_code)
);

CREATE INDEX IF NOT EXISTS tickets_event_id_idx ON tickets (event_id);
CREATE INDEX IF NOT EXISTS tickets_owner_user_id_idx ON tickets (owner_user_id);
CREATE INDEX IF NOT EXISTS tickets_is_active_idx ON tickets (is_active);
