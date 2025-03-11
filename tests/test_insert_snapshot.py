import pytest
import aiosqlite
from hushcrumbs.queries import insert_snapshot_with_env_vars
from hushcrumbs.db import apply_migrations

@pytest.mark.asyncio
async def test_insert_snapshot_with_env_vars(tmp_path):
    # Setup temporary DB path
    db_path = tmp_path / "test.sqlite"

    # Run your real migrations
    await apply_migrations(db_path)

    # Define test data
    context = "test_context"
    project = "test_project"
    instance = "test_instance"
    created_by = "unit-test"
    label = "test-label"
    env_dict = {
        "FOO": "foo-value",
        "BAR": "bar-value"
    }

    # Run the snapshot insertion
    snapshot_id = await insert_snapshot_with_env_vars(
        db_path=db_path,
        context=context,
        project=project,
        instance=instance,
        created_by=created_by,
        label=label,
        env_dict=env_dict
    )
    print(f"Snapshot ID: {snapshot_id}")

    # Verify the snapshot and key-values exist
    async with aiosqlite.connect(db_path) as db:
        async with db.execute("SELECT id, instance_id, label FROM env_snapshot") as cursor:
            snapshots = await cursor.fetchall()
            print("All snapshots:", snapshots)

        async with db.execute("SELECT label FROM env_snapshot WHERE id = ?", (snapshot_id,)) as cursor:
            row = await cursor.fetchone()
            assert row[0] == label

        async with db.execute("SELECT key, value FROM env_kv WHERE snapshot_id = ?", (snapshot_id,)) as cursor:
            rows = await cursor.fetchall()
            kv_dict = dict(rows)
            assert kv_dict["FOO"] == "foo-value"
            assert kv_dict["BAR"] == "bar-value"


@pytest.mark.asyncio
async def test_snapshot_overwrite_behavior(tmp_path):
    db_path = tmp_path / "test.sqlite"
    await apply_migrations(db_path)

    env_a = {"FOO": "1", "BAR": "2"}
    env_b = {"BAR": "999"}  # missing FOO on purpose

    # First insert
    id1 = await insert_snapshot_with_env_vars(
        db_path, "ctx", "proj", "inst", "test", "v1", env_a
    )

    # Second insert (new label, missing some vars)
    id2 = await insert_snapshot_with_env_vars(
        db_path, "ctx", "proj", "inst", "test", "v2", env_b
    )

    assert id1 != id2

    async with aiosqlite.connect(db_path) as db:
        async with db.execute("SELECT COUNT(*) FROM env_kv WHERE snapshot_id = ?", (id2,)) as cursor:
            (count,) = await cursor.fetchone()
            assert count == 1  # only BAR in this snapshot
