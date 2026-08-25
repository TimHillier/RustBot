CREATE TABLE messages (
    message_id TEXT PRIMARY KEY,
    user_id TEXT NOT NULL,
    message_content TEXT NOT NULL,
    timestamp TEXT NOT NULL,
    channel_id TEXT NOT NULL,
    guild_id TEXT,
    author_name TEXT NOT NULL,
    author_global_name TEXT,
    is_bot INTEGER NOT NULL,
    kind TEXT NOT NULL,
    referenced_message_id TEXT,
    edited_timestamp TEXT
);
