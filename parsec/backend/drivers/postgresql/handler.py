import asyncpg
from trio_asyncio import trio2aio


async def init_db(url, force=False):
    conn = await asyncpg.connect(url)

    async with conn.transaction():

        if force:
            for table in (
                "blockstore",
                "groups",
                "group_identities",
                "messages",
                "pubkeys",
                "users",
                "user_devices",
                "invitations",
                "vlobs",
                "user_vlobs",
                "device_configure_tries",
            ):
                await conn.execute("DROP TABLE IF EXISTS %s" % table)

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

        for signal in self.signals:
            sighandler = self.signal_ns.signal("message_arrived")
            sighandler.connect(self.get_signal_handler(signal))

    @trio2aio
    async def init(self):
        await init_db(self.url)
        self.pool = await asyncpg.create_pool(self.url)

        for signal in self.signals:
            async with self.pool.acquire() as conn:
                await conn.add_listener(signal, self.notification_handler)
                await conn.execute("LISTEN %s" % signal)

    @trio2aio
    async def teardown(self):
        await self.pool.close()

    @trio2aio
    async def fetch_one(self, sql, *params):
        async with self.pool.acquire() as conn:
            return await conn.fetchrow(sql, *params)

    @trio2aio
    async def fetch_many(self, sql, *params):
        async with self.pool.acquire() as conn:
            return await conn.fetch(sql, *params)

    @trio2aio
    async def insert_one(self, sql, *params):
        async with self.pool.acquire() as conn:
            return await conn.execute(sql, *params)

    @trio2aio
    async def insert_many(self, sql, *paramslist):
        async with self.pool.acquire() as conn:
            return await conn.executemany(sql, *paramslist)

    @trio2aio
    async def update_one(self, sql, *params):
        async with self.pool.acquire() as conn:
            return await conn.execute(sql, *params)

    @trio2aio
    async def update_many(self, sql, *paramslist):
        async with self.pool.acquire() as conn:
            return await conn.executemany(sql, *paramslist)

    @trio2aio
    async def delete_one(self, sql, *params):
        async with self.pool.acquire() as conn:
            return await conn.execute(sql, *params)

    @trio2aio
    async def delete_many(self, sql, *paramslist):
        async with self.pool.acquire() as conn:
            return await conn.executemany(sql, *paramslist)

    def notification_handler(self, connection, pid, channel, payload):
        signal = self.signal_ns.signal(channel)
        signal.send(payload, propagate=False)

    def get_signal_handler(self, signal):
        @trio2aio
        def signal_handler(sender, propagate=True):
            if propagate:

                async def notify(self, signal=None, sender=None):
                    async with self.pool.acquire() as conn:
                        await conn.execute("NOTIFY $1, $2", signal, sender)

        return signal_handler
