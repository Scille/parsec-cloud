import pytest

from parsec.core.backend_connection import backend_cmds_factory, backend_anonymous_cmds_factory


@pytest.mark.trio
async def test_anonymous_ping(running_backend):
    async with backend_anonymous_cmds_factory(running_backend.addr) as cmds:
        pong = await cmds.ping("Hello World !")
        assert pong == "Hello World !"


@pytest.mark.trio
async def test_ping(alice, running_backend):
    async with backend_cmds_factory(
        running_backend.addr, alice.device_id, alice.signing_key
    ) as cmds:
        pong = await cmds.ping("Hello World !")
        assert pong == "Hello World !"
