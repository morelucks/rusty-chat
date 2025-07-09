-- Create users table
CREATE TABLE IF NOT EXISTS users (
    id UUID PRIMARY KEY,
    username VARCHAR(255) UNIQUE NOT NULL,
    email VARCHAR(255) UNIQUE NOT NULL,
    wallet_address VARCHAR(255) UNIQUE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL
);

-- sample
-- NB: add both receiver and sender id(Team Lead)
-- CREATE TABLE IF NOT EXISTS messages (
--     id UUID PRIMARY KEY,
--     user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
--     content TEXT NOT NULL,
--     created_at TIMESTAMP WITH TIME ZONE NOT NULL
-- );

CREATE INDEX IF NOT EXISTS idx_users_username ON users(username);
CREATE INDEX IF NOT EXISTS idx_users_email ON users(email);
-- CREATE INDEX idx_messages_user_id ON messages(user_id);
-- CREATE INDEX idx_messages_created_at ON messages(created_at);