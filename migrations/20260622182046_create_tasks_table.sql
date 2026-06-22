-- Add migration script here
-- migrations/20240101000000_create_tasks_table.sql
-- Create the task status enum type
CREATE TYPE task_status AS ENUM ('pending', 'inprogress', 'completed');

-- Create the tasks table with all required fields
CREATE TABLE tasks (
    id UUID PRIMARY KEY,
    title VARCHAR(255) NOT NULL,
    description TEXT,
    status task_status NOT NULL DEFAULT 'pending',
    created_at TIMESTAMPTZ NOT NULL DEFAULT NOW(),
    updated_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

-- Index on status for filtering
CREATE INDEX idx_tasks_status ON tasks(status);

-- Index on created_at for sorting
CREATE INDEX idx_tasks_created_at ON tasks(created_at DESC);