-- Context
-- name: get_context$
SELECT id FROM context WHERE name = :name;

-- name: insert_context$
INSERT INTO context (name) VALUES (:name)
RETURNING id;

-- Project
-- name: get_project$
SELECT id FROM project WHERE name = :name AND context_id = :context_id;

-- name: insert_project$
INSERT INTO project (name, context_id) VALUES (:name, :context_id)
RETURNING id;

-- name: get_instance$
SELECT id FROM instance WHERE name = :name AND project_id = :project_id;

-- name: insert_instance$
INSERT INTO instance (name, project_id) VALUES (:name, :project_id)
RETURNING id;

-- name: insert_snapshot$
INSERT INTO env_snapshot (instance_id, created_by, label)
VALUES (:instance_id, :created_by, :label)
RETURNING id;

-- Env Vars
-- name: insert_env_kv!
INSERT INTO env_kv (snapshot_id, key, value)
VALUES (:snapshot_id, :key, :value);

