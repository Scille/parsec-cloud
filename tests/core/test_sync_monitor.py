# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import re
import trio
import pytest
from unittest.mock import ANY

from parsec.api.data import EntryName
from parsec.api.protocol import RealmID, VlobID
from parsec.backend.backend_events import BackendEvent
from parsec.core.backend_connection import BackendConnStatus
from parsec.core.core_events import CoreEvent
from parsec.core.types import WorkspaceRole
from parsec.core.logged_core import logged_core_factory
from parsec.core.fs.exceptions import FSReadOnlyError

from tests.common import create_shared_workspace, customize_fixtures


@pytest.mark.trio
async def test_monitors_idle(autojump_clock, running_backend, alice_core, alice):
    autojump_clock.setup()

    assert alice_core.are_monitors_idle()

    # Force wakeup of the sync monitor
    alice_core.event_bus.send(CoreEvent.FS_ENTRY_UPDATED, id=alice.user_manifest_id)
    assert not alice_core.are_monitors_idle()
    with trio.fail_after(60):  # autojump, so not *really* 60s
        await alice_core.wait_idle_monitors()
    assert alice_core.are_monitors_idle()


@pytest.mark.trio
async def test_monitor_switch_offline(autojump_clock, running_backend, alice_core, alice):
    autojump_clock.setup()

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
    autojump_clock.setup()

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
@customize_fixtures(backend_not_populated=True)
async def test_autosync_placeholder_user_manifest(
    autojump_clock,
    running_backend,
    backend_data_binder,
    event_bus_factory,
    core_config,
    coolorg,
    alice,
    alice2,
):
    # Local database run sqlite operations in thread that can create timeout
    # if autojump is too greedy
    autojump_clock.setup(autojump_threshold=0.1)

    # Sync with realm&vlob not creation on server side
    await backend_data_binder.bind_organization(coolorg, alice, initial_user_manifest="not_synced")
    # Don't use `core_factory` fixture given it whole point is to waits for
    # monitors to be idle before returning the core
    async with logged_core_factory(core_config, alice, event_bus=event_bus_factory()) as alice_core:
        # Wait for the sync monitor to sync the new workspace
        with alice_core.event_bus.listen() as spy:
            await spy.wait_with_timeout(
                CoreEvent.FS_ENTRY_SYNCED, {"id": alice.user_manifest_id}, timeout=60
            )

    # Sync with existing realm&vlob on server side
    await backend_data_binder.bind_device(alice2)
    async with logged_core_factory(
        core_config, alice2, event_bus=event_bus_factory()
    ) as alice2_core:
        with alice2_core.event_bus.listen() as spy:
            # Wait for the sync monitor to sync the new workspace
            await spy.wait_with_timeout(
                CoreEvent.FS_ENTRY_REMOTE_CHANGED,
                {"id": alice2.user_manifest_id, "path": "/"},
                timeout=60,
            )


@pytest.mark.trio
@customize_fixtures(backend_not_populated=True)
async def test_autosync_placeholder_workspace_manifest(
    autojump_clock,
    running_backend,
    backend_data_binder,
    event_bus_factory,
    core_config,
    coolorg,
    alice,
    alice2,
):
    # Local database run sqlite operations in thread that can create timeout
    # if autojump is too greedy
    autojump_clock.setup(autojump_threshold=0.1)

    # Workspace created before user manifest placeholder sync
    await backend_data_binder.bind_organization(coolorg, alice, initial_user_manifest="not_synced")
    # Don't use `core_factory` fixture given it whole point is to waits for
    # monitors to be idle before returning the core
    async with logged_core_factory(core_config, alice, event_bus=event_bus_factory()) as alice_core:
        with alice_core.event_bus.listen() as spy:
            w1id = await alice_core.user_fs.workspace_create(EntryName("w1"))
            # Wait for the sync monitor to sync the new workspace
            await spy.wait_multiple_with_timeout(
                [
                    (CoreEvent.FS_ENTRY_SYNCED, {"id": alice.user_manifest_id}),
                    (CoreEvent.FS_ENTRY_SYNCED, {"workspace_id": w1id, "id": w1id}),
                ],
                in_order=False,
                timeout=60,
            )

    # Workspace created on a synced user manifest
    await backend_data_binder.bind_device(alice2)
    async with logged_core_factory(
        core_config, alice2, event_bus=event_bus_factory()
    ) as alice2_core:
        # Workspace created before user manifest placeholder sync
        with alice2_core.event_bus.listen() as spy:
            w2id = await alice2_core.user_fs.workspace_create(EntryName("w2"))
            await spy.wait_multiple_with_timeout(
                [
                    (CoreEvent.FS_ENTRY_SYNCED, {"id": alice2.user_manifest_id}),
                    (CoreEvent.FS_ENTRY_SYNCED, {"workspace_id": w2id, "id": w2id}),
                ],
                in_order=False,
                timeout=60,
            )


