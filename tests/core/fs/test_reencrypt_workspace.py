# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import pytest
from pendulum import datetime
from unittest.mock import ANY

from parsec.api.data import EntryName
from parsec.api.protocol import RealmID
from parsec.core.types import EntryID
from parsec.core.fs import (
    FSError,
    FSWorkspaceNotFoundError,
    FSWorkspaceNotInMaintenance,
    FSWorkspaceInMaintenance,
    FSBadEncryptionRevision,
)
from parsec.backend.backend_events import BackendEvent

from tests.common import freeze_time


@pytest.fixture
async def workspace(running_backend, alice_user_fs):
    with freeze_time("2000-01-02"):
        wid = await alice_user_fs.workspace_create(EntryName("w1"))
        # Sync workspace manifest v1
        await alice_user_fs.sync()
        w = alice_user_fs.get_workspace(wid)
        await w.touch("/foo.txt")
        # Sync workspace manifest v1 + file manifest v1
        await w.sync()
        await w.write_bytes("/foo.txt", b"v2")
        # Sync file manifest v2
        await w.sync()

        # Should end up with 4 vlob atoms in the workspace

    return wid


@pytest.mark.trio
async def test_do_reencryption(running_backend, workspace, alice, alice_user_fs):
    with running_backend.backend.event_bus.listen() as spy:
        job = await alice_user_fs.workspace_start_reencryption(workspace)

        # Check events
        await spy.wait_multiple_with_timeout(
            [
                (
                    BackendEvent.REALM_MAINTENANCE_STARTED,
                    {
                        "organization_id": alice.organization_id,
                        "author": alice.device_id,
                        "realm_id": RealmID(workspace.uuid),
                        "encryption_revision": 2,
                    },
                ),
                (
                    BackendEvent.MESSAGE_RECEIVED,
                    {
                        "organization_id": alice.organization_id,
                        "author": alice.device_id,
                        "recipient": alice.user_id,
                        "index": 1,
                    },
                ),
            ]
        )

    total, done = await job.do_one_batch(size=1)
    assert total == 4
    assert done == 1

    total, done = await job.do_one_batch(size=2)
    assert total == 4
    assert done == 3

    total, done = await job.do_one_batch(size=2)
    assert total == 4
    assert done == 4

    with pytest.raises(FSWorkspaceNotInMaintenance):
        await job.do_one_batch()


@pytest.mark.trio
async def test_reencrypt_placeholder(running_backend, alice, alice_user_fs):
    wid = await alice_user_fs.workspace_create(EntryName("w1"))
    with pytest.raises(FSError):
        await alice_user_fs.workspace_start_reencryption(wid)


@pytest.mark.trio
async def test_unknown_workspace(alice_user_fs):
    bad_wid = EntryID.new()

    with pytest.raises(FSWorkspaceNotFoundError):
        await alice_user_fs.workspace_start_reencryption(bad_wid)

    with pytest.raises(FSWorkspaceNotFoundError):
        await alice_user_fs.workspace_continue_reencryption(bad_wid)


@pytest.mark.trio
async def test_concurrent_start_reencryption(workspace, alice_user_fs):
    await alice_user_fs.workspace_start_reencryption(workspace)

    with pytest.raises(FSWorkspaceInMaintenance):
        await alice_user_fs.workspace_start_reencryption(workspace)


@pytest.mark.trio
async def test_continue_reencryption_not_in_maintenance(workspace, alice_user_fs):
    with pytest.raises(FSWorkspaceNotInMaintenance):
        await alice_user_fs.workspace_continue_reencryption(workspace)


@pytest.mark.trio
async def test_continue_reencryption_with_bad_encryption_revision(workspace, alice_user_fs):
    await alice_user_fs.workspace_start_reencryption(workspace)

    with pytest.raises(FSError):
        await alice_user_fs.workspace_continue_reencryption(workspace)


