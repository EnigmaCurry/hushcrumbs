import os
import sys
import aiosqlite
import logging
import base64
from cryptography.fernet import Fernet, InvalidToken
from cryptography.hazmat.primitives.kdf.pbkdf2 import PBKDF2HMAC
from cryptography.hazmat.primitives import hashes
from cryptography.hazmat.backends import default_backend
from dicewarepy import diceware

log = logging.getLogger(__name__)

# Known clear text to hash with encryption key for validation:
AUTH_CHECK_VALUE = b"hushcrumbs-auth-check-text"
AUTH_STATIC_SALT = b"hushcrumbs-static-salt"

def generate_passphrase():
    return "-".join(diceware(7))

def derive_key(passphrase):
    # Derive a 32-byte key from the passphrase
    kdf = PBKDF2HMAC(
        algorithm=hashes.SHA256(),
        length=32,
        salt=AUTH_STATIC_SALT,
        iterations=100_000,
        backend=default_backend()
    )
    try:
        key = base64.urlsafe_b64encode(kdf.derive(passphrase.encode()))
        Fernet(key)  # Validate key
        return key
    except Exception as e:
        log.error(f"❌ Encryption passphrase (ENCRYPTION_KEY) is invalid: {e}")
        raise SystemExit(1)

def get_encryption_key():
    passphrase = os.environ.get("ENCRYPTION_KEY")
    if not passphrase:
        log.error("❌ ENCRYPTION_KEY environment variable not set.")
        raise SystemExit(1)
    return derive_key(passphrase)

def encrypt_value(value: str) -> str:
    key = get_encryption_key()
    fernet = Fernet(key)
    return fernet.encrypt(value.encode()).decode()

def decrypt_value(encrypted: str) -> str:
    key = get_encryption_key()
    fernet = Fernet(key)
    return fernet.decrypt(encrypted.encode()).decode()

def encrypt_token(key: bytes) -> str:
    return Fernet(key).encrypt(AUTH_CHECK_VALUE).decode()

def validate_encrypted_token(encrypted: str, key: bytes) -> bool:
    try:
        if isinstance(key, str):
            key = key.encode()

        print(f"key: {key}")
        print(f"encrypted: {encrypted}")
        decrypted = Fernet(key).decrypt(encrypted.encode())
        print(f"decrypted: {decrypted}")
        return decrypted == AUTH_CHECK_VALUE

    except InvalidToken:
        return False

    except Exception as e:
        print(f"Unexpected error during token validation: {e}")
        raise
