-- This file should undo anything in `up.sql`
DROP TRIGGER set_timestamp ON services;
DROP TABLE IF EXISTS services;
