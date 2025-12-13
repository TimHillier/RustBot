-- Add migration script here

-- drop the table if it exists
DROP TABLE IF EXISTS ge_aliases;

CREATE TABLE ge_aliases (
    alias TEXT PRIMARY KEY,
    item TEXT
);