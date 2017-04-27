import asyncio
import aiopg
from psycopg2 import ProgrammingError
import click
from blinker import signal
from urllib import parse

from parsec.backend.message_service import BaseMessageService
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
                await cur.execute("""
                    CREATE TABLE messages (
                    recipient text,
                    count     integer NOT NULL,
                    body      text,
                    PRIMARY KEY(recipient, count)
                );""")


def _connect(url):
    url = parse.urlparse(url)
    return aiopg.create_pool(
        database=url.path[1:],
        user=url.username,
        password=url.password,
        host=url.hostname,
        port=url.port
    )


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
