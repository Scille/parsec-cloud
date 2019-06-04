# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest

from tests.common import create_shared_workspace


@pytest.mark.trio
# @pytest.mark.linux  # TODO: investigate why this test fail on appveyor
@pytest.mark.parametrize("sync", ("/", "/foo"))
@pytest.mark.parametrize("type", ("folder", "file"))
async def test_realm_notif_on_new_entry_sync(
    running_backend, alice_core, alice2_user_fs, mock_clock, sync, type
):
    mock_clock.rate = 1
    wid = await create_shared_workspace("w", alice_core, alice2_user_fs)
    workspace = alice2_user_fs.get_workspace(wid)

    # Suspend time to freeze core background tasks
    mock_clock.rate = 0

    if type == "folder":
        await workspace.mkdir("/foo")
    elif type == "file":
        await workspace.touch("/foo")
    entry_id = await workspace.path_id("/foo")

    # Expected events
    entry_events = [
        (
            "backend.realm.vlobs_updated",
            {"realm_id": wid, "checkpoint": 2, "src_id": entry_id, "src_version": 1},
        ),
        # TODO: add ("fs.entry.downsynced", {"workspace_id": wid, "id": entry_id}),
    ]
    root_events = [
        (
            "backend.realm.vlobs_updated",
            {"realm_id": wid, "checkpoint": 3, "src_id": wid, "src_version": 2},
        ),
        ("fs.entry.downsynced", {"workspace_id": wid, "id": wid}),
    ]
    with alice_core.event_bus.listen() as spy:
        await workspace.sync(sync)
        mock_clock.rate = 1
        expected = entry_events if sync == "/foo" else entry_events + root_events
        await spy.wait_multiple_with_timeout(expected, 3)


@pytest.mark.trio
async def test_realm_notif_on_new_workspace_sync(
    mock_clock, running_backend, alice_core, alice2_user_fs
):

    # Suspend time to freeze core background tasks
    mock_clock.rate = 0
    uid = alice2_user_fs.user_manifest_id
    await alice2_user_fs.workspace_create("foo")

    expected = [
        (
            "backend.realm.vlobs_updated",
            {"realm_id": uid, "checkpoint": 2, "src_id": uid, "src_version": 2},
        )
    ]

    with alice_core.event_bus.listen() as spy:
        await alice2_user_fs.sync()
        mock_clock.rate = 1
        await spy.wait_multiple_with_timeout(expected, 3)
