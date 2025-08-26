-- This file should undo anything in `up.sql`

-- This file should undo anything in `up.sql`
DROP INDEX IF EXISTS idx_rp_role_id;
DROP INDEX IF EXISTS idx_rp_permission_id;

-- Drop the join table
DROP TABLE IF EXISTS role_permissions;
