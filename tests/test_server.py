import os
import pytest
import tempfile
import asyncio
import aiosqlite
from fastapi.testclient import TestClient
from cryptography.fernet import Fernet
from hushcrumbs.auth import encrypt_token
from hushcrumbs.crypto import AUTH_CHECK_VALUE
from hushcrumbs.db import apply_migrations
from hushcrumbs.queries import load_queries, insert_auth_token

DB_PATH = "test_api.sqlite"
API_TOKEN = "test-token"
AUTH_KEY = Fernet.generate_key()

@pytest.fixture(scope="module", autouse=True)
def setup_db():
    os.environ["DB_PATH"] = DB_PATH
    os.environ["AUTH_KEY"] = AUTH_KEY.decode()
    print("AUTH_KEY (setup_db):", os.environ.get("AUTH_KEY", None))
    os.environ["API_TOKEN"] = API_TOKEN

    from hushcrumbs.server import app

    # Initialize the DB
    import asyncio
    asyncio.run(apply_migrations(DB_PATH))

    # Insert encrypted auth token
    encrypted_token = encrypt_token(AUTH_KEY)
    async def init_auth():
        load_queries()
        async with aiosqlite.connect(DB_PATH) as db:
            await insert_auth_token(db, encrypted_token)
            await db.commit()

    asyncio.run(init_auth())

    yield

    os.remove(DB_PATH)

@pytest.fixture
def test_key_and_token():
    os.environ["AUTH_KEY"] = AUTH_KEY.decode()
    os.environ["API_TOKEN"] = "test-token"
    return AUTH_KEY.decode()

@pytest.fixture
def client(tmp_path, test_key_and_token):
    db_path = tmp_path / "test.sqlite"
    os.environ["DB_PATH"] = str(db_path)

    key_str = test_key_and_token.decode() if isinstance(test_key_and_token, bytes) else test_key_and_token
    os.environ["AUTH_KEY"] = key_str
    os.environ["API_TOKEN"] = API_TOKEN

    loop = asyncio.new_event_loop()
    asyncio.set_event_loop(loop)
    loop.run_until_complete(apply_migrations(db_path))

    # Store encrypted token
    encrypted_token = encrypt_token(key_str.encode())
    load_queries()

    async def _insert():
        async with aiosqlite.connect(db_path) as db:
            await insert_auth_token(db, encrypted_value=encrypted_token)
            await db.commit()
    loop.run_until_complete(_insert())

    # Import AFTER env + DB setup
    from hushcrumbs.server import app
    return TestClient(app)

def auth_headers():
    return {"Authorization": "Bearer test-token"}


def test_list_snapshots_empty(client):
    res = client.get("/snapshots", headers=auth_headers())
    assert res.status_code == 200
    assert res.json() == []


def test_upload_snapshot(client):
    env_content = b"# Some comment\nFOO=value1\nBAR=value2\n"
    files = {
        "file": ("env", env_content),
    }
    data = {
        "context": "test",
        "project": "demo",
        "instance": "default",
        "label": "v1",
        "created_by": "test-suite"
    }

    res = client.post("/snapshots/", data=data, files=files, headers=auth_headers())
    assert res.status_code == 200
    assert res.json()["label"] == "v1"


def test_list_snapshots_has_one(client):
    res = client.get("/snapshots", headers=auth_headers())
    assert res.status_code == 200
    snapshots = res.json()
    assert len(snapshots) == 1
    assert snapshots[0]["label"] == "v1"
    assert snapshots[0]["context"] == "test"
    assert snapshots[0]["project"] == "demo"
    assert snapshots[0]["instance"] == "default"


def test_download_snapshot(client):
    res = client.get(
        "/snapshots/test/demo/default",
        headers=auth_headers()
    )
    assert res.status_code == 200
    text = res.text
    assert "FOO=value1" in text
    assert "BAR=value2" in text
    assert "# Some comment" in text
