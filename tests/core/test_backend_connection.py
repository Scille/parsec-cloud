import pytest
import trio
from unittest.mock import patch

from tests.common import with_backend

from parsec.core.backend_connection import BackendConnection, BackendNotAvailable


async def _testbed(addr, user, tester):
    conn = BackendConnection(user, addr)

    async with trio.open_nursery() as nursery:
        nursery.start_soon(tester, nursery, conn)


@pytest.mark.trio
@with_backend()
async def test_base(backend, alice):
    async def tester(nursery, conn):
        await conn.init(nursery)
        rep = await conn.send({'cmd': 'ping', 'ping': 'hello'})
        assert rep == {'status': 'ok', 'pong': 'hello'}
        await conn.teardown()

    await _testbed(backend.addr, alice, tester)


@pytest.mark.trio
@with_backend()
async def test_concurrency_sends(backend, alice):
    done_queue = trio.Queue(10)

    async def tester(nursery, conn):
        await conn.init(nursery)
        for x in range(5):
            nursery.start_soon(sender, conn, str(x))
        done = 0
        while True:
            await done_queue.get()
            done += 1
            if done == 5:
                await conn.teardown()
                break

    async def sender(conn, x):
        rep = await conn.send({'cmd': 'ping', 'ping': x})
        assert rep == {'status': 'ok', 'pong': x}
        await done_queue.put(rep)

    await _testbed(backend.addr, alice, tester)


@pytest.mark.trio
async def test_backend_offline(unused_tcp_port, alice):
    addr = 'tcp://127.0.0.1:%s' % unused_tcp_port

    async def tester(nursery, conn):
        await conn.init(nursery)
        with pytest.raises(BackendNotAvailable):
            await conn.send({'cmd': 'ping', 'ping': 'hello'})
        with pytest.raises(BackendNotAvailable):
            await conn.send({'cmd': 'ping', 'ping': 'hello'})
        await conn.teardown()

    await _testbed(addr, alice, tester)


@pytest.mark.trio
@with_backend()
async def test_backend_switch_offline(backend, alice):
    async def tester(nursery, conn):
        await conn.init(nursery)
        # Connection ok
        rep = await conn.send({'cmd': 'ping', 'ping': 'hello'})
        assert rep == {'status': 'ok', 'pong': 'hello'}

        # Current socket is down, but opening another socket
        # should solve the trouble

        def _broken_stream(*args, **kwargs):
            raise trio.BrokenStreamError()

        conn._sock.send = _broken_stream

        rep = await conn.send({'cmd': 'ping', 'ping': 'hello'})
        assert rep == {'status': 'ok', 'pong': 'hello'}

        # Now sockets will never be able to reach the backend no matter what

        with patch('parsec.utils.CookedSocket.send') as mock_send, \
             patch('parsec.utils.CookedSocket.recv') as mock_recv:
            mock_send.side_effect = _broken_stream
            mock_recv.side_effect = _broken_stream
            with pytest.raises(BackendNotAvailable):
                await conn.send({'cmd': 'ping', 'ping': 'hello'})

        # Finally make sure we can still connect to the backend

        rep = await conn.send({'cmd': 'ping', 'ping': 'hello'})
        assert rep == {'status': 'ok', 'pong': 'hello'}

        await conn.teardown()

    await _testbed(backend.addr, alice, tester)
