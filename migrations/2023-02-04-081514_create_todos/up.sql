-- Your SQL goes here
CREATE TABLE todos (
  id SERIAL PRIMARY KEY,
  text VARCHAR(255) NOT NULL,
--   body TEXT NOT NULL,
  done BOOLEAN NOT NULL DEFAULT FALSE
)
