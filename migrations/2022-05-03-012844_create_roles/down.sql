-- This file should undo anything in `up.sql`
DROP TRIGGER set_timestamp ON roles;
DROP TABLE IF EXISTS roles;
