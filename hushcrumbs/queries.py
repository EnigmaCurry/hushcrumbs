import aiosql
import aiosqlite
import importlib.resources
import logging
from .crypto import encrypt_value, decrypt_value

log = logging.getLogger(__name__)

_queries = None

def load_queries():
    global _queries
    if _queries is None:
        with importlib.resources.path("hushcrumbs", "queries.sql") as sql_path:
            _queries = aiosql.from_path(sql_path, "aiosqlite")
    return _queries

async def load_encrypted_auth_text(db_path):
    async with aiosqlite.connect(db_path) as db:
        return await _queries.get_auth_token(db)

async def insert_auth_token(db, encrypted_value):
    await _queries.insert_auth_token(db, encrypted_value=encrypted_value)

async def insert_snapshot_with_env_vars(
    db_path,
    context,
    project,
    instance,
    created_by,
    label,
    env_dict,
    env_comments=None,
    force=False,
):
    async with aiosqlite.connect(db_path) as db:
        await db.execute("BEGIN")

        # Ensure context
        context_id = await _queries.get_context(db, name=context)
        if not context_id:
            context_id = await _queries.insert_context(db, name=context)

        # Ensure project
        project_id = await _queries.get_project(db, name=project, context_id=context_id)
        if not project_id:
            project_id = await _queries.insert_project(db, name=project, context_id=context_id)

        # Ensure instance
        instance_id = await _queries.get_instance(db, name=instance, project_id=project_id)
        if not instance_id:
            instance_id = await _queries.insert_instance(db, name=instance, project_id=project_id)

        # If force is enabled, check for and delete existing snapshot with same label
        existing_snapshot_id = await _queries.get_snapshot_by_instance_and_label(
            db, instance_id=instance_id, label=label
        )
        if existing_snapshot_id:
            if not force:
                raise ValueError(f"Snapshot with label '{label}' already exists for instance '{instance}'.")
            await _queries.delete_env_kv_by_snapshot_id(db, snapshot_id=existing_snapshot_id)
            await _queries.delete_snapshot(db, snapshot_id=existing_snapshot_id)

        # Insert snapshot
        snapshot_id = await _queries.insert_snapshot(
            db, instance_id=instance_id, created_by=created_by, label=label
        )

        # Insert env vars with optional comments
        for key, value in env_dict.items():
            comment = env_comments.get(key) if env_comments else None
            encrypted_comment = encrypt_value(comment) if comment else None
            await _queries.insert_env_kv(
                db, snapshot_id=snapshot_id, key=key, value=encrypt_value(value), comment=encrypted_comment
            )

        await db.commit()
        return snapshot_id


async def get_latest_snapshots(db_path):
    async with aiosqlite.connect(db_path) as db:
        db.row_factory = aiosqlite.Row
        rows = await _queries.get_latest_snapshots(db)
        if not rows:
            log.error("No snapshots found.")
            return

        headers = rows[0].keys()
        return rows, headers

async def export_snapshot_to_file(
    db_path, context, project, instance, snapshot_label, out_path
):
    async with aiosqlite.connect(db_path) as db:
        db.row_factory = aiosqlite.Row

        snapshot_id = await _queries.get_snapshot_id(
            db,
            context=context,
            project=project,
            instance=instance,
            label=snapshot_label,
        )

        if snapshot_id is None:
            raise ValueError("Snapshot not found.")

        rows = await _queries.get_env_kv_by_snapshot_id(db, snapshot_id=snapshot_id)

        with open(out_path, "w") as f:
            for row in rows:
                if row["comment"]:
                    try:
                        comment_lines = decrypt_value(row["comment"]).splitlines()
                        for line in comment_lines:
                            f.write(f"# {line}\n")
                    except Exception:
                        log.warning(f"⚠️ Could not decrypt comment for key {row['key']}")
                f.write(f"{row['key']}={decrypt_value(row['value'])}\n\n")

        return len(rows)

