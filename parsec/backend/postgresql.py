import attr
import asyncio
import aiopg
from uuid import uuid4
from psycopg2 import ProgrammingError, IntegrityError
import blinker
import click
from blinker import signal
from urllib import parse
from effect2 import do, async_do, Effect, AsyncFunc, TypeDispatcher

from parsec.base import EEvent
from parsec.crypto import load_public_key
from parsec.tools import ejson_dumps, ejson_loads
from parsec.backend.message import EMessageNew, EMessageGet
from parsec.backend.vlob import EVlobCreate, EVlobUpdate, EVlobRead, VlobAtom
from parsec.backend.user_vlob import EUserVlobUpdate, EUserVlobRead, UserVlobAtom
from parsec.backend.group import (
    EGroupCreate, EGroupRead, EGroupAddIdentities, EGroupRemoveIdentities, Group
)
from parsec.backend.pubkey import EPubKeyGet, EPubKeyAdd
from parsec.backend.session import EGetAuthenticatedUser
from parsec.exceptions import (
    VlobNotFound, TrustSeedError, UserVlobError, GroupAlreadyExist,
    GroupNotFound, PubKeyNotFound, PubKeyError
)


@click.group()
def cli():
    pass


@cli.add_command
@click.command()
@click.argument('url')
@click.option('--force', '-f', is_flag=True, default=False)
def init(url, force):
    """Create the tables in database."""
    loop = asyncio.get_event_loop()
    try:
        loop.run_until_complete(_init_db(url, force=force))
    except ProgrammingError as exc:
        raise SystemExit(exc)


async def _init_db(url, force=False, loop=None):
    async with _connect(url, loop=loop) as pool:
        with await pool.cursor() as cur:
            if force:
                await cur.execute(
                    "DROP TABLE IF EXISTS vlobs, user_vlobs, messages, groups, pubkeys;")
            # messages
            await cur.execute("""
                CREATE TABLE messages (
                recipient text,
                count     integer NOT NULL,
                body      bytea,
                PRIMARY KEY(recipient, count)
            );""")
            # vlobs
            await cur.execute("""
                CREATE TABLE vlobs (
                id               char(32),
                version          integer NOT NULL,
                read_trust_seed  text,
                write_trust_seed text,
                blob             bytea,
                PRIMARY KEY(id, version)
            );""")
            # vlobs
            await cur.execute("""
                CREATE TABLE user_vlobs (
                id               text,
                version          integer NOT NULL,
                blob             bytea,
                PRIMARY KEY(id, version)
            );""")
            # groups
            await cur.execute("""
                CREATE TABLE groups (
                id               text PRIMARY KEY,
                body             text
            );""")
            # pubkeys
            await cur.execute("""
                CREATE TABLE pubkeys (
                identity         text PRIMARY KEY,
                key              bytea
            );""")


def _connect(url, loop=None):
    url = parse.urlparse(url)
    kwargs = dict(database=url.path[1:])
    if loop:
        kwargs['loop'] = loop
    if url.hostname:
        kwargs['host'] = url.hostname
    if url.port:
        kwargs['port'] = url.port
    if url.username:
        kwargs['user'] = url.username
    if url.password:
        kwargs['password'] = url.password
    return aiopg.create_pool(**kwargs)


@attr.s
class PostgreSQLConnection:

    _url = attr.ib()
    _pool = attr.ib(default=None)

    async def open_connection(self, loop=None):
        assert not self._pool, "Service already bootstraped"
        self._pool = await _connect(self._url, loop=loop)
        self._notification_handler_task = asyncio.ensure_future(self._notification_handler())

    async def close_connection(self):
        assert self._pool, "Service hasn't been bootstraped"
        self._notification_handler_task.cancel()
        try:
            await self._notification_handler_task
        except asyncio.CancelledError:
            pass
        self._pool.terminate()
        await self._pool.wait_closed()

    async def _notification_handler(self):
        with await self._pool.cursor() as cur:
            await cur.execute('LISTEN message_arrived;')
            await cur.execute('LISTEN vlob_updated;')
            await cur.execute('LISTEN user_vlob_updated;')
            while True:
                msg = await cur.connection.notifies.get()
                signal(msg.channel).send(msg.payload)

    def acquire(self):
        assert self._pool, "Service hasn't been bootstraped"
        return self._pool.acquire()


async def postgresql_connection_factory(url):
    conn = PostgreSQLConnection(url)
    await conn.open_connection()

    @do
    def on_app_stop(app):
        yield Effect(AsyncFunc(conn.close_connection()))

    blinker.signal('app_stop').connect(on_app_stop, weak=False)
    return conn


