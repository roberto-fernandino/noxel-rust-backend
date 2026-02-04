-- Change gov_identification to an integer type
-- Note: CNPJ has 14 digits, so we must use BIGINT (int4 is too small).

-- Drop old text-format constraint (if present)
ALTER TABLE users
  DROP CONSTRAINT IF EXISTS users_gov_identification_format_chk;

-- Convert column type
ALTER TABLE users
  ALTER COLUMN gov_identification TYPE bigint
  USING NULLIF(gov_identification::text, '')::bigint;

-- Enforce 11-digit (CPF) or 14-digit (CNPJ) ranges
ALTER TABLE users
  ADD CONSTRAINT users_gov_identification_digits_chk
  CHECK (
    gov_identification IS NULL
    OR (
      (gov_identification BETWEEN 10000000000 AND 99999999999)
      OR (gov_identification BETWEEN 10000000000000 AND 99999999999999)
    )
  );
