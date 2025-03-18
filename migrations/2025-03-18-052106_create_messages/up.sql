-- Your SQL goes here
CREATE TABLE messages (
    id SERIAL PRIMARY KEY,
    sender VARCHAR(320) NOT NULL,  -- Sender's ID or username
    recipient VARCHAR(320) NOT NULL, -- Supports group messages
    content TEXT,  -- Message text content
    is_read BOOLEAN DEFAULT FALSE,
    sent_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);
