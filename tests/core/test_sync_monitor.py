# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from unittest.mock import ANY, Mock
from urllib.error import HTTPError, URLError

import pytest
import trio

from parsec._parsec import CoreEvent, Regex
from parsec.api.data import EntryName
from parsec.api.protocol import RealmID, VlobID
from parsec.backend.backend_events import BackendEvent
from parsec.backend.sequester import SequesterServiceType
from parsec.core.backend_connection import BackendConnStatus
from parsec.core.fs.exceptions import FSReadOnlyError
from parsec.core.logged_core import logged_core_factory
from parsec.core.types import WorkspaceRole
from tests.common import create_shared_workspace, customize_fixtures, sequester_service_factory


@pytest.mark.trio
async def test_monitors_idle(frozen_clock, running_backend, alice_core, alice):
    assert alice_core.are_monitors_idle()

    # Force wakeup of the sync monitor
    alice_core.event_bus.send(CoreEvent.FS_ENTRY_UPDATED, id=alice.user_manifest_id)
    assert not alice_core.are_monitors_idle()
    await frozen_clock.sleep_with_autojump(60)
    async with frozen_clock.real_clock_timeout():
        await alice_core.wait_idle_monitors()
    assert alice_core.are_monitors_idle()


@pytest.mark.trio
async def test_monitor_switch_offline(frozen_clock, running_backend, alice_core, alice):
    assert alice_core.are_monitors_idle()
    assert alice_core.backend_status == BackendConnStatus.READY

    with alice_core.event_bus.listen() as spy:
        with running_backend.offline():
            await frozen_clock.sleep_with_autojump(60)
            await spy.wait_with_timeout(
                CoreEvent.BACKEND_CONNECTION_CHANGED,
                {"status": BackendConnStatus.LOST, "status_exc": spy.ANY},
            )
            await alice_core.wait_idle_monitors()
            assert alice_core.backend_status == BackendConnStatus.LOST

        # Switch backend online

        await frozen_clock.sleep_with_autojump(60)
        await spy.wait_with_timeout(
            CoreEvent.BACKEND_CONNECTION_CHANGED,
            {"status": BackendConnStatus.READY, "status_exc": None},
        )
        await alice_core.wait_idle_monitors()
        assert alice_core.backend_status == BackendConnStatus.READY


@pytest.mark.trio
async def test_process_while_offline(
    frozen_clock, running_backend, alice_core, bob_user_fs, alice, bob
):
    assert alice_core.backend_status == BackendConnStatus.READY

    with running_backend.offline():
        with alice_core.event_bus.listen() as spy:
            # Force wakeup of the sync monitor
            alice_core.event_bus.send(CoreEvent.FS_ENTRY_UPDATED, id=alice.user_manifest_id)
            assert not alice_core.are_monitors_idle()

            await frozen_clock.sleep_with_autojump(60)
            async with frozen_clock.real_clock_timeout():
                await spy.wait(
                    CoreEvent.BACKEND_CONNECTION_CHANGED,
                    {"status": BackendConnStatus.LOST, "status_exc": spy.ANY},
                )
                await alice_core.wait_idle_monitors()
            assert alice_core.backend_status == BackendConnStatus.LOST


# This test has been detected as flaky.
# Using re-runs is a valid temporary solutions but the problem should be investigated in the future.
@pytest.mark.trio
@pytest.mark.flaky(reruns=3)
@customize_fixtures(backend_not_populated=True)
async def test_autosync_placeholder_user_manifest(
    frozen_clock,
    running_backend,
    backend_data_binder,
    event_bus_factory,
    core_config,
    coolorg,
    alice,
    alice2,
):
    # Sync with realm&vlob not creation on server side
    await backend_data_binder.bind_organization(coolorg, alice, initial_user_manifest="not_synced")
    # Don't use `core_factory` fixture given it whole point is to waits for
    # monitors to be idle before returning the core
    async with logged_core_factory(core_config, alice, event_bus=event_bus_factory()) as alice_core:
        # Wait for the sync monitor to sync the new workspace
        with alice_core.event_bus.listen() as spy:
            await frozen_clock.sleep_with_autojump(60)
            await spy.wait_with_timeout(CoreEvent.FS_ENTRY_SYNCED, {"id": alice.user_manifest_id})

    # Sync with existing realm&vlob on server side
    await backend_data_binder.bind_device(alice2)
    async with logged_core_factory(
        core_config, alice2, event_bus=event_bus_factory()
    ) as alice2_core:
        with alice2_core.event_bus.listen() as spy:
            # Wait for the sync monitor to sync the new workspace
            await frozen_clock.sleep_with_autojump(60)
            await spy.wait_with_timeout(
                CoreEvent.FS_ENTRY_REMOTE_CHANGED, {"id": alice2.user_manifest_id, "path": "/"}
            )


