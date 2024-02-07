-- Add migration script here

CREATE TABLE transactions (
  id INTEGER PRIMARY KEY NOT NULL,
  client_id INTEGER REFERENCES clients(id) NOT NULL,
  amount INTEGER NOT NULL,
  description TEXT NOT NULL,
  type TEXT NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE INDEX transactions_client_id_index ON transactions(client_id);
