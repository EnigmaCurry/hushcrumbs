import os
import sys
import click
import asyncio
import pathlib

from .db import apply_migrations
from .queries import (
    insert_snapshot_with_env_vars,
    list_latest_snapshots,
    export_snapshot_to_file,
)

DB_PATH = "db.sqlite"


@click.group()
def cli():
    """hushcrumbs - Manage .env files in sqlite"""
    pass


@cli.command()
def init():
    """Initialize the database."""
    asyncio.run(apply_migrations(DB_PATH))
    click.echo("Database initialized.")


@cli.command()
@click.argument("env_file", type=click.Path(exists=True, dir_okay=False))
@click.option("--context", help="Context name (e.g., docker server)")
@click.option("--project", help="Project name (defaults to parent directory)")
@click.option("--instance", help="Instance name (e.g., container name)")
@click.option("--label", required=True, help="Label for the snapshot")
@click.option(
    "--created-by", default="cli", help="User or system creating the snapshot"
)
def add(env_file, context, project, instance, label, created_by):
    """Add a .env file to the database."""
    env_path = pathlib.Path(env_file)
    env_vars = {}

    with open(env_path) as f:
        for line in f:
            line = line.strip()
            if not line or line.startswith("#") or "=" not in line:
                continue
            key, value = line.split("=", 1)
            env_vars[key.strip()] = value.strip()

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

    # This assumes instance_id lookup or creation is handled inside this function
    asyncio.run(
        insert_snapshot_with_env_vars(
            db_path=DB_PATH,
            context=context,
            project=project,
            instance=instance,
            created_by=created_by,
            label=label,
            env_dict=env_vars,
        )
    )
    click.echo(f"Snapshot '{label}' added for {context}/{project}/{instance}.")


@cli.command()
@click.option("--context", required=True, help="Context name")
@click.option("--project", required=True, help="Project name")
@click.option("--instance", required=True, help="Instance name")
@click.option("--snapshot", help="Snapshot label (optional, defaults to latest)")
@click.argument("path", type=click.Path(file_okay=False, dir_okay=True))
def restore(context, project, instance, snapshot, path):
    """Restore a snapshot to PATH as a .env file."""
    env_file_path = os.path.join(path, f".env_{context}_{instance}")
    count = asyncio.run(
        export_snapshot_to_file(
            db_path=DB_PATH,
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
    asyncio.run(list_latest_snapshots(DB_PATH))


@cli.command()
def help():
    """Show help message."""
    click.echo(cli.get_help(click.Context(cli)))


if __name__ == "__main__":
    cli()
