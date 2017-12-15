import pytest
from trio.testing import trio_test

from tests.common import with_backend


@pytest.mark.trio
# @trio_test
@with_backend()
async def test_connection(backend):
    async with backend.test_connect('alice@test') as sock:
        await sock.send({'cmd': 'ping', 'ping': '42'})
        rep = await sock.recv()
        assert rep == {'status': 'ok', 'pong': '42'}
