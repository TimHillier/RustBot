CREATE TABLE message_attachments (
    attachment_id TEXT PRIMARY KEY,
    message_id TEXT NOT NULL,
    filename TEXT NOT NULL,
    content_type TEXT,
    size INTEGER NOT NULL,
    url TEXT NOT NULL,
    proxy_url TEXT NOT NULL,
    width INTEGER,
    height INTEGER,
    local_path TEXT,
    FOREIGN KEY (message_id) REFERENCES messages(message_id)
);

CREATE INDEX idx_message_attachments_message_id ON message_attachments(message_id);
