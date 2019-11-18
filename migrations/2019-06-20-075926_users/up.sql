--migrations/TIMESTAMP_users/up.sql
CREATE TABLE users (
  email VARCHAR(100) NOT NULL PRIMARY KEY,
  hash_pass VARCHAR(128) NOT NULL, -- TODO: maybe too long for argon hash
  created_at TIMESTAMP NOT NULL
);