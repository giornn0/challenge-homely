-- Your SQL goes here-- Your SQL goes here
CREATE TABLE ticket_statuses (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  active  boolean DEFAULT 'true',
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER set_timestamp
BEFORE UPDATE ON ticket_statuses
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();

INSERT INTO ticket_statuses (name) VALUES ('Abierto');
INSERT INTO ticket_statuses (name) VALUES ('En progreso');
INSERT INTO ticket_statuses (name) VALUES ('Resuelto');
INSERT INTO ticket_statuses (name) VALUES ('Necesita reembolso');