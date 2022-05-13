-- This file should undo anything in `up.sql`
DROP TRIGGER set_timestamp ON ticket_statuses;
DROP TABLE IF EXISTS ticket_statuses;
