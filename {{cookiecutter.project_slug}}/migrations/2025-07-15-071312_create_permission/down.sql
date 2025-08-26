-- This file should undo anything in `up.sql`
DROP INDEX IF EXISTS idx_permissions_name;

-- Drop table
DROP TABLE IF EXISTS permissions;
