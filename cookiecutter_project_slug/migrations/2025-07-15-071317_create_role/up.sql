-- Your SQL goes here
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";


  CREATE TABLE roles (
  id UUID PRIMARY KEY DEFAULT uuid_generate_v4(),
  name VARCHAR(50) UNIQUE NOT NULL,
  description TEXT NOT NULL
);

CREATE UNIQUE INDEX idx_roles_name ON roles(name);
