import pytest
from hushcrumbs import crypto
from cryptography.fernet import Fernet


def test_generate_key_is_valid():
    key = crypto.generate_key()
    assert isinstance(key, bytes)
    # Should be 44-byte base64 encoded 32-byte key
    assert len(key) == 44
    Fernet(key)  # Should not raise


def test_encrypt_decrypt_roundtrip():
    key = crypto.generate_key()
    f = Fernet(key)

    original = "SECRET=VALUE"
    encrypted = f.encrypt(original.encode()).decode()
    decrypted = f.decrypt(encrypted.encode()).decode()

    assert decrypted == original


def test_encrypt_value_and_decrypt_value(monkeypatch):
    key = Fernet.generate_key()
    monkeypatch.setenv("ENCRYPTION_KEY", key.decode())

    secret = "super_secret"
    encrypted = crypto.encrypt_value(secret)
    assert encrypted != secret

    decrypted = crypto.decrypt_value(encrypted)
    assert decrypted == secret


def test_encrypt_token_and_validate():
    key = Fernet.generate_key()
    token = crypto.encrypt_token(key)

    assert crypto.validate_encrypted_token(token, key) is True
    wrong_key = Fernet.generate_key()
    assert crypto.validate_encrypted_token(token, wrong_key) is False
