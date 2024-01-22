# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import importlib.resources
import re
from base64 import b64decode, b64encode
from dataclasses import dataclass
from datetime import datetime
from functools import wraps
from typing import Awaitable, Callable, Coroutine, Iterable
from uuid import uuid4

import anyio
import asyncpg
from anyio.abc import TaskGroup
from asyncpg import PostgresError, UndefinedTableError, UniqueViolationError
from structlog.stdlib import get_logger
from typing_extensions import ParamSpec

from parsec._parsec import ActiveUsersLimit, DateTime
from parsec.components.events import BaseEventsComponent
from parsec.joinable_task import JoinableTaskStatus, start_joinable_task

from . import migrations as migrations_module

P = ParamSpec("P")

logger = get_logger()

CREATE_MIGRATION_TABLE_ID = 2
MIGRATION_FILE_PATTERN = r"^(?P<id>\d{4})_(?P<name>\w*).sql$"

# The duration between 1970 and 2000 in microseconds.
#
# Internally, PostgreSQL stores timestamps as 4bytes floating number representing the number of seconds since
# 2000-01-01T00:00:00Z, hence we need to do back-and-forth conversion with the regular 1970-based POSIX epoch
# > We use int to avoid float error
MICRO_SECONDS_BETWEEN_1970_AND_2000: int = 946684800 * 1000000


@dataclass(slots=True)
class MigrationItem:
    idx: int
    name: str
    file_name: str
    sql: str


def retrieve_migrations() -> list[MigrationItem]:
    migrations = []
    ids = []
    for file in importlib.resources.files(migrations_module).iterdir():
        file_name = file.name
        match = re.search(MIGRATION_FILE_PATTERN, file_name)
        if match:
            idx = int(match.group("id"))
            # Sanity check
            if idx in ids:
                raise AssertionError(
                    f"Inconsistent package (multiples migrations with {idx} as id)"
                )
            ids.append(idx)
            sql = importlib.resources.files(migrations_module).joinpath(file_name).read_text()
            if not sql:
                raise AssertionError(f"Empty migration file {file_name}")
            migrations.append(
                MigrationItem(idx=idx, name=match.group("name"), file_name=file_name, sql=sql)
            )

    return sorted(migrations, key=lambda item: item.idx)


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
    conn: asyncpg.Connection, migrations: Iterable[MigrationItem], dry_run: bool
) -> MigrationResult:
    error = None
    already_applied = []
    new_apply = []

    idx_limit = await _idx_limit(conn)
    for migration in migrations:
        if migration.idx <= idx_limit:
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


async def _apply_migration(conn: asyncpg.Connection, migration: MigrationItem) -> None:
    async with conn.transaction():
        await conn.execute(migration.sql)
        if migration.idx >= CREATE_MIGRATION_TABLE_ID:
            # The migration table is created in the second migration
            sql = "INSERT INTO migration (_id, name, applied) VALUES ($1, $2, $3)"
            await conn.execute(sql, migration.idx, migration.name, datetime.now())


async def _last_migration_row(conn: asyncpg.Connection) -> int:
    query = "SELECT _id FROM migration ORDER BY applied desc LIMIT 1"
    return await conn.fetchval(query)


async def _is_initial_migration_applied(conn: asyncpg.Connection) -> bool:
    query = "SELECT _id FROM organization LIMIT 1"
    try:
        await conn.fetchval(query)
    except UndefinedTableError:
        return False
    else:
        return True


async def _idx_limit(conn: asyncpg.Connection) -> int:
    idx_limit = 0
    try:
        idx_limit = await _last_migration_row(conn)
    except UndefinedTableError:
        # The migration table is created in the second migration
        if await _is_initial_migration_applied(conn):
            # The initial table may be created before the migrate command with the old way
            idx_limit = 1
    return idx_limit


def retry_on_unique_violation(
    fn: Callable[P, Awaitable[None]]
) -> Callable[P, Coroutine[None, None, None]]:
    @wraps(fn)
    async def wrapper(*args: P.args, **kwargs: P.kwargs) -> None:
        while True:
            try:
                return await fn(*args, **kwargs)
            except UniqueViolationError as exc:
                logger.warning("unique violation error, retrying...", exc_info=exc)

    return wrapper


async def handle_datetime(conn: asyncpg.Connection) -> None:
    await conn.set_type_codec(
        "timestamptz",
        encoder=lambda x: (int(x.timestamp() * 1000000) - MICRO_SECONDS_BETWEEN_1970_AND_2000,),
        decoder=lambda x: DateTime.from_timestamp(
            (x[0] + MICRO_SECONDS_BETWEEN_1970_AND_2000) / 1000000
        ),
        schema="pg_catalog",
        format="tuple",
    )


