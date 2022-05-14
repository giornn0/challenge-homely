-- Your SQL goes here
CREATE TABLE service_types (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  active  boolean DEFAULT 'true',
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER set_timestamp
BEFORE UPDATE ON service_types
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

INSERT INTO service_types (name) VALUES ('Limpieza');
INSERT INTO service_types (name) VALUES ('Mantenimiento');
INSERT INTO service_types (name) VALUES ('Cuidador');