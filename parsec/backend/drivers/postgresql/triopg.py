from functools import wraps
import asyncpg
import trio_asyncio


@trio_asyncio.trio2aio
async def connect(url):
    return TrioConnectionProxy(await asyncpg.connect(url))


@trio_asyncio.trio2aio
async def create_pool(url):
    return TrioPoolProxy(await asyncpg.create_pool(url))


class TrioTransactionProxy:
    def __init__(self, transaction):
        self.transaction = transaction

    @trio_asyncio.trio2aio
    async def __aenter__(self, *args):
        return await self.transaction.__aenter__(*args)

    @trio_asyncio.trio2aio
    async def __aexit__(self, *args):
        return await self.transaction.__aexit__(*args)


class TrioConnectionProxy:
    def __init__(self, conn):
        self.conn = conn

    def transaction(self, *args, **kwargs):
        transaction = self.conn.transaction(*args, **kwargs)
        return TrioTransactionProxy(transaction)

    def __getattr__(self, attr):
        target = getattr(self.conn, attr)

        @wraps(target)
        @trio_asyncio.trio2aio
        async def wrapper(*args, **kwargs):
            return await target(*args, **kwargs)

        if callable(target):
            return wrapper
        return target


class TrioPoolAcquireContextProxy:
    def __init__(self, acquire_context):
        self.acquire_context = acquire_context

    @trio_asyncio.trio2aio
    async def __aenter__(self, *args):
        proxy = await self.acquire_context.__aenter__(*args)
        return TrioConnectionProxy(proxy._con)

    @trio_asyncio.trio2aio
    async def __aexit__(self, *args):
        return await self.acquire_context.__aexit__(*args)


class TrioPoolProxy:
    def __init__(self, pool):
        self.pool = pool

    def acquire(self):
        return TrioPoolAcquireContextProxy(self.pool.acquire())

    @trio_asyncio.trio2aio
    async def close(self):
        return await self.pool.close()
