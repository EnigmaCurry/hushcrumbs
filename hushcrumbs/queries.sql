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
INSERT INTO env_kv (snapshot_id, key, value, comment)
VALUES (:snapshot_id, :key, :value, :comment);

-- name: get_latest_snapshots
SELECT
    context.name AS context,
    project.name AS project,
    instance.name AS instance,
    env_snapshot.label AS label,
    env_snapshot.created_at AS created_at
FROM env_snapshot
JOIN instance ON env_snapshot.instance_id = instance.id
JOIN project ON instance.project_id = project.id
JOIN context ON project.context_id = context.id
WHERE env_snapshot.id IN (
    SELECT id FROM (
        SELECT id,
               instance_id,
               MAX(created_at) OVER (PARTITION BY instance_id) AS max_created
        FROM env_snapshot
    ) WHERE created_at = max_created
)
ORDER BY context.name, project.name, instance.name;

-- name: get_snapshot_id$
SELECT env_snapshot.id
FROM env_snapshot
JOIN instance ON env_snapshot.instance_id = instance.id
JOIN project ON instance.project_id = project.id
JOIN context ON project.context_id = context.id
WHERE context.name = :context
  AND project.name = :project
  AND instance.name = :instance
  AND (:label IS NULL OR env_snapshot.label = :label)
ORDER BY env_snapshot.created_at DESC
LIMIT 1;

-- name: get_env_kv_by_snapshot_id
SELECT key, value, comment
FROM env_kv
WHERE snapshot_id = :snapshot_id
ORDER BY key;

-- name: get_snapshot_by_instance_and_label$
SELECT id FROM env_snapshot
WHERE instance_id = :instance_id AND label = :label;

-- name: delete_env_kv_by_snapshot_id!
DELETE FROM env_kv
WHERE snapshot_id = :snapshot_id;

-- name: delete_snapshot!
DELETE FROM env_snapshot
WHERE id = :snapshot_id;
