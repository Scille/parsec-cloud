import pytest
import trio

from parsec.core.fs.base import ResourcesLocker


@pytest.mark.trio
async def test_simple_lock():
    locker = ResourcesLocker()

    assert not locker.is_locked("foo")
    with locker.lock("foo"):
        assert locker.is_locked("foo")
    assert not locker.is_locked("foo")


@pytest.mark.trio
async def test_lock_on_concurrent_sync(autojump_clock):
    events = []
    locker = ResourcesLocker()

    async def syncer(key, index):
        while locker.is_locked(key):
            await locker.wait_not_locked(key)

        with locker.lock(key):
            events.append(("lock start", index))
            await trio.sleep(1)
            events.append(("lock done", index))

    async with trio.open_nursery() as nursery:
        for i in range(10):
            nursery.start_soon(syncer, "foo", i)

    assert len(events) == 20
    remaining = iter(events)
    for i in range(5):
        expect_start, expect_start_i = next(remaining)
        expect_done, expect_done_i = next(remaining)

        assert expect_start_i == expect_done_i
        assert expect_start == "lock start"
        assert expect_done == "lock done"
