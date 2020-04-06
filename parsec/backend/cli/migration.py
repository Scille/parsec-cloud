# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import click
import glob
import re

from pathlib import Path

MIGRATIONS_FOLDER = "parsec/backend/postgresql/migrations"
FILE_PATTERN = r"(?P<idx>\d{4})_\w*.sql"


def _sorted_file_migrations():
    file_pattern = r"\S*%s" % FILE_PATTERN
    files = [f for f in glob.glob(f"{MIGRATIONS_FOLDER}/*.sql") if re.match(file_pattern, f)]
    return sorted(files, key=lambda f: re.sub(file_pattern, r"\g<1>", f))


def _migration_idx(file_name):
    match = re.search(FILE_PATTERN, file_name)
    return match.group("idx")


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
