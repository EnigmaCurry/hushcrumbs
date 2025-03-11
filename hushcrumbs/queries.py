import aiosql
import aiosqlite
import importlib.resources
from tabulate import tabulate

with importlib.resources.path("hushcrumbs", "queries.sql") as sql_path:
    queries = aiosql.from_path(sql_path, "aiosqlite")

async def insert_snapshot_with_env_vars(db_path, context, project, instance, created_by, label, env_dict):
    async with aiosqlite.connect(db_path) as db:
        await db.execute("BEGIN")

        # Ensure context
        context_id = await queries.get_context(db, name=context)
        if not context_id:
            context_id = await queries.insert_context(db, name=context)

        # Ensure project
        project_id = await queries.get_project(db, name=project, context_id=context_id)
        if not project_id:
            project_id = await queries.insert_project(db, name=project, context_id=context_id)

        # Ensure instance
        instance_id = await queries.get_instance(db, name=instance, project_id=project_id)
        if not instance_id:
            instance_id = await queries.insert_instance(db, name=instance, project_id=project_id)

        # Insert snapshot
        snapshot_id = await queries.insert_snapshot(db, instance_id=instance_id, created_by=created_by, label=label)

        # Insert env vars
        for key, value in env_dict.items():
            await queries.insert_env_kv(db, snapshot_id=snapshot_id, key=key, value=value)

        await db.commit()
        return snapshot_id

async def list_latest_snapshots(db_path):
    async with aiosqlite.connect(db_path) as db:
        db.row_factory = aiosqlite.Row
        rows = await queries.get_latest_snapshots(db)
        if not rows:
            print("No snapshots found.")
            return

        headers = rows[0].keys()
        print(tabulate(rows, headers=headers, tablefmt="plain"))

async def export_snapshot_to_file(db_path, context, project, instance, snapshot_label, out_path):
    async with aiosqlite.connect(db_path) as db:
        db.row_factory = aiosqlite.Row

        snapshot_id = await queries.get_snapshot_id(
            db,
            context=context,
            project=project,
            instance=instance,
            label=snapshot_label
        )

        if snapshot_id is None:
            raise ValueError("Snapshot not found.")

        rows = await queries.get_env_kv_by_snapshot_id(db, snapshot_id=snapshot_id)

        with open(out_path, "w") as f:
            for row in rows:
                f.write(f"{row['key']}={row['value']}\n")

        return len(rows)
