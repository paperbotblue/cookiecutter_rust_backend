-- This file should undo anything in `up.sql`
DROP TABLE todos;

DROP INDEX IF EXISTS idx_todos_name;
