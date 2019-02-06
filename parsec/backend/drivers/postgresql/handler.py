import trio
import triopg
from structlog import get_logger
from base64 import b64decode, b64encode

from parsec.event_bus import EventBus
from parsec.serde import packb, unpackb
from parsec.utils import start_task


logger = get_logger()


async def init_db(url: str, force: bool = False) -> bool:
    """
    Raises:
        triopg.exceptions.PostgresError
    """
    async with triopg.connect(url) as conn:
        return await _init_db(conn, force)


async def _init_db(conn, force: bool = False) -> bool:
    if force:
        async with conn.transaction():
            await _drop_db(conn)

    already_initialized = await _is_db_initialized(conn)

    if not already_initialized:
        async with conn.transaction():
            await _create_db_tables(conn)

    return already_initialized


async def _drop_db(conn):
    # TODO: Ideally we would totally drop the db here instead of just
    # the databases we know...
    await conn.execute(
        """
        DROP TABLE IF EXISTS
            organizations,

            users,
            devices,

            user_invitations,
            device_invitations,

            messages,
            vlobs,
            beacons,

            blockstore
        CASCADE;

        DROP TYPE IF EXISTS
            USER_INVITATION_STATUS,
            DEVICE_INVITATION_STATUS,
            DEVICE_CONF_TRY_STATUS
        CASCADE;
    """
    )


async def _is_db_initialized(conn):
    # TODO: not a really elegant way to determine if the database
    # is already initialized...
    root_key = await conn.fetchrow(
        """
        SELECT true FROM pg_catalog.pg_tables WHERE tablename = 'users';
        """
    )
    return bool(root_key)


async def _create_db_tables(conn):
    await conn.execute(
        """
        CREATE TABLE organizations (
            _id SERIAL PRIMARY KEY,
            organization_id VARCHAR(32) UNIQUE NOT NULL,
            bootstrap_token TEXT NOT NULL,
            root_verify_key BYTEA
        );

        CREATE TABLE users (
            _id SERIAL PRIMARY KEY,
            organization INTEGER REFERENCES organizations (_id),
            user_id VARCHAR(32) NOT NULL,
            is_admin BOOLEAN,
            certified_user BYTEA,
            user_certifier INTEGER,
            created_on TIMESTAMPTZ NOT NULL,
            UNIQUE(organization, user_id)
        );

        CREATE TABLE devices (
            _id SERIAL PRIMARY KEY,
            organization INTEGER REFERENCES organizations (_id),
            user_ INTEGER REFERENCES users (_id) NOT NULL,
            device_id VARCHAR(65) NOT NULL,
            certified_device BYTEA,
            device_certifier INTEGER REFERENCES devices (_id),
            created_on TIMESTAMPTZ NOT NULL,
            revocated_on TIMESTAMPTZ,
            certified_revocation BYTEA,
            revocation_certifier INTEGER REFERENCES devices (_id),
            UNIQUE(organization, device_id),
            UNIQUE(user_, device_id)
        );

        ALTER TABLE users
        ADD CONSTRAINT FK_users_devices FOREIGN KEY (user_certifier) REFERENCES devices (_id);

        CREATE TABLE user_invitations (
            _id SERIAL PRIMARY KEY,
            organization INTEGER REFERENCES organizations (_id),
            user_id VARCHAR(32) UNIQUE NOT NULL,
            creator INTEGER REFERENCES devices (_id) NOT NULL,
            created_on TIMESTAMPTZ NOT NULL
        );

        CREATE TABLE device_invitations (
            _id SERIAL PRIMARY KEY,
            organization INTEGER REFERENCES organizations (_id),
            device_id VARCHAR(65) UNIQUE NOT NULL,
            creator INTEGER REFERENCES devices (_id) NOT NULL,
            created_on TIMESTAMPTZ NOT NULL
        );

        CREATE TABLE messages (
            _id SERIAL PRIMARY KEY,
            organization INTEGER REFERENCES organizations (_id),
            recipient INTEGER REFERENCES users (_id) NOT NULL,
            sender INTEGER REFERENCES devices (_id) NOT NULL,
            body BYTEA NOT NULL
        );

        CREATE TABLE vlobs (
            _id SERIAL PRIMARY KEY,
            organization INTEGER REFERENCES organizations (_id),
            vlob_id UUID NOT NULL,
            version INTEGER NOT NULL,
            rts TEXT NOT NULL,
            wts TEXT NOT NULL,
            blob BYTEA NOT NULL,
            author INTEGER REFERENCES devices (_id) NOT NULL,
            UNIQUE(vlob_id, version)
        );

        -- TODO: link to organization...
        CREATE TABLE beacons (
            _id SERIAL PRIMARY KEY,
            organization INTEGER REFERENCES organizations (_id),
            beacon_id UUID NOT NULL,
            beacon_index INTEGER NOT NULL,
            src_id UUID NOT NULL,
            -- src_id UUID REFERENCES vlobs (vlob_id) NOT NULL,
            src_version INTEGER NOT NULL
        );

        CREATE TABLE blockstore (
            _id SERIAL PRIMARY KEY,
            organization INTEGER REFERENCES organizations (_id),
            block_id UUID UNIQUE NOT NULL,
            block BYTEA NOT NULL,
            author INTEGER REFERENCES devices (_id) NOT NULL
        );
    """
    )


# TODO: replace by a fonction
class PGHandler:
    def __init__(self, url: str, event_bus: EventBus):
        self.url = url
        self.event_bus = event_bus
        self.pool = None
        self.notification_conn = None
        self._task_status = None

    async def init(self, nursery):
        self._task_status = await start_task(nursery, self._run_connections)

    async def _run_connections(self, task_status=trio.TASK_STATUS_IGNORED):
        async with triopg.create_pool(self.url) as self.pool:
            async with self.pool.acquire() as conn:
                if not await _is_db_initialized(conn):
                    raise RuntimeError("Database not initialized !")
            # This connection is dedicated to the notifications listening, so it
            # would only complicate stuff to include it into the connection pool
            async with triopg.connect(self.url) as self.notification_conn:
                await self.notification_conn.add_listener("app_notification", self._on_notification)
                task_status.started()
                await trio.sleep_forever()

    def _on_notification(self, connection, pid, channel, payload):
        data = unpackb(b64decode(payload.encode("ascii")))
        signal = data.pop("__signal__")
        logger.debug("notif received", pid=pid, channel=channel, payload=payload)
        self.event_bus.send(signal, **data)

    async def teardown(self):
        if self._task_status:
            await self._task_status.cancel_and_join()


async def send_signal(conn, signal, **kwargs):
    # PostgreSQL's NOTIFY only accept string as payload, hence we must
    # use base64 on our payload...
    raw_data = b64encode(packb({"__signal__": signal, **kwargs})).decode("ascii")
    await conn.execute("SELECT pg_notify($1, $2)", "app_notification", raw_data)
    logger.debug("notif sent", signal=signal, kwargs=kwargs)
