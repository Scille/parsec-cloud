import pytest
import trio
import collections

from parsec.rwlock import RWLock


class RWEventsStack:
    # Register read/write begin/end events and make sure there are done in
    # a proper way.

    def __init__(self):
        self._events_history = []
        self._events_stack = []

    def _assert_allowed_previous_stack_events(self, *allowed_events):
        last_event = self._events_stack[0] if self._events_stack else 'empty'
        assert last_event in allowed_events

    def push_read(self):
        self._assert_allowed_previous_stack_events('empty', 'push_read')
        self._events_history.append('push_read')
        self._events_stack.append('push_read')

    def push_write(self):
        self._assert_allowed_previous_stack_events('empty')
        self._events_history.append('push_write')
        self._events_stack.append('push_write')

    def pop_read(self):
        self._assert_allowed_previous_stack_events('push_read')
        self._events_history.append('pop_read')
        self._events_stack.pop()

    def pop_write(self):
        self._assert_allowed_previous_stack_events('push_write')
        self._events_history.append('pop_write')
        self._events_stack.pop()

    def get_stats(self):
        counters = collections.Counter(self._events_history)
        assert counters['pop_write'] == counters['push_write']
        assert counters['pop_read'] == counters['push_read']
        return {'writes': counters['pop_write'], 'reads': counters['pop_read']}


@pytest.mark.trio
async def test_parallel_reads():
    lock = RWLock()
    events = RWEventsStack()

    async def do_read():
        async with lock.acquire_read():
            events.push_read()
            await trio.sleep(0.01)  # Force coroutine yield and order
            events.pop_read()

    async with trio.open_nursery() as nursery:
        for _ in range(10):
            nursery.start_soon(do_read)
    assert events.get_stats() == {'writes': 0, 'reads': 10}


@pytest.mark.trio
async def test_simple_concurrent_writes():
    lock = RWLock()
    events = RWEventsStack()

    async def do_write(x):
        async with lock.acquire_write():
            events.push_write()
            await trio.sleep(0.01)  # Force coroutine yield and order
            events.pop_write()

    async with trio.open_nursery() as nursery:
        for x in range(10):
            nursery.start_soon(do_write, x)
    assert events.get_stats() == {'writes': 10, 'reads': 0}


@pytest.mark.trio
async def test_parallel_writes():
    lock = RWLock()
    events = RWEventsStack()

    async def do_read():
        async with lock.acquire_read():
            events.push_read()
            await trio.sleep(0.01)  # Force coroutine yield and order
            events.pop_read()

    async def do_write(x, nursery):
        async with lock.acquire_write():
            events.push_write()
            for _ in range(3):
                nursery.start_soon(do_read)
                await trio.sleep(0)  # Force coroutine yield
            events.pop_write()

    async with trio.open_nursery() as nursery:
        for x in range(3):
            nursery.start_soon(do_write, x, nursery)
    assert events.get_stats() == {'reads': 9, 'writes': 3}


@pytest.mark.trio
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
