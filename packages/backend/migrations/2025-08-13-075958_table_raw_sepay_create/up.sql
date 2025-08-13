-- Your SQL goes here
CREATE TABLE raw__sepay
(
    gateway         TEXT,
    transactionDate TEXT,
    accountNumber   TEXT,
    subAccount      TEXT,
    code            TEXT,
    content         TEXT,
    transferType    TEXT,
    description     TEXT,
    transferAmount  INTEGER,
    referenceCode   TEXT,
    accumulated     INTEGER,
    id              INTEGER PRIMARY KEY
);
