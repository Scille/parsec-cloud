import asyncio
import aiopg
from psycopg2 import ProgrammingError
import click
from blinker import signal
from urllib import parse

from parsec.backend.message_service import BaseMessageService
from parsec.backend.vlob_service import (
    BaseVlobService, VlobAtom, VlobNotFound, TrustSeedError)
from parsec.tools import logger


@click.group()
def cli():
    pass


@cli.add_command
@click.command()
@click.argument('url')
@click.option('--force', '-f', is_flag=True, default=False)
def init(url, force):
    loop = asyncio.get_event_loop()
    try:
        loop.run_until_complete(_init_db(url, force=force))
    except ProgrammingError as exc:
        raise SystemExit(exc)


async def _init_db(url, force=False):
    async with _connect(url) as pool:
        async with pool.acquire() as conn:
            async with conn.cursor() as cur:
                if force:
                    await cur.execute("DROP TABLE IF EXISTS vlobs, named_vlobs, messages;")
                # messages
                await cur.execute("""
                    CREATE TABLE messages (
                    recipient text,
                    count     integer NOT NULL,
                    body      text,
                    PRIMARY KEY(recipient, count)
                );""")
                # vlobs
                await cur.execute("""
                    CREATE TABLE vlobs (
                    id               char(32),
                    version          integer NOT NULL,
                    read_trust_seed  text,
                    write_trust_seed text,
                    blob             text,
                    PRIMARY KEY(id, version)
                );""")
                # vlobs
                await cur.execute("""
                    CREATE TABLE named_vlobs (
                    id               text,
                    version          integer NOT NULL,
                    read_trust_seed  text,
                    write_trust_seed text,
                    blob             text,
                    PRIMARY KEY(id, version)
                );""")


def _connect(url):
    url = parse.urlparse(url)
    kwargs = dict(database=url.path[1:])
    if url.hostname:
        kwargs['host'] = url.hostname
    if url.port:
        kwargs['port'] = url.port
    if url.username:
        kwargs['user'] = url.username
    if url.password:
        kwargs['password'] = url.password
    return aiopg.create_pool(**kwargs)


class PostgreSQLMessageService(BaseMessageService):

    _NOTIFY_CMD = "NOTIFY %s, %%s;" % BaseMessageService.on_arrived.name
    _LISTEN_CMD = "LISTEN %s;" % BaseMessageService.on_arrived.name

    async def bootstrap(self):
        assert not self._pool, "Service already bootstraped"
        self._pool = await _connect(self._url)
        self._notification_handler_task = asyncio.ensure_future(self._notification_handler())

    async def teardown(self):
        assert self._pool, "Service hasn't been bootstraped"
        self._notification_handler_task.cancel()
        try:
            await self._notification_handler_task
        except asyncio.CancelledError:
            pass
        self._pool.terminate()
        await self._pool.wait_closed()

    def __init__(self, url):
        super().__init__()
        self._url = url
        self._pool = None

    async def _notification_handler(self):
        async with await self._pool.acquire() as conn:
            async with await conn.cursor() as cur:
                while True:
                    await cur.execute(self._LISTEN_CMD)
                    msg = await conn.notifies.get()
                    if msg.channel != 'on_message_arrived':
                        logger.warning('Invalid notification message: %s' % msg)
                    else:
                        signal(msg.channel).send(msg.payload)

    async def new(self, recipient, body):
        async with self._pool.acquire() as conn:
            async with conn.cursor() as cur:
                await cur.execute("SELECT max(count) FROM messages WHERE recipient=%s;", (recipient, ))
                count, = await cur.fetchone()
                count = 0 if count is None else count + 1
                await cur.execute("INSERT INTO messages VALUES (%s, %s, %s);", (recipient, count, body))
                await cur.execute(self._NOTIFY_CMD, (recipient, ))

    async def get(self, recipient, offset=0):
        async with self._pool.acquire() as conn:
            async with conn.cursor() as cur:
                await cur.execute("SELECT body FROM messages WHERE recipient=%s AND count>=%s ORDER BY count;", (recipient, offset))
                return [x[0] for x in await cur.fetchall()]


