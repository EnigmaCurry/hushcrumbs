import asyncio
import aiosql
import aiosqlite
import logging

log = logging.getLogger(__name__)

async def init_db(db_path: str, schema_path: str = "schema.sql"):
    async with aiosqlite.connect(db_path) as db:
        with open(schema_path, "r") as f:
            await db.executescript(f.read())
        await db.commit()
        log.info("Database initialized.")
