# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import pytest

from parsec._parsec import Regex
from parsec.api.data import EntryName
from parsec.core.fs import FsPath
from tests.common import create_shared_workspace
from parsec.core.logged_core import get_prevent_sync_pattern


async def assert_path_info(workspace, path, **kwargs):
    info = await workspace.path_info(path)
    for key, value in kwargs.items():
        assert info[key] == value, key


@pytest.mark.trio
async def test_local_confinement_points(alice_workspace, running_backend):

    # Apply a *.tmp pattern
    pattern = Regex.from_regex_str(".*\\.tmp$")
    await alice_workspace.set_and_apply_prevent_sync_pattern(pattern)
    assert alice_workspace.local_storage.get_prevent_sync_pattern() == pattern
    assert alice_workspace.local_storage.get_prevent_sync_pattern_fully_applied()

    # Use foo as working directory
    await assert_path_info(alice_workspace, "/foo", confinement_point=None, need_sync=False)
    foo_id = await alice_workspace.path_id("/foo")

    # Create a temporary file and temporary folder
    await alice_workspace.mkdir("/foo/x.tmp")
    await alice_workspace.touch("/foo/y.tmp")

    # Check status
    await assert_path_info(alice_workspace, "/foo", confinement_point=None, need_sync=False)
    await assert_path_info(alice_workspace, "/foo/x.tmp", confinement_point=foo_id, need_sync=True)
    await assert_path_info(alice_workspace, "/foo/y.tmp", confinement_point=foo_id, need_sync=True)

    # Force a sync
    await alice_workspace.sync()

    # There should be nothing to synchronize
    await assert_path_info(alice_workspace, "/foo", confinement_point=None, need_sync=False)
    await assert_path_info(alice_workspace, "/foo/x.tmp", confinement_point=foo_id, need_sync=True)
    await assert_path_info(alice_workspace, "/foo/y.tmp", confinement_point=foo_id, need_sync=True)

    # Create a non-temporary file
    await alice_workspace.touch("/foo/z.txt")

    # Check status
    await assert_path_info(alice_workspace, "/foo", confinement_point=None, need_sync=True)
    await assert_path_info(alice_workspace, "/foo/x.tmp", confinement_point=foo_id, need_sync=True)
    await assert_path_info(alice_workspace, "/foo/y.tmp", confinement_point=foo_id, need_sync=True)
    await assert_path_info(alice_workspace, "/foo/z.txt", confinement_point=None, need_sync=True)

    # Force a sync
    await alice_workspace.sync()

    # Check status
    await assert_path_info(alice_workspace, "/foo", confinement_point=None, need_sync=False)
    await assert_path_info(alice_workspace, "/foo/x.tmp", confinement_point=foo_id, need_sync=True)
    await assert_path_info(alice_workspace, "/foo/y.tmp", confinement_point=foo_id, need_sync=True)
    await assert_path_info(alice_workspace, "/foo/z.txt", confinement_point=None, need_sync=False)

    # Remove a temporary file
    await alice_workspace.rmdir("/foo/x.tmp")
    assert not await alice_workspace.exists("/foo/x.tmp")

    # Check status
    await assert_path_info(alice_workspace, "/foo", confinement_point=None, need_sync=False)
    await assert_path_info(alice_workspace, "/foo/y.tmp", confinement_point=foo_id, need_sync=True)
    await assert_path_info(alice_workspace, "/foo/z.txt", confinement_point=None, need_sync=False)

    # Force a sync
    await alice_workspace.sync()

    # Nothing has changed
    await assert_path_info(alice_workspace, "/foo", confinement_point=None, need_sync=False)
    await assert_path_info(alice_workspace, "/foo/y.tmp", confinement_point=foo_id, need_sync=True)
    await assert_path_info(alice_workspace, "/foo/z.txt", confinement_point=None, need_sync=False)

    # Rename a temporary file
    await alice_workspace.rename("/foo/y.tmp", "/foo/y.txt")
    assert not await alice_workspace.exists("/foo/y.tmp")

    # Check status
    await assert_path_info(alice_workspace, "/foo", confinement_point=None, need_sync=True)
    await assert_path_info(alice_workspace, "/foo/y.txt", confinement_point=None, need_sync=True)
    await assert_path_info(alice_workspace, "/foo/z.txt", confinement_point=None, need_sync=False)

    # Force a sync
    await alice_workspace.sync()

    # Check status
    await assert_path_info(alice_workspace, "/foo", confinement_point=None, need_sync=False)
    await assert_path_info(alice_workspace, "/foo/y.txt", confinement_point=None, need_sync=False)
    await assert_path_info(alice_workspace, "/foo/z.txt", confinement_point=None, need_sync=False)


