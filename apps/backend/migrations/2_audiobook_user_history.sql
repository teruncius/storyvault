CREATE TABLE IF NOT EXISTS audiobook_user_history (
    audiobook_id BLOB NOT NULL,
    user_id BLOB NOT NULL,
    accessed_at DATETIME NOT NULL,
    PRIMARY KEY (audiobook_id, user_id)
);
