import pytest
import trio
import attr
from pendulum import Pendulum
from copy import deepcopy
from async_generator import asynccontextmanager

from tests.common import freeze_time


@attr.s
class SyncWaiter:
    events_received = attr.ib(factory=list)
    _event = attr.ib(factory=trio.Event)

    async def wait(self):
        await self._event.wait()


@asynccontextmanager
async def wait_for_entries_synced(signal_ns, entries_pathes):
    sw = SyncWaiter()
    to_sync = set(entries_pathes)
    synced = set()

    def _on_entry_synced(sender, id, path):
        if sw._event.is_set():
            return

        if path not in to_sync:
            raise AssertionError(f"{path} wasn't supposed to be synced, expected only {to_sync}")
        if path in synced:
            raise AssertionError(
                f"{path} synced two time while waiting synchro for {to_sync - synced}"
            )

        synced.add(path)
        sw.events_received.append(("fs.entry.synced", id, path))
        if synced == to_sync:
            sw._event.set()

    with signal_ns.signal("fs.entry.synced").temporarily_connected_to(_on_entry_synced):

        yield sw

        await sw.wait()


@pytest.mark.trio
@pytest.mark.skip(reason="Recursive sync strategy need to be reworked")
async def test_online_sync_in_background(
    mock_clock, alice, alice2, fs_factory, signal_ns_factory, monitor
):
    async with fs_factory(alice) as fs:
        mock_clock.rate = 100

        async with wait_for_entries_synced(fs.signal_ns, ["/", "/foo.txt"]):
            with freeze_time("2000-01-02"):
                await fs.file_create("/foo.txt")

            with freeze_time("2000-01-03"):
                await fs.file_write("/foo.txt", b"hello world !")

        # On startup, sync monitor does a full sync
        fs2_signal_ns = signal_ns_factory()
        async with wait_for_entries_synced(fs2_signal_ns, ["/"]) as sw:
            async with fs_factory(alice2, signal_ns=fs2_signal_ns) as fs2:
                await sw.wait()

                stat = await fs.stat("/foo.txt")
                stat2 = await fs2.stat("/foo.txt")
                assert stat2 == stat

                async with wait_for_entries_synced(fs2.signal_ns, ["/"]):
                    with freeze_time("2000-01-04"):
                        await fs.file_create("/bar.txt")

                stat = await fs.stat("/bar.txt")
                stat2 = await fs2.stat("/bar.txt")
                assert stat2 == stat


@pytest.mark.trio
@pytest.mark.skip(reason="Recursive sync strategy need to be reworked")
async def test_online_sync_explicit(mock_clock, alice, fs_factory, monitor):
    mock_clock.rate = 100

    async with fs_factory(alice, auto_sync=False) as fs:

        with freeze_time("2000-01-02"):
            await fs.file_create("/foo.txt")

        with freeze_time("2000-01-03"):
            await fs.file_write("/foo.txt", b"hello world !")

        async with wait_for_entries_synced(fs.signal_ns, ["/", "/foo.txt"]):
            await fs.sync("/foo.txt")

        stat = await fs.stat("/foo.txt")
        assert stat == {
            "type": "file",
            "created": Pendulum(2000, 1, 2),
            "updated": Pendulum(2000, 1, 3),
            "base_version": 1,
            "is_placeholder": False,
            "need_sync": False,
            "size": 13,
        }


@pytest.mark.trio
@pytest.mark.skip(reason="Recursive sync strategy need to be reworked")
async def test_sync_then_clean_start(
    mock_clock, alice_fs, alice, alice2, fs_factory, signal_ns_factory
):
    mock_clock.rate = 100

    async with fs_factory(alice2) as alice_fs2:
        async with wait_for_entries_synced(alice_fs.signal_ns, ["/", "/foo.txt"]):

            with freeze_time("2000-01-02"):
                await alice_fs.file_create("/foo.txt")

            with freeze_time("2000-01-03"):
                await alice_fs.file_write("/foo.txt", b"hello world !")

    # fs2_signal_ns = signal_ns_factory()
    # async with wait_for_entries_synced(fs2_signal_ns, ["/"]):
    #     async with fs_factory(alice2, signal_ns=fs2_signal_ns) as alice_fs2:
    #     # TODO: not sure of the order...
    #         pass

    # async with fs_factory(alice2, signal_ns=fs2_signal_ns) as alice_fs2:
    #     for path in ("/", "/foo.txt"):
    #         stat = await alice_fs.stat(path)
    #         stat2 = await alice_fs2.stat(path)
    #         assert stat2 == stat


@pytest.mark.trio
@pytest.mark.skip(reason="Recursive sync strategy need to be reworked")
async def test_sync_growth_by_truncate_file(mock_clock, alice_fs):
    mock_clock.rate = 100

    async with wait_for_entries_synced(alice_fs.signal_ns, ["/", "/foo.txt"]):

        with freeze_time("2000-01-02"):
            await alice_fs.file_create("/foo.txt")

        with freeze_time("2000-01-03"):
            await alice_fs.file_truncate("/foo.txt", length=24)

    stat = await alice_fs.stat("/foo.txt")
    assert stat["size"] == 24
    data = await alice_fs.file_read("/foo.txt")
    assert data == b"\x00" * 24


@pytest.mark.trio
@pytest.mark.skip(reason="Recursive sync strategy need to be reworked")
async def test_fast_forward_on_offline_during_sync(
    mock_clock, signal_ns_factory, fs_factory, alice, alice2, alice_fs
):
    mock_clock.rate = 100

    # Create data in alice
    async with wait_for_entries_synced(alice_fs.signal_ns, ["/", "/foo.txt"]):

        with freeze_time("2000-01-02"):
            await alice_fs.file_create("/foo.txt")

        with freeze_time("2000-01-03"):
            await alice_fs.file_write("/foo.txt", b"v1")

        await alice_fs.sync("/foo.txt")

    # Artificially sync alice2's local data
    alice2.local_db = deepcopy(alice.local_db)

    # Now make alice2 out of date
    async with wait_for_entries_synced(alice_fs.signal_ns, ["/", "/foo.txt", "/bar"]):
        with freeze_time("2000-01-04"):
            await alice_fs.file_write("/foo.txt", b"v2")
            await alice_fs.folder_create("/bar")

        await alice_fs.sync("/")

    # Starting a fs with alice2 should update it local data
    fs2_signal_ns = signal_ns_factory()
    async with fs_factory(alice2, signal_ns=fs2_signal_ns) as fs2:
        async with wait_for_entries_synced(fs2_signal_ns, ["/", "/foo.txt"]):
            pass

        for path in ("/", "/bar", "/foo.txt"):
            stat = await alice_fs.stat(path)
            stat2 = await fs2.stat(path)
            assert stat2 == stat


@pytest.mark.trio
@pytest.mark.skip(reason="Recursive sync strategy need to be reworked")
async def test_concurrent_update(mock_clock, alice, alice2, fs_factory):
    mock_clock.rate = 100

    async with fs_factory(alice) as fs, fs_factory(alice2) as fs2:

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
        ) as events_received, wait_for_entries_synced(
            fs2.signal_ns, ["/foo.txt"]
        ) as events_received:
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
