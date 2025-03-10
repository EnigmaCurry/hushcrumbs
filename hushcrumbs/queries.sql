-- name: insert_snapshot!
INSERT INTO env_snapshot (instance_id, created_by, label)
VALUES (:instance_id, :created_by, :label)
RETURNING id;

-- name: insert_env_kv!
INSERT INTO env_kv (snapshot_id, key, value)
VALUES (:snapshot_id, :key, :value);
