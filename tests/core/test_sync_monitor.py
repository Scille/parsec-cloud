# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from unittest.mock import ANY

from parsec.core.backend_connection import BackendConnStatus


@pytest.mark.trio
async def test_autosync_on_modification(mock_clock, running_backend, alice_core, alice2_user_fs):
    mock_clock.rate = 0  # Avoid potential concurrency with monitors

    wid = await alice_core.user_fs.workspace_create("w")
    workspace = alice_core.user_fs.get_workspace(wid)
    await alice_core.user_fs.sync()

    mock_clock.rate = 1
    mock_clock.autojump_threshold = 0.1

    with alice_core.event_bus.listen() as spy:
        await workspace.mkdir("/foo")
        foo_id = await workspace.path_id("/foo")
        root_id = await workspace.path_id("/")
        await spy.wait_multiple_with_timeout(
            [
                ("fs.entry.synced", {"workspace_id": wid, "id": foo_id}),
                ("fs.entry.synced", {"workspace_id": wid, "id": root_id}),
            ],
            timeout=60,  # autojump, so not *really* 60s
            in_order=False,
        )

    await alice2_user_fs.sync()
    workspace2 = alice2_user_fs.get_workspace(wid)
    path_info = await workspace.path_info("/foo")
    path_info2 = await workspace2.path_info("/foo")
    assert path_info == path_info2


@pytest.mark.trio
async def test_sync_on_missing_workspace(
    mock_clock, running_backend, alice, alice_core, alice2_user_fs
):
    mock_clock.rate = 1
    mock_clock.autojump_threshold = 0.1

    with alice_core.event_bus.listen() as spy:
        wid = await alice2_user_fs.workspace_create("w")
        await alice2_user_fs.sync()

        await spy.wait_multiple_with_timeout(
            [
                (
                    "backend.realm.vlobs_updated",
                    {
                        "realm_id": alice.user_manifest_id,
                        "checkpoint": 2,
                        "src_id": alice.user_manifest_id,
                        "src_version": 2,
                    },
                ),
                ("fs.entry.remote_changed", {"id": alice.user_manifest_id, "path": "/"}),
            ],
            timeout=60,  # autojump, so not *really* 60s
        )

        alice2_w = alice2_user_fs.get_workspace(wid)
        await alice2_w.mkdir("/foo")
        foo_id = await alice2_w.path_id("/foo")
        await alice2_w.sync()

        # Client receive vlob updated events, but doesn't process
        # them given the corresponding workspace is not loaded
        await spy.wait_multiple_with_timeout(
            [
                (
                    "backend.realm.vlobs_updated",
                    {"realm_id": wid, "checkpoint": 2, "src_id": foo_id, "src_version": 1},
                ),
                (
                    "backend.realm.vlobs_updated",
                    {"realm_id": wid, "checkpoint": 3, "src_id": wid, "src_version": 2},
                ),
            ],
            timeout=60,  # autojump, so not *really* 60s
        )


@pytest.mark.trio
async def test_reconnect_with_remote_changes(
    mock_clock, alice2, running_backend, alice_core, alice2_user_fs
):
    mock_clock.rate = 1

    wid = await alice_core.user_fs.workspace_create("w")
    alice_w = alice_core.user_fs.get_workspace(wid)
    await alice_w.mkdir("/foo")
    await alice_w.touch("/bar.txt")
    await alice_core.user_fs.sync()
    await alice_w.sync()

    with running_backend.offline_for(alice_core.device.device_id):
        # Modify the workspace while alice is offline
        await alice2_user_fs.sync()
        alice2_w = alice2_user_fs.get_workspace(wid)
        await alice2_w.mkdir("/foo/spam")
        await alice2_w.write_bytes("/bar.txt", b"v2")

        foo_id = await alice2_w.path_id("/foo")
        spam_id = await alice2_w.path_id("/foo/spam")
        bar_id = await alice2_w.path_id("/bar.txt")

        with running_backend.backend.event_bus.listen() as spy:
            await alice2_w.sync()
            # Alice misses the vlob updated events before being back online
            await spy.wait_multiple_with_timeout(
                [
                    (
                        "realm.vlobs_updated",
                        {
                            "organization_id": alice2.organization_id,
                            "author": alice2.device_id,
                            "realm_id": wid,
                            "checkpoint": ANY,
                            "src_id": spam_id,
                            "src_version": 1,
                        },
                    ),
                    (
                        "realm.vlobs_updated",
                        {
                            "organization_id": alice2.organization_id,
                            "author": alice2.device_id,
                            "realm_id": wid,
                            "checkpoint": ANY,
                            "src_id": foo_id,
                            "src_version": 2,
                        },
                    ),
                    (
                        "realm.vlobs_updated",
                        {
                            "organization_id": alice2.organization_id,
                            "author": alice2.device_id,
                            "realm_id": wid,
                            "checkpoint": ANY,
                            "src_id": bar_id,
                            "src_version": 2,
                        },
                    ),
                ],
                in_order=False,
            )

    mock_clock.autojump_threshold = 0.1

    with alice_core.event_bus.listen() as spy:
        # Now alice should sync back the changes
        await spy.wait_with_timeout(
            "backend.connection.changed",
            {"status": BackendConnStatus.READY, "status_exc": spy.ANY},
            timeout=60,  # autojump, so not *really* 60s
        )
        await spy.wait_multiple_with_timeout(
            [
                ("fs.entry.downsynced", {"workspace_id": wid, "id": foo_id}),
                ("fs.entry.downsynced", {"workspace_id": wid, "id": bar_id}),
            ],
            in_order=False,
            timeout=60,  # autojump, so not *really* 60s
        )
