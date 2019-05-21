# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from functools import partial
import pytest

from parsec.core.types import FsPath

from tests.common import create_shared_workspace


@pytest.fixture
@pytest.mark.trio
async def shared_workspaces(alice_user_fs, bob_user_fs, running_backend):
    wid = await create_shared_workspace("w", alice_user_fs, bob_user_fs)
    alice_workspace = alice_user_fs.get_workspace(wid)
    bob_workspace = bob_user_fs.get_workspace(wid)
    await alice_workspace.sync("/")
    await bob_workspace.sync("/")
    return alice_workspace, bob_workspace


@pytest.fixture
def alice_workspace(shared_workspaces):
    alice_workspace, bob_workspace = shared_workspaces
    return alice_workspace


@pytest.fixture
def bob_workspace(shared_workspaces):
    alice_workspace, bob_workspace = shared_workspaces
    return bob_workspace


@pytest.mark.trio
@pytest.mark.parametrize("remote_changed", [False, True])
async def test_sync_by_access_single(alice_workspace, remote_changed):
    # Assuming the the remote has changed when it hasn't should not be an issue
    sync_by_access = partial(alice_workspace.sync_by_access, remote_changed=remote_changed)
    entry = alice_workspace.get_workspace_entry()
    w_access = entry.access

    def get_access(name):
        manifest = alice_workspace.local_storage.get_manifest(w_access)
        return manifest.children[name]

    # Empty workspace
    await sync_by_access(w_access)

    # Empty directory
    await alice_workspace.mkdir("/a")
    a_access = get_access("a")
    await sync_by_access(a_access)

    # Non empty workspace
    await alice_workspace.sync_by_access(w_access)

    # Directory with folder placeholders
    await alice_workspace.mkdir("/i/j/k", parents=True)
    i_access = get_access("i")
    await sync_by_access(i_access)

    # Workspace with folder placeholders
    await alice_workspace.mkdir("/x/y/z", parents=True)
    await sync_by_access(w_access)

    # Workspace with file placeholders
    await alice_workspace.touch("/f")
    await sync_by_access(w_access)


@pytest.mark.trio
async def test_sync_by_access_couple(alice_workspace, bob_workspace):
    alice_entry = alice_workspace.get_workspace_entry()
    alice_w_access = alice_entry.access
    bob_entry = bob_workspace.get_workspace_entry()
    bob_w_access = bob_entry.access

    # Create directories
    await alice_workspace.mkdir("/a")
    await bob_workspace.mkdir("/b")

    # Alice sync /a
    await alice_workspace.sync_by_access(alice_w_access)

    # Bob sync /b and doesn't know about alice /a yet
    await bob_workspace.sync_by_access(bob_w_access)

    # Alice knows about bob /b
    await alice_workspace.sync_by_access(alice_w_access, remote_changed=True)

    # Everyone should be up to date by now
    expected = [FsPath("/a"), FsPath("/b")]
    assert await bob_workspace.listdir("/") == expected
    assert await alice_workspace.listdir("/") == expected