class PostgreSQLVlobService(BaseVlobService):

    _NOTIFY_CMD = "NOTIFY %s, %%s;" % BaseVlobService.on_updated.name
    _LISTEN_CMD = "LISTEN %s;" % BaseVlobService.on_updated.name

    async def bootstrap(self):
        assert not self._pool, "Service already bootstraped"
        self._pool = await _connect(self._url)
        self._notification_handler_task = asyncio.ensure_future(self._notification_handler())

    async def teardown(self):
        assert self._pool, "Service hasn't been bootstraped"
        self._notification_handler_task.cancel()
        try:
            await self._notification_handler_task
        except asyncio.CancelledError:
            pass
        self._pool.terminate()
        await self._pool.wait_closed()

    def __init__(self, url):
        super().__init__()
        self._url = url
        self._pool = None

    async def _notification_handler(self):
        async with await self._pool.acquire() as conn:
            async with await conn.cursor() as cur:
                while True:
                    await cur.execute(self._LISTEN_CMD)
                    msg = await conn.notifies.get()
                    if msg.channel != 'on_vlob_updated':
                        logger.warning('Invalid notification message: %s' % msg)
                    else:
                        signal(msg.channel).send(msg.payload)

    async def new(self, recipient, body):
        async with self._pool.acquire() as conn:
            async with conn.cursor() as cur:
                await cur.execute("SELECT max(count) FROM messages WHERE recipient=%s;", (recipient, ))
                count, = await cur.fetchone()
                count = 0 if count is None else count + 1
                await cur.execute("INSERT INTO messages VALUES (%s, %s, %s);", (recipient, count, body))
                await cur.execute(self._NOTIFY_CMD, (recipient, ))

    async def get(self, recipient, offset=0):
        async with self._pool.acquire() as conn:
            async with conn.cursor() as cur:
                await cur.execute("SELECT body FROM messages WHERE recipient=%s AND count>=%s ORDER BY count;", (recipient, offset))
                return [x[0] for x in await cur.fetchall()]

    async def create(self, blob=None):
        atom = VlobAtom(blob=blob)
        async with self._pool.acquire() as conn:
            async with conn.cursor() as cur:
                await cur.execute("INSERT INTO vlobs VALUES (%s, 1, %s, %s, %s);",
                    (atom.id, atom.read_trust_seed, atom.write_trust_seed, atom.blob))
        return atom

    async def read(self, id, version=None, check_trust_seed=False):
        async with self._pool.acquire() as conn:
            async with conn.cursor() as cur:
                if version:
                    await cur.execute("SELECT * FROM vlobs WHERE id=%s AND version=%s;", (id, version))
                else:
                    await cur.execute("SELECT * FROM vlobs WHERE id=%s ORDER BY version DESC;", (id, ))
                ret = await cur.fetchone()
        if not ret:
            raise VlobNotFound('Vlob not found.')
        _, version, rts, wts, blob = ret
        if check_trust_seed and rts != check_trust_seed:
            raise TrustSeedError('Invalid read trust seed.')
        return VlobAtom(id=id, version=version, read_trust_seed=rts, write_trust_seed=wts, blob=blob)

    async def update(self, id, version, blob, check_trust_seed=False):
        async with self._pool.acquire() as conn:
            async with conn.cursor() as cur:
                await cur.execute("SELECT version, read_trust_seed, write_trust_seed FROM vlobs WHERE id=%s ORDER BY version DESC;", (id, ))
                ret = await cur.fetchone()
                if ret is None:
                    raise VlobNotFound('Vlob not found.')
                last_version, rts, wts = ret
                if check_trust_seed and wts != check_trust_seed:
                    raise TrustSeedError('Invalid write trust seed.')
                if version != last_version + 1:
                    raise VlobNotFound('Wrong blob version.')
                await cur.execute("INSERT INTO vlobs VALUES (%s, %s, %s, %s, %s);", (id, version, rts, wts, blob))
                await cur.execute(self._NOTIFY_CMD, (id, ))
