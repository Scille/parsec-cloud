import pytest
from trio.testing import trio_test
from nacl.public import PrivateKey

from parsec.utils import to_jsonb64

from tests.common import with_backend, async_patch, TEST_USERS


@trio_test
@with_backend()
async def test_vlob_create(backend):
    async with backend.test_connect('alice@test') as sock:
        await sock.send({'cmd': 'vlob_create', 'id': '123', 'blob': to_jsonb64(b'foo')})
        rep = await sock.recv()
        assert rep['status'] == 'ok'
        assert rep['read_trust_seed']
        assert rep['write_trust_seed']
        assert rep['id'] == '123'


@trio_test
@with_backend(populated_for='alice')
async def test_vlob_read(backend):
    async with backend.test_connect('alice@test') as sock:
        await sock.send({'cmd': 'vlob_create', 'id': '123', 'blob': to_jsonb64(b'foo')})
        rep = await sock.recv()
        assert rep['status'] == 'ok'
        assert rep['read_trust_seed']
        assert rep['write_trust_seed']
        assert rep['id'] == '123'
