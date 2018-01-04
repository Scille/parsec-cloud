import pytest
import trio
from trio._util import acontextmanager

from parsec.core.backend_connection import BackendConnection, BackendNotAvailable

from tests.open_tcp_stream_mock_wrapper import offline


@acontextmanager
async def open_backend_connection(user, backend_addr):
    conn = BackendConnection(user, backend_addr)
    async with trio.open_nursery() as nursery:
        await conn.init(nursery)
        yield conn
        await conn.teardown()


@pytest.mark.trio
async def test_base(running_backend, backend_addr, alice):
    async with open_backend_connection(alice, backend_addr) as conn:
        rep = await conn.send({'cmd': 'ping', 'ping': 'hello'})
        assert rep == {'status': 'ok', 'pong': 'hello'}


@pytest.mark.trio
async def test_concurrency_sends(running_backend, backend_addr, alice):
    count = 0

    async def sender(conn, x):
        nonlocal count
        rep = await conn.send({'cmd': 'ping', 'ping': x})
        assert rep == {'status': 'ok', 'pong': x}
        count += 1

    async with open_backend_connection(alice, backend_addr) as conn:
        async with trio.open_nursery() as nursery:
            for x in range(5):
                nursery.start_soon(sender, conn, str(x))

    assert count == 5


@pytest.mark.trio
async def test_backend_offline(unused_tcp_port, alice):
    addr = 'tcp://127.0.0.1:%s' % unused_tcp_port

    conn = BackendConnection(alice, addr)
    async with trio.open_nursery() as nursery:
        await conn.init(nursery)

        with pytest.raises(BackendNotAvailable):
            await conn.send({'cmd': 'ping', 'ping': 'hello'})
        with pytest.raises(BackendNotAvailable):
            await conn.send({'cmd': 'ping', 'ping': 'hello'})

        await conn.teardown()


@pytest.mark.trio
async def test_backend_switch_offline(running_backend, backend_addr, alice, tcp_stream_spy):
    async with open_backend_connection(alice, backend_addr) as conn:

        # Connection ok

        rep = await conn.send({'cmd': 'ping', 'ping': 'hello'})
        assert rep == {'status': 'ok', 'pong': 'hello'}

        # Current socket is down, but opening another socket
        # should solve the trouble

        tcp_stream_spy.socks[backend_addr][-1].send_stream.close()

        rep = await conn.send({'cmd': 'ping', 'ping': 'hello'})
        assert rep == {'status': 'ok', 'pong': 'hello'}

        # Now sockets will never be able to reach the backend no matter what

        with offline(backend_addr):
            with pytest.raises(BackendNotAvailable):
                await conn.send({'cmd': 'ping', 'ping': 'hello'})

        # Finally make sure we can still connect to the backend

        rep = await conn.send({'cmd': 'ping', 'ping': 'hello'})
        assert rep == {'status': 'ok', 'pong': 'hello'}
