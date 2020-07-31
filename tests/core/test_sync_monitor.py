# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio
import pytest
from unittest.mock import ANY

from parsec.core.backend_connection import BackendConnStatus
from parsec.backend.backend_events import BackendEvent
from parsec.core.core_events import CoreEvent


@pytest.mark.trio
async def test_monitors_idle(autojump_clock, running_backend, alice_core, alice):
    assert alice_core.are_monitors_idle()

    # Force wakeup of the sync monitor
    alice_core.event_bus.send(CoreEvent.FS_ENTRY_UPDATED, id=alice.user_manifest_id)
    assert not alice_core.are_monitors_idle()
    with trio.fail_after(60):  # autojump, so not *really* 60s
        await alice_core.wait_idle_monitors()
    assert alice_core.are_monitors_idle()


@pytest.mark.trio
async def test_monitor_switch_offline(autojump_clock, running_backend, alice_core, alice):
    assert alice_core.are_monitors_idle()
    assert alice_core.backend_status == BackendConnStatus.READY

    with alice_core.event_bus.listen() as spy:
        with running_backend.offline():
            await spy.wait_with_timeout(
                CoreEvent.BACKEND_CONNECTION_CHANGED,
                {"status": BackendConnStatus.LOST, "status_exc": spy.ANY},
                timeout=60,  # autojump, so not *really* 60s
            )
            await alice_core.wait_idle_monitors()
            assert alice_core.backend_status == BackendConnStatus.LOST

        # Switch backend online

        await spy.wait_with_timeout(
            CoreEvent.BACKEND_CONNECTION_CHANGED,
            {"status": BackendConnStatus.READY, "status_exc": None},
            timeout=60,  # autojump, so not *really* 60s
        )
        await alice_core.wait_idle_monitors()
        assert alice_core.backend_status == BackendConnStatus.READY


@pytest.mark.trio
async def test_process_while_offline(
    autojump_clock, running_backend, alice_core, bob_user_fs, alice, bob
):
    assert alice_core.backend_status == BackendConnStatus.READY

    with running_backend.offline():
        with alice_core.event_bus.listen() as spy:
            # Force wakeup of the sync monitor
            alice_core.event_bus.send(CoreEvent.FS_ENTRY_UPDATED, id=alice.user_manifest_id)
            assert not alice_core.are_monitors_idle()

            with trio.fail_after(60):  # autojump, so not *really* 60s
                await spy.wait(
                    CoreEvent.BACKEND_CONNECTION_CHANGED,
                    {"status": BackendConnStatus.LOST, "status_exc": spy.ANY},
                )
                await alice_core.wait_idle_monitors()
            assert alice_core.backend_status == BackendConnStatus.LOST


@pytest.mark.trio
async def test_autosync_on_modification(
    autojump_clock, running_backend, alice, alice_core, alice2_user_fs
):
    with alice_core.event_bus.listen() as spy:
        wid = await alice_core.user_fs.workspace_create("w")
        workspace = alice_core.user_fs.get_workspace(wid)
        # Wait for the sync monitor to sync the new workspace
        with trio.fail_after(60):  # autojump, so not *really* 60s
            await alice_core.wait_idle_monitors()
        spy.assert_events_occured(
            [
                (CoreEvent.FS_ENTRY_SYNCED, {"id": alice.user_manifest_id}),
                (CoreEvent.FS_ENTRY_SYNCED, {"workspace_id": wid, "id": wid}),
            ],
            in_order=False,
        )

    with alice_core.event_bus.listen() as spy:
        await workspace.mkdir("/foo")
        foo_id = await workspace.path_id("/foo")
        # Wait for the sync monitor to sync the new folder
        with trio.fail_after(60):  # autojump, so not *really* 60s
            await alice_core.wait_idle_monitors()
        spy.assert_events_occured(
            [
                (CoreEvent.FS_ENTRY_SYNCED, {"workspace_id": wid, "id": foo_id}),
                (CoreEvent.FS_ENTRY_SYNCED, {"workspace_id": wid, "id": wid}),
            ],
            in_order=False,
        )

    # Check workspace and folder have been correctly synced
    await alice2_user_fs.sync()
    workspace2 = alice2_user_fs.get_workspace(wid)
    path_info = await workspace.path_info("/foo")
    path_info2 = await workspace2.path_info("/foo")
    assert path_info == path_info2


