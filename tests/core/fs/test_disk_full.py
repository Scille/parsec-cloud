# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio
import pytest
from async_generator import asynccontextmanager

try:
    from tests.fuse_loopback import loopback_fs
except ImportError:
    pytest.skip("Fuse is required", allow_module_level=True)


from parsec.core.fs import UserFS
from parsec.core.logged_core import get_prevent_sync_pattern
from parsec.core.remote_devices_manager import RemoteDevicesManager
from parsec.core.backend_connection import backend_authenticated_cmds_factory
from parsec.core.fs.exceptions import FSLocalStorageOperationalError, FSLocalStorageClosedError

fixtures = (loopback_fs,)


@pytest.fixture
async def alice_fs_context(loopback_fs, event_bus_factory, alice):

    event_bus = event_bus_factory()
    path = loopback_fs.path / alice.device_name
    await trio.Path(path).mkdir()

    @asynccontextmanager
    async def _alice_context():
        async with backend_authenticated_cmds_factory(
            alice.organization_addr, alice.device_id, alice.signing_key
        ) as cmds:
            rdm = RemoteDevicesManager(cmds, alice.root_verify_key)
            async with UserFS.run(
                alice, path, cmds, rdm, event_bus, get_prevent_sync_pattern()
            ) as user_fs:
                yield user_fs

    async with _alice_context() as user_fs:
        wid = await user_fs.workspace_create("w")

    @asynccontextmanager
    async def _alice_fs_context(allow_sqlite_error_at_exit=False):
        try:
            exiting = False
            async with _alice_context() as user_fs:
                yield user_fs.get_workspace(wid)
                exiting = True
        except FSLocalStorageOperationalError:
            if not exiting:
                raise
            if not allow_sqlite_error_at_exit:
                raise

    return _alice_fs_context


@pytest.mark.trio
@pytest.mark.linux
@pytest.mark.mountpoint
async def test_workspace_fs_with_disk_full_simple(alice_fs_context, loopback_fs):
    async with alice_fs_context() as fs:
        await fs.write_bytes("/test.txt", b"abc")
        assert await fs.read_bytes("/test.txt") == b"abc"

    async with alice_fs_context() as fs:
        assert await fs.read_bytes("/test.txt") == b"abc"
        loopback_fs.full = True
        with pytest.raises(FSLocalStorageOperationalError):
            await fs.write_bytes("/test.txt", b"efg")

    with pytest.raises(FSLocalStorageOperationalError):
        async with alice_fs_context() as fs:
            pass

    loopback_fs.full = False
    async with alice_fs_context() as fs:
        assert await fs.read_bytes("/test.txt") == b"abc"


@pytest.mark.trio
@pytest.mark.linux
@pytest.mark.mountpoint
@pytest.mark.parametrize("number_of_write_before_full", range(50))
async def test_workspace_fs_with_disk_full_extended(
    alice_fs_context, loopback_fs, number_of_write_before_full
):
    async with alice_fs_context() as fs:
        await fs.write_bytes("/test.txt", b"aaa")
        loopback_fs.number_of_write_before_full = number_of_write_before_full
        with pytest.raises(FSLocalStorageOperationalError):
            while True:
                await fs.write_bytes("/test.txt", b"bbb")

    loopback_fs.full = False
    async with alice_fs_context() as fs:
        assert await fs.read_bytes("/test.txt") in (b"aaa", b"bbb", b"")


@pytest.mark.trio
@pytest.mark.linux
@pytest.mark.mountpoint
@pytest.mark.parametrize("number_of_write_before_full", range(50))
async def test_workspace_fs_with_disk_full_reloaded(
    alice_fs_context, loopback_fs, number_of_write_before_full
):
    async with alice_fs_context() as fs:
        await fs.write_bytes("/test.txt", b"")
        loopback_fs.number_of_write_before_full = number_of_write_before_full
        with pytest.raises(FSLocalStorageOperationalError):
            async with await fs.open_file("/test.txt", "wb") as f:
                while True:
                    await f.write(b"aaa")
                    await f.write(b"aaa")
                    await f.flush()

    loopback_fs.full = False
    async with alice_fs_context() as fs:
        result = await fs.read_bytes("/test.txt")
        assert result == b"a" * len(result)


@pytest.mark.trio
@pytest.mark.linux
@pytest.mark.mountpoint
@pytest.mark.parametrize("number_of_write_before_full", range(50))
async def test_workspace_fs_with_disk_full_ultimate(
    alice_fs_context, loopback_fs, number_of_write_before_full, running_backend
):
    async with alice_fs_context(allow_sqlite_error_at_exit=True) as fs:
        await fs.write_bytes("/test.txt", b"")
        loopback_fs.number_of_write_before_full = number_of_write_before_full
        with pytest.raises(FSLocalStorageOperationalError):
            async with await fs.open_file("/test.txt", "wb") as f:
                while True:
                    await f.write(b"aaa")
                    await f.write(b"aaa")
                    await fs.sync()

    loopback_fs.full = False
    async with alice_fs_context() as fs:
        result = await fs.read_bytes("/test.txt")
        assert result == b"a" * len(result)
