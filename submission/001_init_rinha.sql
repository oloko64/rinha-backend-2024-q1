-- drop index transactions_client_id_index;
-- drop index transactions_created_at_index;
-- drop table transactions ;
-- drop table clients;

create table clients (
  id SERIAL PRIMARY KEY,
  balance_limit INT DEFAULT 0 NOT NULL,
  balance INT DEFAULT 0 NOT NULL,
  last_nt INT DEFAULT 0 NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

create TABLE IF NOT EXISTS transactions (
  id SERIAL PRIMARY KEY,
  nt INT NOT NULL,
  valid BOOLEAN DEFAULT FALSE NOT NULL,
  client_id SERIAL REFERENCES clients(id) NOT NULL,
  amount INT NOT NULL,
  description VARCHAR(10) NOT NULL,
  type VARCHAR(1) NOT NULL,
  created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP NOT NULL
);

delete from transactions;
delete from clients;

CREATE index if not exists transactions_client_id_index ON transactions(client_id);
create INDEX if not exists transactions_created_at_index ON transactions(created_at DESC);

INSERT INTO clients (balance_limit) VALUES (100000);
INSERT INTO clients (balance_limit) VALUES (80000);
INSERT INTO clients (balance_limit) VALUES (1000000);
INSERT INTO clients (balance_limit) VALUES (10000000);
INSERT INTO clients (balance_limit) VALUES (500000);

insert into transactions (client_id, nt, amount, description, type) values (1, 0, 0, '', '');
insert into transactions (client_id, nt, amount, description, type) values (1, 1, 0, '', '');
insert into transactions (client_id, nt, amount, description, type) values (1, 2, 0, '', '');
insert into transactions (client_id, nt, amount, description, type) values (1, 3, 0, '', '');
insert into transactions (client_id, nt, amount, description, type) values (1, 4, 0, '', '');
insert into transactions (client_id, nt, amount, description, type) values (1, 5, 0, '', '');
insert into transactions (client_id, nt, amount, description, type) values (1, 6, 0, '', '');
insert into transactions (client_id, nt, amount, description, type) values (1, 7, 0, '', '');
insert into transactions (client_id, nt, amount, description, type) values (1, 8, 0, '', '');
insert into transactions (client_id, nt, amount, description, type) values (1, 9, 0, '', '');

insert into transactions (client_id, nt, amount, description, type) values (2, 0, 0, '', '');
insert into transactions (client_id, nt, amount, description, type) values (2, 1, 0, '', '');
insert into transactions (client_id, nt, amount, description, type) values (2, 2, 0, '', '');
insert into transactions (client_id, nt, amount, description, type) values (2, 3, 0, '', '');
insert into transactions (client_id, nt, amount, description, type) values (2, 4, 0, '', '');
insert into transactions (client_id, nt, amount, description, type) values (2, 5, 0, '', '');
insert into transactions (client_id, nt, amount, description, type) values (2, 6, 0, '', '');
insert into transactions (client_id, nt, amount, description, type) values (2, 7, 0, '', '');
insert into transactions (client_id, nt, amount, description, type) values (2, 8, 0, '', '');
insert into transactions (client_id, nt, amount, description, type) values (2, 9, 0, '', '');

insert into transactions (client_id, nt, amount, description, type) values (3, 0, 0, '', '');
insert into transactions (client_id, nt, amount, description, type) values (3, 1, 0, '', '');
insert into transactions (client_id, nt, amount, description, type) values (3, 2, 0, '', '');
insert into transactions (client_id, nt, amount, description, type) values (3, 3, 0, '', '');
insert into transactions (client_id, nt, amount, description, type) values (3, 4, 0, '', '');
insert into transactions (client_id, nt, amount, description, type) values (3, 5, 0, '', '');
insert into transactions (client_id, nt, amount, description, type) values (3, 6, 0, '', '');
insert into transactions (client_id, nt, amount, description, type) values (3, 7, 0, '', '');
insert into transactions (client_id, nt, amount, description, type) values (3, 8, 0, '', '');
insert into transactions (client_id, nt, amount, description, type) values (3, 9, 0, '', '');

insert into transactions (client_id, nt, amount, description, type) values (4, 0, 0, '', '');
insert into transactions (client_id, nt, amount, description, type) values (4, 1, 0, '', '');
insert into transactions (client_id, nt, amount, description, type) values (4, 2, 0, '', '');
insert into transactions (client_id, nt, amount, description, type) values (4, 3, 0, '', '');
insert into transactions (client_id, nt, amount, description, type) values (4, 4, 0, '', '');
insert into transactions (client_id, nt, amount, description, type) values (4, 5, 0, '', '');
insert into transactions (client_id, nt, amount, description, type) values (4, 6, 0, '', '');
insert into transactions (client_id, nt, amount, description, type) values (4, 7, 0, '', '');
insert into transactions (client_id, nt, amount, description, type) values (4, 8, 0, '', '');
insert into transactions (client_id, nt, amount, description, type) values (4, 9, 0, '', '');

insert into transactions (client_id, nt, amount, description, type) values (5, 0, 0, '', '');
insert into transactions (client_id, nt, amount, description, type) values (5, 1, 0, '', '');
insert into transactions (client_id, nt, amount, description, type) values (5, 2, 0, '', '');
insert into transactions (client_id, nt, amount, description, type) values (5, 3, 0, '', '');
insert into transactions (client_id, nt, amount, description, type) values (5, 4, 0, '', '');
insert into transactions (client_id, nt, amount, description, type) values (5, 5, 0, '', '');
insert into transactions (client_id, nt, amount, description, type) values (5, 6, 0, '', '');
insert into transactions (client_id, nt, amount, description, type) values (5, 7, 0, '', '');
insert into transactions (client_id, nt, amount, description, type) values (5, 8, 0, '', '');
insert into transactions (client_id, nt, amount, description, type) values (5, 9, 0, '', '');