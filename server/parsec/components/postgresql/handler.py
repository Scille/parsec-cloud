# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import importlib.resources
import re
from contextlib import asynccontextmanager
from dataclasses import dataclass
from datetime import datetime
from functools import wraps
from typing import AsyncIterator, Awaitable, Callable, Coroutine, Iterable
from uuid import uuid4

import asyncpg
from asyncpg import PostgresError, UndefinedTableError, UniqueViolationError
from typing_extensions import ParamSpec

from parsec._parsec import ActiveUsersLimit, DateTime
from parsec.components.postgresql import AsyncpgConnection, AsyncpgPool
from parsec.events import AnyEvent, Event
from parsec.logging import get_logger

from . import migrations as migrations_module

P = ParamSpec("P")

logger = get_logger()

MIGRATION_FILE_PATTERN = r"^(?P<id>\d{4})_(?P<name>\w*).sql$"
# Expose migration table here to simply modify it during tests
MIGRATION_TABLE = "migration"

# The duration between 1970 and 2000 in microseconds.
#
# Internally, PostgreSQL stores timestamps as 4bytes floating number representing the number of seconds since
# 2000-01-01T00:00:00Z, hence we need to do back-and-forth conversion with the regular 1970-based POSIX epoch
# > We use int to avoid float error
MICRO_SECONDS_BETWEEN_1970_AND_2000: int = 946684800 * 1000000


@dataclass(slots=True)
class MigrationItem:
    index: int
    name: str
    file_name: str
    sql: str


def retrieve_migrations() -> list[MigrationItem]:
    migrations = []
    indexes = []
    for file in importlib.resources.files(migrations_module).iterdir():
        file_name = file.name
        match = re.search(MIGRATION_FILE_PATTERN, file_name)
        if match:
            index = int(match.group("id"))
            # Sanity checks
            if index == 0:
                raise AssertionError(
                    f"Inconsistent package: migration with index {index} is not allowed"
                )
            if index in indexes:
                raise AssertionError(
                    f"Inconsistent package: multiples migrations with index {index}"
                )
            indexes.append(index)
            sql = importlib.resources.files(migrations_module).joinpath(file_name).read_text()
            contains_only_comments = all(line.strip().startswith("--") for line in sql.splitlines())
            if contains_only_comments:
                raise AssertionError(f"Empty migration file {file_name}")
            migrations.append(
                MigrationItem(index=index, name=match.group("name"), file_name=file_name, sql=sql)
            )

    migrations = sorted(migrations, key=lambda item: item.index)

    previous_migration = None
    for migration in migrations:
        if previous_migration and migration.index != previous_migration.index + 1:
            raise AssertionError(
                f"Inconsistent package: there is hole in indexes between `{previous_migration.file_name}` and `{migration.file_name}`"
            )
        previous_migration = migration

    return migrations


@dataclass(slots=True)
class MigrationResult:
    already_applied: list[MigrationItem]
    new_apply: list[MigrationItem]
    error: tuple[MigrationItem, str] | None


async def apply_migrations(
    url: str, migrations: Iterable[MigrationItem], dry_run: bool
) -> MigrationResult:
    """
    Returns: MigrationResult
    """
    conn = await asyncpg.connect(url)
    try:
        return await _apply_migrations(conn, migrations, dry_run)
    finally:
        await conn.close()


async def _apply_migrations(
    conn: AsyncpgConnection, migrations: Iterable[MigrationItem], dry_run: bool
) -> MigrationResult:
    error = None
    already_applied = []
    new_apply = []

    last_migration_index = await _get_last_migration_index(conn)
    for migration in migrations:
        if last_migration_index is not None and migration.index <= last_migration_index:
            already_applied.append(migration)
        else:
            if not dry_run:
                try:
                    await _apply_migration(conn, migration)
                except PostgresError as e:
                    error = (migration, e.message)
                    break
            new_apply.append(migration)

    return MigrationResult(already_applied=already_applied, new_apply=new_apply, error=error)