# This test has been detected as flaky.
# Using re-runs is a valid temporary solutions but the problem should be investigated in the future.
@pytest.mark.trio
@pytest.mark.flaky(reruns=3)
@customize_fixtures(backend_not_populated=True)
async def test_autosync_placeholder_workspace_manifest(
    frozen_clock,
    running_backend,
    backend_data_binder,
    event_bus_factory,
    core_config,
    coolorg,
    alice,
    alice2,
):
    # Workspace created before user manifest placeholder sync
    await backend_data_binder.bind_organization(coolorg, alice, initial_user_manifest="not_synced")
    # Don't use `core_factory` fixture given it whole point is to waits for
    # monitors to be idle before returning the core
    async with logged_core_factory(core_config, alice, event_bus=event_bus_factory()) as alice_core:
        with alice_core.event_bus.listen() as spy:
            w1id = await alice_core.user_fs.workspace_create(EntryName("w1"))
            # Wait for the sync monitor to sync the new workspace
            await frozen_clock.sleep_with_autojump(60)
            await spy.wait_multiple_with_timeout(
                [
                    (CoreEvent.FS_ENTRY_SYNCED, {"id": alice.user_manifest_id}),
                    (CoreEvent.FS_ENTRY_SYNCED, {"workspace_id": w1id, "id": w1id}),
                ],
                in_order=False,
            )

    # Workspace created on a synced user manifest
    await backend_data_binder.bind_device(alice2)
    async with logged_core_factory(
        core_config, alice2, event_bus=event_bus_factory()
    ) as alice2_core:
        # Workspace created before user manifest placeholder sync
        with alice2_core.event_bus.listen() as spy:
            w2id = await alice2_core.user_fs.workspace_create(EntryName("w2"))
            await frozen_clock.sleep_with_autojump(60)
            await spy.wait_multiple_with_timeout(
                [
                    (CoreEvent.FS_ENTRY_SYNCED, {"id": alice2.user_manifest_id}),
                    (CoreEvent.FS_ENTRY_SYNCED, {"workspace_id": w2id, "id": w2id}),
                ],
                in_order=False,
            )


@pytest.mark.trio
async def test_autosync_on_modification(
    frozen_clock, running_backend, alice, alice_core, alice2_user_fs
):
    with alice_core.event_bus.listen() as spy:
        wid = await alice_core.user_fs.workspace_create(EntryName("w"))
        workspace = alice_core.user_fs.get_workspace(wid)
        # Wait for the sync monitor to sync the new workspace
        await frozen_clock.sleep_with_autojump(60)
        async with frozen_clock.real_clock_timeout():
            await alice_core.wait_idle_monitors()
        spy.assert_events_occurred(
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
        await frozen_clock.sleep_with_autojump(60)
        async with frozen_clock.real_clock_timeout():
            await alice_core.wait_idle_monitors()
        spy.assert_events_occurred(
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
    frozen_clock, running_backend, alice, alice_core, alice2_user_fs
):
    with alice_core.event_bus.listen() as spy:
        wid = await alice2_user_fs.workspace_create(EntryName("w"))
        await alice2_user_fs.sync()

        # Wait for event to come back to alice_core, then alice_core's sync
        await frozen_clock.sleep_with_autojump(60)
        async with frozen_clock.real_clock_timeout():
            await spy.wait_multiple(
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
                    (
                        CoreEvent.FS_ENTRY_REMOTE_CHANGED,
                        {"id": alice.user_manifest_id, "path": "/"},
                    ),
                ]
            )
            # Now wait for alice_core's sync
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
        await frozen_clock.sleep_with_autojump(60)
        async with frozen_clock.real_clock_timeout():
            await spy.wait_multiple(
                [
                    (
                        CoreEvent.BACKEND_REALM_VLOBS_UPDATED,
                        {"realm_id": wid, "checkpoint": 2, "src_id": foo_id, "src_version": 1},
                    ),
                    (
                        CoreEvent.BACKEND_REALM_VLOBS_UPDATED,
                        {"realm_id": wid, "checkpoint": 3, "src_id": wid, "src_version": 2},
                    ),
                ]
            )
            await alice_core.wait_idle_monitors()

    # Check folder has been correctly synced
    path_info = await alice_w.path_info("/foo")
    path_info2 = await alice2_w.path_info("/foo")
    assert path_info == path_info2


