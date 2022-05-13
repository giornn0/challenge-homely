-- This file should undo anything in `up.sql`
DROP TRIGGER set_timestamp ON customers;
DROP TABLE IF EXISTS customers;
