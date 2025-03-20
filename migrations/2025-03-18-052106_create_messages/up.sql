-- Your SQL goes here
-- CREATE TABLE users (
--     id SERIAL PRIMARY KEY,
--     username VARCHAR(320) UNIQUE NOT NULL,
--     password VARCHAR(320) NOT NULL
-- );
CREATE TABLE
    messages (
        id SERIAL PRIMARY KEY,
        sender VARCHAR(320) NOT NULL,
        recipient VARCHAR(320) NOT NULL,
        content TEXT,
        is_read BOOLEAN DEFAULT FALSE,
        sent_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
    );

-- CREATE TABLE conversations (
--     id SERIAL PRIMARY KEY,
-- );
-- CREATE TABLE conversation_participants (
--     id SERIAL PRIMARY KEY,
--     user_id INT REFERENCES users(id),
--     conversation_id INT REFERENCES conversations(id)
--     type enum('group', 'private') DEFAULT 'private'
--     name VARCHAR(320) -- Only for group conversations
-- );
-- CREATE TABLE conversation_messages (
--     conversation_id INT REFERENCES conversations(id),
-- );