async def handle_uuid(conn: asyncpg.Connection) -> None:
    await conn.set_type_codec(
        "uuid",
        encoder=lambda x: x.hex,
        decoder=lambda x: x,
        schema="pg_catalog",
    )


async def handle_integer(conn: asyncpg.Connection) -> None:
    def _encode(x: int | ActiveUsersLimit) -> str:
        if isinstance(x, ActiveUsersLimit):
            # encoder cannot return `None`. `NO_LIMIT` case should be handled before the insertion/update
            assert x is not ActiveUsersLimit.NO_LIMIT
            return str(x.to_int())
        return str(x)

    await conn.set_type_codec(
        "integer",
        encoder=_encode,
        decoder=lambda x: int(x) if x is not None else None,
        schema="pg_catalog",
        format="text",
    )


# TODO: replace by a function
class PGHandler:
    def __init__(self, url: str, min_connections: int, max_connections: int, event_bus: EventBus):
        self.url = url
        self.min_connections = min_connections
        self.max_connections = max_connections
        self.event_bus = event_bus
        self.pool: asyncpg.Pool
        self.notification_conn: asyncpg.Connection
        self._task_status: JoinableTaskStatus[None] | None = None
        self._connection_lost = False
        self._events_component: BaseEventsComponent | None = None

    async def init(
        self, task_group: TaskGroup, events_component: BaseEventsComponent | None
    ) -> None:
        self._task_status = await start_joinable_task(task_group, self._run_connections)
        self._events_component = events_component

    async def _run_connections(
        self, task_status: anyio.TaskStatus[None] = anyio.TASK_STATUS_IGNORED
    ) -> None:
        # By default AsyncPG only work with Python standard `datetime.DateTime`
        # for timestamp types, here we override this behavior to uses our own custom
        # - DateTime type
        # - Uuid type
        async def _init_connection(conn: asyncpg.Connection) -> None:
            await handle_datetime(conn)
            await handle_uuid(conn)
            await handle_integer(conn)

        async with asyncpg.create_pool(
            self.url,
            min_size=self.min_connections,
            max_size=self.max_connections,
            init=_init_connection,
        ) as self.pool:
            # This connection is dedicated to the notifications listening, so it
            # would only complicate stuff to include it into the connection pool
            self.notification_conn = await asyncpg.connect(self.url)
            try:
                self.notification_conn.add_termination_listener(
                    self._on_notification_conn_termination
                )
                await self.notification_conn.add_listener("app_notification", self._on_notification)
                task_status.started()
                try:
                    await anyio.sleep_forever()
                finally:
                    if self._connection_lost:
                        raise ConnectionError("PostgreSQL notification query has been lost")

            finally:
                await self.notification_conn.close()

    # Notification listening is achieve by a never-ending LISTEN
    # query to PostgreSQL.
    # If this query is terminated (most likely because the database has
    # been restarted) we might miss some notifications.
    # Hence all client connections should be closed in order not to mislead
    # them into thinking no notifications has occurred.
    # And the simplest way to do that is to raise a big exception in _run_connections ;-)
    def _on_notification_conn_termination(self, conn: asyncpg.Connection) -> None:
        self._connection_lost = True
        if self._task_status:
            self._task_status.cancel()

    def _on_notification(
        self, conn: asyncpg.Connection, pid: int, channel: str, payload: str
    ) -> None:
        try:
            event_id, raw_event = payload.split(":")
            event = BackendEvent.load(b64decode(raw_event.encode("ascii")))
        except ValueError as exc:
            logger.warning(
                "Invalid notif received", pid=pid, channel=channel, payload=payload, exc_info=exc
            )
            return

        if self._events_component:
            self._events_component.add_event_to_cache(event_id, event)
        self.event_bus.send(type(event), event_id=event_id, payload=event)

    async def teardown(self) -> None:
        if self._task_status:
            await self._task_status.cancel_and_join()


async def send_signal(conn: asyncpg.Connection, event: BackendEvent) -> None:
    # PostgreSQL's NOTIFY only accept string as payload, hence we must
    # use base64 on our payload...
    raw_event = b64encode(event.dump()).decode("ascii")
    event_id = uuid4().hex
    payload = f"{event_id}:{raw_event}"
    await conn.execute("SELECT pg_notify($1, $2)", "app_notification", payload)
