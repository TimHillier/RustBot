-- Keep per-message vote totals for the message_votes triggers.
ALTER TABLE messages
    ADD COLUMN score INTEGER NOT NULL DEFAULT 0;
