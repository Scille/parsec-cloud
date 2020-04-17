# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import click
import re

import importlib_resources

from parsec.utils import trio_run
from parsec.cli_utils import spinner, cli_exception_handler
from parsec.backend.postgresql import migrate_db, migrations


MIGRATION_FILE_PATTERN = r"^(?P<id>\d{4})_(?P<name>\w*).sql$"


def _sorted_file_migrations():
    files = []
    ids = []
    for f in importlib_resources.contents(migrations):
        match = re.search(MIGRATION_FILE_PATTERN, f)
        if match:
            idx = int(match.group("id"))
            if idx in ids:
                raise click.ClickException(
                    f"Inconsistent package (multiples migrations with {idx} as id)"
                )
            ids.append(idx)
            files.append((idx, match.group("name"), f))

    return sorted(files, key=lambda f: f[0])


def _validate_postgres_db_url(ctx, param, value):
    if not (value.startswith("postgresql://") or value.startswith("postgres://")):
        raise click.BadParameter("Must start with `postgresql://` or `postgres://`")
    return value


def _echo_migration(name, color, is_done=False):
    done = "[ ]"
    if is_done:
        done = "[X]"
    click.secho(f"{name} {done}", fg=color)


@click.command(short_help="Updates database schema")
@click.option(
    "--db",
    required=True,
    callback=_validate_postgres_db_url,
    envvar="PARSEC_DB",
    help="PostgreSQL database url",
)
@click.option("--dry-run", is_flag=True)
@click.option("--debug", is_flag=True, envvar="PARSEC_DEBUG")
def migrate(db, debug, dry_run):
    """
    Updates the database schema
    """
    result_colors = {
        "error": "red",
        "new_apply": "green",
        "to_apply": "white",
        "already_applied": "white",
    }
    with cli_exception_handler(debug):
        migrations = _sorted_file_migrations()

        async def _migrate(db):
            async with spinner("Migrate"):
                result = await migrate_db(db, migrations, dry_run)
                for key, values in zip(result._fields, result):
                    color = result_colors.get(key, "white")
                    if key == "error":
                        if values:
                            _echo_migration(values[0], color)
                            raise click.ClickException(values[1])
                    else:
                        is_done = False
                        for value in values:
                            if key in ["already_applied", "new_apply"]:
                                is_done = True
                            else:
                                is_done = False
                            _echo_migration(value, color, is_done)

        trio_run(_migrate, db, use_asyncio=True)
