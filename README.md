# hushcrumbs

**hushcrumbs** is an encrypted `.env` (dotenv) file snapshot manager
backed by a SQLite database. It provides both a **command-line
interface (CLI)** and an **HTTP API**. It lets you store, encrypt,
snapshot, restore, and audit environment variable files (and their
documentation comments) across different contexts, projects, and
instances.

Status: Experimental - this is intended as an integration piece for
[d.rymcg.tech](https://github.com/EnigmaCurry/d.rymcg.tech) - but it
may have some utility outside of this domain.

## ✨ Features

- 📦 Store `.env` snapshots in SQLite
- 🔐 Encrypt the variable values (using [Fernet](https://cryptography.io/en/latest/fernet/))
- 📝 Preserve multi-line comments for each variable (also encrypted)
- 🔁 Restore snapshots as `.env` files
- 💻 CLI for automation and scripting
- 🌐 HTTP API for integration
- 🐋 Dockerfile to build OCI container
- 🔑 API token validation to protect access to the HTTP service
---

## 🚀 Getting Started

### 📦 Install
 
You need the following dependencies installed first:
 
 * Python 3.13+
 * Poetry (`pip install poetry`)
 * PipX (`pip install pipx`)

Install hushcrumbs via the Makefile:

```bash
make install
```

### Generate an encryption key

You must generate a 10 lower-case word encryption passphrase, joined with hyphens:

```bash
export ENCRYPTION_KEY=$(hushcrumbs gen-key)
echo ${ENCRYPTION_KEY}
```

> 🛑 If you lose this key, you won't be able to decrypt your stored variables!

You must have the `ENCRYPTION_KEY` environment variable set in order
to use the rest of the commands. In the future you can set it like
this:

```bash
# EXAMPLE: don't use this key!
export ENCRYPTION_KEY=correct-horse-battery-staple-foo-bar-fee-fi-fo-fum
```

### 🔐 Initialize the database

```bash
hushcrumbs init
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

- The encryption key cannot be easily changed (yet), so make sure to
  keep it a secret on the backend.
- The HTTP server protects access using a separate bearer token (set
  via `hushcrumbs server --token XXX`). The token should NOT be the
  same as `ENCRYPTION_KEY`! You may freely change the token at any
  time (but you must restart the server).

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

