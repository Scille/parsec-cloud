# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import click

from parsec.utils import trio_run
from parsec.cli_utils import spinner, cli_exception_handler
from parsec.backend.postgresql import init_db


def _validate_postgres_db_url(ctx, param, value):
    if not (value.startswith("postgresql://") or value.startswith("postgres://")):
        raise click.BadParameter("Must start with `postgresql://` or `postgres://`")
    return value


@click.command(short_help="init the database")
@click.option(
    "--db",
    required=True,
    callback=_validate_postgres_db_url,
    envvar="PARSEC_DB",
    help="PostgreSQL database url",
)
@click.option("--debug", is_flag=True, envvar="PARSEC_DEBUG")
def init_cmd(db, debug):
    """
    Initialize a new backend's PostgreSQL database.
    """
    with cli_exception_handler(debug):

        async def _init_db(db):
            async with spinner("Initializing database"):
                already_initialized = await init_db(db)
            if already_initialized:
                click.echo("Database already initialized, nothing to do.")

        trio_run(_init_db, db, use_asyncio=True)