@pytest.mark.trio
async def test_autosync_on_modification(
    autojump_clock, running_backend, alice, alice_core, alice2_user_fs
):
    autojump_clock.setup()

    with alice_core.event_bus.listen() as spy:
        wid = await alice_core.user_fs.workspace_create(EntryName("w"))
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
    await workspace2.sync()
    path_info = await workspace.path_info("/foo")
    path_info2 = await workspace2.path_info("/foo")
    assert path_info == path_info2


@pytest.mark.trio
async def test_autosync_on_remote_modifications(
    autojump_clock, running_backend, alice, alice_core, alice2_user_fs
):
    autojump_clock.setup()

    with alice_core.event_bus.listen() as spy:
        wid = await alice2_user_fs.workspace_create(EntryName("w"))
        await alice2_user_fs.sync()

        # Wait for event to come back to alice_core
        await spy.wait_multiple_with_timeout(
            [
                (
                    CoreEvent.BACKEND_REALM_VLOBS_UPDATED,
                    {"realm_id": wid, "checkpoint": 1, "src_id": wid, "src_version": 1},
                ),
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

    # Remote changes in workspace
    with alice_core.event_bus.listen() as spy:

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
    autojump_clock.setup()

    wid = await alice_core.user_fs.workspace_create(EntryName("w"))
    alice_w = alice_core.user_fs.get_workspace(wid)
    await alice_w.mkdir("/foo")
    await alice_w.touch("/bar.txt")
    # Wait for sync monitor to do it job
    await alice_core.wait_idle_monitors()

    with running_backend.offline_for(alice_core.device.device_id):
        # Get back modifications from alice
        await alice2_user_fs.sync()
        alice2_w = alice2_user_fs.get_workspace(wid)
        await alice2_w.sync()
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
                            "realm_id": RealmID(wid.uuid),
                            "checkpoint": ANY,
                            "src_id": VlobID(spam_id.uuid),
                            "src_version": 1,
                        },
                    ),
                    (
                        BackendEvent.REALM_VLOBS_UPDATED,
                        {
                            "organization_id": alice2.organization_id,
                            "author": alice2.device_id,
                            "realm_id": RealmID(wid.uuid),
                            "checkpoint": ANY,
                            "src_id": VlobID(foo_id.uuid),
                            "src_version": 2,
                        },
                    ),
                    (
                        BackendEvent.REALM_VLOBS_UPDATED,
                        {
                            "organization_id": alice2.organization_id,
                            "author": alice2.device_id,
                            "realm_id": RealmID(wid.uuid),
                            "checkpoint": ANY,
                            "src_id": VlobID(bar_id.uuid),
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


@pytest.mark.trio
async def test_sync_confined_children_after_rename(
    autojump_clock, alice, running_backend, alice_core
):
    autojump_clock.setup()

    # Create a workspace
    wid = await alice_core.user_fs.workspace_create(EntryName("w"))
    alice_w = alice_core.user_fs.get_workspace(wid)

    # Set a filter
    pattern = re.compile(r".*\.tmp$")
    await alice_w.set_and_apply_prevent_sync_pattern(pattern)

    # Create a confined path
    await alice_w.mkdir("/test.tmp/a/b/c", parents=True)

    # Wait for sync monitor to be idle
    await alice_core.wait_idle_monitors()

    # Make sure the root is synced
    info = await alice_w.path_info("/")
    assert not info["need_sync"]
    assert not info["confinement_point"]

    # Make sure the rest of the path is confined
    for path in ["/test.tmp", "/test.tmp/a", "/test.tmp/a/b", "/test.tmp/a/b/c"]:
        info = await alice_w.path_info(path)
        assert info["need_sync"]
        assert info["confinement_point"]

    # Rename to another confined path
    await alice_w.rename("/test.tmp", "/test2.tmp")

    # Wait for sync monitor to be idle
    await alice_core.wait_idle_monitors()

    # Make sure the root is synced
    info = await alice_w.path_info("/")
    assert not info["need_sync"]
    assert not info["confinement_point"]

    # Make sure the rest of the path is confined
    for path in ["/test2.tmp", "/test2.tmp/a", "/test2.tmp/a/b", "/test2.tmp/a/b/c"]:
        info = await alice_w.path_info(path)
        assert info["need_sync"]
        assert info["confinement_point"]

    # Rename to non-confined path
    await alice_w.rename("/test2.tmp", "/test2")

    # Wait for sync monitor to be idle
    await alice_core.wait_idle_monitors()

    # Make sure the root is synced
    info = await alice_w.path_info("/")
    assert not info["need_sync"]
    assert not info["confinement_point"]

    # Make sure the rest of the path is confined
    for path in ["/test2", "/test2/a", "/test2/a/b", "/test2/a/b/c"]:
        info = await alice_w.path_info(path)
        assert not info["need_sync"]
        assert not info["confinement_point"]

    # Rename to a confined path
    await alice_w.rename("/test2", "/test3.tmp")

    # Wait for sync monitor to be idle
    await alice_core.wait_idle_monitors()

    # Make sure the root is synced
    info = await alice_w.path_info("/")
    assert not info["need_sync"]
    assert not info["confinement_point"]

    # Make sure the rest of the path is confined
    for path in ["/test3.tmp", "/test3.tmp/a", "/test3.tmp/a/b", "/test3.tmp/a/b/c"]:
        info = await alice_w.path_info(path)
        assert not info["need_sync"]
        assert info["confinement_point"]


@pytest.mark.trio
async def test_sync_monitor_while_changing_roles(
    autojump_clock, running_backend, alice_core, bob_core
):
    autojump_clock.setup()

    # Create a shared workspace
    wid = await create_shared_workspace(EntryName("w"), alice_core, bob_core)
    alice_workspace = alice_core.user_fs.get_workspace(wid)
    bob_workspace = bob_core.user_fs.get_workspace(wid)

    # Alice creates a files, let it sync
    with bob_core.event_bus.listen() as spy:
        with trio.fail_after(60):  # autojump, so not *really* 60s
            await alice_workspace.write_bytes("/test.txt", b"test")
            await alice_core.wait_idle_monitors()
            # Ensure bob receive the change notification and process it
            await spy.wait(CoreEvent.FS_ENTRY_DOWNSYNCED, kwargs={"workspace_id": wid, "id": wid})
            await bob_core.wait_idle_monitors()

    # Bob edit the files..
    assert await bob_workspace.read_bytes("/test.txt") == b"test"
    await bob_workspace.write_bytes("/test.txt", b"test2")

    # But gets his role changed to READER
    with bob_core.event_bus.listen() as spy:
        with trio.fail_after(60):  # autojump, so not *really* 60s
            await alice_core.user_fs.workspace_share(
                wid, bob_core.device.user_id, WorkspaceRole.READER
            )
            await spy.wait(CoreEvent.SHARING_UPDATED)
            await bob_core.wait_idle_monitors()

    # The file cannot be synced
    info = await bob_workspace.path_info("/test.txt")
    assert info["need_sync"]

    # And the workspace is now read-only
    with pytest.raises(FSReadOnlyError):
        await bob_workspace.write_bytes("/this-should-fail", b"abc")

    # Alice restores CONTRIBUTOR rights to Bob
    with bob_core.event_bus.listen() as spy:
        with trio.fail_after(60):  # autojump, so not *really* 60s
            await alice_core.user_fs.workspace_share(
                wid, bob_core.device.user_id, WorkspaceRole.CONTRIBUTOR
            )
            await spy.wait(CoreEvent.SHARING_UPDATED)
            await bob_core.wait_idle_monitors()

    # The edit file has been synced
    info = await bob_workspace.path_info("/test.txt")
    assert not info["need_sync"]

    # So Alice can read it
    await alice_core.wait_idle_monitors()
    assert await alice_workspace.read_bytes("/test.txt") == b"test2"

    # The workspace can be written again
    await bob_workspace.write_bytes("/this-should-not-fail", b"abc")
    await bob_core.wait_idle_monitors()
    info = await bob_workspace.path_info("/this-should-not-fail")
    assert not info["need_sync"]


@pytest.mark.trio
async def test_sync_with_concurrent_reencryption(
    autojump_clock, running_backend, alice_core, bob_user_fs, monkeypatch
):
    autojump_clock.setup()

    # Create a shared workspace
    wid = await create_shared_workspace(EntryName("w"), bob_user_fs, alice_core)
    alice_workspace = alice_core.user_fs.get_workspace(wid)
    bob_workspace = bob_user_fs.get_workspace(wid)

    # Alice creates a files, let it sync
    await alice_workspace.write_bytes("/test.txt", b"v1")
    await alice_core.wait_idle_monitors()
    await bob_user_fs.sync()

    # Freeze Alice message processing so she won't process `sharing.reencrypted` messages
    allow_message_processing = trio.Event()

    async def _mockpoint_sleep():
        await allow_message_processing.wait()

    monkeypatch.setattr(
        "parsec.core.messages_monitor.freeze_messages_monitor_mockpoint", _mockpoint_sleep
    )

    # Now Bob reencrypt the workspace
    reencryption_job = await bob_user_fs.workspace_start_reencryption(wid)
    await bob_user_fs.process_last_messages()
    total, done = await reencryption_job.do_one_batch()
    assert total == done  # Sanity check to make sure the encryption is finished

    # Alice modify the workspace and try to do the sync...
    await alice_workspace.write_bytes("/test.txt", b"v2")
    # Sync monitor will try and fail to do the sync of the workspace
    await trio.sleep(300)  # autojump, so not *really* 300s
    assert not alice_core.are_monitors_idle()

    # Now let Alice process the `sharing.reencrypted` messages, this should
    # allow to do the sync
    allow_message_processing.set()
    with trio.fail_after(60):  # autojump, so not *really* 60s
        await alice_core.wait_idle_monitors()

    # Just make sure the sync is done
    await bob_workspace.sync()
    for workspace in (bob_workspace, alice_workspace):
        info = await workspace.path_info("/test.txt")
        assert not info["need_sync"]
        assert info["base_version"] == 3
