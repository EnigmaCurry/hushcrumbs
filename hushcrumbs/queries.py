import aiosql
import aiosqlite

# Load queries from file
queries = aiosql.from_path("queries.sql", "aiosqlite")


async def insert_snapshot_with_env_vars(
    db_path, instance_id, created_by, label, env_dict
):
    async with aiosqlite.connect(db_path) as db:
        async with db.execute("BEGIN"):
            # 1. Insert snapshot
            snapshot_id = await queries.insert_snapshot(
                db, instance_id=instance_id, created_by=created_by, label=label
            )

            # 2. Insert each key-value pair
            for key, value in env_dict.items():
                await queries.insert_env_kv(
                    db, snapshot_id=snapshot_id, key=key, value=value
                )

            await db.commit()
            print(f"Snapshot '{label}' inserted for instance {instance_id}.")

