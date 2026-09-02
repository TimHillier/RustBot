-- Add last bomb time to user table so 
-- we know when they last stepped on a bomb.
ALTER TABLE user
    ADD COLUMN last_bomb_time DATETIME DEFAULT NULL;