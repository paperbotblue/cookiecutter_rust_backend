-- This file should undo anything in `up.sql`

-- Drop indexes if they exist
DROP INDEX IF EXISTS idx_users_role;
DROP INDEX IF EXISTS idx_users_is_active;
DROP INDEX IF EXISTS idx_users_is_verified;
DROP INDEX IF EXISTS idx_users_first_name;
DROP INDEX IF EXISTS idx_users_last_name;
DROP INDEX IF EXISTS idx_users_date_of_birth;
DROP INDEX IF EXISTS idx_users_created_at;
DROP INDEX IF EXISTS idx_users_last_login;

-- Finally, drop the table
DROP TABLE IF EXISTS users CASCADE;