@attr.s
class PostgreSQLMessageComponent:
    connection = attr.ib()

    @do
    def perform_message_new(self, intent):
        yield AsyncFunc(self._perform_message_new(intent))
        yield Effect(EEvent('message_arrived', intent.recipient))

    async def _perform_message_new(self, intent):
        async with self.connection.acquire() as conn:
            async with conn.cursor() as cur:
                await cur.execute(
                    "SELECT max(count) FROM messages WHERE recipient=%s;", (intent.recipient, ))
                count, = await cur.fetchone()
                count = 0 if count is None else count + 1
                await cur.execute("INSERT INTO messages VALUES (%s, %s, %s);",
                    (intent.recipient, count, intent.body))
                await cur.execute('NOTIFY message_arrived, %s;', (intent.recipient, ))

    @do
    def perform_message_get(self, intent):
        id = yield Effect(EGetAuthenticatedUser())
        return (yield AsyncFunc(self._perform_message_get(id, intent)))

    async def _perform_message_get(self, id, intent):
        async with self.connection.acquire() as conn:
            async with conn.cursor() as cur:
                await cur.execute(
                    "SELECT body FROM messages WHERE recipient=%s AND count>=%s ORDER BY count;",
                    (id, intent.offset))
                return [bytes(x[0]) for x in await cur.fetchall()]

    def get_dispatcher(self):
        return TypeDispatcher({
            EMessageNew: self.perform_message_new,
            EMessageGet: self.perform_message_get,
        })


@attr.s
class PostgreSQLVlobComponent:
    connection = attr.ib()

    @async_do
    async def perform_vlob_create(self, intent):
        # Generate opaque id if not provided
        if not intent.id:
            intent.id = uuid4().hex
        atom = VlobAtom(id=intent.id, blob=intent.blob)
        async with self.connection.acquire() as conn:
            async with conn.cursor() as cur:
                await cur.execute("INSERT INTO vlobs VALUES (%s, 1, %s, %s, %s);",
                    (atom.id, atom.read_trust_seed, atom.write_trust_seed, atom.blob))
        return atom

    @async_do
    async def perform_vlob_read(self, intent):
        async with self.connection.acquire() as conn:
            async with conn.cursor() as cur:
                if intent.version:
                    await cur.execute("SELECT * FROM vlobs WHERE "
                                      "id=%s AND version=%s;", (intent.id, intent.version))
                else:
                    await cur.execute("SELECT * FROM vlobs WHERE "
                                      "id=%s ORDER BY version DESC;", (intent.id, ))
                ret = await cur.fetchone()
        if not ret:
            raise VlobNotFound('Vlob not found.')
        _, version, rts, wts, blob = ret
        if rts != intent.trust_seed:
            raise TrustSeedError('Invalid read trust seed.')
        return VlobAtom(id=intent.id, version=version, read_trust_seed=rts,
                        write_trust_seed=wts, blob=bytes(blob))

    @do
    def perform_vlob_update(self, intent):
        yield AsyncFunc(self._perform_vlob_update(intent))
        yield Effect(EEvent('vlob_updated', intent.id))

    async def _perform_vlob_update(self, intent):
        async with self.connection.acquire() as conn:
            async with conn.cursor() as cur:
                await cur.execute("SELECT version, read_trust_seed, write_trust_seed FROM "
                                  "vlobs WHERE id=%s ORDER BY version DESC;", (intent.id, ))
                ret = await cur.fetchone()
                if ret is None:
                    raise VlobNotFound('Vlob not found.')
                last_version, rts, wts = ret
                if wts != intent.trust_seed:
                    raise TrustSeedError('Invalid write trust seed.')
                if intent.version != last_version + 1:
                    raise VlobNotFound('Wrong blob version.')
                # TODO: insertion doesn't do atomic check of version
                await cur.execute("INSERT INTO vlobs VALUES (%s, %s, %s, %s, %s);",
                    (intent.id, intent.version, rts, wts, intent.blob))
                await cur.execute("NOTIFY vlob_updated, %s", (intent.id, ))

    def get_dispatcher(self):
        return TypeDispatcher({
            EVlobRead: self.perform_vlob_read,
            EVlobCreate: self.perform_vlob_create,
            EVlobUpdate: self.perform_vlob_update
        })


