CREATE TABLE IF NOT EXISTS "user"
(
    user_id   INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL
);
INSERT OR IGNORE INTO "user" (user_id, name) VALUES (0, 'null-island');

CREATE TABLE IF NOT EXISTS "task"
(
    task_id             INTEGER PRIMARY KEY AUTOINCREMENT,
    owner_id            INTEGER NOT NULL,
    name                TEXT NOT NULL,
    description         TEXT NOT NULL,
    template            TEXT NOT NULL,
    input_schema        JSON NOT NULL,
    output_schema       JSON NOT NULL,
    available_tool_ids  JSON NOT NULL,
    FOREIGN KEY (owner_id) REFERENCES "user"(user_id)
);

-- Create index on task.owner to improve query performance when filtering by owner
CREATE INDEX IF NOT EXISTS idx_task_owner ON task(owner_id);
