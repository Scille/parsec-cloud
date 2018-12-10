import pytest
import trio

from parsec.core.backend_connection2 import BackendNotAvailable, backend_cmds_pool_factory

from tests.open_tcp_stream_mock_wrapper import offline


@pytest.mark.trio
async def test_backend_switch_offline(running_backend, alice, tcp_stream_spy):
    async with backend_cmds_pool_factory(
        running_backend.addr, alice.device_id, alice.signing_key
    ) as cmds:
        # First, have a good request to make sure a socket has been opened
        pong = await cmds.ping("Hello World !")
        assert pong == "Hello World !"

        # Current socket is down, but opening another socket
        # should solve the trouble
        async def _broken_send_stream():
            raise trio.BrokenResourceError("Huho!")

        tcp_stream_spy.get_socks(running_backend.addr)[
            -1
        ].send_stream.send_all_hook = _broken_send_stream
        pong = await cmds.ping("Hello World !")
        assert pong == "Hello World !"

        # Now sockets will never be able to reach the backend no matter what
        with offline(running_backend.addr):
            with pytest.raises(BackendNotAvailable):
                await cmds.ping("Hello World !")

        # Finally make sure we can still connect to the backend
        pong = await cmds.ping("Hello World !")
        assert pong == "Hello World !"
