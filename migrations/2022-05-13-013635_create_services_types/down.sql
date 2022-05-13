-- This file should undo anything in `up.sql`
DROP TRIGGER set_timestamp ON service_types;
DROP TABLE IF EXISTS service_types;
