import pytest
import trio
from unittest.mock import patch

from parsec.core.backend_cmds_sender import BackendNotAvailable
from parsec.networking import CookedSocket

from tests.open_tcp_stream_mock_wrapper import offline
from tests.common import AsyncMock


@pytest.fixture
async def backend_cmds_sender(running_backend, backend_cmds_sender_factory, alice):
    async with backend_cmds_sender_factory(alice) as backend_cmds_sender:
        yield backend_cmds_sender


@pytest.mark.trio
async def test_base(backend_cmds_sender):
    rep = await backend_cmds_sender.send({"cmd": "ping", "ping": "hello"})
    assert rep == {"status": "ok", "pong": "hello"}


@pytest.mark.trio
async def test_concurrency_sends(backend_cmds_sender):
    CONCURRENCY = 10

    work_done_counter = 0
    work_all_done = trio.Event()

    async def sender(conn, x):
        nonlocal work_done_counter
        rep = await conn.send({"cmd": "ping", "ping": x})
        assert rep == {"status": "ok", "pong": x}
        work_done_counter += 1
        if work_done_counter == CONCURRENCY:
            work_all_done.set()

    async with trio.open_nursery() as nursery:
        for x in range(CONCURRENCY):
            nursery.start_soon(sender, backend_cmds_sender, str(x))

    # TODO: ideally we would have to spy on the socket to make sure
    # the trames were strictly on a request/response order pattern
    with trio.fail_after(1):
        await work_all_done.wait()


@pytest.mark.trio
async def test_too_slow_request(
    autojump_clock, backend_cmds_sender_factory, unused_tcp_addr, alice
):
    sockets = []

    # Socket accept request but never reply
    def _bcf_hook(addr, device_id, device_signkey):
        sock = AsyncMock(spec_set=CookedSocket)

        async def _mcs_recv():
            await trio.sleep_forever()

        sock.recv.side_effect = _mcs_recv

        sockets.append(sock)
        return sock

    mocked_bcf = AsyncMock()
    mocked_bcf.is_async = True
    mocked_bcf.side_effect = _bcf_hook

    with patch("parsec.core.backend_cmds_sender.backend_connection_factory", new=mocked_bcf):

        async with backend_cmds_sender_factory(
            alice, backend_addr=unused_tcp_addr
        ) as backend_cmds_sender:

            # First we send a request that won't get any answer
            with pytest.raises(BackendNotAvailable):
                await backend_cmds_sender.send({"cmd": "req1"})

            # Now the first opened socket should have been closed and should
            # never be reused.
            def _crash_on_reuse_socket(*args, **kwargs):
                raise AssertionError("Shouldn't reuse the socket")

            assert len(sockets) == 1
            sockets[0].aclose.assert_called_once()
            sockets[0].send.side_effect = _crash_on_reuse_socket
            sockets[0].recv.side_effect = _crash_on_reuse_socket

            # Finally we retry the request, this time with a socket that will answer
            good_sock = AsyncMock(spec=CookedSocket)
            good_sock.send.side_effect = [None]
            good_sock.recv.side_effect = [{"cmd": "rep-2"}]
            mocked_bcf.side_effect = [good_sock]

            rep = await backend_cmds_sender.send({"cmd": "req-2"})
            assert rep == {"cmd": "rep-2"}


@pytest.mark.trio
async def test_backend_offline(tcp_stream_spy, backend_addr, backend_cmds_sender_factory, alice):
    # Using tcp_stream_spy make us avoid long wait for time
    with offline(backend_addr):
        async with backend_cmds_sender_factory(alice) as backend_cmds_sender:

            with pytest.raises(BackendNotAvailable):
                await backend_cmds_sender.send({"cmd": "ping", "ping": "hello"})
            with pytest.raises(BackendNotAvailable):
                await backend_cmds_sender.send({"cmd": "ping", "ping": "hello"})


@pytest.mark.trio
async def test_backend_switch_offline(backend_cmds_sender, tcp_stream_spy):
    # First, have a good request to make sure a socket has been opened

    rep = await backend_cmds_sender.send({"cmd": "ping", "ping": "hello"})
    assert rep == {"status": "ok", "pong": "hello"}

    # Current socket is down, but opening another socket
    # should solve the trouble

    tcp_stream_spy.socks[backend_cmds_sender.backend_addr][-1].send_stream.close()

    rep = await backend_cmds_sender.send({"cmd": "ping", "ping": "hello"})
    assert rep == {"status": "ok", "pong": "hello"}

    # Now sockets will never be able to reach the backend no matter what

    with offline(backend_cmds_sender.backend_addr):
        with pytest.raises(BackendNotAvailable):
            await backend_cmds_sender.send({"cmd": "ping", "ping": "hello"})

    # Finally make sure we can still connect to the backend

    rep = await backend_cmds_sender.send({"cmd": "ping", "ping": "hello"})
    assert rep == {"status": "ok", "pong": "hello"}
