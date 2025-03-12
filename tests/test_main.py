import os
import re
import tempfile
import shutil
import subprocess
import pathlib
import pytest
from cryptography.fernet import Fernet

TEST_ENV = "FOO=bar\n# a comment\nBAR=baz\n"

@pytest.fixture
def temp_project_dir(tmp_path):
    project_dir = tmp_path / "testproj"
    project_dir.mkdir()
    env_file = project_dir / ".env_test_default"
    env_file.write_text(TEST_ENV)
    return project_dir


def test_init_generates_key(tmp_path, monkeypatch):
    db_path = tmp_path / "db.sqlite"
    monkeypatch.setenv("DB_PATH", str(db_path))
    result = subprocess.run(["python", "-m", "hushcrumbs", "init"], capture_output=True, text=True)

    assert "IMPORTANT" in result.stdout
    assert re.search(r"^AUTH_KEY=[\_\=\-a-zA-Z0-9]+$", result.stdout, re.MULTILINE)
    assert db_path.exists()


def test_add_and_restore(tmp_path, monkeypatch):
    # Setup test project and .env
    project = tmp_path / "myproject"
    project.mkdir()
    env_path = project / ".env_test_default"
    env_path.write_text("""
# This is foo
FOO=123 asdf foo
BAR=456 basdf bar
""")

    db_path = tmp_path / "db.sqlite"
    monkeypatch.setenv("DB_PATH", str(db_path))

    # Init DB
    result = subprocess.run(["python", "-m", "hushcrumbs", "init"], capture_output=True, text=True)
    lines = result.stdout.splitlines()
    key_line = next((l for l in lines if "AUTH_KEY=" in l), None)
    assert key_line is not None
    key = key_line.split("=", 1)[1].strip()
    monkeypatch.setenv("AUTH_KEY", key)

    # Add snapshot
    result = subprocess.run([
        "python", "-m", "hushcrumbs", "add",
        "--label", "v1",
        str(env_path)
    ], capture_output=True, text=True)
    assert result.returncode == 0

    # Restore snapshot
    restore_dir = tmp_path / "restore"
    restore_dir.mkdir()
    result = subprocess.run([
        "python", "-m", "hushcrumbs", "restore",
        "--context", "test",
        "--instance", "default",
        "--project", "myproject",
        "--force",
        str(restore_dir)
    ], capture_output=True, text=True)

    assert result.returncode == 0
    restored_env = (restore_dir / ".env_test_default").read_text()
    assert re.search(r"^FOO=123 asdf foo$", restored_env, re.MULTILINE)
    assert re.search(r"^BAR=456 basdf bar$", restored_env, re.MULTILINE)


def test_add_duplicate_label_fails(tmp_path, monkeypatch):
    # Setup test project and .env
    project = tmp_path / "myproject"
    project.mkdir()
    env_path = project / ".env_test_default"
    env_path.write_text("FOO=duplicate\n")

    db_path = tmp_path / "db.sqlite"
    monkeypatch.setenv("DB_PATH", str(db_path))

    # Init DB
    result = subprocess.run(["python", "-m", "hushcrumbs", "init"], capture_output=True, text=True)
    lines = result.stdout.splitlines()
    key_line = next((l for l in lines if "AUTH_KEY=" in l), None)
    assert key_line is not None
    key = key_line.split("=", 1)[1].strip()
    monkeypatch.setenv("AUTH_KEY", key)

    # First insert
    result = subprocess.run([
        "python", "-m", "hushcrumbs", "add",
        "--label", "v1",
        str(env_path)
    ], capture_output=True, text=True)
    assert result.returncode == 0

    # Second insert (should fail)
    result = subprocess.run([
        "python", "-m", "hushcrumbs", "add",
        "--label", "v1",
        str(env_path)
    ], capture_output=True, text=True)

    assert result.returncode != 0
    assert "already been used" in result.stderr or "already exists" in result.stderr

import os
import re
import tempfile
import shutil
import subprocess
import pathlib
import pytest
from cryptography.fernet import Fernet

TEST_ENV = "FOO=bar\n# a comment\nBAR=baz\n"

@pytest.fixture
def temp_project_dir(tmp_path):
    project_dir = tmp_path / "testproj"
    project_dir.mkdir()
    env_file = project_dir / ".env_test_default"
    env_file.write_text(TEST_ENV)
    return project_dir


def test_init_generates_key(tmp_path, monkeypatch):
    db_path = tmp_path / "db.sqlite"
    monkeypatch.setenv("DB_PATH", str(db_path))
    result = subprocess.run(["python", "-m", "hushcrumbs", "init"], capture_output=True, text=True)

    assert "IMPORTANT" in result.stdout
    assert re.search(r"^AUTH_KEY=[\_\=\-a-zA-Z0-9]+$", result.stdout, re.MULTILINE)
    assert db_path.exists()


