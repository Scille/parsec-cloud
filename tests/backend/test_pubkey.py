import pytest
from trio.testing import trio_test
from nacl.public import PrivateKey

from parsec.utils import to_jsonb64

from tests.common import with_backend, async_patch


@trio_test
@with_backend()
async def test_pubkey_get_ok(backend):
    async with backend.test_connect('alice@test') as sock:
        await sock.send({
            'cmd': 'pubkey_get',
            'id': 'bob@test'
        })
        rep = await sock.recv()
    assert rep == {
        'status': 'ok',
        'id': 'bob@test',
        'public': 'PJyDuQC6ZXlhLqnwP9cwLZE9ye18fydKmzAgT/dvS04=\n',
        'verify': 'p9KzHZjz4qjlFSTvMNNMU4QVD5CfdWEJ/Yprw/G9Xl4=\n'
    }


@pytest.mark.parametrize('bad_msg', [
    {'id': 42},
    {'id': None},
    {'id': 'alice@test.com', 'unknown': 'field'},
    {}
])
@trio_test
@with_backend()
async def test_pubkey_get_bad_msg(backend, bad_msg):
    async with backend.test_connect('alice@test') as sock:
        await sock.send({'cmd': 'pubkey_get', **bad_msg})
        rep = await sock.recv()
        assert rep['status'] == 'bad_message'


@trio_test
@with_backend()
async def test_pubkey_get_not_found(backend):
    async with backend.test_connect('alice@test') as sock:
        await sock.send({'cmd': 'pubkey_get', 'id': 'dummy@test'})
        rep = await sock.recv()
        assert rep == {
            'status': 'pubkey_not_found',
            'reason': 'No public key for identity `dummy@test`'
        }
