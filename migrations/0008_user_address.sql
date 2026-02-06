-- Create user_address table (1:1 with users)

CREATE TABLE IF NOT EXISTS user_address (
  id uuid PRIMARY KEY DEFAULT uuid_generate_v4 (),
  user_id uuid NOT NULL UNIQUE REFERENCES users (id) ON DELETE CASCADE,

  -- Brazilian postal code (CEP). Accept with or without hyphen.
  cep text NOT NULL CHECK (cep ~ '^[0-9]{5}-?[0-9]{3}$'),

  logradouro text NOT NULL,
  numero text NOT NULL,
  complemento text,
  bairro text,
  cidade text NOT NULL,
  estado char(2) NOT NULL CHECK (char_length(estado) = 2),

  created_at timestamptz NOT NULL DEFAULT now ()
);

CREATE INDEX IF NOT EXISTS user_address_user_id_idx ON user_address (user_id);
