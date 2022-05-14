-- Your SQL goes here
CREATE TABLE services (
  id SERIAL PRIMARY KEY,
  name VARCHAR NOT NULL,
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

INSERT INTO services (name,description,cost,type_id) VALUES ('Test','descrip test 1','4563.56',1);
INSERT INTO services (name,description,cost,type_id) VALUES ('Test2','descrip test 2','1234.56',3);
INSERT INTO services (name,description,cost,type_id) VALUES ('Test3','descrip test 3','121.56',2);
INSERT INTO services (name,description,cost,type_id) VALUES ('Test4','descrip test 4','12412.56',1);