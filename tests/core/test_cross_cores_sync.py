import pytest
import trio
from async_generator import asynccontextmanager

from tests.common import freeze_time
from tests.open_tcp_stream_mock_wrapper import offline


@asynccontextmanager
async def wait_for_entries_synced(core, entries_pathes):
    event = trio.Event()
    to_sync = set(entries_pathes)
    synced = set()

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
        if synced == to_sync:
            event.set()

    with core.signal_ns.signal("fs.entry.synced").temporarily_connected_to(_on_entry_synced):
        yield event
        await event.wait()


@pytest.mark.trio
async def test_online_sync(autojump_clock, running_backend, core_factory, alice, alice2):
    # Given the cores are initialized while the backend is online, we are
    # guaranteed they are connected
    alice_core = await core_factory()
    alice2_core2 = await core_factory()
    await alice_core.login(alice)
    await alice2_core2.login(alice2)

    # FS does a full sync at startup, wait for it to finish
    await alice_core.fs.wait_not_syncing()
    await alice2_core2.fs.wait_not_syncing()

    async with wait_for_entries_synced(alice2_core2, ["/"]), wait_for_entries_synced(
        alice_core, ("/", "/foo.txt")
    ):

        with freeze_time("2000-01-02"):
            await alice_core.fs.file_create("/foo.txt")

        with freeze_time("2000-01-03"):
            await alice_core.fs.file_write("/foo.txt", b"hello world !")

        await alice_core.fs.sync("/foo.txt")

    stat = await alice_core.fs.stat("/foo.txt")
    stat2 = await alice2_core2.fs.stat("/foo.txt")
    assert stat2 == stat


@pytest.mark.trio
async def test_sync_then_clean_start(running_backend, core_factory, alice, alice2):
    # Given the cores are initialized while the backend is online, we are
    # guaranteed they are connected
    alice_core = await core_factory()
    await alice_core.login(alice)

    async with wait_for_entries_synced(alice_core, ("/", "/foo.txt")):

        with freeze_time("2000-01-02"):
            await alice_core.fs.file_create("/foo.txt")

        with freeze_time("2000-01-03"):
            await alice_core.fs.file_write("/foo.txt", b"v1")

        await alice_core.fs.sync("/foo.txt")

    alice2_core2 = await core_factory()
    async with wait_for_entries_synced(alice2_core2, ["/"]):
        await alice2_core2.login(alice2)

    for path in ("/", "/foo.txt"):
        stat = await alice_core.fs.stat(path)
        stat2 = await alice2_core2.fs.stat(path)
        assert stat2 == stat


@pytest.mark.trio
async def test_sync_then_fast_forward_on_start(
    autojump_clock, running_backend, core_factory, alice, alice2
):
    # Given the cores are initialized while the backend is online, we are
    # guaranteed they are connected
    alice_core = await core_factory()
    alice2_core2 = await core_factory()
    await alice_core.login(alice)
    await alice2_core2.login(alice2)

    with freeze_time("2000-01-02"):
        await alice_core.fs.file_create("/foo.txt")

    with freeze_time("2000-01-03"):
        await alice_core.fs.file_write("/foo.txt", b"v1")

    async with wait_for_entries_synced(alice2_core2, ["/"]), wait_for_entries_synced(
        alice_core, ("/", "/foo.txt")
    ):
        await alice_core.fs.sync("/foo.txt")

    await alice2_core2.logout()

    with freeze_time("2000-01-04"):
        await alice_core.fs.file_write("/foo.txt", b"v2")
        await alice_core.fs.folder_create("/bar")

    async with wait_for_entries_synced(alice_core, ["/", "/bar", "/foo.txt"]):
        await alice_core.fs.sync("/")

    async with wait_for_entries_synced(alice2_core2, ["/"]):
        await alice2_core2.login(alice2)

    for path in ("/", "/bar", "/foo.txt"):
        stat = await alice_core.fs.stat(path)
        stat2 = await alice2_core2.fs.stat(path)
        assert stat2 == stat


@pytest.mark.trio
async def test_fast_forward_on_offline_during_sync(
    autojump_clock, server_factory, backend, core_factory, alice, alice2
):
    server1 = server_factory(backend.handle_client)
    server2 = server_factory(backend.handle_client)

    # Given the cores are initialized while the backend is online, we are
    # guaranteed they are connected
    alice_core = await core_factory(config={"backend_addr": server1.addr})
    alice2_core2 = await core_factory(config={"backend_addr": server2.addr})
    await alice_core.login(alice)
    await alice2_core2.login(alice2)

    async with wait_for_entries_synced(alice2_core2, ["/"]), wait_for_entries_synced(
        alice_core, ("/", "/foo.txt")
    ):
        with freeze_time("2000-01-02"):
            await alice_core.fs.file_create("/foo.txt")

        with freeze_time("2000-01-03"):
            await alice_core.fs.file_write("/foo.txt", b"v1")

        await alice_core.fs.sync("/foo.txt")

    # core2 goes offline, other core is still connected to backend
    async with wait_for_entries_synced(alice_core, ("/", "/foo.txt")):
        with offline(server1.addr):

            with freeze_time("2000-01-04"):
                await alice2_core2.fs.file_write("/foo.txt", b"v2")
                await alice2_core2.fs.folder_create("/bar")

            async with wait_for_entries_synced(alice2_core2, ("/", "/bar", "/foo.txt")):
                await alice2_core2.fs.sync("/")

    for path in ("/", "/bar", "/foo.txt"):
        stat = await alice_core.fs.stat(path)
        stat2 = await alice2_core2.fs.stat(path)
        assert stat2 == stat
