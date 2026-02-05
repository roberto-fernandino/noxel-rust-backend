-- Add gov_identification (accepts CPF or CNPJ). Added as text here; 0004 converts to bigint.

-- Nullable so existing rows are valid; app must require it for new signups.
ALTER TABLE users
  ADD COLUMN gov_identification text;

-- CPF = 11 digits, CNPJ = 14 digits. Enforce digits-only and length when set.
ALTER TABLE users
  ADD CONSTRAINT users_gov_identification_format_chk
  CHECK (
    gov_identification IS NULL
    OR (
      gov_identification ~ '^[0-9]+$'
      AND length(gov_identification) IN (11, 14)
    )
  );
