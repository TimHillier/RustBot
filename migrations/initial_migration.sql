CREATE TABLE IF NOT EXISTS user (
    user_id TEXT PRIMARY KEY,
    user_name TEXT  NOT NULL,
    score INTEGER,
    plus_two_given INTEGER,
    plus_two_received INTEGER,
    minus_two_given INTEGER,
    minus_two_received INTEGER
);

CREATE TABLE IF NOT EXISTS  judgedPosts (
    message_id TEXT PRIMARY KEY,
    message_owner TEXT,
    command_caller TEXT,
    result TEXT
);