@pytest.mark.trio
async def test_reconnect_with_remote_changes(
    frozen_clock, alice2, running_backend, running_backend_factory, alice_core, user_fs_factory
):
    wid = await alice_core.user_fs.workspace_create(EntryName("w"))
    alice_w = alice_core.user_fs.get_workspace(wid)
    await alice_w.mkdir("/foo")
    await alice_w.touch("/bar.txt")
    # Wait for sync monitor to do it job
    await frozen_clock.sleep_with_autojump(60)
    async with frozen_clock.real_clock_timeout():
        await alice_core.wait_idle_monitors()

    # Alice2 connect to the backend through a different server so that we can
    # switch alice offline while keeping alice2 connected
    async with running_backend_factory(running_backend.backend) as running_backend2:
        alice2 = running_backend2.correct_addr(alice2)
        async with user_fs_factory(alice2) as alice2_user_fs:

            # Switch backend offline for alice (but not alice2 !)
            with running_backend.offline():
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
                                    "realm_id": RealmID.from_entry_id(wid),
                                    "checkpoint": ANY,
                                    "src_id": VlobID.from_entry_id(spam_id),
                                    "src_version": 1,
                                },
                            ),
                            (
                                BackendEvent.REALM_VLOBS_UPDATED,
                                {
                                    "organization_id": alice2.organization_id,
                                    "author": alice2.device_id,
                                    "realm_id": RealmID.from_entry_id(wid),
                                    "checkpoint": ANY,
                                    "src_id": VlobID.from_entry_id(foo_id),
                                    "src_version": 2,
                                },
                            ),
                            (
                                BackendEvent.REALM_VLOBS_UPDATED,
                                {
                                    "organization_id": alice2.organization_id,
                                    "author": alice2.device_id,
                                    "realm_id": RealmID.from_entry_id(wid),
                                    "checkpoint": ANY,
                                    "src_id": VlobID.from_entry_id(bar_id),
                                    "src_version": 2,
                                },
                            ),
                        ],
                        in_order=False,
                    )

            with alice_core.event_bus.listen() as spy:
                # Now alice should sync back the changes
                await frozen_clock.sleep_with_autojump(60)
                await spy.wait_multiple_with_timeout(
                    [
                        (
                            CoreEvent.BACKEND_CONNECTION_CHANGED,
                            {"status": BackendConnStatus.READY, "status_exc": spy.ANY},
                        ),
                        (CoreEvent.FS_ENTRY_DOWNSYNCED, {"workspace_id": wid, "id": foo_id}),
                        (CoreEvent.FS_ENTRY_DOWNSYNCED, {"workspace_id": wid, "id": bar_id}),
                    ],
                    in_order=False,
                )


