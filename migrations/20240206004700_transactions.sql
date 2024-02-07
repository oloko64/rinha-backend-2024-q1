-- Add migration script here

CREATE TABLE transactions (
  id BIGSERIAL PRIMARY KEY,
  client_id BIGINT REFERENCES clients(id) NOT NULL,
  amount BIGINT NOT NULL,
  description TEXT NOT NULL,
  type TEXT NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE INDEX transactions_client_id_index ON transactions(client_id);
