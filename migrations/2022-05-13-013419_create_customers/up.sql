-- Your SQL goes here
CREATE TABLE customers (
  id SERIAL PRIMARY KEY,
  user_id SERIAL REFERENCES users(id),
  profile VARCHAR NOT NULL,
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER set_timestamp
BEFORE UPDATE ON customers
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();