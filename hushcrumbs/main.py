import asyncio
from queries import insert_snapshot_with_env_vars
from db import ensure_database_is_ready, DB_PATH


# Example usage in main
async def main():
    await ensure_database_is_ready()

    # your logic here, e.g. insert snapshot
    env_vars = {
        "DATABASE_URL": "postgres://localhost/db",
        "DEBUG": "false",
        "SECRET_KEY": "s3cr3t",
    }

    await insert_snapshot_with_env_vars(
        db_path=DB_PATH,
        instance_id=1,
        created_by="deploy-bot",
        label="v1.0.0",
        env_dict=env_vars,
    )


if __name__ == "__main__":
    asyncio.run(main())
