CREATE TABLE users (
    id BIGINT NOT NULL PRIMARY KEY,
    nickname TEXT
);

CREATE TABLE phrases (
    id SERIAL PRIMARY KEY,
    user_id BIGINT NOT NULL,
    emoji TEXT NOT NULL,
    text TEXT NOT NULL,
    bot_text TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE
);
