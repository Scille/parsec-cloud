import pytest
from trio.testing import trio_test
from nacl.public import PrivateKey

from tests.common import with_backend, async_patch, TEST_USERS


@trio_test
@with_backend()
async def test_connection(backend):
    async with backend.test_connect('alice@test') as sock:
        await sock.send({'cmd': 'ping', 'ping': '42'})
        rep = await sock.recv()
        assert rep == {'status': 'ok', 'pong': '42'}


@trio_test
@with_backend()
async def test_pubkey_get(backend):
    async with backend.test_connect('alice@test') as sock:
        await sock.send({'cmd': 'pubkey_get', 'id': 'bob@test'})
        rep = await sock.recv()
        assert rep == {
            'status': 'ok',
            'id': 'bob@test',
            'public': 'PJyDuQC6ZXlhLqnwP9cwLZE9ye18fydKmzAgT/dvS04=\n',
            'verify': 'p9KzHZjz4qjlFSTvMNNMU4QVD5CfdWEJ/Yprw/G9Xl4=\n'
        }
