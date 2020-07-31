# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import re
import pytest


async def assert_path_info(workspace, path, **kwargs):
    info = await workspace.path_info(path)
    for key, value in kwargs.items():
        assert info[key] == value


@pytest.mark.trio
async def test_confined_entries(alice_workspace, running_backend):

    # Apply a *.tmp filter
    pattern = re.compile(r".*\.tmp$")
    await alice_workspace.set_and_apply_pattern_filter(pattern)
    assert alice_workspace.local_storage.get_pattern_filter() == pattern
    assert alice_workspace.local_storage.get_pattern_filter_fully_applied()

    # Use foo as working directory
    await assert_path_info(alice_workspace, "/foo", confined=False, need_sync=False)

    # Create a temporary file and temporary folder
    await alice_workspace.mkdir("/foo/x.tmp")
    await alice_workspace.touch("/foo/y.tmp")

    # Check status
    await assert_path_info(alice_workspace, "/foo", confined=False, need_sync=False)
    await assert_path_info(alice_workspace, "/foo/x.tmp", confined=True, need_sync=True)
    await assert_path_info(alice_workspace, "/foo/y.tmp", confined=True, need_sync=True)

    # Force a sync
    await alice_workspace.sync()

    # There should be nothing to synchronize
    await assert_path_info(alice_workspace, "/foo", confined=False, need_sync=False)
    await assert_path_info(alice_workspace, "/foo/x.tmp", confined=True, need_sync=True)
    await assert_path_info(alice_workspace, "/foo/y.tmp", confined=True, need_sync=True)

    # Create a non-temporary file
    await alice_workspace.touch("/foo/z.txt")

    # Check status
    await assert_path_info(alice_workspace, "/foo", confined=False, need_sync=True)
    await assert_path_info(alice_workspace, "/foo/x.tmp", confined=True, need_sync=True)
    await assert_path_info(alice_workspace, "/foo/y.tmp", confined=True, need_sync=True)
    await assert_path_info(alice_workspace, "/foo/z.txt", confined=False, need_sync=True)

    # Force a sync
    await alice_workspace.sync()

    # Check status
    await assert_path_info(alice_workspace, "/foo", confined=False, need_sync=False)
    await assert_path_info(alice_workspace, "/foo/x.tmp", confined=True, need_sync=True)
    await assert_path_info(alice_workspace, "/foo/y.tmp", confined=True, need_sync=True)
    await assert_path_info(alice_workspace, "/foo/z.txt", confined=False, need_sync=False)

    # Remove a temporary file
    await alice_workspace.rmdir("/foo/x.tmp")
    assert not await alice_workspace.exists("/foo/x.tmp")

    # Check status
    await assert_path_info(alice_workspace, "/foo", confined=False, need_sync=False)
    await assert_path_info(alice_workspace, "/foo/y.tmp", confined=True, need_sync=True)
    await assert_path_info(alice_workspace, "/foo/z.txt", confined=False, need_sync=False)

    # Force a sync
    await alice_workspace.sync()

    # Nothing has changed
    await assert_path_info(alice_workspace, "/foo", confined=False, need_sync=False)
    await assert_path_info(alice_workspace, "/foo/y.tmp", confined=True, need_sync=True)
    await assert_path_info(alice_workspace, "/foo/z.txt", confined=False, need_sync=False)

    # Rename a temporary file
    await alice_workspace.rename("/foo/y.tmp", "/foo/y.txt")
    assert not await alice_workspace.exists("/foo/y.tmp")

    # Check status
    await assert_path_info(alice_workspace, "/foo", confined=False, need_sync=True)
    await assert_path_info(alice_workspace, "/foo/y.txt", confined=False, need_sync=True)
    await assert_path_info(alice_workspace, "/foo/z.txt", confined=False, need_sync=False)

    # Force a sync
    await alice_workspace.sync()

    # Check status
    await assert_path_info(alice_workspace, "/foo", confined=False, need_sync=False)
    await assert_path_info(alice_workspace, "/foo/y.txt", confined=False, need_sync=False)
    await assert_path_info(alice_workspace, "/foo/z.txt", confined=False, need_sync=False)
