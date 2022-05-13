-- This file should undo anything in `up.sql`
DROP TRIGGER set_timestamp ON tickets;
DROP TABLE IF EXISTS tickets;
