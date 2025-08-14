-- This file should undo anything in `up.sql`
ALTER TABLE user__transaction
    RENAME COLUMN date_timestamp TO date_timestamp_ms;
