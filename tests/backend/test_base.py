import pytest

from tests.common import connect_backend


@pytest.mark.trio
async def test_connection(backend):
    async with connect_backend(backend, auth_as='alice@test') as sock:
        await sock.send({'cmd': 'ping', 'ping': '42'})
        rep = await sock.recv()
        assert rep == {'status': 'ok', 'pong': '42'}
