-- Your SQL goes here
CREATE TABLE services (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
  customer_id SERIAL REFERENCES customers(id),
  description VARCHAR NOT NULL,
  cost NUMERIC NOT NULL,
  type_id SERIAL REFERENCES service_types(id),
  active  boolean DEFAULT 'true',
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER set_timestamp
BEFORE UPDATE ON services
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();
