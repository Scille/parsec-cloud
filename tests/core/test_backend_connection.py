import pytest
import trio

from tests.common import with_backend

from parsec.core.backend_connection import BackendConnection


@pytest.mark.trio
@with_backend()
async def test_base(backend, alice):
    addr = 'tcp://127.0.0.1:%s' % backend.port
    conn = BackendConnection(alice, addr)

    async def tester():
        await conn.init(nursery)
        rep = await conn.send({'cmd': 'ping', 'ping': 'hello'})
        assert rep == {'status': 'ok', 'pong': 'hello'}
        await conn.teardown()
        
    async with trio.open_nursery() as nursery:
        nursery.start_soon(tester)


@pytest.mark.trio
@with_backend()
async def test_concurrency_sends(backend, alice):
    addr = 'tcp://127.0.0.1:%s' % backend.port
    conn = BackendConnection(alice, addr)
    done_queue = trio.Queue(10)

    async def tester(nursery):
        await conn.init(nursery)
        for x in range(5):
            nursery.start_soon(sender, str(x))
        done = 0
        while True:
            await done_queue.get()
            done += 1
            if done == 5:
                await conn.teardown()
                break

    async def sender(x):
        rep = await conn.send({'cmd': 'ping', 'ping': x})
        assert rep == {'status': 'ok', 'pong': x}
        await done_queue.put(rep)

    async with trio.open_nursery() as nursery:
        nursery.start_soon(tester, nursery)
