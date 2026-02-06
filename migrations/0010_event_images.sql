-- Event images in multiple resolutions (banner, thumbnail, gallery, etc.)

CREATE TABLE IF NOT EXISTS event_images (
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),

  event_id uuid NOT NULL REFERENCES events (id) ON DELETE CASCADE,

  -- Logical group (e.g. banner, gallery, logo)
  kind text NOT NULL CHECK (kind IN ('banner', 'thumbnail', 'gallery', 'logo')),

  -- Variant of the same kind (e.g. original, 1600x838, 800x419)
  variant text NOT NULL,

  -- Where the file lives (S3 key, CDN path, etc.)
  storage_key text NOT NULL,
  url text,

  width int,
  height int,
  mime_type text,
  bytes bigint,

  created_at timestamptz NOT NULL DEFAULT now (),

  -- avoid duplicate variants per event/kind
  UNIQUE (event_id, kind, variant)
);

CREATE INDEX IF NOT EXISTS event_images_event_id_idx ON event_images (event_id);
CREATE INDEX IF NOT EXISTS event_images_kind_idx ON event_images (kind);
