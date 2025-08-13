-- Your SQL goes here
CREATE TABLE user__transaction
(
    id                TEXT PRIMARY KEY,
    date_timestamp_ms INTEGER NOT NULL,
    description       TEXT    NOT NULL,
    amount            REAL    NOT NULL,
    category_id       TEXT,
    source_id         TEXT NOT NULL,
    -- either "user" or "sepay"

    created_at        TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
