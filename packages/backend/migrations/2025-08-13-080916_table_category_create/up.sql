-- Your SQL goes here
CREATE TABLE user__category
(
    id         TEXT PRIMARY KEY,
    name       TEXT NOT NULL,

    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
