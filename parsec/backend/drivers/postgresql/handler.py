import triopg
from structlog import get_logger
from uuid import UUID

from parsec.event_bus import EventBus
from parsec.utils import call_with_control, ejson_dumps, ejson_loads


logger = get_logger()


async def init_db(url: str, force: bool = False) -> bool:
    """
    Raises:
        triopg.exceptions.PostgresError
    """
    async with triopg.connect(url) as conn:
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
            unconfigured_devices,
            device_configuration_tries,

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
            name VARCHAR(32) PRIMARY KEY,
            bootstrap_token VARCHAR(32) NOT NULL,
            root_verify_key BYTEA
        );

        CREATE TABLE users (
            user_id VARCHAR(32) PRIMARY KEY,
            certified_user BYTEA,
            user_certifier VARCHAR(32),
            created_on TIMESTAMP NOT NULL
        );

        CREATE TABLE devices (
            device_id VARCHAR(65) PRIMARY KEY,
            user_id VARCHAR(32) REFERENCES users (user_id) NOT NULL,
            certified_device BYTEA,
            device_certifier VARCHAR(32) REFERENCES devices (device_id),
            created_on TIMESTAMP NOT NULL,
            revocated_on TIMESTAMP,
            certified_revocation BYTEA,
            revocation_certifier VARCHAR(32) REFERENCES devices (device_id)
        );

        ALTER TABLE users
        ADD CONSTRAINT FK_users_devices FOREIGN KEY (user_certifier) REFERENCES devices(device_id);

        CREATE TYPE USER_INVITATION_STATUS AS ENUM ('pending', 'claimed', 'rejected');
        CREATE TABLE user_invitations (
            _id SERIAL PRIMARY KEY,
            status USER_INVITATION_STATUS NOT NULL,
            user_id VARCHAR(32) NOT NULL,
            invited_on TIMESTAMP NOT NULL,
            invitation_token TEXT NOT NULL,
            claim_tries INTEGER NOT NULL,
            claimed_on TIMESTAMP
        );

        CREATE TABLE unconfigured_devices (
            _id SERIAL PRIMARY KEY,
            user_id VARCHAR(32) REFERENCES users (user_id) NOT NULL,
            device_name VARCHAR(32) NOT NULL,
            created_on TIMESTAMP NOT NULL,
            configure_token TEXT NOT NULL,
            UNIQUE(user_id, device_name)
        );

        CREATE TYPE DEVICE_CONF_TRY_STATUS AS ENUM ('waiting_answer', 'accepted', 'refused');
        CREATE TABLE device_configuration_tries (
            config_try_id VARCHAR(32) PRIMARY KEY,
            user_id VARCHAR(32) REFERENCES users (user_id) NOT NULL,
            status DEVICE_CONF_TRY_STATUS NOT NULL,
            refused_reason TEXT,
            device_name VARCHAR(32) NOT NULL,
            device_verify_key BYTEA NOT NULL,
            exchange_cipherkey BYTEA NOT NULL,
            salt BYTEA NOT NULL,
            ciphered_user_privkey BYTEA,
            ciphered_user_manifest_access BYTEA,
            UNIQUE(user_id, config_try_id)
        );

        CREATE TABLE messages (
            _id SERIAL PRIMARY KEY,
            recipient VARCHAR(32) REFERENCES users (user_id) NOT NULL,
            sender VARCHAR(65) REFERENCES devices (device_id) NOT NULL,
            body BYTEA NOT NULL
        );

        CREATE TABLE vlobs (
            _id SERIAL PRIMARY KEY,
            vlob_id UUID NOT NULL,
            version INTEGER NOT NULL,
            rts TEXT NOT NULL,
            wts TEXT NOT NULL,
            blob BYTEA NOT NULL,
            author VARCHAR(65) REFERENCES devices (device_id) NOT NULL,
            UNIQUE(vlob_id, version)
        );

        CREATE TABLE beacons (
            _id SERIAL PRIMARY KEY,
            beacon_id UUID NOT NULL,
            beacon_index INTEGER NOT NULL,
            src_id UUID NOT NULL,
            -- src_id UUID REFERENCES vlobs (vlob_id) NOT NULL,
            src_version INTEGER NOT NULL
        );

        CREATE TABLE blockstore (
            _id SERIAL PRIMARY KEY,
            block_id UUID UNIQUE NOT NULL,
            block BYTEA NOT NULL,
            author VARCHAR(65) REFERENCES devices (device_id) NOT NULL
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
        self._run_connections_control = None

    async def init(self, nursery):
        self._run_connections_control = await nursery.start(
            call_with_control, self._run_connections
        )

    async def _run_connections(self, started_cb):
        async with triopg.create_pool(self.url) as self.pool:
            async with self.pool.acquire() as conn:
                if not await _is_db_initialized(conn):
                    raise RuntimeError("Database not initialized !")
            # This connection is dedicated to the notifications listening, so it
            # would only complicate stuff to include it into the connection pool
            async with triopg.connect(self.url) as self.notification_conn:
                await self.notification_conn.add_listener("app_notification", self._on_notification)

                await started_cb()

    def _on_notification(self, connection, pid, channel, payload):
        data = ejson_loads(payload)
        signal = data.pop("__signal__")
        if signal == "beacon.updated":
            data["beacon_id"] = UUID(data["beacon_id"])
            data["src_id"] = UUID(data["src_id"])
        logger.debug("notif received", pid=pid, channel=channel, payload=payload)
        self.event_bus.send(signal, **data)

    async def teardown(self):
        if self._run_connections_control:
            await self._run_connections_control.stop()


async def send_signal(conn, signal, **kwargs):
    raw_data = ejson_dumps({"__signal__": signal, **kwargs})
    await conn.execute("SELECT pg_notify($1, $2)", "app_notification", raw_data)
    logger.debug("notif sent", signal=signal, kwargs=kwargs)
