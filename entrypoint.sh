#!/bin/sh
set -e

# Default path if not set
DB_FILE="${DB_PATH:-/data/db.sqlite}"

# Require ENCRYPTION_KEY to be set
if [ -z "$ENCRYPTION_KEY" ]; then
    echo "❌ ENCRYPTION_KEY must be set as an environment variable."
    echo "Example (but don't use this key):"
    echo "  docker run -e ENCRYPTION_KEY=correct-horse-battery-staple-foo-bar-fee-fi-fo-fum ...."
    exit 1
fi

# If DB does not exist, initialize it
if [ ! -f "$DB_FILE" ]; then
    echo "📦 Database not found at $DB_FILE. Initializing with provided ENCRYPTION_KEY..."
    python -m hushcrumbs init
else
    echo "✅ Database already exists at $DB_FILE"
fi

# Run the given command (usually: start the server)
exec "$@"