@pytest.mark.trio
async def test_concurrent_continue_reencryption(running_backend, workspace, alice_user_fs):
    with running_backend.backend.event_bus.listen() as spy:
        job1 = await alice_user_fs.workspace_start_reencryption(workspace)

        await spy.wait_with_timeout(BackendEvent.MESSAGE_RECEIVED)

    # Update encryption_revision in user manifest
    await alice_user_fs.process_last_messages()

    job2 = await alice_user_fs.workspace_continue_reencryption(workspace)

    total, done = await job1.do_one_batch(size=1)
    assert total == 4
    assert done == 1

    total, done = await job2.do_one_batch(size=2)
    assert total == 4
    assert done == 3

    total, done = await job1.do_one_batch(size=2)
    assert total == 4
    assert done == 4

    with pytest.raises(FSWorkspaceNotInMaintenance):
        await job2.do_one_batch()

    with pytest.raises(FSWorkspaceNotInMaintenance):
        await job1.do_one_batch()


@pytest.mark.trio
async def test_reencryption_already_started(running_backend, alice_user_fs):
    with freeze_time("2000-01-02"):
        wid = await alice_user_fs.workspace_create(EntryName("w1"))
    await alice_user_fs.sync()

    await alice_user_fs.workspace_start_reencryption(wid)

    with pytest.raises(FSError):
        await alice_user_fs.workspace_start_reencryption(wid)


@pytest.mark.trio
async def test_no_access_during_reencryption(running_backend, alice2_user_fs, workspace):
    # Workspace have been created with alice_user_fs, hence user alice2_user_fs
    # start with no local cache
    await alice2_user_fs.sync()
    aw = alice2_user_fs.get_workspace(workspace)

    # Populate local cache for workspace root manifest
    await aw.sync()

    # Start reencryption
    job = await alice2_user_fs.workspace_start_reencryption(workspace)

    # WorkspaceFS doesn't have encryption revision until user messages are processed
    assert aw.get_encryption_revision() == 1
    # Data in local cache can be accessed
    root_info = await aw.path_info("/")
    assert root_info == {
        "id": workspace,
        "type": "folder",
        "base_version": 2,
        "created": datetime(2000, 1, 2),
        "updated": datetime(2000, 1, 2),
        "is_placeholder": False,
        "need_sync": False,
        "children": [EntryName("foo.txt")],
        "confinement_point": None,
    }
    # Data not in local cache can be downloaded
    foo_info = await aw.path_info("/foo.txt")
    assert foo_info["type"] == "file"
    assert foo_info["need_sync"] is False

    # But data cannot be synced
    with freeze_time("2000-01-03"):
        await aw.touch("/bar.txt")
        bar_id = await aw.path_id("/bar.txt")
    with pytest.raises(FSWorkspaceInMaintenance):
        await aw.sync_by_id(bar_id)

    # Finish reencryption
    while True:
        total, done = await job.do_one_batch()
        if total == done:
            break

    # The data in cache in still available
    root_info2 = await aw.path_info("/")
    assert root_info2 == {
        "id": workspace,
        "type": "folder",
        "base_version": 2,
        "created": datetime(2000, 1, 2),
        "updated": datetime(2000, 1, 3),
        "is_placeholder": False,
        "need_sync": True,
        "children": [EntryName("bar.txt"), EntryName("foo.txt")],
        "confinement_point": None,
    }
    foo_info = await aw.path_info("/foo.txt")
    assert foo_info["type"] == "file"
    assert foo_info["need_sync"] is False

    # But we're still not allowed to do the sync
    with pytest.raises(FSBadEncryptionRevision):
        await aw.sync_by_id(bar_id)

    # Update encryption_revision in user manifest and check access is ok
    await alice2_user_fs.process_last_messages()
    assert aw.get_encryption_revision() == 2
    file_info = await aw.path_info("/foo.txt")
    assert {
        "id": ANY,
        "type": "file",
        "base_version": 2,
        "created": datetime(2000, 1, 2),
        "updated": datetime(2000, 1, 2),
        "is_placeholder": False,
        "need_sync": False,
        "size": 2,
        "confinement_point": None,
    } == file_info

    # Finally sync is ok
    await aw.sync_by_id(bar_id)
