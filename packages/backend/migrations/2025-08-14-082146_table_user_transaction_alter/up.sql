-- Your SQL goes here
ALTER TABLE user__transaction
    RENAME COLUMN date_timestamp_ms TO date_timestamp;
