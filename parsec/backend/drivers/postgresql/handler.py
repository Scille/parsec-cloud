import trio

from parsec.backend.drivers.postgresql import triopg


async def init_db(url, force=False):
    conn = await triopg.connect(url)

    async with conn.transaction():

        if force:
            await conn.execute(
                """
                DROP TABLE IF EXISTS
                    blockstore,
                    groups,
                    group_identities,
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
            CREATE TABLE IF NOT EXISTS groups (
                id SERIAL PRIMARY KEY,
                name TEXT UNIQUE
            )"""
        )
        await conn.execute(
            """
            CREATE TABLE IF NOT EXISTS group_identities (
                id SERIAL PRIMARY KEY,
                group_id INTEGER,
                name TEXT,
                admin BOOLEAN,
                UNIQUE (group_id, name)
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
                user_privkey_cypherkey BYTEA,
                cyphered_user_privkey BYTEA,
                refused_reason TEXT
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
                blob BYTEA
            )"""
        )
        await conn.execute(
            """
            CREATE TABLE IF NOT EXISTS user_vlobs (
                _id SERIAL PRIMARY KEY,
                user_id VARCHAR(32),
                version INTEGER,
                blob BYTEA
            )"""
        )

    await conn.close()


class PGHandler:
    def __init__(self, url, signal_ns, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.url = url
        self.signal_ns = signal_ns
        self.signals = ["message_arrived", "user_claimed", "user_vlob_updated", "vlob_updated"]
        self.notifications_to_ignore = []
        self.signals_to_ignore = []

        self.signal_handlers = {}
        for signal in self.signals:
            sighandler = self.signal_ns.signal(signal)
            self.signal_handlers[signal] = self.get_signal_handler(signal)
            sighandler.connect(self.signal_handlers[signal])

        self.queue = trio.Queue(100)

    async def init(self, nursery):
        await init_db(self.url)

        self.pool = await triopg.create_pool(self.url)
        self.conn = await triopg.connect(self.url)

        for signal in self.signals:
            await self.conn.add_listener(signal, self.notification_handler)

        await nursery.start(self.notification_sender)

    def notification_handler(self, connection, pid, channel, payload):
        try:
            self.notifications_to_ignore.remove((channel, payload))
        except ValueError:
            signal = self.signal_ns.signal(channel)
            signal.send(payload)
            self.notifications_to_ignore.append((channel, payload))

    async def notification_sender(self, task_status=trio.TASK_STATUS_IGNORED):
        async def send(signal, sender):
            async with self.pool.acquire() as conn:
                await conn.execute("SELECT pg_notify($1, $2)", signal, sender)

        task_status.started()
        while True:
            req = await self.queue.get()
            self.notifications_to_ignore.append((req["signal"], req["sender"]))
            await send(req["signal"], req["sender"])

    def get_signal_handler(self, signal):
        def signal_handler(sender):
            try:
                self.signals_to_ignore.remove((signal, sender))
            except ValueError:
                self.queue.put_nowait({"signal": signal, "sender": sender})

        return signal_handler

    async def teardown(self):
        await self.conn.close()
        await self.pool.close()
