import trio
import triopg


async def init_db(url, force=False):
    conn = await triopg.connect(url)

    async with conn.transaction():

        if force:
            await conn.execute(
                """
                DROP TABLE IF EXISTS
                    blockstore,
                    messages,
                    pubkeys,
                    users,
                    user_devices,
                    invitations,
                    vlobs,
                    user_vlobs,
                    device_configure_tries
                """
            )

        await conn.execute(
            """
            CREATE TABLE IF NOT EXISTS blockstore (
                _id SERIAL PRIMARY KEY,
                id VARCHAR(32),
                block BYTEA
            )"""
        )
        await conn.execute(
            """
            CREATE TABLE IF NOT EXISTS messages (
                id SERIAL PRIMARY KEY,
                recipient_user_id TEXT,
                sender_device_id TEXT,
                body BYTEA
            )"""
        )
        await conn.execute(
            """
            CREATE TABLE IF NOT EXISTS pubkeys (
                _id SERIAL PRIMARY KEY,
                id VARCHAR(32) UNIQUE,
                pubkey BYTEA,
                verifykey BYTEA
            )"""
        )
        await conn.execute(
            """
            CREATE TABLE IF NOT EXISTS users (
                _id SERIAL PRIMARY KEY,
                user_id VARCHAR(32) UNIQUE,
                created_on INTEGER,
                created_by VARCHAR(32),
                broadcast_key BYTEA
            )"""
        )
        await conn.execute(
            """
            CREATE TABLE IF NOT EXISTS user_devices (
                _id SERIAL PRIMARY KEY,
                user_id VARCHAR(32),
                device_name TEXT,
                created_on INTEGER,
                configure_token TEXT,
                verify_key BYTEA,
                revocated_on INTEGER
            )"""
        )
        await conn.execute(
            """
            CREATE TABLE IF NOT EXISTS invitations (
                _id SERIAL PRIMARY KEY,
                user_id VARCHAR(32) UNIQUE,
                ts INTEGER,
                author VARCHAR(32),
                invitation_token TEXT,
                claim_tries INTEGER
            )"""
        )
        await conn.execute(
            """
            CREATE TABLE IF NOT EXISTS device_configure_tries (
                _id SERIAL PRIMARY KEY,
                user_id VARCHAR(32),
                config_try_id TEXT,
                status TEXT,
                device_name TEXT,
                device_verify_key BYTEA,
                exchange_cipherkey BYTEA,
                ciphered_user_privkey BYTEA,
                refused_reason TEXT,
                salt BYTEA
            )"""
        )
        await conn.execute(
            """
            CREATE TABLE IF NOT EXISTS vlobs (
                _id SERIAL PRIMARY KEY,
                id VARCHAR(32),
                version INTEGER,
                rts TEXT,
                wts TEXT,
                blob BYTEA,
                UNIQUE(id, version)
            )"""
        )
        await conn.execute(
            """
            CREATE TABLE IF NOT EXISTS user_vlobs (
                _id SERIAL PRIMARY KEY,
                user_id VARCHAR(32),
                version INTEGER,
                blob BYTEA,
                UNIQUE(user_id, version)
            )"""
        )

    await conn.close()


class PGHandler:
    def __init__(self, url, signal_ns):
        self.url = url
        self.signal_ns = signal_ns
        self.signals = ["message_arrived", "user_claimed", "user_vlob_updated", "vlob_updated"]
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