@pytest.mark.trio
async def test_sync_confined_children_after_rename(
    frozen_clock, alice, running_backend, alice_core
):
    # Create a workspace
    wid = await alice_core.user_fs.workspace_create(EntryName("w"))
    alice_w = alice_core.user_fs.get_workspace(wid)

    # Set a filter
    pattern = Regex.from_regex_str(r".*\.tmp$")
    await alice_w.set_and_apply_prevent_sync_pattern(pattern)

    # Create a confined path
    await alice_w.mkdir("/test.tmp/a/b/c", parents=True)

    # Wait for sync monitor to be idle
    await frozen_clock.sleep_with_autojump(60)
    async with frozen_clock.real_clock_timeout():
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
    await frozen_clock.sleep_with_autojump(60)
    async with frozen_clock.real_clock_timeout():
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
    await frozen_clock.sleep_with_autojump(60)
    async with frozen_clock.real_clock_timeout():
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
    await frozen_clock.sleep_with_autojump(60)
    async with frozen_clock.real_clock_timeout():
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
    frozen_clock, running_backend, alice_core, bob_core
):
    # Create a shared workspace
    wid = await create_shared_workspace(EntryName("w"), alice_core, bob_core)
    alice_workspace = alice_core.user_fs.get_workspace(wid)
    bob_workspace = bob_core.user_fs.get_workspace(wid)

    # Alice creates a files, let it sync
    with bob_core.event_bus.listen() as spy:
        await alice_workspace.write_bytes("/test.txt", b"test")
        await frozen_clock.sleep_with_autojump(60)
        async with frozen_clock.real_clock_timeout():
            await alice_core.wait_idle_monitors()

        # Ensure bob receive the change notification and process it
        await spy.wait_with_timeout(
            CoreEvent.FS_ENTRY_DOWNSYNCED, kwargs={"workspace_id": wid, "id": wid}
        )
        await frozen_clock.sleep_with_autojump(60)
        async with frozen_clock.real_clock_timeout():
            await bob_core.wait_idle_monitors()

    # Bob edit the files..
    assert await bob_workspace.read_bytes("/test.txt") == b"test"
    await bob_workspace.write_bytes("/test.txt", b"test2")

    # But gets his role changed to READER
    with bob_core.event_bus.listen() as spy:
        await alice_core.user_fs.workspace_share(wid, bob_core.device.user_id, WorkspaceRole.READER)
        await frozen_clock.sleep_with_autojump(60)
        async with frozen_clock.real_clock_timeout():
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
        await alice_core.user_fs.workspace_share(
            wid, bob_core.device.user_id, WorkspaceRole.CONTRIBUTOR
        )
        await frozen_clock.sleep_with_autojump(60)
        async with frozen_clock.real_clock_timeout():
            await spy.wait(CoreEvent.SHARING_UPDATED)
            await bob_core.wait_idle_monitors()

    # The edit file has been synced
    info = await bob_workspace.path_info("/test.txt")
    assert not info["need_sync"]

    # So Alice can read it
    await frozen_clock.sleep_with_autojump(60)
    async with frozen_clock.real_clock_timeout():
        await alice_core.wait_idle_monitors()
    assert await alice_workspace.read_bytes("/test.txt") == b"test2"

    # The workspace can be written again
    await bob_workspace.write_bytes("/this-should-not-fail", b"abc")
    await frozen_clock.sleep_with_autojump(60)
    async with frozen_clock.real_clock_timeout():
        await bob_core.wait_idle_monitors()
    info = await bob_workspace.path_info("/this-should-not-fail")
    assert not info["need_sync"]


@pytest.mark.trio
async def test_sync_with_concurrent_reencryption(
    frozen_clock, running_backend, alice_core, bob_user_fs, monkeypatch
):
    # Create a shared workspace
    wid = await create_shared_workspace(EntryName("w"), bob_user_fs, alice_core)
    alice_workspace = alice_core.user_fs.get_workspace(wid)
    bob_workspace = bob_user_fs.get_workspace(wid)

    # Alice creates a files, let it sync
    await alice_workspace.write_bytes("/test.txt", b"v1")
    await frozen_clock.sleep_with_autojump(60)
    async with frozen_clock.real_clock_timeout():
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
    await frozen_clock.sleep_with_autojump(300)
    assert not alice_core.are_monitors_idle()

    # Now let Alice process the `sharing.reencrypted` messages, this should
    # allow to do the sync
    allow_message_processing.set()
    await frozen_clock.sleep_with_autojump(60)
    async with frozen_clock.real_clock_timeout():
        await alice_core.wait_idle_monitors()

    # Just make sure the sync is done
    await bob_workspace.sync()
    for workspace in (bob_workspace, alice_workspace):
        info = await workspace.path_info("/test.txt")
        assert not info["need_sync"]
        assert info["base_version"] == 3


