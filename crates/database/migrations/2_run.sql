CREATE TABLE IF NOT EXISTS "task_run"
(
    run_id              INTEGER PRIMARY KEY AUTOINCREMENT,
    owner_id            INTEGER NOT NULL,
    step_id             INTEGER,
    parent_step_id      INTEGER,
    inputs              JSON NOT NULL,
    outputs             JSON NOT NULL,
    -- If the step is a task then this not be null
    task_id             INTEGER,
    -- If the step is a tool then this not be null
    tool_id             TEXT,
    -- An error message if an error occurs
    error               TEXT,
    FOREIGN KEY (owner_id) REFERENCES "user"(user_id),
    FOREIGN KEY (task_id)  REFERENCES "task"(id)
);

-- Create index on task_run.owner_id
CREATE INDEX IF NOT EXISTS idx_task_run_owner_id ON task_run(owner_id);
-- Create index on task_run.task_id
CREATE INDEX IF NOT EXISTS idx_task_run_task_id ON task_run(task_id);