def test_add_and_restore(tmp_path, monkeypatch):
    # Setup test project and .env
    project = tmp_path / "myproject"
    project.mkdir()
    env_path = project / ".env_test_default"
    env_path.write_text("""
# This is foo
FOO=123 asdf foo
BAR=456 basdf bar
""")

    db_path = tmp_path / "db.sqlite"
    monkeypatch.setenv("DB_PATH", str(db_path))

    # Init DB
    result = subprocess.run(["python", "-m", "hushcrumbs", "init"], capture_output=True, text=True)
    lines = result.stdout.splitlines()
    key_line = next((l for l in lines if "AUTH_KEY=" in l), None)
    assert key_line is not None
    key = key_line.split("=", 1)[1].strip()
    monkeypatch.setenv("AUTH_KEY", key)

    # Add snapshot
    result = subprocess.run([
        "python", "-m", "hushcrumbs", "add",
        "--label", "v1",
        str(env_path)
    ], capture_output=True, text=True)
    assert result.returncode == 0

    # Restore snapshot
    restore_dir = tmp_path / "restore"
    restore_dir.mkdir()
    result = subprocess.run([
        "python", "-m", "hushcrumbs", "restore",
        "--context", "test",
        "--instance", "default",
        "--project", "myproject",
        "--force",
        str(restore_dir)
    ], capture_output=True, text=True)

    assert result.returncode == 0
    restored_env = (restore_dir / ".env_test_default").read_text()
    assert re.search(r"^FOO=123 asdf foo$", restored_env, re.MULTILINE)
    assert re.search(r"^BAR=456 basdf bar$", restored_env, re.MULTILINE)


def test_add_duplicate_label_fails(tmp_path, monkeypatch):
    # Setup test project and .env
    project = tmp_path / "myproject"
    project.mkdir()
    env_path = project / ".env_test_default"
    env_path.write_text("FOO=duplicate\n")

    db_path = tmp_path / "db.sqlite"
    monkeypatch.setenv("DB_PATH", str(db_path))

    # Init DB
    result = subprocess.run(["python", "-m", "hushcrumbs", "init"], capture_output=True, text=True)
    lines = result.stdout.splitlines()
    key_line = next((l for l in lines if "AUTH_KEY=" in l), None)
    assert key_line is not None
    key = key_line.split("=", 1)[1].strip()
    monkeypatch.setenv("AUTH_KEY", key)

    # First insert
    result = subprocess.run([
        "python", "-m", "hushcrumbs", "add",
        "--label", "v1",
        str(env_path)
    ], capture_output=True, text=True)
    assert result.returncode == 0

    # Second insert (should fail)
    result = subprocess.run([
        "python", "-m", "hushcrumbs", "add",
        "--label", "v1",
        str(env_path)
    ], capture_output=True, text=True)

    assert result.returncode != 0
    assert "already been used" in result.stderr or "already exists" in result.stderr


def test_wrong_auth_key_rejected(tmp_path, monkeypatch):
    # Setup test project and .env
    project = tmp_path / "myproject"
    project.mkdir()
    env_path = project / ".env_test_default"
    env_path.write_text("FOO=secret\n")

    db_path = tmp_path / "db.sqlite"
    monkeypatch.setenv("DB_PATH", str(db_path))

    # Init DB
    result = subprocess.run(["python", "-m", "hushcrumbs", "init"], capture_output=True, text=True)
    lines = result.stdout.splitlines()
    key_line = next((l for l in lines if "AUTH_KEY=" in l), None)
    assert key_line is not None
    correct_key = key_line.split("=", 1)[1].strip()

    # Use incorrect key
    wrong_key = Fernet.generate_key().decode()
    monkeypatch.setenv("AUTH_KEY", wrong_key)

    # Attempt to add snapshot with wrong key
    result = subprocess.run([
        "python", "-m", "hushcrumbs", "add",
        "--label", "bad-key",
        str(env_path)
    ], capture_output=True, text=True)

    assert result.returncode != 0
    assert "Invalid encryption key" in result.stderr or "corrupted auth token" in result.stderr

def test_restore_fails_with_wrong_key(tmp_path, monkeypatch):
    # Setup test project and .env
    project = tmp_path / "myproject"
    project.mkdir()
    env_path = project / ".env_test_default"
    env_path.write_text("FOO=restoretest\n")

    db_path = tmp_path / "db.sqlite"
    monkeypatch.setenv("DB_PATH", str(db_path))

    # Init DB
    result = subprocess.run(["python", "-m", "hushcrumbs", "init"], capture_output=True, text=True)
    lines = result.stdout.splitlines()
    key_line = next((l for l in lines if "AUTH_KEY=" in l), None)
    assert key_line is not None
    correct_key = key_line.split("=", 1)[1].strip()
    monkeypatch.setenv("AUTH_KEY", correct_key)

    # Add snapshot with correct key
    result = subprocess.run([
        "python", "-m", "hushcrumbs", "add",
        "--label", "v1",
        str(env_path)
    ], capture_output=True, text=True)
    assert result.returncode == 0

    # Switch to wrong key before restoring
    monkeypatch.setenv("AUTH_KEY", Fernet.generate_key().decode())

    restore_dir = tmp_path / "restore"
    restore_dir.mkdir()
    result = subprocess.run([
        "python", "-m", "hushcrumbs", "restore",
        "--context", "test",
        "--instance", "default",
        "--project", "myproject",
        "--force",
        str(restore_dir)
    ], capture_output=True, text=True)

    assert result.returncode != 0
    assert "Invalid encryption key" in result.stderr or "corrupted auth token" in result.stderr