async def _apply_migration(conn: AsyncpgConnection, migration: MigrationItem) -> None:
    async with conn.transaction():
        await conn.execute(migration.sql)
        sql = f"INSERT INTO {MIGRATION_TABLE} (_id, name, applied) VALUES ($1, $2, $3)"
        result = await conn.execute(sql, migration.index, migration.name, datetime.now())
        assert result == "INSERT 0 1", result


async def _get_last_migration_index(conn: AsyncpgConnection) -> int | None:
    query = f"SELECT _id FROM {MIGRATION_TABLE} ORDER BY applied desc LIMIT 1"
    try:
        return await conn.fetchval(query)
    except UndefinedTableError:
        # The database is empty (it is the first migration that creates, among other
        # things, the `migration` table).
        return None


def retry_on_unique_violation(
    fn: Callable[P, Awaitable[None]],
) -> Callable[P, Coroutine[None, None, None]]:
    @wraps(fn)
    async def wrapper(*args: P.args, **kwargs: P.kwargs) -> None:
        while True:
            try:
                return await fn(*args, **kwargs)
            except UniqueViolationError as exc:
                logger.warning("unique violation error, retrying...", exc_info=exc)

    return wrapper


async def handle_datetime(conn: AsyncpgConnection) -> None:
    await conn.set_type_codec(
        "timestamptz",
        encoder=lambda x: (x.as_timestamp_micros() - MICRO_SECONDS_BETWEEN_1970_AND_2000,),
        decoder=lambda x: DateTime.from_timestamp_micros(
            x[0] + MICRO_SECONDS_BETWEEN_1970_AND_2000
        ),
        schema="pg_catalog",
        format="tuple",
    )


async def handle_uuid(conn: AsyncpgConnection) -> None:
    await conn.set_type_codec(
        "uuid",
        encoder=lambda x: x.hex,
        decoder=lambda x: x,
        schema="pg_catalog",
    )


async def handle_integer(conn: AsyncpgConnection) -> None:
    def _encode(x: int | ActiveUsersLimit) -> str:
        if isinstance(x, ActiveUsersLimit):
            # encoder cannot return `None`. `NO_LIMIT` case should be handled before the insertion/update
            assert x is not ActiveUsersLimit.NO_LIMIT
            value = x.to_maybe_int()
            assert value is not None
            return str(value)
        return str(x)

    await conn.set_type_codec(
        "integer",
        encoder=_encode,
        decoder=lambda x: int(x) if x is not None else None,
        schema="pg_catalog",
        format="text",
    )


@asynccontextmanager
async def asyncpg_pool_factory(
    url: str, min_connections: int, max_connections: int
) -> AsyncIterator[AsyncpgPool]:
    # By default AsyncPG only work with Python standard `datetime.DateTime`
    # for timestamp types, here we override this behavior to uses our own custom
    # - DateTime type
    # - Uuid type
    async def _init_connection(conn: AsyncpgConnection) -> None:
        await handle_datetime(conn)
        await handle_uuid(conn)
        await handle_integer(conn)

    async with asyncpg.create_pool(
        url,
        min_size=min_connections,
        max_size=max_connections,
        init=_init_connection,
    ) as pool:
        yield pool


async def send_signal(conn: AsyncpgConnection, event: Event) -> None:
    # Add UUID to ensure the payload is unique given it seems Postgresql can
    # drop duplicated NOTIFY (same channel/payload)
    # see: https://github.com/Scille/parsec-cloud/issues/199
    event_id = uuid4().hex
    any_event = AnyEvent(event=event)
    raw_event = any_event.model_dump_json()
    payload = f"{event_id}:{raw_event}"
    await conn.execute("SELECT pg_notify($1, $2)", "app_notification", payload)


def parse_signal(payload: str) -> Event:
    _, raw_event = payload.split(":", maxsplit=1)
    any_event = AnyEvent.model_validate_json(raw_event)
    return any_event.event