@pytest.mark.trio
async def test_sync_with_different_patterns(running_backend, alice_user_fs, alice2_user_fs):
    wid = await create_shared_workspace(EntryName("w"), alice_user_fs, alice2_user_fs)
    workspace1 = alice_user_fs.get_workspace(wid)
    workspace2 = alice2_user_fs.get_workspace(wid)

    # Workspace 1 patterns .tmp files
    pattern1 = Regex.from_regex_str(r".*\.tmp$")
    await workspace1.set_and_apply_prevent_sync_pattern(pattern1)
    await workspace1.sync()

    # Workspace 2 patterns ~ files
    pattern2 = Regex.from_regex_str(r".*~$")
    await workspace2.set_and_apply_prevent_sync_pattern(pattern2)
    await workspace2.sync()

    # Workspace 1 create some files and directories
    await workspace1.mkdir("/foo/xyz.tmp/bar", parents=True)
    await workspace1.mkdir("/foo/xyz~/bar", parents=True)
    await workspace1.mkdir("/foo/xyz/bar", parents=True)
    await workspace1.write_bytes("/foo/xyz.tmp/bar/test.txt", b"a1")
    await workspace1.write_bytes("/foo/xyz~/bar/test.txt", b"b1")
    await workspace1.write_bytes("/foo/xyz/bar/test.txt", b"c1")

    # Both workspaces sync
    await workspace1.sync()
    await workspace2.sync()

    # Check workspace 2
    assert await workspace2.listdir("/") == [FsPath("/foo")]
    assert await workspace2.listdir("/foo") == [FsPath("/foo/xyz")]
    assert await workspace2.listdir("/foo/xyz") == [FsPath("/foo/xyz/bar")]
    assert await workspace2.listdir("/foo/xyz/bar") == [FsPath("/foo/xyz/bar/test.txt")]

    # Check file content
    assert await workspace2.read_bytes("/foo/xyz/bar/test.txt") == b"c1"

    # Workspace 2 create the same files and directories
    await workspace2.mkdir("/foo/xyz.tmp/bar", parents=True)
    await workspace2.mkdir("/foo/xyz~/bar", parents=True)
    await workspace2.write_bytes("/foo/xyz.tmp/bar/test.txt", b"a2")
    await workspace2.write_bytes("/foo/xyz~/bar/test.txt", b"b2")
    await workspace2.write_bytes("/foo/xyz/bar/test.txt", b"c2")

    # Both workspaces sync
    await workspace2.sync()
    await workspace1.sync()

    # Check both workspaces
    for workspace in (workspace1, workspace2):
        assert await workspace.listdir("/") == [FsPath("/foo")]
        assert await workspace.listdir("/foo") == [
            FsPath("/foo/xyz"),
            FsPath("/foo/xyz.tmp"),
            FsPath("/foo/xyz~"),
        ]
        assert await workspace.listdir("/foo/xyz") == [FsPath("/foo/xyz/bar")]
        assert await workspace.listdir("/foo/xyz.tmp") == [FsPath("/foo/xyz.tmp/bar")]
        assert await workspace.listdir("/foo/xyz~") == [FsPath("/foo/xyz~/bar")]
        assert await workspace.listdir("/foo/xyz/bar") == [FsPath("/foo/xyz/bar/test.txt")]
        assert await workspace.listdir("/foo/xyz.tmp/bar") == [FsPath("/foo/xyz.tmp/bar/test.txt")]
        assert await workspace.listdir("/foo/xyz~/bar") == [FsPath("/foo/xyz~/bar/test.txt")]

    # Check file contents
    assert await workspace1.read_bytes("/foo/xyz.tmp/bar/test.txt") == b"a1"
    assert await workspace1.read_bytes("/foo/xyz~/bar/test.txt") == b"b1"
    assert await workspace1.read_bytes("/foo/xyz/bar/test.txt") == b"c2"
    assert await workspace2.read_bytes("/foo/xyz.tmp/bar/test.txt") == b"a2"
    assert await workspace2.read_bytes("/foo/xyz~/bar/test.txt") == b"b2"
    assert await workspace2.read_bytes("/foo/xyz/bar/test.txt") == b"c2"


