# hushcrumbs

**hushcrumbs** is an encrypted `.env` (dotenv) manager made
specifically for
[d.rymcg.tech](https://github.com/EnigmaCurry/d.rymcg.tech) (but may
also be used for general purposes). It provides both a **command-line
interface (CLI)** and an **HTTP API**.

It lets you store, encrypt, snapshot, restore, and audit environment
variable files across different contexts with optional metadata and
comments preserved.

## ✨ Features

- 📦 Store `.env` snapshots in SQLite
- 🔐 Encrypt variable values (using [Fernet](https://cryptography.io/en/latest/fernet/))
- 📝 Preserve multi-line comments for each variable
- 🔁 Restore snapshots as `.env` files
- 💻 CLI for automation and scripting
- 🌐 HTTP API for integration
- 🔑 Auth token validation to protect access

---

## 🚀 Getting Started

### 📦 Install

```bash
poetry install
```

Or clone and run directly with `python -m hushcrumbs`.

### 🔐 Initialize the database

```bash
hushcrumbs init
```

You'll be shown an **ENCRYPTION_KEY** environment variable you must store
and set securely. This is used to encrypt and decrypt all secrets.

> 🛑 If you lose this key, you won't be able to decrypt your stored variables!

You must set the `ENCRYPTION_KEY` variable with the value printed during
initialization:

```bash
export ENCRYPTION_KEY=xxxxxxxxxxxxxx
```

---

## 💻 CLI Usage

### Add a `.env` file snapshot

```bash
hushcrumbs add \
  --context my-docker \
  --project webapp \
  --instance api \
  --label "v1.0"
```

This will add the .env file from the path `webapp/.env_my-docker_api`.
If your file is located or named differently, you may specify the path:

```bash
hushcrumbs add /path/to/.env \
  --context some-context \
  --project some-project \
  --instance some-instance \
  --label "v1.0"
```

### Restore a snapshot to a file

```bash
hushcrumbs restore \
  --context docker \
  --project webapp \
  --instance api \
  --snapshot "v1.0" \
  ./restore_dir/
```

If you do not specify the restore path it will assume the following
default: `{PROJECT}/.env_{CONTEXT}_{INSTANCE}`. If the provided
`--project` is mismatched from this style of path, you must use the
`--force` option.

### List latest snapshots

```bash
hushcrumbs list
```

### Run the HTTP server

```bash
hushcrumbs server --host 0.0.0.0 --port 8000 --token example-of-a-secure-token
```

You must choose a secure token with the `--token` option.

---

## 🌐 HTTP API

All endpoints require an `Authorization: Bearer YOUR_API_TOKEN` header.

### POST `/snapshots/`

Upload a new snapshot.

**Form fields:**

- `context`, `project`, `instance`, `label`, `created_by`
- `file`: a `.env` file (multipart upload)

### GET `/snapshots`

List latest snapshot per context/project/instance.

### GET `/snapshots/{context}/{project}/{instance}`

Download a snapshot as plain `.env` text.

You can use curl to test, passing your bearer token, e.g.:

```bash
curl -H "Authorization: Bearer {TOKEN}" http://localhost:8000/snapshots
```

---

## 🧪 Running Tests

```bash
pytest
```

Tests cover both the CLI and HTTP API. Snapshots are encrypted and validated.

---

## 🔒 Security

- Variables are encrypted using [Fernet symmetric encryption](https://cryptography.io/en/latest/fernet/).
- An encrypted auth token is stored in the DB and validated on every operation.
- The `ENCRYPTION_KEY` must be exported in the environment:

```bash
export ENCRYPTION_KEY=...
```

- The HTTP server protects access using a separate bearer token, not to be
  confused with the `ENCRYPTION_KEY`.

---

## 📁 Project Layout

```
hushcrumbs/
├── cli.py            # CLI entry point
├── server.py         # FastAPI server
├── auth.py           # Encryption key management
├── crypto.py         # Encryption utilities
├── queries.py        # SQL queries via aiosql
├── parser.py         # Parses .env files with comment support
├── db.py             # Migration engine
├── migrations/
│   └── 001_init.sql  # Initial schema
```

---

## 📖 Example .env File Format

```env
# Database settings
DB_HOST=localhost
DB_PORT=5432

# Application secrets
# Multi-line
# comment
SECRET_KEY=supersecret
```

Comments will be stored and restored.

---

