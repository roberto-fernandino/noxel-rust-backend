-- Add address fields to events and enforce start/end times.

ALTER TABLE events
  ADD COLUMN IF NOT EXISTS venue_name text,
  ADD COLUMN IF NOT EXISTS cep text,
  ADD COLUMN IF NOT EXISTS logradouro text,
  ADD COLUMN IF NOT EXISTS numero text,
  ADD COLUMN IF NOT EXISTS complemento text,
  ADD COLUMN IF NOT EXISTS bairro text,
  ADD COLUMN IF NOT EXISTS cidade text,
  ADD COLUMN IF NOT EXISTS estado char(2);

-- Enforce required address fields (MVP)
ALTER TABLE events
  ALTER COLUMN cep SET NOT NULL,
  ALTER COLUMN logradouro SET NOT NULL,
  ALTER COLUMN numero SET NOT NULL,
  ALTER COLUMN cidade SET NOT NULL,
  ALTER COLUMN estado SET NOT NULL;

-- Format constraints
ALTER TABLE events
  ADD CONSTRAINT IF NOT EXISTS events_cep_format_chk
  CHECK (cep ~ '^[0-9]{5}-?[0-9]{3}$');

ALTER TABLE events
  ADD CONSTRAINT IF NOT EXISTS events_estado_len_chk
  CHECK (char_length(estado) = 2);

-- Enforce times
ALTER TABLE events
  ALTER COLUMN starts_at SET NOT NULL,
  ALTER COLUMN ends_at SET NOT NULL;

ALTER TABLE events
  ADD CONSTRAINT IF NOT EXISTS events_time_range_chk
  CHECK (ends_at >= starts_at);
