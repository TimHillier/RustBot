CREATE TABLE message_attachments (
    attachment_id TEXT PRIMARY KEY,
    message_id TEXT NOT NULL,
    user_id TEXT NOT NULL,
    author_name TEXT NOT NULL,
    filename TEXT NOT NULL,
    content_type TEXT,
    size INTEGER NOT NULL,
    local_path TEXT,
    FOREIGN KEY (message_id) REFERENCES messages(message_id)
);

CREATE INDEX idx_message_attachments_message_id ON message_attachments(message_id);
