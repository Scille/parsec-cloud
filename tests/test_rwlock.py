import trio
from trio.testing import trio_test

from parsec.rwlock import RWLock


@trio_test
async def test_parallel_reads():
    lock = RWLock()
    events = []

    async def do_read():
        async with lock.acquire_read():
            events.append('read-start')
            await trio.sleep(0.01)  # Force coroutine yield and order
            events.append('read-end')

    async with trio.open_nursery() as nursery:
        for _ in range(10):
            nursery.start_soon(do_read)
    assert events == ['read-start'] * 10 + ['read-end'] * 10


@trio_test
async def test_simple_concurrent_writes():
    lock = RWLock()
    events = []

    async def do_write(x):
        async with lock.acquire_write():
            events.append('write-start')
            await trio.sleep(0.01)  # Force coroutine yield and order
            events.append('write-end')

    async with trio.open_nursery() as nursery:
        for x in range(10):
            nursery.start_soon(do_write, x)
    assert events == ['write-start', 'write-end'] * 10


@trio_test
async def test_parallel_writes():
    lock = RWLock()
    events = []

    async def do_read():
        async with lock.acquire_read():
            events.append('read-start')
            await trio.sleep(0.01)  # Force coroutine yield and order
            events.append('read-end')

    async def do_write(x, nursery):
        async with lock.acquire_write():
            events.append('write-start')
            for _ in range(3):
                nursery.start_soon(do_read)
                await trio.sleep(0)  # Force coroutine yield
            events.append('write-end')

    async with trio.open_nursery() as nursery:
        for x in range(3):
            nursery.start_soon(do_write, x, nursery)
    lookup = (['write-start', 'write-end'] * 3 +
              ['read-start'] * 9 + ['read-end'] * 9)
    assert lookup == events


@trio_test
async def test_exception_handling():
    lock = RWLock()
    events = []

    async def do_read():
        async with lock.acquire_read():
            events.append('read')

    async def do_read_with_error(nursery):
        try:
            async with lock.acquire_read():
                nursery.start_soon(do_read)
                events.append('read with error')
                raise RuntimeError('oops !')
        except RuntimeError:
            pass

    async def do_write_with_error(nursery):
        try:
            async with lock.acquire_write():
                nursery.start_soon(do_read_with_error, nursery)
                events.append('write with error')
                raise RuntimeError('oops !')
        except RuntimeError:
            pass

    async with trio.open_nursery() as nursery:
        nursery.start_soon(do_write_with_error, nursery)
    lookup = ['write with error', 'read with error', 'read']
    assert lookup == events
