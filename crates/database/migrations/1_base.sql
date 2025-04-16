CREATE TABLE IF NOT EXISTS "user"
(
    id   INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL
);
INSERT OR IGNORE INTO "user" (id, name) VALUES (0, 'null-island');

CREATE TABLE IF NOT EXISTS "task"
(
    id                  INTEGER PRIMARY KEY AUTOINCREMENT,
    owner               INTEGER NOT NULL,
    name                TEXT NOT NULL,
    description         TEXT NOT NULL,
    template            TEXT NOT NULL,
    input_schema        JSON NOT NULL,
    output_schema       JSON NOT NULL,
    available_tool_ids  JSON NOT NULL,
    FOREIGN KEY (owner) REFERENCES "user"(id)
);

-- Create index on task.owner to improve query performance when filtering by owner
CREATE INDEX IF NOT EXISTS idx_task_owner ON task(owner);