@pytest.mark.trio
async def test_change_pattern(alice_workspace, running_backend):
    root_id = alice_workspace.workspace_id

    # Apply a *.x pattern
    pattern = Regex.from_regex_str(r".*\.x$")
    await alice_workspace.set_and_apply_prevent_sync_pattern(pattern)
    assert alice_workspace.local_storage.get_prevent_sync_pattern() == pattern
    assert alice_workspace.local_storage.get_prevent_sync_pattern_fully_applied()

    # Create x, y and z files
    await alice_workspace.touch("/test1.x")
    await alice_workspace.touch("/test1.y")
    await alice_workspace.touch("/test1.z")

    # Check status
    await assert_path_info(alice_workspace, "/test1.x", confinement_point=root_id, need_sync=True)
    await assert_path_info(alice_workspace, "/test1.y", confinement_point=None, need_sync=True)
    await assert_path_info(alice_workspace, "/test1.z", confinement_point=None, need_sync=True)

    # Synchronize then create more x, y and z files
    await alice_workspace.sync()
    await alice_workspace.touch("/test2.x")
    await alice_workspace.touch("/test2.y")
    await alice_workspace.touch("/test2.z")

    # Check status
    await assert_path_info(alice_workspace, "/test1.x", confinement_point=root_id, need_sync=True)
    await assert_path_info(alice_workspace, "/test1.y", confinement_point=None, need_sync=False)
    await assert_path_info(alice_workspace, "/test1.z", confinement_point=None, need_sync=False)
    await assert_path_info(alice_workspace, "/test2.x", confinement_point=root_id, need_sync=True)
    await assert_path_info(alice_workspace, "/test2.y", confinement_point=None, need_sync=True)
    await assert_path_info(alice_workspace, "/test2.y", confinement_point=None, need_sync=True)

    # Appy Y pattern
    pattern = Regex.from_regex_str(r".*\.y$")
    await alice_workspace.set_and_apply_prevent_sync_pattern(pattern)
    assert alice_workspace.local_storage.get_prevent_sync_pattern() == pattern
    assert alice_workspace.local_storage.get_prevent_sync_pattern_fully_applied()

    # Check status
    await assert_path_info(alice_workspace, "/test1.x", confinement_point=None, need_sync=True)
    await assert_path_info(alice_workspace, "/test1.y", confinement_point=root_id, need_sync=False)
    await assert_path_info(alice_workspace, "/test1.z", confinement_point=None, need_sync=False)
    await assert_path_info(alice_workspace, "/test2.x", confinement_point=None, need_sync=True)
    await assert_path_info(alice_workspace, "/test2.y", confinement_point=root_id, need_sync=True)
    await assert_path_info(alice_workspace, "/test2.z", confinement_point=None, need_sync=True)

    # Synchronize the workspace
    await alice_workspace.sync()

    await assert_path_info(alice_workspace, "/test1.x", confinement_point=None, need_sync=False)
    await assert_path_info(alice_workspace, "/test1.y", confinement_point=root_id, need_sync=False)
    await assert_path_info(alice_workspace, "/test1.z", confinement_point=None, need_sync=False)
    await assert_path_info(alice_workspace, "/test2.x", confinement_point=None, need_sync=False)
    await assert_path_info(alice_workspace, "/test2.y", confinement_point=root_id, need_sync=True)
    await assert_path_info(alice_workspace, "/test2.z", confinement_point=None, need_sync=False)

    # Rollback to X pattern
    pattern = Regex.from_regex_str(r".*\.x$")
    await alice_workspace.set_and_apply_prevent_sync_pattern(pattern)
    assert alice_workspace.local_storage.get_prevent_sync_pattern() == pattern
    assert alice_workspace.local_storage.get_prevent_sync_pattern_fully_applied()

    # Check status
    await assert_path_info(alice_workspace, "/test1.x", confinement_point=root_id, need_sync=False)
    await assert_path_info(alice_workspace, "/test1.y", confinement_point=None, need_sync=False)
    await assert_path_info(alice_workspace, "/test1.z", confinement_point=None, need_sync=False)
    await assert_path_info(alice_workspace, "/test2.x", confinement_point=root_id, need_sync=False)
    await assert_path_info(alice_workspace, "/test2.y", confinement_point=None, need_sync=True)
    await assert_path_info(alice_workspace, "/test2.z", confinement_point=None, need_sync=False)

    # Synchronize the workspace
    await alice_workspace.sync()

    await assert_path_info(alice_workspace, "/test1.x", confinement_point=root_id, need_sync=False)
    await assert_path_info(alice_workspace, "/test1.y", confinement_point=None, need_sync=False)
    await assert_path_info(alice_workspace, "/test1.z", confinement_point=None, need_sync=False)
    await assert_path_info(alice_workspace, "/test2.x", confinement_point=root_id, need_sync=False)
    await assert_path_info(alice_workspace, "/test2.y", confinement_point=None, need_sync=False)
    await assert_path_info(alice_workspace, "/test2.z", confinement_point=None, need_sync=False)


