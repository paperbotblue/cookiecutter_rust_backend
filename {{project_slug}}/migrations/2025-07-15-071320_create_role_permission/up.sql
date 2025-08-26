-- Your SQL goes here

CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
-- Join table
CREATE TABLE role_permissions (
  role_id UUID NOT NULL,
  permission_id UUID NOT NULL,
  description TEXT NOT NULL,
  PRIMARY KEY (role_id, permission_id),
  FOREIGN KEY (role_id) REFERENCES roles(id) ON DELETE CASCADE,
  FOREIGN KEY (permission_id) REFERENCES permissions(id) ON DELETE CASCADE
);

-- For permission check flow: permission-based lookups
CREATE INDEX idx_rp_permission_id ON role_permissions(permission_id);

-- Optional reverse direction: role-based filtering
CREATE INDEX idx_rp_role_id ON role_permissions(role_id);
