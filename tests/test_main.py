import os
import re
import subprocess
import pathlib
import pytest
from cryptography.fernet import Fernet

TEST_ENV = "FOO=bar\n# a comment\nBAR=baz\n"

ENCRYPTION_KEY = "correct-horse-battery-staple-foo-bar-fee-fie-fo-fum"

@pytest.fixture
def temp_project_dir(tmp_path):
    project_dir = tmp_path / "testproj"
    project_dir.mkdir()
    env_file = project_dir / ".env_test_default"
    env_file.write_text(TEST_ENV)
    return project_dir

def test_add_and_restore(tmp_path, monkeypatch):
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
    monkeypatch.setenv("ENCRYPTION_KEY", ENCRYPTION_KEY)

    subprocess.run(["python", "-m", "hushcrumbs", "init"], capture_output=True, text=True)

    result = subprocess.run([
        "python", "-m", "hushcrumbs", "add",
        "--label", "v1",
        str(env_path)
    ], capture_output=True, text=True)
    assert result.returncode == 0

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
    project = tmp_path / "myproject"
    project.mkdir()
    env_path = project / ".env_test_default"
    env_path.write_text("FOO=duplicate\n")

    db_path = tmp_path / "db.sqlite"
    monkeypatch.setenv("DB_PATH", str(db_path))
    monkeypatch.setenv("ENCRYPTION_KEY", ENCRYPTION_KEY)

    subprocess.run(["python", "-m", "hushcrumbs", "init"], capture_output=True, text=True)

    subprocess.run([
        "python", "-m", "hushcrumbs", "add",
        "--label", "v1",
        str(env_path)
    ], capture_output=True, text=True)

    result = subprocess.run([
        "python", "-m", "hushcrumbs", "add",
        "--label", "v1",
        str(env_path)
    ], capture_output=True, text=True)

    assert result.returncode != 0
    assert "already been used" in result.stderr or "already exists" in result.stderr

def test_wrong_auth_key_rejected(tmp_path, monkeypatch):
    project = tmp_path / "myproject"
    project.mkdir()
    env_path = project / ".env_test_default"
    env_path.write_text("FOO=secret\n")

    db_path = tmp_path / "db.sqlite"
    monkeypatch.setenv("DB_PATH", str(db_path))
    monkeypatch.setenv("ENCRYPTION_KEY", ENCRYPTION_KEY)

    subprocess.run(["python", "-m", "hushcrumbs", "init"], capture_output=True, text=True)

    wrong_key = "one-two-three-four-five-six-seven-eight-nine-ten"
    monkeypatch.setenv("ENCRYPTION_KEY", wrong_key)

    result = subprocess.run([
        "python", "-m", "hushcrumbs", "add",
        "--label", "bad-key",
        str(env_path)
    ], capture_output=True, text=True)

    assert result.returncode != 0
    assert "Invalid encryption key" in result.stderr or "corrupted auth token" in result.stderr

def test_restore_fails_with_wrong_key(tmp_path, monkeypatch):
    project = tmp_path / "myproject"
    project.mkdir()
    env_path = project / ".env_test_default"
    env_path.write_text("FOO=restoretest\n")

    db_path = tmp_path / "db.sqlite"
    monkeypatch.setenv("DB_PATH", str(db_path))
    monkeypatch.setenv("ENCRYPTION_KEY", ENCRYPTION_KEY)

    subprocess.run(["python", "-m", "hushcrumbs", "init"], capture_output=True, text=True)

    subprocess.run([
        "python", "-m", "hushcrumbs", "add",
        "--label", "v1",
        str(env_path)
    ], capture_output=True, text=True)

    monkeypatch.setenv("ENCRYPTION_KEY", Fernet.generate_key().decode())

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
