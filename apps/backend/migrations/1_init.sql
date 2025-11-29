CREATE TABLE IF NOT EXISTS events (
    event_id BLOB PRIMARY KEY,
    topic TEXT NOT NULL,
    payload TEXT NOT NULL,
    created_at DATETIME NOT NULL
);

CREATE TABLE IF NOT EXISTS audiobook_user_progress (
    audiobook_id BLOB NOT NULL,
    user_id BLOB NOT NULL,
    last_position_seconds INTEGER NOT NULL,
    PRIMARY KEY (audiobook_id, user_id)
);

CREATE TABLE IF NOT EXISTS users (
    id BLOB PRIMARY KEY,
    first_name TEXT NOT NULL,
    last_name TEXT NOT NULL,
    email TEXT NOT NULL UNIQUE,
    password_hash TEXT NOT NULL,
    avatar_url TEXT
);