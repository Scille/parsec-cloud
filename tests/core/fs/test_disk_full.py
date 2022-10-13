# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import trio
import pytest
from contextlib import asynccontextmanager

try:
    from tests.fuse_loopback import loopback_fs
except ImportError:
    pytest.skip("Fuse is required", allow_module_level=True)

from parsec.api.data import EntryName
from parsec.core.fs import UserFS
from parsec.core.logged_core import get_prevent_sync_pattern
from parsec.core.remote_devices_manager import RemoteDevicesManager
from parsec.core.backend_connection import backend_authenticated_cmds_factory
from parsec.core.fs.exceptions import FSLocalStorageOperationalError, FSLocalStorageClosedError
from tests.common import customize_fixtures

fixtures = (loopback_fs,)


@pytest.fixture
async def alice_fs_context(loopback_fs, event_bus_factory, alice, monkeypatch):
    @staticmethod
    async def _run_in_thread_patched(fn, *args):
        return fn(*args)

    # Disable running sqlite in threads as it causes inconsistent issues with the loopback FS
    monkeypatch.setattr(
        "parsec.core.fs.storage.local_database.LocalDatabase.run_in_thread", _run_in_thread_patched
    )

    event_bus = event_bus_factory()
    path = loopback_fs.path / alice.device_name.str
    await trio.Path(path).mkdir()

    @asynccontextmanager
    async def _alice_context():
        async with backend_authenticated_cmds_factory(
            alice.organization_addr, alice.device_id, alice.signing_key
        ) as cmds:
            rdm = RemoteDevicesManager(cmds, alice.root_verify_key, alice.time_provider)
            async with UserFS.run(
                path, alice, cmds, rdm, event_bus, get_prevent_sync_pattern()
            ) as user_fs:
                yield user_fs

    async with _alice_context() as user_fs:
        wid = await user_fs.workspace_create(EntryName("w"))

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
@pytest.mark.diskfull
@customize_fixtures(real_data_storage=True)
async def test_workspace_fs_with_disk_full_simple(alice_fs_context, loopback_fs):
    """Make sure the sqlite database can recover from a full disk error."""
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
@pytest.mark.diskfull
@customize_fixtures(real_data_storage=True)
async def test_workspace_fs_with_disk_full_large_write(alice_fs_context, loopback_fs):
    """Make sure the sqlite database can recover from a full disk error."""
    async with alice_fs_context() as fs:
        await fs.write_bytes("/test.txt", b"abc")
        assert await fs.read_bytes("/test.txt") == b"abc"

    async with alice_fs_context() as fs:
        assert await fs.read_bytes("/test.txt") == b"abc"
        loopback_fs.full = True
        async with await fs.open_file("/test.txt", "wb") as f:
            # This block doesn't commit the transaction since no flushing is performed
            # However, it might still raises an operational error since sqlite3 starts
            # writing to the disk when it holds too much data in memory. If this write
            # operation fails, the current transaction is rolled back, causing the same
            # problems as those discussed in issue #1535 with the commit operation.
            with pytest.raises(FSLocalStorageOperationalError):
                # Write 100 MB of "a" (100 * 256 blocks of 4K)
                for x in range(256 * 100):
                    await f.write(b"a" * 4 * 1024)

    with pytest.raises(FSLocalStorageOperationalError):
        async with alice_fs_context() as fs:
            pass

    loopback_fs.full = False
    async with alice_fs_context() as fs:
        assert await fs.read_bytes("/test.txt") == b"abc"


@pytest.mark.trio
@pytest.mark.linux
@pytest.mark.diskfull
@customize_fixtures(real_data_storage=True)
async def test_workspace_fs_with_disk_full_issue_1535(
    alice_fs_context, loopback_fs, running_backend
):
    """Perform the scenario described in issue #1535"""
    # Prepare the scenario
    async with alice_fs_context() as fs:
        await fs.write_bytes("/test.txt", b"")
        async with await fs.open_file("/test.txt", "wb") as f:

            # A file is being written
            await f.write(b"aaa")
            # The disk is full
            loopback_fs.full = True
            # Some task performs a commit (failing because of disk full)
            with pytest.raises(FSLocalStorageOperationalError):
                await fs.local_storage.data_localdb.commit()
            # The disk is no longer full (or the commit is smaller)
            loopback_fs.full = False
            # Force the manifest to be written to the disk
            with pytest.raises(FSLocalStorageClosedError):
                await f.flush()

    # Make sure the file is not corrupted
    async with alice_fs_context() as fs:
        result = await fs.read_bytes("/test.txt")
        assert result == b"a" * len(result)


@pytest.mark.trio
@pytest.mark.slow
@pytest.mark.linux
@pytest.mark.diskfull
@customize_fixtures(real_data_storage=True)
async def test_workspace_fs_with_disk_full_systematic(alice_fs_context, loopback_fs):
    """A more systematic but slower search for corruption."""
    for number_of_write_before_full in range(50):
        async with alice_fs_context() as fs:
            await fs.write_bytes("/test.txt", b"aaa")
            loopback_fs.number_of_write_before_full = number_of_write_before_full
            with pytest.raises(FSLocalStorageOperationalError):
                while True:
                    await fs.write_bytes("/test.txt", b"bbb")

        loopback_fs.full = False
        async with alice_fs_context() as fs:
            assert await fs.read_bytes("/test.txt") in (b"aaa", b"bbb")


@pytest.mark.trio
@pytest.mark.slow
@pytest.mark.linux
@pytest.mark.diskfull
@customize_fixtures(real_data_storage=True)
async def test_workspace_fs_with_disk_full_systematic_with_flush(alice_fs_context, loopback_fs):
    """A more systematic but slower search for corruption."""
    for number_of_write_before_full in range(50):
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
@pytest.mark.slow
@pytest.mark.linux
@pytest.mark.diskfull
@customize_fixtures(real_data_storage=True)
async def test_workspace_fs_with_disk_full_systematic_with_sync(
    alice_fs_context, loopback_fs, running_backend
):
    """A more systematic but slower search for corruption."""
    for number_of_write_before_full in range(50):
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
