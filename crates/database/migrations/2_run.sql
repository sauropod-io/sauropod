CREATE TABLE IF NOT EXISTS "task_run"
(
    run_id              INTEGER NOT NULL PRIMARY KEY AUTOINCREMENT,
    owner_id            INTEGER NOT NULL,
    -- The status is either "running", "failed", or "succeeded"
    status              TEXT NOT NULL,
    -- When the run started executing
    start_time          TIMESTAMP,
    -- When the run finished executing
    end_time            TIMESTAMP

);

-- Create index on task_run.owner_id
CREATE INDEX IF NOT EXISTS idx_task_run_owner_id ON task_run(owner_id);

CREATE TABLE IF NOT EXISTS "task_run_steps"
(
    -- Each step has a unique ID
    step_id             INTEGER PRIMARY KEY AUTOINCREMENT,
    -- Multiple steps will share a common run ID
    run_id              INTEGER NOT NULL,
    owner_id            INTEGER NOT NULL,
    -- Steps triggered by other steps have parents
    parent_step_id      INTEGER,
    inputs              JSON NOT NULL,
    outputs             JSON NOT NULL,
    -- If the step is a task then this not be null
    task_id             INTEGER,
    -- If the step is a tool then this not be null
    tool_id             TEXT,
    -- An error message if an error occurs
    error               TEXT,
    -- When the step started executing
    start_time          TIMESTAMP,
    -- When the step finished executing
    end_time            TIMESTAMP,
    FOREIGN KEY (owner_id) REFERENCES "user"(user_id),
    FOREIGN KEY (task_id)  REFERENCES "task"(id),
    FOREIGN KEY (run_id)   REFERENCES "task_run"(run_id)
);

-- Create index on task_run_steps.run_id
CREATE INDEX IF NOT EXISTS idx_task_run_steps_run_id ON task_run_steps(run_id);
-- Create index on task_run_steps.owner_id
CREATE INDEX IF NOT EXISTS idx_task_run_steps_owner_id ON task_run_steps(owner_id);
-- Create index on task_run_steps.task_id
CREATE INDEX IF NOT EXISTS idx_task_run_steps_task_id ON task_run_steps(task_id);
