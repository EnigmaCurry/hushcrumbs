import os
import sys
import click
import asyncio
import pathlib
import logging
from tabulate import tabulate

from .db import apply_migrations, get_db_path
from .queries import (
    load_queries,
    insert_snapshot_with_env_vars,
    get_latest_snapshots,
    export_snapshot_to_file,
)
from .auth import save_auth_key, validate_encryption_key, generate_passphrase
from .parser import parse_env_file_contents

logging.basicConfig(level=logging.INFO)
log = logging.getLogger(__name__)


@click.group()
def cli():
    """hushcrumbs - Manage .env files in sqlite"""
    pass


@cli.command()
def gen_key():
    """Generate a secure encryption key"""
    print(generate_passphrase())


@cli.command()
def init():
    """Initialize the database."""
    asyncio.run(apply_migrations(get_db_path()))

    async def _init():
        passphrase, key = await save_auth_key(get_db_path())
        click.echo(
            "\nIMPORTANT - SAVE THIS KEY - YOU WILL NEED THIS KEY TO UNLOCK YOUR DATABASE!"
        )
        click.echo("ENCRYPTION_KEY=" + passphrase)

    asyncio.run(_init())


@cli.command()
@click.argument("env_file", type=click.Path(exists=True, dir_okay=False))
@click.option("--context", help="Context name (e.g., docker server)")
@click.option("--project", help="Project name (defaults to parent directory)")
@click.option("--instance", help="Instance name (e.g., container name)")
@click.option("--label", required=True, help="Label for the snapshot")
@click.option(
    "--force", is_flag=True, help="Force adding snapshot even if label already exists."
)
@click.option(
    "--created-by", default="cli", help="User or system creating the snapshot"
)
def add(env_file, context, project, instance, label, force, created_by):
    """Add a .env file to the database."""
    env_path = pathlib.Path(env_file)

    load_queries()
    asyncio.run(validate_encryption_key(get_db_path()))
    with open(env_path) as f:
        contents = f.read()

    env_dict, env_comments = parse_env_file_contents(contents)

    if not project:
        project = env_path.parent.name

    if not context or not instance:
        name = env_path.name
        if not name.startswith(".env") or name.count("_") != 2:
            click.echo(
                "Error: Filename must start with .env and contain exactly two underscores.",
                err=True,
            )
            sys.exit(1)

        _, context_part, instance_part = name.split("_")
        context = context or context_part
        instance = instance or instance_part

    # Assumes instance_id lookup or creation is handled inside this function
    try:
        asyncio.run(
            insert_snapshot_with_env_vars(
                db_path=get_db_path(),
                context=context,
                project=project,
                instance=instance,
                created_by=created_by,
                label=label,
                env_dict=env_dict,
                env_comments=env_comments,
                force=force,
            )
        )
    except ValueError:
        if not force:
            log.error(
                "That label has already been used for this instance. Use --force to overwrite it."
            )
            sys.exit(1)
        else:
            raise

    click.echo(f"Snapshot '{label}' added for {context}/{project}/{instance}.")


@cli.command()
@click.option("--context", required=True, help="Context name")
@click.option("--project", help="Project name (defaults to path basename if not given)")
@click.option("--instance", required=True, help="Instance name")
@click.option("--snapshot", help="Snapshot label (optional, defaults to latest)")
@click.option("--force", is_flag=True, help="Allow mismatch between --project and path")
@click.argument(
    "path",
    required=False,
    type=click.Path(file_okay=False, dir_okay=True),
)
def restore(context, project, instance, snapshot, path, force):
    """Restore a .env snapshot to PATH or --project directory."""
    if not project and not path:
        raise click.UsageError("You must provide either --project or PATH.")

    load_queries()
    asyncio.run(validate_encryption_key(get_db_path()))
    if not project and path:
        project = os.path.basename(os.path.abspath(path))

    if not path and project:
        path = project

    if path and project:
        path_basename = os.path.basename(os.path.abspath(path))
        if path_basename != project and not force:
            raise click.UsageError(
                f"Project '{project}' does not match directory '{path_basename}'. Use --force to override."
            )

    os.makedirs(path, exist_ok=True)
    env_file_path = os.path.join(path, f".env_{context}_{instance}")

    count = asyncio.run(
        export_snapshot_to_file(
            db_path=get_db_path(),
            context=context,
            project=project,
            instance=instance,
            snapshot_label=snapshot,
            out_path=env_file_path,
        )
    )
    click.echo(f"Restored {count} variables to {env_file_path}")


@cli.command()
def list():
    """List the latest snapshot for each context/project/instance."""
    load_queries()
    try:
        rows, headers = asyncio.run(get_latest_snapshots(get_db_path()))
    except TypeError:
        return
    else:
        print(tabulate(rows, headers=headers, tablefmt="plain"))


@cli.command()
@click.option("--host", default="127.0.0.1", help="Host to bind to.")
@click.option("--port", default=8000, help="Port to listen on.")
@click.option("--reload", is_flag=True, help="Enable auto-reload (for development).")
@click.option("--token", required=True, help="Token required for accessing the API.")
def server(host, port, reload, token):
    """Run the hushcrumbs HTTP API server."""
    import uvicorn

    if not os.path.exists(get_db_path()):
        log.error("Database does not exist - please run init command.")
        return 1

    os.environ["API_TOKEN"] = token

    config = {
        "app": "hushcrumbs.server:app",
        "host": host,
        "port": port,
        "reload": reload,
        "factory": False,
    }
    click.echo(f"🌐 Starting hushcrumbs API at http://{host}:{port}")
    uvicorn.run(**config)


@cli.command()
def help():
    """Show help message."""
    click.echo(cli.get_help(click.Context(cli)))


if __name__ == "__main__":
    cli()
