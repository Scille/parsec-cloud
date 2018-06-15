from functools import wraps
import trio
import asyncpg
import trio_asyncio


def _shielded(f):
    @wraps(f)
    async def wrapper(*args, **kwargs):
        with trio.open_cancel_scope(shield=True):
            return await f(*args, **kwargs)

    return wrapper


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

    @_shielded
    @trio_asyncio.trio2aio
    async def __aexit__(self, *args):
        return await self._asyncpg_transaction.__aexit__(*args)


class TrioConnectionProxy:
    def __init__(self, asyncpg_conn):
        self._asyncpg_conn = asyncpg_conn

    def transaction(self, *args, **kwargs):
        asyncpg_transaction = self._asyncpg_conn.transaction(*args, **kwargs)
        return TrioTransactionProxy(asyncpg_transaction)

    def __getattr__(self, attr):
        target = getattr(self._asyncpg_conn, attr)

        if callable(target):

            @wraps(target)
            @trio_asyncio.trio2aio
            async def wrapper(*args, **kwargs):
                return await target(*args, **kwargs)

            # Only generate the function wrapper once per connection instance
            setattr(self, attr, wrapper)

            return wrapper

        return target

    @_shielded
    @trio_asyncio.trio2aio
    async def close(self):
        return await self._asyncpg_conn.close()


class TrioPoolAcquireContextProxy:
    def __init__(self, asyncpg_acquire_context):
        self._asyncpg_acquire_context = asyncpg_acquire_context

    @trio_asyncio.trio2aio
    async def __aenter__(self, *args):
        proxy = await self._asyncpg_acquire_context.__aenter__(*args)
        return TrioConnectionProxy(proxy._con)

    @_shielded
    @trio_asyncio.trio2aio
    async def __aexit__(self, *args):
        return await self._asyncpg_acquire_context.__aexit__(*args)


class TrioPoolProxy:
    def __init__(self, asyncpg_pool):
        self._asyncpg_pool = asyncpg_pool

    def acquire(self):
        return TrioPoolAcquireContextProxy(self._asyncpg_pool.acquire())

    @_shielded
    @trio_asyncio.trio2aio
    async def close(self):
        return await self._asyncpg_pool.close()

    def terminate(self):
        return self._asyncpg_pool.terminate()

    async def __aenter__(self, *args):
        return self

    async def __aexit__(self, *args):
        await self.close()
        return False