@pytest.mark.trio
@pytest.mark.xfail(reason="TODO: should work once TimeProvider is ready")
@customize_fixtures(coolorg_is_sequestered_organization=True)
async def test_sync_timeout_and_rejected_by_sequester_service(
    monkeypatch, caplog, frozen_clock, coolorg, alice, running_backend, alice_core, unused_tcp_port
):
    async def _wait_sync_is_done():
        assert not alice_core.are_monitors_idle()
        await frozen_clock.sleep_with_autojump(40)
        async with frozen_clock.real_clock_timeout():
            await alice_core.wait_idle_monitors()
        assert alice_core.are_monitors_idle()

    # Create a workspace & make sure everything is sync so far
    wid = await alice_core.user_fs.workspace_create(EntryName("w"))
    await _wait_sync_is_done()
    alice_workspace = alice_core.user_fs.get_workspace(wid)

    # Now add a sequester service that will work... unhelpfully ;-)
    webhook_url = f"https://localhost:{unused_tcp_port}/webhook"
    s1 = sequester_service_factory(
        authority=coolorg.sequester_authority,
        label="Sequester service 1",
        service_type=SequesterServiceType.WEBHOOK,
        webhook_url=webhook_url,
    )
    await running_backend.backend.sequester.create_service(
        organization_id=coolorg.organization_id, service=s1.backend_service
    )

    ################################
    # 1) First test workspacefs
    ################################

    # 1.a) Test sequester timeout

    # Very first call to sequester service is not available, this should
    # make the sync monitor wait for some time before retrying
    webhook_calls = 0

    async def _mocked_http_request(**kwargs):
        nonlocal webhook_calls
        webhook_calls += 1
        # if webhook_calls == 4:
        #     breakpoint()
        if webhook_calls == 1:
            raise URLError("[Errno -2] Name or service not known")
        else:
            return b""

    monkeypatch.setattr("parsec.backend.vlob.http_request", _mocked_http_request)

    # Do a change and way for the sync to be finished
    await alice_workspace.write_bytes("/test.txt", b"v1")
    await _wait_sync_is_done()

    # We called the webhook 4 times:
    # - first time failed
    # - second time for the file manifest minimal sync
    # - third time for the parent folder manifest sync
    # - fourth time for the actual file manifest sync
    assert webhook_calls == 4
    caplog.assert_occurred_once(
        f"[warning  ] Sync failure due to server upload temporarily unavailable [parsec.core.sync_monitor] workspace_id={wid.str}"
    )
    bad_file_info = await alice_workspace.path_info("/test.txt")
    assert bad_file_info["need_sync"] is False
    assert bad_file_info["base_version"] == 1

    # 1.b) Test sequester rejection

    async def _mocked_http_request(**kwargs):
        nonlocal webhook_calls
        webhook_calls += 1
        fp = Mock()
        fp.read.return_value = b'{"reason": "some_error_from_service"}'
        raise HTTPError(webhook_url, 400, "", None, fp)

    monkeypatch.setattr("parsec.backend.vlob.http_request", _mocked_http_request)

    await alice_workspace.write_bytes("/test.txt", b"v2 with virus !")
    await _wait_sync_is_done()

    # The trick is the invalid entry has not been synchronized, but sync monitor
    # just forget about it to avoid busy sync loops with the server...
    bad_file_info = await alice_workspace.path_info("/test.txt")
    assert bad_file_info["need_sync"] is True
    assert bad_file_info["base_version"] == 2

    # ...so if we modify again the file everything should be synced fine

    async def _mocked_http_request(**kwargs):
        return b""

    monkeypatch.setattr("parsec.backend.vlob.http_request", _mocked_http_request)

    await alice_workspace.write_bytes("/test.txt", b"v2")
    await _wait_sync_is_done()
    bad_file_info = await alice_workspace.path_info("/test.txt")
    assert bad_file_info["need_sync"] is False
    assert bad_file_info["base_version"] == 3

    ################################
    # 2) Now test userfs
    ################################

    # 2.a) Test sequester service timeout

    webhook_calls = 0

    async def _mocked_http_request(**kwargs):
        nonlocal webhook_calls
        webhook_calls += 1
        if webhook_calls == 4:
            breakpoint()
        # print(webhook_calls, kwargs)
        if webhook_calls == 1:
            raise URLError("[Errno -2] Name or service not known")
        else:
            return b""

    monkeypatch.setattr("parsec.backend.vlob.http_request", _mocked_http_request)

    # Do a change and way for the sync to be finished
    await alice_core.user_fs.workspace_rename(wid, EntryName("new_name"))
    await _wait_sync_is_done()

    assert webhook_calls == 2
    caplog.assert_occurred_once(
        f"[warning  ] Sync failure due to server upload temporarily unavailable [parsec.core.sync_monitor] workspace_id={alice_core.user_fs.user_manifest_id.str}"
    )

    # 2.b) Test sequester service rejection

    async def _mocked_http_request(**kwargs):
        fp = Mock()
        fp.read.return_value = b'{"reason": "some_error_from_service"}'
        raise HTTPError(webhook_url, 400, "", None, fp)

    monkeypatch.setattr("parsec.backend.vlob.http_request", _mocked_http_request)

    await alice_core.user_fs.workspace_rename(wid, EntryName("new_new_name"))
    await _wait_sync_is_done()

    um = alice_core.user_fs.get_user_manifest()
    assert um.need_sync is True

    # Modifying user manifest should trigger a new sync

    async def _mocked_http_request(**kwargs):
        return b""

    monkeypatch.setattr("parsec.backend.vlob.http_request", _mocked_http_request)

    await alice_core.user_fs.workspace_rename(wid, EntryName("new_new_name"))
    await _wait_sync_is_done()
    um = alice_core.user_fs.get_user_manifest()
    assert um.need_sync is False
