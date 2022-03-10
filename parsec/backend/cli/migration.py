# Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

import click

from parsec.utils import trio_run
from parsec.cli_utils import logging_config_options, spinner, cli_exception_handler, ok, ko
from parsec.backend.postgresql import apply_migrations, retrieve_migrations


def _validate_postgres_db_url(ctx, param, value: str) -> str:
    if not (value.startswith("postgresql://") or value.startswith("postgres://")):
        raise click.BadParameter("Must start with `postgresql://` or `postgres://`")
    return value


@click.command(short_help="Updates database schema")
@click.option(
    "--db",
    required=True,
    callback=_validate_postgres_db_url,
    envvar="PARSEC_DB",
    help="PostgreSQL database url",
)
@click.option("--dry-run", is_flag=True)
# Avoid polluting CLI command output with INFO logs
@logging_config_options(default_log_level="WARNING")
@click.option("--debug", is_flag=True, envvar="PARSEC_DEBUG")
def migrate(db: str, debug: bool, dry_run: bool, **kwargs) -> None:
    """
    Updates the database schema
    """
    with cli_exception_handler(debug):
        migrations = retrieve_migrations()

        async def _migrate(db: str) -> None:
            async with spinner("Migrate"):
                result = await apply_migrations(db, migrations, dry_run)

            for migration in result.already_applied:
                click.secho(f"{migration.file_name} (already applied)", fg="white")
            for migration in result.new_apply:
                click.secho(f"{migration.file_name} {ok}", fg="green")
            if result.error:
                migration, msg = result.error
                click.secho(f"{migration.file_name} {ko}: {msg}", fg="red")

        trio_run(_migrate, db, use_asyncio=True)
