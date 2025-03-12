from fastapi import FastAPI, UploadFile, Form, HTTPException
from fastapi.responses import PlainTextResponse, JSONResponse
import tempfile
import asyncio
import os
from .parser import parse_env_file_contents
from .queries import load_queries, export_snapshot_to_file
from .auth import validate_auth_key, get_token_validator
import aiosqlite

import logging
logging.basicConfig(level=logging.INFO)
log = logging.getLogger(__name__)

app = FastAPI(dependencies=[get_token_validator()])
DB_PATH = os.environ.get("DB_PATH", os.path.abspath("db.sqlite"))

@app.on_event("startup")
async def log_db_path():
    print("Using database at:", DB_PATH)


@app.post("/snapshots/")
async def upload_env_file(
    context: str = Form(...),
    project: str = Form(...),
    instance: str = Form(...),
    label: str = Form(...),
    created_by: str = Form("api"),
    file: UploadFile = Form(...)
):
    contents = await file.read()
    decoded = contents.decode()

    env_dict, env_comments = parse_env_file_contents(decoded)
    q = load_queries()
    await validate_auth_key(DB_PATH)
    try:
        await q.insert_snapshot_with_env_vars(
            db_path=DB_PATH,
            context=context,
            project=project,
            instance=instance,
            created_by=created_by,
            label=label,
            env_dict=env_dict,
            env_comments=env_comments,
            force=False,
        )
    except ValueError as e:
        raise HTTPException(status_code=400, detail=str(e))

    return {"status": "ok", "label": label}

@app.get("/snapshots", response_class=JSONResponse)
async def list_latest_snapshots():
    q = load_queries()

    await validate_auth_key(DB_PATH)

    async with aiosqlite.connect(DB_PATH) as db:
        db.row_factory = aiosqlite.Row
        rows = await q.get_latest_snapshots(db)
        return [dict(row) for row in rows]

@app.get("/snapshots/{context}/{project}/{instance}", response_class=PlainTextResponse)
async def download_env_file(context: str, project: str, instance: str, snapshot: str = None):
    q = load_queries()
    await validate_auth_key(DB_PATH)

    with tempfile.NamedTemporaryFile(delete=False) as tmp:
        path = tmp.name

    try:
        try:
            count = await export_snapshot_to_file(
                db_path=DB_PATH,
                context=context,
                project=project,
                instance=instance,
                snapshot_label=snapshot,
                out_path=path
            )
        except ValueError as e:
            raise HTTPException(status_code=404, detail=str(e))

        with open(path) as f:
            return f.read()

    finally:
        os.unlink(path)
