# --------------------
# Builder stage
# --------------------
FROM python:3.13-slim AS builder

WORKDIR /build

# Install build tools and poetry
RUN apt-get update && apt-get install -y --no-install-recommends \
    curl build-essential gcc \
    && pip install poetry \
    && rm -rf /var/lib/apt/lists/*

# Copy only dependency files first
COPY pyproject.toml poetry.lock ./
RUN touch README.md && \
    poetry config virtualenvs.create false && \
    poetry install --without dev --no-root

# --------------------
# Runtime stage
# --------------------
FROM python:3.13-slim

ENV PYTHONUNBUFFERED=1 \
    PYTHONPATH=/app \
    DB_PATH=/data/db.sqlite \
    ENCRYPTION_KEY= \
    API_TOKEN=

WORKDIR /app

# Copy installed site-packages from builder
COPY --from=builder /usr/local/lib/python3.13/site-packages /usr/local/lib/python3.13/site-packages
COPY . .

# Ensure the database volume is writable
VOLUME /data

# Expose port 8000
EXPOSE 8000

# Copy the entrypoint script
COPY entrypoint.sh /entrypoint.sh
RUN chmod +x /entrypoint.sh

ENTRYPOINT ["/entrypoint.sh"]
CMD python -m hushcrumbs server --host 0.0.0.0 --port 8000 --token ${API_TOKEN}
