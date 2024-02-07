-- Add migration script here

CREATE TABLE clients (
  id BIGSERIAL PRIMARY KEY,
  balance_limit BIGINT DEFAULT 0 NOT NULL,
  balance BIGINT DEFAULT 0 NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

INSERT INTO clients (balance_limit, balance) VALUES (100000, 0);
INSERT INTO clients (balance_limit, balance) VALUES (80000, 0);
INSERT INTO clients (balance_limit, balance) VALUES (1000000, 0);
INSERT INTO clients (balance_limit, balance) VALUES (10000000, 0);
INSERT INTO clients (balance_limit, balance) VALUES (500000, 0);
