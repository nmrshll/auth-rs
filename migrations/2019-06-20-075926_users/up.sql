--migrations/TIMESTAMP_users/up.sql
CREATE TABLE users (
  id BIGSERIAL PRIMARY KEY,
  created_at TIMESTAMP NOT NULL DEFAULT NOW(),
  --
  email VARCHAR(100) NOT NULL,
  hash_pass VARCHAR(128) NOT NULL -- TODO maybe too long for argon hash
);