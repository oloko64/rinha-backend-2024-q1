-- Add migration script here

CREATE TABLE transactions (
  id SERIAL PRIMARY KEY,
  client_id INTEGER REFERENCES clients(id),
  amount INTEGER,
  description TEXT,
  type TEXT,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);