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
    def __init__(self, asyncpg_transaction):
        self._asyncpg_transaction = asyncpg_transaction

    @trio_asyncio.trio2aio
    async def __aenter__(self, *args):
        return await self._asyncpg_transaction.__aenter__(*args)

    @trio_asyncio.trio2aio
    async def __aexit__(self, *args):
        return await self._asyncpg_transaction.__aexit__(*args)


class TrioConnectionProxy:
    def __init__(self, asyncpg_conn):
        self._asyncpg_conn = asyncpg_conn

    def transaction(self, *args, **kwargs):
        transaction = self._asyncpg_conn.transaction(*args, **kwargs)
        return TrioTransactionProxy(transaction)

    def __getattr__(self, attr):
        target = getattr(self._asyncpg_conn, attr)

        if callable(target):

            @wraps(target)
            @trio_asyncio.trio2aio
            async def wrapper(*args, **kwargs):
                return await target(*args, **kwargs)

            return wrapper

        return target


class TrioPoolAcquireContextProxy:
    def __init__(self, asyncpg_acquire_context):
        self._asyncpg_acquire_context = asyncpg_acquire_context

    @trio_asyncio.trio2aio
    async def __aenter__(self, *args):
        proxy = await self._asyncpg_acquire_context.__aenter__(*args)
        return TrioConnectionProxy(proxy._con)

    @trio_asyncio.trio2aio
    async def __aexit__(self, *args):
        return await self._asyncpg_acquire_context.__aexit__(*args)


class TrioPoolProxy:
    def __init__(self, asyncpg_pool):
        self._asyncpg_pool = asyncpg_pool

    def acquire(self):
        return TrioPoolAcquireContextProxy(self._asyncpg_pool.acquire())

    @trio_asyncio.trio2aio
    async def close(self):
        return await self._asyncpg_pool.close()
