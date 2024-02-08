-- Add migration script here

CREATE TABLE clients (
  id BIGSERIAL PRIMARY KEY,
  balance_limit BIGINT DEFAULT 0 NOT NULL,
  balance BIGINT DEFAULT 0 NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE TABLE transactions (
  id BIGSERIAL PRIMARY KEY,
  client_id BIGSERIAL REFERENCES clients(id) NOT NULL,
  amount BIGINT NOT NULL,
  description TEXT NOT NULL,
  type TEXT NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

CREATE INDEX transactions_client_id_index ON transactions(client_id);
CREATE INDEX transactions_created_at_index ON transactions(created_at);

INSERT INTO clients (balance_limit, balance) VALUES (100000, 0);
INSERT INTO clients (balance_limit, balance) VALUES (80000, 0);
INSERT INTO clients (balance_limit, balance) VALUES (1000000, 0);
INSERT INTO clients (balance_limit, balance) VALUES (10000000, 0);
INSERT INTO clients (balance_limit, balance) VALUES (500000, 0);
