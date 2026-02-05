-- Rename cpf -> gov_identification (accepts CPF or CNPJ)

ALTER TABLE users
  RENAME COLUMN cpf TO gov_identification;

-- CPF = 11 digits, CNPJ = 14 digits.
-- Allow NULL. Enforce digits-only and length.
ALTER TABLE users
  ADD CONSTRAINT users_gov_identification_format_chk
  CHECK (
    gov_identification IS NULL
    OR (
      gov_identification ~ '^[0-9]+$'
      AND length(gov_identification) IN (11, 14)
    )
  );
