# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import os
import sys
from pathlib import Path

import pytest
import trio

from parsec.api.data import EntryName
from parsec.core.core_events import CoreEvent
from parsec.core.fs import FsPath
from parsec.core.mountpoint.manager import mountpoint_manager_factory
from tests.common import create_shared_workspace


@pytest.mark.mountpoint
def test_fuse_grow_by_truncate(tmpdir, mountpoint_service):
    mountpoint = mountpoint_service.wpath

    oracle_fd = os.open(tmpdir / f"oracle-test", os.O_RDWR | os.O_CREAT)
    fd = os.open(mountpoint / "bar.txt", os.O_RDWR | os.O_CREAT)

    length = 1
    os.ftruncate(fd, length)
    os.ftruncate(oracle_fd, length)

    size = 1
    data = os.read(fd, size)
    expected_data = os.read(oracle_fd, size)
    assert data == expected_data


@pytest.mark.mountpoint
def test_empty_read_then_reopen(tmpdir, mountpoint_service):
    mountpoint = mountpoint_service.wpath

    oracle_fd = os.open(tmpdir / f"oracle-test", os.O_RDWR | os.O_CREAT)
    fd = os.open(mountpoint / "bar.txt", os.O_RDWR | os.O_CREAT)

    content = b"\x00"
    expected_ret = os.write(oracle_fd, content)
    ret = os.write(fd, content)
    assert ret == expected_ret

    size = 1
    expected_data = os.read(oracle_fd, size)
    data = os.read(fd, size)
    assert data == expected_data

    size = 0
    expected_data = os.read(oracle_fd, size)
    data = os.read(fd, size)
    assert data == expected_data

    os.close(oracle_fd)
    os.close(fd)
    oracle_fd = os.open(tmpdir / f"oracle-test", os.O_RDWR)
    fd = os.open(mountpoint / "bar.txt", os.O_RDWR)

    size = 1
    expected_data = os.read(oracle_fd, size)
    data = os.read(fd, size)
    assert data == expected_data


@pytest.mark.trio
@pytest.mark.mountpoint
@pytest.mark.skipif(sys.platform == "darwin", reason="Tests crash with offline backend")
async def test_remote_error_event(
    tmpdir, monkeypatch, caplog, running_backend, alice_user_fs, bob_user_fs
):
    wid = await create_shared_workspace(EntryName("w1"), bob_user_fs, alice_user_fs)

    base_mountpoint = Path(tmpdir / "alice_mountpoint")
    async with mountpoint_manager_factory(
        alice_user_fs, alice_user_fs.event_bus, base_mountpoint, debug=False
    ) as mountpoint_manager:

        await mountpoint_manager.mount_workspace(wid)

        # Create shared data
        bob_w = bob_user_fs.get_workspace(wid)
        await bob_w.touch("/foo.txt")
        await bob_w.write_bytes("/foo.txt", b"hello")
        await bob_w.sync()
        alice_w = alice_user_fs.get_workspace(wid)
        await alice_w.sync()
        # Force manifest cache
        await alice_w.path_id("/")
        await alice_w.path_id("/foo.txt")

        trio_w = trio.Path(mountpoint_manager.get_path_in_mountpoint(wid, FsPath("/")))

        # Offline test

        with running_backend.offline():

            with alice_user_fs.event_bus.listen() as spy:
                # Accessing workspace data in the backend should end up in remote error
                fd = await trio.to_thread.run_sync(os.open, str(trio_w / "foo.txt"), os.O_RDONLY)
                with pytest.raises(OSError):
                    await trio.to_thread.run_sync(os.read, fd, 10)
                await spy.wait_with_timeout(CoreEvent.MOUNTPOINT_REMOTE_ERROR)

            with alice_user_fs.event_bus.listen() as spy:
                # But should still be able to do local stuff though without remote errors
                await trio.to_thread.run_sync(
                    os.open, str(trio_w / "bar.txt"), os.O_RDWR | os.O_CREAT
                )
                assert await trio.to_thread.run_sync(os.listdir, str(trio_w)) == [
                    "bar.txt",
                    "foo.txt",
                ]
                # Let the loop process the potential `MOUNTPOINT_REMOTE_ERROR` we want to check are absent
                await trio.testing.wait_all_tasks_blocked()
                assert CoreEvent.MOUNTPOINT_REMOTE_ERROR not in [e.event for e in spy.events]

        # Online test

        def _crash(*args, **kwargs):
            raise RuntimeError("D'Oh !")

        monkeypatch.setattr(
            "parsec.core.fs.workspacefs.entry_transactions.EntryTransactions.folder_create", _crash
        )

        with alice_user_fs.event_bus.listen() as spy:
            with pytest.raises(OSError):
                await trio.to_thread.run_sync(os.mkdir, str(trio_w / "dummy"))
            await spy.wait_with_timeout(CoreEvent.MOUNTPOINT_UNHANDLED_ERROR)

        if sys.platform == "win32":
            expected_log = "[error    ] Unhandled exception in winfsp mountpoint [parsec.core.mountpoint.winfsp_operations]"
        else:
            expected_log = "[error    ] Unhandled exception in fuse mountpoint [parsec.core.mountpoint.fuse_operations]"

        caplog.assert_occured_once(expected_log)