@pytest.mark.trio
async def test_autosync_on_remote_modifications(
    autojump_clock, running_backend, alice, alice_core, alice2_user_fs
):
    with alice_core.event_bus.listen() as spy:
        wid = await alice2_user_fs.workspace_create("w")
        await alice2_user_fs.sync()

        # Wait for event to come back to alice_core
        await spy.wait_multiple_with_timeout(
            [
                (
                    CoreEvent.BACKEND_REALM_VLOBS_UPDATED,
                    {
                        "realm_id": alice.user_manifest_id,
                        "checkpoint": 2,
                        "src_id": alice.user_manifest_id,
                        "src_version": 2,
                    },
                ),
                (CoreEvent.FS_ENTRY_REMOTE_CHANGED, {"id": alice.user_manifest_id, "path": "/"}),
            ],
            timeout=60,  # autojump, so not *really* 60s
        )
        # Now wait for alice_core's sync
        with trio.fail_after(60):  # autojump, so not *really* 60s
            await alice_core.wait_idle_monitors()
        # Check workspace has been correctly synced
        alice_w = alice_core.user_fs.get_workspace(wid)

        alice2_w = alice2_user_fs.get_workspace(wid)
        await alice2_w.mkdir("/foo")
        foo_id = await alice2_w.path_id("/foo")
        await alice2_w.sync()

        # Wait for event to come back to alice_core
        await spy.wait_multiple_with_timeout(
            [
                (
                    CoreEvent.BACKEND_REALM_VLOBS_UPDATED,
                    {"realm_id": wid, "checkpoint": 2, "src_id": foo_id, "src_version": 1},
                ),
                (
                    CoreEvent.BACKEND_REALM_VLOBS_UPDATED,
                    {"realm_id": wid, "checkpoint": 3, "src_id": wid, "src_version": 2},
                ),
            ],
            timeout=60,  # autojump, so not *really* 60s
        )
        with trio.fail_after(60):  # autojump, so not *really* 60s
            await alice_core.wait_idle_monitors()
        # Check folder has been correctly synced
        path_info = await alice_w.path_info("/foo")
        path_info2 = await alice2_w.path_info("/foo")
        assert path_info == path_info2


@pytest.mark.trio
async def test_reconnect_with_remote_changes(
    autojump_clock, alice2, running_backend, alice_core, alice2_user_fs
):
    wid = await alice_core.user_fs.workspace_create("w")
    alice_w = alice_core.user_fs.get_workspace(wid)
    await alice_w.mkdir("/foo")
    await alice_w.touch("/bar.txt")
    # Wait for sync monitor to do it job
    await alice_core.wait_idle_monitors()

    with running_backend.offline_for(alice_core.device.device_id):
        # Get back modifications from alice
        await alice2_user_fs.sync()
        alice2_w = alice2_user_fs.get_workspace(wid)
        # Modify the workspace while alice is offline
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
                        BackendEvent.REALM_VLOBS_UPDATED,
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
                        BackendEvent.REALM_VLOBS_UPDATED,
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
                        BackendEvent.REALM_VLOBS_UPDATED,
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

    with alice_core.event_bus.listen() as spy:
        # Now alice should sync back the changes
        await spy.wait_with_timeout(
            CoreEvent.BACKEND_CONNECTION_CHANGED,
            {"status": BackendConnStatus.READY, "status_exc": spy.ANY},
            timeout=60,  # autojump, so not *really* 60s
        )
        await spy.wait_multiple_with_timeout(
            [
                (CoreEvent.FS_ENTRY_DOWNSYNCED, {"workspace_id": wid, "id": foo_id}),
                (CoreEvent.FS_ENTRY_DOWNSYNCED, {"workspace_id": wid, "id": bar_id}),
            ],
            in_order=False,
            timeout=60,  # autojump, so not *really* 60s
        )
