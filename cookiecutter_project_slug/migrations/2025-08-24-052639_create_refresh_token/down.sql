-- This file should undo anything in `up.sql`
DROP INDEX IF EXISTS idx_refresh_tokens_token;
DROP INDEX IF EXISTS idx_refresh_tokens_user_id;
DROP INDEX IF EXISTS idx_refresh_tokens_family_id;

-- Drop table (with constraints automatically removed)
DROP TABLE IF EXISTS refresh_tokens CASCADE;
