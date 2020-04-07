# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import click
import glob
import re

from pathlib import Path

from parsec.utils import trio_run
from parsec.cli_utils import spinner, cli_exception_handler
from parsec.backend.postgresql import migrate_db


MIGRATIONS_FOLDER = "parsec/backend/postgresql/migrations"
MIGRATION_FILE_PATTERN = r"(?P<idx>\d{4})_\w*.sql"


def _sorted_file_migrations():
    full_file_pattern = r"\S*(?P<name>%s)" % MIGRATION_FILE_PATTERN
    files = []
    for f in glob.glob(f"{MIGRATIONS_FOLDER}/*.sql"):
        match = re.search(full_file_pattern, f)
        if match:
            files.append(match.group("name"))
    return sorted(files, key=lambda f: re.sub(full_file_pattern, r"\g<1>", f))


def _migration_idx(file_name):
    match = re.search(MIGRATION_FILE_PATTERN, file_name)
    return match.group("idx")


def _validate_postgres_db_url(ctx, param, value):
    if not (value.startswith("postgresql://") or value.startswith("postgres://")):
        raise click.BadParameter("Must start with `postgresql://` or `postgres://`")
    return value


@click.command(short_help="init a new migration file")
@click.option(
    "--name", required=True, help="Migration file name. Use to described the migration content"
)
def make_migration(name):
    """
    Initialize a new migration file.
    """
    migrations = _sorted_file_migrations()
    idx = None
    try:
        last_migration = migrations[-1]
    except IndexError:
        idx = "0001"
    else:
        idx = _migration_idx(last_migration)
        new_idx = "%04d" % (int(idx) + 1)
        slugify_name = "_".join(name.split())
        new_file_name = f"{MIGRATIONS_FOLDER}/{new_idx}_{slugify_name}.sql"
        Path(f"{new_file_name}").touch()
        click.echo(f"The migration file is created: {new_file_name}")


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
        "errors": "red",
        "new_apply": "green",
        "to_apply": "white",
        "already_applied": "white",
    }
    migrations = _sorted_file_migrations()
    with cli_exception_handler(debug):

        async def _migrate(db):
            async with spinner("Migrate"):
                result = await migrate_db(db, migrations, dry_run)
                error = None
                for key, values in result.items():
                    color = result_colors.get(key, "white")
                    for value in values:
                        if key in ["already_applied", "new_apply"]:
                            value = f"{value} [X]"
                        else:
                            if key == "errors":
                                error = value[1]
                                value = value[0]
                            value = f"{value} [ ]"
                        click.secho(value, fg=color)
                        if error:
                            raise click.ClickException(error)

        trio_run(_migrate, db, use_asyncio=True)
