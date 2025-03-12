import asyncio
import aiosqlite
import logging
from .queries import load_queries, insert_auth_token, load_encrypted_auth_text
from .crypto import get_encryption_key, encrypt_token, validate_encrypted_token, generate_key

log = logging.getLogger(__name__)

async def generate_auth_key(db_path: str):
    key = generate_key()
    encrypted = encrypt_token(key)
    load_queries()
    existing_token = await load_encrypted_auth_text(db_path)
    if existing_token is not None:
        log.error("❌ Error: Database is already initialized with an encryption key.")
        raise SystemExit(1)

    async with aiosqlite.connect(db_path) as db:
        await insert_auth_token(db, encrypted_value=encrypted)
        await db.commit()
    return key

async def validate_auth_key(db_path: str):
    load_queries()
    encryption_key = get_encryption_key()
    encrypted = await load_encrypted_auth_text(db_path)
    if not validate_encrypted_token(encrypted, encryption_key.decode()):
        raise ValueError("Invalid encryption key or corrupted auth token")
