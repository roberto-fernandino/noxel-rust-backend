-- Enable trigram indexes for fast ILIKE/partial search

CREATE EXTENSION IF NOT EXISTS pg_trgm;

-- Search indexes (use with ILIKE / similarity)
CREATE INDEX IF NOT EXISTS events_name_trgm_idx ON events USING gin (name gin_trgm_ops);
CREATE INDEX IF NOT EXISTS events_cidade_trgm_idx ON events USING gin (cidade gin_trgm_ops);
CREATE INDEX IF NOT EXISTS events_logradouro_trgm_idx ON events USING gin (logradouro gin_trgm_ops);
