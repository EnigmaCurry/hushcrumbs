-- 001_init.sql

CREATE TABLE IF NOT EXISTS schema_version (
    version INTEGER NOT NULL
);

INSERT INTO schema_version (version)
SELECT 1
WHERE NOT EXISTS (SELECT 1 FROM schema_version);

CREATE TABLE IF NOT EXISTS context (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL UNIQUE
);

CREATE TABLE IF NOT EXISTS project (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    context_id INTEGER NOT NULL,
    FOREIGN KEY (context_id) REFERENCES context(id) ON DELETE CASCADE,
    UNIQUE(name, context_id)
);

CREATE TABLE IF NOT EXISTS instance (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    name TEXT NOT NULL,
    project_id INTEGER NOT NULL,
    FOREIGN KEY (project_id) REFERENCES project(id) ON DELETE CASCADE,
    UNIQUE(name, project_id)
);

CREATE TABLE IF NOT EXISTS env_snapshot (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    instance_id INTEGER NOT NULL,
    created_at DATETIME DEFAULT CURRENT_TIMESTAMP,
    created_by TEXT,
    label TEXT NOT NULL,
    FOREIGN KEY (instance_id) REFERENCES instance(id) ON DELETE CASCADE,
    UNIQUE(instance_id, label)
);

CREATE TABLE IF NOT EXISTS env_kv (
    id INTEGER PRIMARY KEY AUTOINCREMENT,
    snapshot_id INTEGER NOT NULL,
    key TEXT NOT NULL,
    value TEXT NOT NULL,
    comment TEXT,
    FOREIGN KEY (snapshot_id) REFERENCES env_snapshot(id) ON DELETE CASCADE
);
