# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 (eventually AGPL-3.0) 2016-present Scille SAS
from __future__ import annotations

import importlib.resources
import re
from base64 import b64decode, b64encode
from datetime import datetime
from functools import wraps
from typing import Any, Awaitable, Callable, Coroutine, Iterable, List, Tuple
from uuid import uuid4

import attr
import trio
import trio_typing
import triopg
from structlog import get_logger
from triopg import PostgresError, UndefinedTableError, UniqueViolationError
from typing_extensions import ParamSpec

from parsec._parsec import DateTime
from parsec.backend.backend_events import BackendEvent, backend_event_serializer
from parsec.backend.postgresql import migrations as migrations_module
from parsec.event_bus import EventBus
from parsec.serde import SerdePackingError, SerdeValidationError
from parsec.utils import TaskStatus, start_task

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


@attr.s(slots=True, auto_attribs=True)
class MigrationItem:
    idx: int
    name: str
    file_name: str
    sql: str


def retrieve_migrations() -> List[MigrationItem]:
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


@attr.s(slots=True, auto_attribs=True)
class MigrationResult:
    already_applied: List[MigrationItem]
    new_apply: List[MigrationItem]
    error: Tuple[MigrationItem, str] | None


async def apply_migrations(
    url: str, migrations: Iterable[MigrationItem], dry_run: bool
) -> MigrationResult:
    """
    Returns: MigrationResult
    """
    async with triopg.connect(url) as conn:
        return await _apply_migrations(conn, migrations, dry_run)


async def _apply_migrations(
    conn: triopg._triopg.TrioConnectionProxy, migrations: Iterable[MigrationItem], dry_run: bool
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


async def _apply_migration(
    conn: triopg._triopg.TrioConnectionProxy, migration: MigrationItem
) -> None:
    async with conn.transaction():
        await conn.execute(migration.sql)
        if migration.idx >= CREATE_MIGRATION_TABLE_ID:
            # The migration table is created in the second migration
            sql = "INSERT INTO migration (_id, name, applied) VALUES ($1, $2, $3)"
            await conn.execute(sql, migration.idx, migration.name, datetime.now())


async def _last_migration_row(conn: triopg._triopg.TrioConnectionProxy) -> int:
    query = "SELECT _id FROM migration ORDER BY applied desc LIMIT 1"
    return await conn.fetchval(query)


async def _is_initial_migration_applied(conn: triopg._triopg.TrioConnectionProxy) -> bool:
    query = "SELECT _id FROM organization LIMIT 1"
    try:
        await conn.fetchval(query)
    except UndefinedTableError:
        return False
    else:
        return True


async def _idx_limit(conn: triopg._triopg.TrioConnectionProxy) -> int:
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


async def handle_datetime(conn: triopg._triopg.TrioConnectionProxy) -> None:
    await conn.set_type_codec(
        "timestamptz",
        encoder=lambda x: (int(x.timestamp() * 1000000) - MICRO_SECONDS_BETWEEN_1970_AND_2000,),
        decoder=lambda x: DateTime.from_timestamp(
            (x[0] + MICRO_SECONDS_BETWEEN_1970_AND_2000) / 1000000
        ),
        schema="pg_catalog",
        format="tuple",
    )


async def handle_uuid(conn: triopg._triopg.TrioConnectionProxy) -> None:
    await conn.set_type_codec(
        "uuid",
        encoder=lambda x: x.hex,
        decoder=lambda x: x,
        schema="pg_catalog",
    )


# TODO: replace by a function
class PGHandler:
    def __init__(self, url: str, min_connections: int, max_connections: int, event_bus: EventBus):
        self.url = url
        self.min_connections = min_connections
        self.max_connections = max_connections
        self.event_bus = event_bus
        self.pool: triopg._triopg.TrioPoolProxy
        self.notification_conn: triopg._triopg.TrioConnectionProxy
        self._task_status: TaskStatus[None] | None = None
        self._connection_lost = False

    async def init(self, nursery: trio.Nursery) -> None:
        self._task_status = await start_task(nursery, self._run_connections)

    async def _run_connections(
        self, task_status: trio_typing.TaskStatus[None] = trio.TASK_STATUS_IGNORED
    ) -> None:

        # By default AsyncPG only work with Python standard `datetime.DateTime`
        # for timestamp types, here we override this behavior to uses our own custom
        # - DateTime type
        # - Uuid type
        async def _init_connection(conn: triopg._triopg.TrioConnectionProxy) -> None:
            await handle_datetime(conn)
            await handle_uuid(conn)

        async with triopg.create_pool(
            self.url,
            min_size=self.min_connections,
            max_size=self.max_connections,
            init=_init_connection,
        ) as self.pool:
            # This connection is dedicated to the notifications listening, so it
            # would only complicate stuff to include it into the connection pool
            async with triopg.connect(self.url) as self.notification_conn:
                self.notification_conn.add_termination_listener(
                    self._on_notification_conn_termination
                )
                await self.notification_conn.add_listener("app_notification", self._on_notification)
                task_status.started()
                try:
                    await trio.sleep_forever()
                finally:
                    if self._connection_lost:
                        raise ConnectionError("PostgreSQL notification query has been lost")

    # Notification listening is achieve by a never-ending LISTEN
    # query to PostgreSQL.
    # If this query is terminated (most likely because the database has
    # been restarted) we might miss some notifications.
    # Hence all client connections should be closed in order not to mislead
    # them into thinking no notifications has occurred.
    # And the simplest way to do that is to raise a big exception in _run_connections ;-)
    def _on_notification_conn_termination(self, conn: triopg._triopg.TrioConnectionProxy) -> None:
        self._connection_lost = True
        if self._task_status:
            self._task_status.cancel()

    def _on_notification(
        self, conn: triopg._triopg.TrioConnectionProxy, pid: int, channel: str, payload: str
    ) -> None:
        try:
            data = backend_event_serializer.loads(b64decode(payload.encode("ascii")))
        except (SerdeValidationError, SerdePackingError, ValueError) as exc:
            logger.warning(
                "Invalid notif received", pid=pid, channel=channel, payload=payload, exc_info=exc
            )
            return

        logger.debug("notif received", pid=pid, channel=channel, payload=payload)
        data.pop("__id__")  # Simply discard the notification id
        signal = data.pop("__signal__")
        self.event_bus.send(signal, **data)

    async def teardown(self) -> None:
        if self._task_status:
            await self._task_status.cancel_and_join()


async def send_signal(
    conn: triopg._triopg.TrioConnectionProxy, signal: BackendEvent, **kwargs: Any
) -> None:
    # PostgreSQL's NOTIFY only accept string as payload, hence we must
    # use base64 on our payload...

    # Add UUID to ensure the payload is unique given it seems Postgresql can
    # drop duplicated NOTIFY (same channel/payload)
    # see: https://github.com/Scille/parsec-cloud/issues/199
    kwargs["__id__"] = uuid4().hex
    kwargs["__signal__"] = signal
    raw_data = b64encode(backend_event_serializer.dumps(kwargs)).decode("ascii")
    await conn.execute("SELECT pg_notify($1, $2)", "app_notification", raw_data)
    logger.debug("notif sent", signal=signal, kwargs=kwargs)
