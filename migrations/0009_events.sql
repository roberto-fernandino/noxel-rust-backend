-- Events core table

CREATE TABLE IF NOT EXISTS events (
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),

  organizer_user_id uuid NOT NULL REFERENCES users (id) ON DELETE RESTRICT,

  name text NOT NULL,

  -- Google Maps coordinates (WGS84)
  latitude double precision NOT NULL CHECK (latitude BETWEEN -90 AND 90),
  longitude double precision NOT NULL CHECK (longitude BETWEEN -180 AND 180),

  status text NOT NULL DEFAULT 'draft' CHECK (status IN ('draft', 'published', 'cancelled')),

  starts_at timestamptz,
  ends_at timestamptz,

  created_at timestamptz NOT NULL DEFAULT now (),
  updated_at timestamptz NOT NULL DEFAULT now ()
);

CREATE INDEX IF NOT EXISTS events_organizer_user_id_idx ON events (organizer_user_id);
CREATE INDEX IF NOT EXISTS events_status_idx ON events (status);
CREATE INDEX IF NOT EXISTS events_starts_at_idx ON events (starts_at);

-- Helpful for nearby queries without PostGIS (bounding box scans)
CREATE INDEX IF NOT EXISTS events_lat_lon_idx ON events (latitude, longitude);
