# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import asyncio
from typing import Any

import click

from parsec._version import __version__ as version
from parsec.cli.options import debug_config_options, logging_config_options
from parsec.cli.utils import (
    KO,
    OK,
    cli_exception_handler,
    spinner,
)
from parsec.components.postgresql import apply_migrations, retrieve_migrations


def _validate_postgres_db_url(ctx: Any, param: Any, value: str) -> str:
    if not (value.startswith("postgresql://") or value.startswith("postgres://")):
        raise click.BadParameter("Must start with `postgresql://` or `postgres://`")
    return value


@click.command(short_help="Updates database schema")
@click.option(
    "--db",
    required=True,
    callback=_validate_postgres_db_url,
    envvar="PARSEC_DB",
    show_envvar=True,
    help="PostgreSQL database url",
)
@click.option("--dry-run", is_flag=True)
# Avoid polluting CLI command output with INFO logs
@logging_config_options(default_log_level="WARNING")
# Add --debug & --version
@debug_config_options
def migrate(db: str, debug: bool, dry_run: bool, **kwargs: Any) -> None:
    """
    Updates the database schema
    """
    click.echo("Will apply migrations for version ", nl=False)
    click.secho(version, fg="yellow")
    with cli_exception_handler(debug):
        migrations = retrieve_migrations()

        async def _migrate(db: str) -> None:
            async with spinner("Migrate"):
                result = await apply_migrations(db, migrations, dry_run)

            for migration in result.already_applied:
                click.secho(f"{migration.file_name} (already applied)", fg="white")

            for migration in result.new_apply:
                click.secho(f"{migration.file_name} {OK}", fg="green")

            if not result.new_apply:
                click.secho(
                    "No migrations applied, is this expected?", blink=True, fg="red", err=True
                )

            if result.error:
                migration, msg = result.error
                click.secho(f"{migration.file_name} {KO}: {msg}", fg="red")

        asyncio.run(_migrate(db))
