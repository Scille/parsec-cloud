# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio
import datetime
import triopg
import collections
from triopg import UniqueViolationError, UndefinedTableError, PostgresError
from uuid import uuid4
from functools import wraps
from structlog import get_logger
from base64 import b64decode, b64encode
from importlib_resources import read_text


from parsec.event_bus import EventBus
from parsec.serde import packb, unpackb
from parsec.utils import start_task
from parsec.backend.postgresql.tables import STR_TO_REALM_ROLE
from parsec.backend.postgresql import migrations

logger = get_logger()


async def migrate_db(url: str, migration_files: list, dry_run: bool) -> None:
    """
    Returns: A result dict
    Raises:
        triopg.exceptions.PostgresError
    """
    async with triopg.connect(url) as conn:
        return await _migrate_db(conn, migration_files, dry_run)


async def _migrate_db(conn, migration_files, dry_run):

    errors = []
    allready_applied = []
    new_apply = []
    to_apply = []
    rows = []

    idx_limit, error = await _idx_limit(conn, migration_files)
    if not error:
        for idx, migration in enumerate(migration_files, 1):
            if idx <= idx_limit:
                allready_applied.append(migration)
            else:
                if not dry_run:
                    error = await _apply_migration(conn, migration)
                    if error:
                        errors.append(error)
                        break
                    else:
                        new_apply.append(migration)
                        rows.append((migration, datetime.datetime.utcnow()))
                else:
                    to_apply.append(migration)

        if rows:
            statement = "INSERT INTO migration (name, applied) VALUES ($1, $2)"
            await conn.executemany(statement, rows)

    return collections.OrderedDict(
        (
            ("already_applied", allready_applied),
            ("new_apply", new_apply),
            ("to_apply", to_apply),
            ("errors", errors),
        )
    )


async def _apply_migration(conn, migration_file):
    async with conn.transaction():
        query = read_text(migrations, f"{migration_file}")
        error = None
        if not query:
            error = (migration_file, "Empty migration file")
        else:
            try:
                await conn.execute(query)
            except PostgresError as e:
                error = (migration_file, e.message)
    return error


async def _last_migration_row(conn):
    query = """
        SELECT name FROM migration ORDER BY applied desc LIMIT 1
    """
    return await conn.fetchval(query)


async def _idx_limit(conn, migration_files):
    idx_limit = None
    error = None
    try:
        last_migration_row = await _last_migration_row(conn)
    except UndefinedTableError:
        idx_limit = 0
    else:
        if last_migration_row:
            idx_limit, error = _last_applied_migration_file_idx(migration_files, last_migration_row)
        else:
            idx_limit = 0

    return idx_limit, error


def _last_applied_migration_file_idx(migration_files, last_applied_migration):
    idx = None
    error = None
    try:
        idx = migration_files.index(last_applied_migration) + 1
    except ValueError:
        error = ("migrations", "Inconsistent package")

    return idx, error


async def init_db(url: str) -> None:
    """
    Returns: if the database was already initialized
    Raises:
        triopg.exceptions.PostgresError
    """
    async with triopg.connect(url) as conn:
        return await _init_db(conn)


async def _init_db(conn):
    if await _is_db_initialized(conn):
        return True

    async with conn.transaction():
        query = read_text(__package__, f"init_tables.sql")
        await conn.execute(query)
    return False


async def _is_db_initialized(conn):
    # TODO: not a really elegant way to determine if the database
    # is already initialized...
    root_key = await conn.fetchrow(
        """
        SELECT true FROM pg_catalog.pg_tables WHERE tablename = 'user_';
        """
    )
    return bool(root_key)


def retry_on_unique_violation(fn):
    @wraps(fn)
    async def wrapper(*args, **kwargs):
        while True:
            try:
                return await fn(*args, **kwargs)
            except UniqueViolationError as exc:
                logger.warning("unique violation error, retrying...", exc_info=exc)

    return wrapper


# TODO: replace by a fonction
class PGHandler:
    def __init__(self, url: str, min_connections: int, max_connections: int, event_bus: EventBus):
        self.url = url
        self.min_connections = min_connections
        self.max_connections = max_connections
        self.event_bus = event_bus
        self.pool = None
        self.notification_conn = None
        self._task_status = None

    async def init(self, nursery):
        self._task_status = await start_task(nursery, self._run_connections)

    async def _run_connections(self, task_status=trio.TASK_STATUS_IGNORED):
        async with triopg.create_pool(
            self.url, min_size=self.min_connections, max_size=self.max_connections
        ) as self.pool:
            # This connection is dedicated to the notifications listening, so it
            # would only complicate stuff to include it into the connection pool
            async with triopg.connect(self.url) as self.notification_conn:
                await self.notification_conn.add_listener("app_notification", self._on_notification)
                task_status.started()
                await trio.sleep_forever()

    def _on_notification(self, connection, pid, channel, payload):
        data = unpackb(b64decode(payload.encode("ascii")))
        data.pop("__id__")  # Simply discard the notification id
        signal = data.pop("__signal__")
        logger.debug("notif received", pid=pid, channel=channel, payload=payload)
        # Kind of a hack, but fine enough for the moment
        if signal == "realm.roles_updated":
            data["role"] = STR_TO_REALM_ROLE.get(data.pop("role_str"))
        self.event_bus.send(signal, **data)

    async def teardown(self):
        if self._task_status:
            await self._task_status.cancel_and_join()


async def send_signal(conn, signal, **kwargs):
    # PostgreSQL's NOTIFY only accept string as payload, hence we must
    # use base64 on our payload...

    # Add UUID to ensure the payload is unique given it seems Postgresql can
    # drop duplicated NOTIFY (same channel/payload)
    # see: https://github.com/Scille/parsec-cloud/issues/199
    raw_data = b64encode(packb({"__id__": uuid4().hex, "__signal__": signal, **kwargs})).decode(
        "ascii"
    )
    await conn.execute("SELECT pg_notify($1, $2)", "app_notification", raw_data)
    logger.debug("notif sent", signal=signal, kwargs=kwargs)
