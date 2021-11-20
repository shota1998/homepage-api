-- Your SQL goes here
CREATE TABLE users(
  id SERIAL PRIMARY KEY,
  username VARCHAR NOT NULL,
  email VARCHAR NOT NULL,
  password VARCHAR NOT NULL,
  unique_id VARCHAR NOT NULL,
  UNIQUE (email),
  UNIQUE (username)
);

-- create a master user
INSERT INTO users (username, email, password, unique_id)
VALUES ('placeholder', 'placeholder email', 'placeholder password', 'placeholder unique id');

-- update the to do table
ALTER TABLE to_do ADD user_id integer default 1
CONSTRAINT user_id REFERENCES users NOT NULL;