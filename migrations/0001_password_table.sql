CREATE TABLE password (
  id VARCHAR NOT NULL,
  username VARCHAR NOT NULL,
  password VARCHAR NOT NULL,
  service VARCHAR NOT NULL
);

CREATE UNIQUE INDEX password_id_idx ON password (id);
