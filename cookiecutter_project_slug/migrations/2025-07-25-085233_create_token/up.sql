-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE refresh_tokens (
    id UUID PRIMARY KEY NOT NULL DEFAULT uuid_generate_v4(),
    client_id UUID NOT NULL,
    client_type TEXT NOT NULL CHECK (client_type IN ('admin', 'user', 'invalid')),
    is_revoked BOOLEAN NOT NULL DEFAULT FALSE,
    token TEXT NOT NULL UNIQUE,
    expires_at TIMESTAMP WITH TIME ZONE NOT NULL,
    created_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE NOT NULL DEFAULT CURRENT_TIMESTAMP
);
