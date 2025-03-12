import os
import asyncio
import aiosqlite
import logging
from .queries import load_queries, insert_auth_token, load_encrypted_auth_text
from .crypto import get_encryption_key, encrypt_token, validate_encrypted_token, generate_key
from fastapi import Request, HTTPException, status, Depends

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
        print("AUTH_KEY (validate_auth_key):", os.environ.get("AUTH_KEY", None))
        raise ValueError("Invalid encryption key or corrupted auth token")

def get_token_validator():
    expected_token = os.environ.get("API_TOKEN")

    async def validate_token(request: Request):
        auth_header = request.headers.get("Authorization")
        if not expected_token or not auth_header:
            raise HTTPException(status_code=status.HTTP_401_UNAUTHORIZED, detail="Missing token")
        if auth_header != f"Bearer {expected_token}":
            raise HTTPException(status_code=status.HTTP_403_FORBIDDEN, detail="Invalid token")

    return Depends(validate_token)
