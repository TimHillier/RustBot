-- Keep messages.score in sync when a vote value is updated (e.g. +2 and -2 net to 0).
CREATE TRIGGER update_message_score_on_update
AFTER UPDATE OF value ON message_votes
BEGIN
    UPDATE messages
    SET score = score - OLD.value + NEW.value
    WHERE message_id = NEW.message_id;
END;
