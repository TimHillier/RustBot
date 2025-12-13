-- Add migration script here

CREATE TABLE ge_aliases (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    alias TEXT,
    item TEXT,
    UNIQUE(alias, item)
);

