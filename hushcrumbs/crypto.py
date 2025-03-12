import os
import sys
import aiosqlite
import logging
from cryptography.fernet import Fernet

log = logging.getLogger(__name__)

# Known clear text to hash with encryption key for validation:
AUTH_CHECK_VALUE = b"hushcrumbs-auth-check-text"

def get_encryption_key():
    key = os.environ.get("ENCRYPTION_KEY")
    if not key:
        log.error("❌ ENCRYPTION_KEY environment variable not set.")
        raise SystemExit(1)
    try:
        Fernet(key)
    except Exception:
        log.error("❌ ENCRYPTION_KEY is not a valid Fernet key.")
        raise SystemExit(1)
    return key.encode()

def encrypt_value(value: str) -> str:
    key = get_encryption_key()
    fernet = Fernet(key)
    return fernet.encrypt(value.encode()).decode()

def decrypt_value(encrypted: str) -> str:
    key = get_encryption_key()
    fernet = Fernet(key)
    return fernet.decrypt(encrypted.encode()).decode()

def generate_key():
    return Fernet.generate_key()

def encrypt_token(key: bytes) -> str:
    return Fernet(key).encrypt(AUTH_CHECK_VALUE).decode()

def validate_encrypted_token(encrypted: str, key: bytes) -> bool:
    try:
        decrypted = Fernet(key).decrypt(encrypted.encode())
        return decrypted == AUTH_CHECK_VALUE
    except Exception:
        return False