@pytest.mark.trio
async def test_common_temporary_files(alice_workspace):
    file_list = ["test.txt", "test" "t" ".test"]
    for path in file_list:
        path = "/" + path
        await alice_workspace.touch(path)
        await assert_path_info(alice_workspace, path, confinement_point=None)

    confined_file_list = [
        "test.tmp",
        "test.temp",
        "test.swp",
        "test~",
        ".fuse_hidden000001",
        ".directory",
        ".Trash-0001",
        ".nfsxxx",
        ".goutputstream-U6EGP0",
        ".DS_Store",
        ".AppleDouble",
        ".LSOverride",
        "._test",
        ".fseventsd",
        "Thumbs.db",
        "test.stackdump",
        "Desktop.ini",
        "desktop.ini",
        "$RECYCLE.BIN",
        "test.lnk",
        ".~test",
        "~$test",
        "mydoc.docx.sb-324kJJ4-AGBJ32A",
    ]
    for path in confined_file_list:
        path = "/" + path
        await alice_workspace.touch(path)
        print(path)
        await assert_path_info(
            alice_workspace, path, confinement_point=alice_workspace.workspace_id
        )


def test_stable_prevent_sync_pattern():
    """
    Prevent sync pattern are compared in the local database
    so we need to make they are stable.

    See issue #3145 for more information.
    """
    a = get_prevent_sync_pattern()
    b = get_prevent_sync_pattern()
    assert a.pattern == b.pattern


@pytest.mark.trio
async def test_database_with_invalid_pattern_resilience(core_factory, alice):
    with pytest.raises(ValueError):
        Regex.from_regex_str("[")

    class InvalidRegex:
        pattern = "["

    # Set an invalid regex in the local database
    async with core_factory(alice) as core:
        wid = await core.user_fs.workspace_create(EntryName("w"))
        workspace = core.user_fs.get_workspace(wid)
        await workspace.local_storage.manifest_storage.set_prevent_sync_pattern(InvalidRegex())

    async with core_factory(alice) as core:
        workspace = core.user_fs.get_workspace(wid)
        await workspace.path_info("/")
        assert workspace.local_storage.get_prevent_sync_pattern() == get_prevent_sync_pattern()
        assert workspace.local_storage.get_prevent_sync_pattern_fully_applied()
