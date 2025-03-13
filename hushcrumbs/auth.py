import os
import re
import asyncio
import aiosqlite
import logging
from .queries import load_queries, insert_auth_token, load_encrypted_auth_text
from .crypto import get_encryption_key, encrypt_token, validate_encrypted_token, generate_passphrase, derive_key
from fastapi import Request, HTTPException, status, Depends

log = logging.getLogger(__name__)

def validate_passphrase(passphrase: str) -> str:
    """
    Validates that the passphrase is a 10-word string joined by hyphens.
    Each word must consist of only lowercase letters.
    Returns the passphrase if valid, otherwise raises AssertionError
    """
    if passphrase is None:
        raise AssertionError("ENCRYPTION_KEY is not set.")
    if not isinstance(passphrase, str):
        raise AssertionError("ENCRYPTION_KEY is not a string.")
    words = passphrase.split("-")
    if len(words) != 10:
        raise AssertionError("ENCRYPTION_KEY is not 10 words long.")
    # Match each word: only lowercase a-z
    for word in words:
        if not re.fullmatch(r"[a-z]+", word):
            raise AssertionError("ENCRYPTION_KEY is not composed of 10 lowercase words.")
    return passphrase

async def save_auth_key(db_path: str):
    passphrase = validate_passphrase(os.environ.get("ENCRYPTION_KEY", None))
    key = derive_key(passphrase)
    encrypted = encrypt_token(key)
    load_queries()
    existing_token = await load_encrypted_auth_text(db_path)
    if existing_token is not None:
        log.error("❌ Error: Database is already initialized with an encryption key.")
        raise SystemExit(1)

    async with aiosqlite.connect(db_path) as db:
        await insert_auth_token(db, encrypted_value=encrypted)
        await db.commit()
    return passphrase, key

async def validate_encryption_key(db_path: str):
    load_queries()
    encryption_key = get_encryption_key()
    encrypted = await load_encrypted_auth_text(db_path)
    if not validate_encrypted_token(encrypted, encryption_key.decode()):
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
