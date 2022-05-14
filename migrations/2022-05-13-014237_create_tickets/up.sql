-- Your SQL goes here
CREATE TABLE tickets (
  id SERIAL PRIMARY KEY,
  description VARCHAR NOT NULL,
  customer_id SERIAL REFERENCES customers(id),
  service_id SERIAL REFERENCES services(id),
  in_charge_user_id INT,
  changed_by_user_id INT,
  status_id SERIAL REFERENCES ticket_statuses(id),
  created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
  updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE TRIGGER set_timestamp
BEFORE UPDATE ON tickets
FOR EACH ROW
EXECUTE PROCEDURE trigger_set_timestamp();