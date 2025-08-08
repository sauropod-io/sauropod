PRAGMA foreign_keys = ON;

-- Users
CREATE TABLE IF NOT EXISTS "user"
(
    user_id   INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL
);
INSERT OR IGNORE INTO "user" (user_id, name) VALUES (0, 'anonymous');

-- API keys issued to users
CREATE TABLE api_keys (
  id          INTEGER PRIMARY KEY AUTOINCREMENT,
  user_id     INTEGER NOT NULL,
  key         TEXT    NOT NULL UNIQUE,
  created_at  TEXT    NOT NULL DEFAULT (datetime('now')),
  revoked     INTEGER NOT NULL DEFAULT 0,
  FOREIGN KEY (user_id) REFERENCES "user"(user_id) ON DELETE CASCADE
);
CREATE INDEX idx_api_keys_user_id ON api_keys(user_id);

-- Response data
CREATE TABLE IF NOT EXISTS "response"
(
    response_id        TEXT PRIMARY KEY,
    created_at         TEXT NOT NULL DEFAULT (datetime('now')),
    user_id            INTEGER NOT NULL, -- The user that created this response
    parent_response_id TEXT REFERENCES "response"(response_id) ON DELETE CASCADE,
    response_request TEXT NOT NULL,
    response_output TEXT NOT NULL,
    FOREIGN KEY (user_id) REFERENCES "user"(user_id)
);

-- Index to query responses by user
CREATE INDEX IF NOT EXISTS idx_response_user_id ON response(user_id);
