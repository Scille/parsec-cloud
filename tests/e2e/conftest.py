import pytest
import asyncio
from aiohttp import web
from collections import namedtuple
from effect2 import asyncio_perform, Effect

from parsec.backend import (
    mocked_components_factory, register_backend_api, register_start_api,
    register_in_memory_block_store_api
)
from parsec.core import (
    components_factory as core_components_factory,
    register_core_api,
)
from parsec.unix_socket_app import UnixSocketApplication, run_unix_socket_server
from parsec.tools import ejson_loads, ejson_dumps


JOHN_DOE_IDENTITY = 'John_Doe'
JOHN_DOE_PASSWORD = 'P@ssw0rd.'
JOHN_DOE_PRIVATE_KEY = b"""
-----BEGIN RSA PRIVATE KEY-----
MIICXQIBAAKBgQDCqVQVdVhJqW9rrbObvDZ4ET6FoIyVn6ldWhOJaycMeFYBN3t+
cGr9/xHPGrYXK63nc8x4IVxhfXZ7JwrQ+AJv935S3rAV6JhDKDfDFrkzUVZmcc/g
HhjiP7rTAt4RtACvhZwrDuj3Pc4miCpGN/T3tbOKG889JN85nABKR9WkdwIDAQAB
AoGBAJFU3Dr9FgJA5rfMwpiV51CzByu61trqjgbtNkLVZhzwRr23z5Jxmd+yLHik
J6ia6sYvdUuHFLKQegGt/2xOjXn8UBpa725gLojHn2umtJDL7amTlBwiJfNXuZrF
BSKK9+xZnNDWMq1IuCqPeintbve+MNSc62JYuGGtXSz9L5f5AkEA/xBkUksBfEUl
65oEPgxvMKHNjLq48otRmCaG+i3MuQqTYQ+c8Z/l26yQL4OV2b36a8/tTaLhwhAZ
Ibtv05NKfQJBAMNgMbOsUWpY8A1Cec79Oj6RVe79E5ciZ4mW3lx5tjJRyNxwlQag
u+T6SwBIa6xMfLBQeizzxqXqxAyW/riQ6QMCQQCadUu7Re6tWZaAGTGufYsr8R/v
s/dh8ZpEwDgG8otCFzRul6zb6Y+huttJ2q55QIGQnka/N/7srSD6+23Zux1lAkBx
P30PzL6UimD7DqFUnev5AH1zPjbwz/x8AHt71wEJQebQAGIhqWHAZGS9ET14bg2I
ld172QI4glCJi6yyhyzJAkBzfmHZEE8FyLCz4z6b+Z2ghMds2Xz7RwgVqCIXt9Ku
P7Bq0eXXgyaBo+jpr3h4K7QnPh+PaHSlGqSfczZ6GIpx
-----END RSA PRIVATE KEY-----
"""
JOHN_DOE_PUBLIC_KEY = b"""
-----BEGIN PUBLIC KEY-----
MIGfMA0GCSqGSIb3DQEBAQUAA4GNADCBiQKBgQDCqVQVdVhJqW9rrbObvDZ4ET6F
oIyVn6ldWhOJaycMeFYBN3t+cGr9/xHPGrYXK63nc8x4IVxhfXZ7JwrQ+AJv935S
3rAV6JhDKDfDFrkzUVZmcc/gHhjiP7rTAt4RtACvhZwrDuj3Pc4miCpGN/T3tbOK
G889JN85nABKR9WkdwIDAQAB
-----END PUBLIC KEY-----
"""


@pytest.fixture
def debug_loop(loop):
    loop.set_debug(True)


@pytest.fixture
def backend(loop, test_server):
    app = web.Application()
    blockstore = '/blockstore'
    register_in_memory_block_store_api(app, blockstore)
    app.components = mocked_components_factory(blockstore)
    app.dispatcher = app.components.get_dispatcher()
    register_backend_api(app, app.dispatcher)
    register_start_api(app, app.dispatcher)
    yield loop.run_until_complete(test_server(app))


@pytest.fixture
def johndoe(loop, backend):
    async def insert_pubkey_in_backend():
        from parsec.backend.pubkey import EPubKeyAdd
        await asyncio_perform(backend.app.dispatcher,
            Effect(EPubKeyAdd(JOHN_DOE_IDENTITY, JOHN_DOE_PUBLIC_KEY)))

    loop.run_until_complete(insert_pubkey_in_backend())
    return namedtuple('User', 'id,password,privkey,pubkey')(
        JOHN_DOE_IDENTITY,
        JOHN_DOE_PASSWORD,
        JOHN_DOE_PRIVATE_KEY,
        JOHN_DOE_PUBLIC_KEY)


@pytest.fixture
def backend_host(backend):
    return 'ws://localhost:%s' % backend.port


@pytest.fixture
def core(loop, test_server, tmpdir, backend_host):
    app = UnixSocketApplication()
    components = core_components_factory(app, backend_host)
    dispatcher = components.get_dispatcher()
    register_core_api(app, dispatcher)
    socket = str(tmpdir + '/parsec')
    server = loop.run_until_complete(
        run_unix_socket_server(app.on_connection, socket, loop=loop))
    yield server
    loop.run_until_complete(app.shutdown())
    loop.run_until_complete(server.stop())


@pytest.fixture
def core_socket(core):
    return core.socket_path


@pytest.fixture
def client(loop, core_socket):
    reader, writer = loop.run_until_complete(
        asyncio.open_unix_connection(path=core_socket))

    class CoreClient:
        def __init__(self, reader, writer):
            self.reader = reader
            self.writer = writer

        async def send_cmd(self, cmd, **kwargs):
            msg = {'cmd': cmd, **kwargs}
            raw_msg = ejson_dumps(msg).encode()
            self.writer.write(raw_msg)
            self.writer.write(b'\n')
            raw_resp = await self.reader.readline()
            return ejson_loads(raw_resp.decode())

    yield CoreClient(reader, writer)
    writer.close()


@pytest.fixture
def johndoe_client(loop, client, johndoe):
    loop.run_until_complete(
        client.send_cmd('identity_load', id=johndoe.id, key=johndoe.privkey))
    return client