@attr.s
class PostgreSQLUserVlobComponent:
    connection = attr.ib()

    @do
    def perform_user_vlob_read(self, intent):
        id = yield Effect(EGetAuthenticatedUser())
        return (yield AsyncFunc(self._perform_user_vlob_read(id, intent.version)))

    async def _perform_user_vlob_read(self, id, version):
        async with self.connection.acquire() as conn:
            async with conn.cursor() as cur:
                if version:
                    await cur.execute("SELECT * FROM user_vlobs WHERE "
                                      "id=%s AND version=%s;", (id, version))
                else:
                    await cur.execute("SELECT * FROM user_vlobs WHERE "
                                      "id=%s ORDER BY version DESC;", (id, ))
                ret = await cur.fetchone()
        if version == 0 or (version is None and not ret):
            return UserVlobAtom(id=id)
        if not ret:
            raise UserVlobError('Wrong blob version.')
        _, version, blob = ret
        return UserVlobAtom(id=id, version=version, blob=bytes(blob))

    @do
    def perform_user_vlob_update(self, intent):
        id = yield Effect(EGetAuthenticatedUser())
        yield AsyncFunc(self._perform_user_vlob_update(id, intent.version, intent.blob))
        yield Effect(EEvent('user_vlob_updated', id))

    async def _perform_user_vlob_update(self, id, version, blob):
        async with self.connection.acquire() as conn:
            async with conn.cursor() as cur:
                await cur.execute("SELECT version FROM user_vlobs WHERE "
                                  "id=%s ORDER BY version DESC;", (id, ))
                ret = await cur.fetchone()
                last_version = ret[0] if ret else 0
                if version != last_version + 1:
                    raise UserVlobError('Wrong blob version.')
                # TODO: insertion doesn't do atomic check of version
                await cur.execute("INSERT INTO user_vlobs VALUES (%s, %s, %s);",
                    (id, version, blob))
                await cur.execute('NOTIFY user_vlob_updated, %s', (id, ))

    def get_dispatcher(self):
        return TypeDispatcher({
            EUserVlobRead: self.perform_user_vlob_read,
            EUserVlobUpdate: self.perform_user_vlob_update
        })


@attr.s
class PostgreSQLGroupComponent:
    connection = attr.ib()

    @async_do
    async def perform_group_create(self, intent):
        payload = '{"admins": [], "users": []}'
        async with self.connection.acquire() as conn:
            async with conn.cursor() as cur:
                try:
                    await cur.execute("INSERT INTO groups VALUES (%s, %s);",
                        (intent.name, payload))
                except IntegrityError:
                    raise GroupAlreadyExist('Group already exist.')

    @async_do
    async def perform_group_read(self, intent):
        async with self.connection.acquire() as conn:
            async with conn.cursor() as cur:
                await cur.execute('SELECT body FROM groups WHERE id=%s', (intent.name, ))
                ret = await cur.fetchone()
                if ret is None:
                    raise GroupNotFound('Group not found.')
                data = ejson_loads(ret[0])
                return Group(name=intent.name, **data)

    @async_do
    async def perform_group_add_identities(self, intent):
        async with self.connection.acquire() as conn:
            async with conn.cursor() as cur:
                await cur.execute('SELECT body FROM groups WHERE id=%s', (intent.name, ))
                ret = await cur.fetchone()
                if ret is None:
                    raise GroupNotFound('Group not found.')
                group = ejson_loads(ret[0])
                group_entry = 'admins' if intent.admin else 'users'
                group[group_entry] = list(set(group[group_entry]) | set(intent.identities))
                await cur.execute('UPDATE groups SET body=%s WHERE id=%s',
                    (ejson_dumps(group), intent.name))

    @async_do
    async def perform_group_remove_identities(self, intent):
        async with self.connection.acquire() as conn:
            async with conn.cursor() as cur:
                await cur.execute('SELECT body FROM groups WHERE id=%s', (intent.name, ))
                ret = await cur.fetchone()
                if ret is None:
                    raise GroupNotFound('Group not found.')
                group = ejson_loads(ret[0])
                group_entry = 'admins' if intent.admin else 'users'
                group[group_entry] = [identity for identity in group[group_entry]
                                      if identity not in intent.identities]
                await cur.execute('UPDATE groups SET body=%s WHERE id=%s',
                    (ejson_dumps(group), intent.name))

    def get_dispatcher(self):
        return TypeDispatcher({
            EGroupCreate: self.perform_group_create,
            EGroupRead: self.perform_group_read,
            EGroupAddIdentities: self.perform_group_add_identities,
            EGroupRemoveIdentities: self.perform_group_remove_identities,
        })


@attr.s
class PostgreSQLPubKeyComponent:
    connection = attr.ib()

    @async_do
    async def perform_pubkey_add(self, intent):
        async with self.connection.acquire() as conn:
            async with conn.cursor() as cur:
                try:
                    await cur.execute("INSERT INTO pubkeys VALUES (%s, %s);",
                        (intent.id, intent.key.decode()))
                except IntegrityError:
                    raise PubKeyError('Identity `%s` already has a public key' % intent.id)

    @async_do
    async def perform_pubkey_get(self, intent):
        async with self.connection.acquire() as conn:
            async with conn.cursor() as cur:
                await cur.execute('SELECT key FROM pubkeys WHERE identity=%s', (intent.id, ))
                ret = await cur.fetchone()
                if ret is None:
                    raise PubKeyNotFound('No public key for identity `%s`' % intent.id)
                key = bytes(ret[0])
        return key if intent.raw else load_public_key(key)

    def get_dispatcher(self):
        return TypeDispatcher({
            EPubKeyAdd: self.perform_pubkey_add,
            EPubKeyGet: self.perform_pubkey_get
        })
