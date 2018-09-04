import pytest
import trio

from parsec.core.backend_cmds_sender import BackendCmdsSender, BackendNotAvailable

from tests.open_tcp_stream_mock_wrapper import offline


@pytest.fixture
async def backend_cmds(running_backend, backend_addr, alice):
    async with trio.open_nursery() as nursery:
        backend_cmds = BackendCmdsSender(alice, running_backend.addr)
        await backend_cmds.init(nursery)
        try:
            yield backend_cmds

        finally:
            await backend_cmds.teardown()
            nursery.cancel_scope.cancel()


@pytest.mark.trio
async def test_base(backend_cmds):
    rep = await backend_cmds.send({"cmd": "ping", "ping": "hello"})
    assert rep == {"status": "ok", "pong": "hello"}


@pytest.mark.trio
async def test_concurrency_sends(backend_cmds):
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
            nursery.start_soon(sender, backend_cmds, str(x))

    # TODO: ideally we would have to spy on the socket to make sure
    # the trames were strictly on a request/response order pattern
    with trio.fail_after(1):
        await work_all_done.wait()


@pytest.mark.trio
async def test_backend_offline(unused_tcp_addr, alice):
    async with trio.open_nursery() as nursery:
        backend_cmds = BackendCmdsSender(alice, unused_tcp_addr)
        await backend_cmds.init(nursery)

        with pytest.raises(BackendNotAvailable):
            await backend_cmds.send({"cmd": "ping", "ping": "hello"})
        with pytest.raises(BackendNotAvailable):
            await backend_cmds.send({"cmd": "ping", "ping": "hello"})

        await backend_cmds.teardown()
        nursery.cancel_scope.cancel()


@pytest.mark.trio
async def test_backend_switch_offline(backend_cmds, tcp_stream_spy):
    # First, have a good request to make sure a socket has been opened

    rep = await backend_cmds.send({"cmd": "ping", "ping": "hello"})
    assert rep == {"status": "ok", "pong": "hello"}

    # Current socket is down, but opening another socket
    # should solve the trouble

    tcp_stream_spy.socks[backend_cmds.backend_addr][-1].send_stream.close()

    rep = await backend_cmds.send({"cmd": "ping", "ping": "hello"})
    assert rep == {"status": "ok", "pong": "hello"}

    # Now sockets will never be able to reach the backend no matter what

    with offline(backend_cmds.backend_addr):
        with pytest.raises(BackendNotAvailable):
            await backend_cmds.send({"cmd": "ping", "ping": "hello"})

    # Finally make sure we can still connect to the backend

    rep = await backend_cmds.send({"cmd": "ping", "ping": "hello"})
    assert rep == {"status": "ok", "pong": "hello"}
