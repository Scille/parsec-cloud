import pytest

from parsec.core.backend_connection2 import backend_cmds_connect, backend_anonymous_cmds_connect

# from tests.common import freeze_time


@pytest.mark.trio
async def test_anonymous_ping(running_backend):
    async with backend_anonymous_cmds_connect(running_backend.addr) as conn:
        pong = await conn.ping("Hello World !")
        assert pong == "Hello World !"


@pytest.mark.trio
async def test_ping(alice, running_backend):
    async with backend_cmds_connect(
        running_backend.addr, alice.device_id, alice.signing_key
    ) as conn:
        pong = await conn.ping("Hello World !")
        assert pong == "Hello World !"
