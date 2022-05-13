-- This file should undo anything in `up.sql`
DROP TRIGGER set_timestamp ON users;
DROP TABLE IF EXISTS users;
