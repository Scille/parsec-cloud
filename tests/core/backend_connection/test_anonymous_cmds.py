import pytest
import trio

from parsec.core.backend_connection2 import BackendNotAvailable, backend_anonymous_cmds_factory

from tests.open_tcp_stream_mock_wrapper import offline


@pytest.mark.trio
async def test_anonymous_backend_offline(backend_addr):
    with pytest.raises(BackendNotAvailable):
        async with backend_anonymous_cmds_factory(backend_addr):
            pass


@pytest.mark.trio
async def test_anonymous_backend_switch_offline(running_backend):
    async with backend_anonymous_cmds_factory(running_backend.addr) as cmds:
        with offline(running_backend.addr):
            with pytest.raises(BackendNotAvailable):
                await cmds.ping("Whatever")


@pytest.mark.trio
async def test_anonymous_backend_closed_cmds(running_backend):
    async with backend_anonymous_cmds_factory(running_backend.addr) as cmds:
        pass
    with pytest.raises(trio.ClosedResourceError):
        await cmds.ping("Whatever")
