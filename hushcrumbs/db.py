import aiosqlite
import asyncio
import os

DB_PATH = "db.sqlite"
MIGRATIONS_DIR = "migrations"

async def ensure_database_is_ready():
    # If the file doesn't exist, it will be created during migrations
    db_exists = os.path.exists(DB_PATH)

    if not db_exists:
        print("Database not found. Creating and applying initial schema...")

    await apply_migrations(DB_PATH)
    print("Database is ready and up-to-date.")


async def get_current_version(db):
    try:
        async with db.execute("SELECT version FROM schema_version LIMIT 1") as cursor:
            row = await cursor.fetchone()
            return row[0] if row else 0
    except aiosqlite.OperationalError:
        return 0  # Table doesn't exist yet

async def set_version(db, version):
    await db.execute("UPDATE schema_version SET version = ?", (version,))
    await db.commit()

async def apply_migrations(db_path):
    async with aiosqlite.connect(db_path) as db:
        current_version = await get_current_version(db)

        # List migration files sorted by version number
        files = sorted(f for f in os.listdir(MIGRATIONS_DIR) if f.endswith(".sql"))
        for file in files:
            version = int(file.split("_")[0])
            if version > current_version:
                print(f"Applying migration {file}...")
                with open(os.path.join(MIGRATIONS_DIR, file)) as f:
                    await db.executescript(f.read())
                await set_version(db, version)
                print(f"Migration {version} applied.")
