-- Containers metadata
CREATE TABLE IF NOT EXISTS containers (
    id TEXT PRIMARY KEY,
    name TEXT NOT NULL,
    version TEXT,
    status TEXT NOT NULL DEFAULT 'draft',
    created_at TIMESTAMP DEFAULT CURRENT_TIMESTAMP
);

CREATE INDEX IF NOT EXISTS idx_containers_status ON containers (status);
CREATE INDEX IF NOT EXISTS idx_containers_name ON containers (name);
