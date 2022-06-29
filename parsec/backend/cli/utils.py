# Parsec Cloud (https://parsec.cloud) Copyright (c) BSLv1.1 (eventually AGPLv3) 2016-2021 Scille SAS

import click


def db_backend_options(fn):
    decorators = [
        click.option(
            "--db",
            required=True,
            envvar="PARSEC_DB",
            metavar="URL",
            help="""Database configuration.
Allowed values:
\b
-`MOCKED`: Mocked in memory
-`postgresql://<...>`: Use PostgreSQL database
\b
""",
        ),
        click.option(
            "--db-min-connections",
            default=5,
            show_default=True,
            envvar="PARSEC_DB_MIN_CONNECTIONS",
            help="Minimal number of connections to the database if using PostgreSQL",
        ),
        click.option(
            "--db-max-connections",
            default=7,
            show_default=True,
            envvar="PARSEC_DB_MAX_CONNECTIONS",
            help="Maximum number of connections to the database if using PostgreSQL",
        ),
    ]
    for decorator in decorators:
        fn = decorator(fn)
    return fn
