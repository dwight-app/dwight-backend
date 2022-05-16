-- Your SQL goes here
CREATE TABLE tasks (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  time TIMESTAMP NOT NULL,
  severity INTEGER NOT NULL,
  priority INTEGER NOT NULL,
  completed INTEGER NOT NULL
)
