import pytest
import trio
from copy import deepcopy
from async_generator import asynccontextmanager

from tests.common import freeze_time


@asynccontextmanager
async def wait_for_entries_synced(signal_ns, entries_pathes):
    event = trio.Event()
    to_sync = set(entries_pathes)
    synced = set()
    events_received = []

    def _on_entry_synced(sender, id, path):
        if event.is_set():
            return

        if path not in to_sync:
            raise AssertionError(f"{path} wasn't supposed to be synced, expected only {to_sync}")
        if path in synced:
            raise AssertionError(
                f"{path} synced two time while waiting synchro for {to_sync - synced}"
            )

        synced.add(path)
        events_received.append(("fs.entry.synced", id, path))
        if synced == to_sync:
            event.set()

    with signal_ns.signal("fs.entry.synced").temporarily_connected_to(_on_entry_synced):

        yield events_received

        await event.wait()


@pytest.mark.trio
async def test_online_sync(autojump_clock, alice, alice2, fs_factory):
    fs = await fs_factory(alice)
    fs2 = await fs_factory(alice2)

    async with wait_for_entries_synced(fs.signal_ns, ["/", "/foo.txt"]) as events_received:

        with freeze_time("2000-01-02"):
            await fs.file_create("/foo.txt")

        with freeze_time("2000-01-03"):
            await fs.file_write("/foo.txt", b"hello world !")

        await fs.sync("/foo.txt")

    async with wait_for_entries_synced(fs2.signal_ns, ["/"]):
        for _, id, path in events_received:
            fs2.signal_ns.signal("fs.entry.updated").send(id=id)

    stat = await fs.stat("/foo.txt")
    stat2 = await fs2.stat("/foo.txt")
    assert stat2 == stat


@pytest.mark.trio
async def test_sync_then_clean_start(autojump_clock, alice, alice2, fs_factory, signal_ns_factory):
    fs = await fs_factory(alice)

    async with wait_for_entries_synced(fs.signal_ns, ["/", "/foo.txt"]):

        with freeze_time("2000-01-02"):
            await fs.file_create("/foo.txt")

        with freeze_time("2000-01-03"):
            await fs.file_write("/foo.txt", b"hello world !")

        await fs.sync("/foo.txt")

    fs2_signal_ns = signal_ns_factory()
    async with wait_for_entries_synced(fs2_signal_ns, ["/"]):
        fs2 = await fs_factory(alice2, signal_ns=fs2_signal_ns)

    for path in ("/", "/foo.txt"):
        stat = await fs.stat(path)
        stat2 = await fs2.stat(path)
        assert stat2 == stat


@pytest.mark.trio
async def test_fast_forward_on_offline_during_sync(
    autojump_clock, signal_ns_factory, fs_factory, alice, alice2
):
    fs = await fs_factory(alice)

    # Create data in alice
    async with wait_for_entries_synced(fs.signal_ns, ["/", "/foo.txt"]):

        with freeze_time("2000-01-02"):
            await fs.file_create("/foo.txt")

        with freeze_time("2000-01-03"):
            await fs.file_write("/foo.txt", b"v1")

        await fs.sync("/foo.txt")

    # Artificially sync alice2's local data
    alice2.local_db = deepcopy(alice.local_db)

    # Now make alice2 out of date
    async with wait_for_entries_synced(fs.signal_ns, ["/", "/foo.txt", "/bar"]):
        with freeze_time("2000-01-04"):
            await fs.file_write("/foo.txt", b"v2")
            await fs.folder_create("/bar")

        await fs.sync("/")

    # Starting a fs with alice2 should update it local data
    fs2_signal_ns = signal_ns_factory()
    async with wait_for_entries_synced(fs2_signal_ns, ["/", "/foo.txt"]):
        fs2 = await fs_factory(alice2, signal_ns=fs2_signal_ns)

    for path in ("/", "/bar", "/foo.txt"):
        stat = await fs.stat(path)
        stat2 = await fs2.stat(path)
        assert stat2 == stat


@pytest.mark.trio
async def test_concurrent_update(autojump_clock, alice, alice2, fs_factory):
    fs = await fs_factory(alice)
    fs2 = await fs_factory(alice2)

    async with wait_for_entries_synced(fs.signal_ns, ["/", "/foo.txt"]) as events_received:

        with freeze_time("2000-01-02"):
            await fs.file_create("/foo.txt")

        with freeze_time("2000-01-03"):
            await fs.file_write("/foo.txt", b"v1")

        await fs.sync("/foo.txt")

    async with wait_for_entries_synced(fs2.signal_ns, ["/"]):
        for _, id, path in events_received:
            fs2.signal_ns.signal("fs.entry.updated").send(id=id)

    async with wait_for_entries_synced(
        fs.signal_ns, ["/foo.txt"]
    ) as events_received, wait_for_entries_synced(fs2.signal_ns, ["/foo.txt"]) as events_received:
        with freeze_time("2000-01-04"):
            await fs.file_write("/foo.txt", b"fs's v2")
        with freeze_time("2000-01-05"):
            await fs2.file_write("/foo.txt", b"fs2's v2")

        await fs.sync("/foo.txt")

        with freeze_time("2000-01-06"):
            await fs2.sync("/foo.txt")

    stat = await fs.stat("/foo.txt")
    stat2 = await fs2.stat("/foo.txt")
    assert stat2 == stat

    root_stat2 = await fs2.stat("/")
    assert root_stat2["children"] == ["foo (conflict 2000-01-06 00:00:00).txt", "foo.txt"]
