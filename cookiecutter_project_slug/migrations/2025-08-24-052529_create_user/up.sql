-- Your SQL goes here
CREATE TABLE users (
    id UUID PRIMARY KEY DEFAULT gen_random_uuid(),

    -- Authentication
    email VARCHAR(255) UNIQUE NOT NULL,
    phone_number VARCHAR(20) UNIQUE,
    password_hash VARCHAR(255) NOT NULL,

    -- Identity
    first_name VARCHAR(100) NOT NULL,
    last_name VARCHAR(100),
    username VARCHAR(100) UNIQUE,

    -- Roles & Permissions
    role VARCHAR(50) DEFAULT 'user', -- e.g., admin, teacher, student, customer, seller
    is_active BOOLEAN DEFAULT TRUE,
    is_verified BOOLEAN DEFAULT FALSE,

    -- Metadata
    profile_image TEXT,
    date_of_birth DATE,
    gender VARCHAR(20),

    -- Audit & Security
    last_login TIMESTAMP WITH TIME ZONE,
    created_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP,
    updated_at TIMESTAMP WITH TIME ZONE DEFAULT CURRENT_TIMESTAMP
);


CREATE INDEX idx_users_role ON users(role);

-- Active users filter (frequently used in queries)
CREATE INDEX idx_users_is_active ON users(is_active);

-- Verified users filter
CREATE INDEX idx_users_is_verified ON users(is_verified);

-- Search by name (first + last name separately for better performance)
CREATE INDEX idx_users_first_name ON users(first_name);
CREATE INDEX idx_users_last_name ON users(last_name);

-- Date-based queries (e.g., birthdays, recently created accounts)
CREATE INDEX idx_users_date_of_birth ON users(date_of_birth);
CREATE INDEX idx_users_created_at ON users(created_at);

-- Login activity tracking
CREATE INDEX idx_users_last_login ON users(last_login);
