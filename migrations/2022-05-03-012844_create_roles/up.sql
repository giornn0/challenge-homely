-- Your SQL goes here
CREATE TABLE roles(
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  active boolean DEFAULT 'true',
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);
CREATE TRIGGER set_timestamp
BEFORE UPDATE ON roles
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

INSERT INTO roles (name) VALUES ('admin');
INSERT INTO roles (name) VALUES ('ops');
INSERT INTO roles (name) VALUES ('marketing');
INSERT INTO roles (name) VALUES ('user');