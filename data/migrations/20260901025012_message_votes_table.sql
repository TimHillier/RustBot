-- Create a message votes table to store the votes for each message.
CREATE TABLE message_votes (
    message_id TEXT NOT NULL,
    voter_id   TEXT NOT NULL,
    value      INTEGER NOT NULL,  -- +2 or -2
    PRIMARY KEY (message_id, voter_id),
    FOREIGN KEY (message_id) REFERENCES messages(message_id)
);

-- Keep messages.score in sync when a vote is added or removed.
CREATE TRIGGER update_message_score_on_insert
AFTER INSERT ON message_votes
BEGIN
    UPDATE messages
    SET score = score + NEW.value
    WHERE message_id = NEW.message_id;
END;

CREATE TRIGGER update_message_score_on_delete
AFTER DELETE ON message_votes
BEGIN
    UPDATE messages
    SET score = score - OLD.value
    WHERE message_id = OLD.message_id;
END;