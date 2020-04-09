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

CREATE_MIGRATION_TABLE_ID = 2


async def migrate_db(url: str, migrations: list, dry_run: bool) -> None:
    """
    Returns: A result dict
    Raises:
        triopg.exceptions.PostgresError
    """
    async with triopg.connect(url) as conn:
        return await _migrate_db(conn, migrations, dry_run)


async def _migrate_db(conn, migrations, dry_run):

    errors = []
    allready_applied = []
    new_apply = []
    to_apply = []

    idx_limit = await _idx_limit(conn)
    for idx, name, file_name in migrations:
        if idx <= idx_limit:
            allready_applied.append(file_name)
        else:
            if not dry_run:
                async with conn.transaction():
                    error = await _apply_migration(conn, idx, name, file_name)
                    if error:
                        errors.append(error)
                        break
                    else:
                        if idx >= CREATE_MIGRATION_TABLE_ID:
                            # The migration table is created in the second migration
                            await _add_migration_to_db(conn, idx, name)
                        new_apply.append(file_name)
            else:
                to_apply.append(file_name)

    return collections.OrderedDict(
        (
            ("already_applied", allready_applied),
            ("new_apply", new_apply),
            ("to_apply", to_apply),
            ("errors", errors),
        )
    )


async def _apply_migration(conn, idx, name, migration_file):
    query = read_text(migrations, migration_file)
    error = None
    if not query:
        error = (migration_file, "Empty migration file")
    else:
        try:
            await conn.execute(query)
        except PostgresError as e:
            error = (migration_file, e.message)
    return error


async def _add_migration_to_db(conn, idx, name):
    statement = "INSERT INTO migration (_id, name, applied) VALUES ($1, $2, $3)"
    await conn.execute(statement, idx, name, datetime.datetime.utcnow())


async def _last_migration_row(conn):
    query = """
        SELECT _id FROM migration ORDER BY applied desc LIMIT 1
    """
    return await conn.fetchval(query)


async def _idx_limit(conn):
    idx_limit = 0
    try:
        idx_limit = await _last_migration_row(conn)
    except UndefinedTableError:
        # The migration table is created in the second migration
        idx_limit = 0
    return idx_limit


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
