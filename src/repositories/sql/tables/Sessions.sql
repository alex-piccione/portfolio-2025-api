CREATE TABLE Sessions (
    id SERIAL PRIMARY KEY,
    user_id CHAR(36) NOT NULL,
    access_token VARCHAR(64) NOT NULL,
    access_token_expires_at TIMESTAMPTZ NOT NULL,
    refresh_token VARCHAR(64) NOT NULL,
    refresh_token_expires_at TIMESTAMPTZ NOT NULL,
    created_at TIMESTAMPTZ NOT NULL,
    creation_ip_address VARCHAR(45) NOT NULL,
    creation_user_agent TEXT NOT NULL
);

-- Foreign key to users
ALTER TABLE Sessions
ADD CONSTRAINT sessions_user_fk FOREIGN KEY (user_id) REFERENCES users(id) ON DELETE CASCADE;

-- Index for quick lookups by user_id
CREATE INDEX sessions_user_id_idx ON Sessions (user_id);

-- Unique constraints
ALTER TABLE Sessions
ADD CONSTRAINT sessions_access_token_unique UNIQUE (access_token);

ALTER TABLE Sessions
ADD CONSTRAINT sessions_refresh_token_unique UNIQUE (refresh_token);
