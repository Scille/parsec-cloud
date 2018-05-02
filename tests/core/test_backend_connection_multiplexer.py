import pytest
import trio
import attr

from parsec.core.backend_connections_multiplexer import (
    BackendConnectionsMultiplexer,
    BackendNotAvailable,
    backend_connection_factory,
)

from tests.common import AsyncMock
from tests.open_tcp_stream_mock_wrapper import offline


@attr.s
class MockStreamHook:
    streams = attr.ib(default=attr.Factory(list))

    async def new_stream(self, host, port, **kwargs):
        stream = AsyncMock(spec=trio.abc.Stream)
        stream.receive_some.return_value = b'{"status": "ok"}\n'
        self.streams.append(stream)
        return stream


@pytest.fixture
async def backend_connections_multiplexer(nursery, running_backend, backend_addr, alice):
    bcm = BackendConnectionsMultiplexer(alice, running_backend.addr)
    await bcm.init(nursery)
    try:
        yield bcm

    finally:
        await bcm.teardown()


@pytest.mark.trio
async def test_base(backend_connections_multiplexer):
    rep = await backend_connections_multiplexer.send({"cmd": "ping", "ping": "hello"})
    assert rep == {"status": "ok", "pong": "hello"}


@pytest.mark.trio
async def test_concurrency_sends(backend_connections_multiplexer):
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
            nursery.start_soon(sender, backend_connections_multiplexer, str(x))

    # TODO: ideally we would have to spy on the socket to make sure
    # the trames were strictly on a request/response order pattern
    with trio.fail_after(1):
        await work_all_done.wait()


@pytest.mark.trio
async def test_backend_offline(nursery, unused_tcp_addr, alice):
    bcm = BackendConnectionsMultiplexer(alice, unused_tcp_addr)
    await bcm.init(nursery)

    with pytest.raises(BackendNotAvailable):
        await bcm.send({"cmd": "ping", "ping": "hello"})
    with pytest.raises(BackendNotAvailable):
        await bcm.send({"cmd": "ping", "ping": "hello"})

    await bcm.teardown()


@pytest.mark.trio
async def test_backend_switch_offline(backend_connections_multiplexer, tcp_stream_spy):
    bcm = backend_connections_multiplexer

    # First, have a good request to make sure a socket has been opened

    rep = await bcm.send({"cmd": "ping", "ping": "hello"})
    assert rep == {"status": "ok", "pong": "hello"}

    # Current socket is down, but opening another socket
    # should solve the trouble

    tcp_stream_spy.socks[bcm.backend_addr][-1].send_stream.close()

    rep = await bcm.send({"cmd": "ping", "ping": "hello"})
    assert rep == {"status": "ok", "pong": "hello"}

    # Now sockets will never be able to reach the backend no matter what

    with offline(bcm.backend_addr):
        with pytest.raises(BackendNotAvailable):
            await bcm.send({"cmd": "ping", "ping": "hello"})

    # Finally make sure we can still connect to the backend

    rep = await bcm.send({"cmd": "ping", "ping": "hello"})
    assert rep == {"status": "ok", "pong": "hello"}
