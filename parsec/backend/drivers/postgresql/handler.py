import trio
import triopg


async def init_db(url, force=False):
    conn = await triopg.connect(url)

    async with conn.transaction():

        if force:
            await conn.execute(
                """
                DROP TABLE IF EXISTS
                    users,
                    devices,

                    user_invitations,
                    device_invitations,
                    device_conf_tries,

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
        else:
            db_initialized = await conn.execute("""
            SELECT exists (
                SELECT 1 FROM information_schema.tables
                WHERE table_schema = 'public' AND table_name = 'users'
            )
            """)
            if db_initialized:
                return

        await conn.execute(
            """
            CREATE TABLE users (
                _id SERIAL PRIMARY KEY,
                user_id VARCHAR(32) UNIQUE NOT NULL,
                created_on TIMESTAMP NOT NULL,
                created_by INTEGER NOT NULL,
                broadcast_key BYTEA NOT NULL
            );

            CREATE TABLE devices (
                _id SERIAL PRIMARY KEY,
                user_ INTEGER REFERENCES users (_id) NOT NULL,
                device_id VARCHAR(65) NOT NULL,
                device_name VARCHAR(32) NOT NULL,
                created_on TIMESTAMP NOT NULL,
                created_by INTEGER,
                verify_key BYTEA NOT NULL,
                revocated_on TIMESTAMP,
                UNIQUE (user_, device_name)
            );

            ALTER TABLE users ADD CONSTRAINT users_created_by_fk FOREIGN KEY (created_by) REFERENCES devices (_id);
            ALTER TABLE devices ADD CONSTRAINT devices_created_by_fk FOREIGN KEY (created_by) REFERENCES devices (_id);
            """
        )

        await conn.execute(
            """
            CREATE TYPE USER_INVITATION_STATUS AS ENUM ('pending', 'claimed', 'rejected');
            CREATE TABLE user_invitations (
                _id SERIAL PRIMARY KEY,
                status USER_INVITATION_STATUS NOT NULL,
                user_id VARCHAR(32) NOT NULL,
                device_name VARCHAR(32) NOT NULL,
                invited_on TIMESTAMP NOT NULL,
                invited_by INTEGER REFERENCES devices (_id) NOT NULL,
                invitation_token TEXT NOT NULL,
                claim_tries INTEGER NOT NULL,
                claimed_on TIMESTAMP
            )"""
        )

        await conn.execute(
            """
            CREATE TYPE DEVICE_INVITATION_STATUS AS ENUM ('pending', 'claimed', 'rejected');
            CREATE TABLE device_invitations (
                _id SERIAL PRIMARY KEY,
                status DEVICE_INVITATION_STATUS NOT NULL,
                user_ INTEGER REFERENCES users (_id) NOT NULL,
                device_name VARCHAR(32) NOT NULL,
                invited_on TIMESTAMP NOT NULL,
                invited_by INTEGER REFERENCES devices (_id) NOT NULL,
                invitation_token TEXT NOT NULL,
                claim_tries INTEGER NOT NULL,
                claimed_on TIMESTAMP
            )"""
        )

        await conn.execute(
            """
            CREATE TYPE DEVICE_CONF_TRY_STATUS AS ENUM ('pending', 'accepted', 'refused');
            CREATE TABLE device_conf_tries (
                _id SERIAL PRIMARY KEY,
                status DEVICE_CONF_TRY_STATUS NOT NULL,
                invitation INTEGER REFERENCES device_invitations (_id) NOT NULL,
                device_verify_key BYTEA NOT NULL,
                exchange_cipherkey BYTEA NOT NULL,
                salt BYTEA NOT NULL
            )"""
        )

        await conn.execute(
            """
            CREATE TABLE messages (
                _id SERIAL PRIMARY KEY,
                recipient INTEGER REFERENCES users (_id) NOT NULL,
                sender INTEGER REFERENCES devices (_id) NOT NULL,
                body BYTEA NOT NULL
            )"""
        )

        await conn.execute(
            """
            CREATE TABLE vlobs (
                _id SERIAL PRIMARY KEY,
                vlob_id UUID UNIQUE NOT NULL,
                version INTEGER NOT NULL,
                rts TEXT NOT NULL,
                wts TEXT NOT NULL,
                blob BYTEA NOT NULL,
                UNIQUE(vlob_id, version)
            )"""
        )

        await conn.execute(
            """
            CREATE TABLE beacons (
                _id SERIAL PRIMARY KEY,
                src INTEGER REFERENCES vlobs (_id) NOT NULL,
                src_version INTEGER NOT NULL
            )"""
        )

        await conn.execute(
            """
            CREATE TABLE blockstore (
                _id SERIAL PRIMARY KEY,
                block_id UUID UNIQUE NOT NULL,
                block BYTEA NOT NULL
            )"""
        )

    await conn.close()


class PGHandler:
    def __init__(self, url, signal_ns):
        self.url = url
        self.signal_ns = signal_ns
        self.signals = ["message_arrived", "user_claimed", "vlob_updated"]
        self.pool = None
        self.notifications_to_ignore = []
        self.signals_to_ignore = []
        self._notification_sender_task_info = None

        self.signal_handlers = {}
        for signal in self.signals:
            sighandler = self.signal_ns.signal(signal)
            self.signal_handlers[signal] = self.get_signal_handler(signal)
            sighandler.connect(self.signal_handlers[signal])

        self.queue = trio.Queue(100)

    async def init(self, nursery):
        await init_db(self.url)

        self.pool = await triopg.create_pool(self.url)
        # This connection is dedicated to the notifications listening, so it
        # would only complicate stuff to include it into the connection pool
        self.notification_conn = await triopg.connect(self.url)

        for signal in self.signals:
            await self.notification_conn.add_listener(signal, self._notification_handler)

        self._notification_sender_task_info = await nursery.start(self._notification_sender)

    def _notification_handler(self, connection, pid, channel, payload):
        try:
            self.notifications_to_ignore.remove((channel, payload))
        except ValueError:
            signal = self.signal_ns.signal(channel)
            signal.send(payload)
            self.signals_to_ignore.append((channel, payload))

    async def _notification_sender(self, task_status=trio.TASK_STATUS_IGNORED):
        async def send(signal, sender):
            async with self.pool.acquire() as conn:
                await conn.execute("SELECT pg_notify($1, $2)", signal, sender)

        try:
            closed_event = trio.Event()
            with trio.open_cancel_scope() as cancel_scope:
                task_status.started((cancel_scope, closed_event))
                while True:
                    req = await self.queue.get()
                    self.notifications_to_ignore.append((req["signal"], req["sender"]))
                    await send(req["signal"], req["sender"])
        finally:
            closed_event.set()

    def get_signal_handler(self, signal):
        def signal_handler(sender):
            try:
                self.signals_to_ignore.remove((signal, sender))
            except ValueError:
                self.queue.put_nowait({"signal": signal, "sender": sender})

        return signal_handler

    async def teardown(self):
        cancel_scope, closed_event = self._notification_sender_task_info
        cancel_scope.cancel()
        await closed_event.wait()
        await self.pool.close()
        await self.notification_conn.close()
