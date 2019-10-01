# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
import trio

from parsec.api.protocol import RealmRole
from parsec.core.backend_connection import backend_listen_events

from tests.common import create_shared_workspace


@pytest.fixture
async def running_backend_listen_events(running_backend, event_bus, alice):
    async with trio.open_nursery() as nursery:
        await nursery.start(backend_listen_events, alice, event_bus, None)
        yield
        nursery.cancel_scope.cancel()


@pytest.mark.trio
async def test_init_end_with_backend_online_status_event(running_backend, event_bus, alice):
    async with trio.open_nursery() as nursery:

        with event_bus.listen() as spy:
            await nursery.start(backend_listen_events, alice, event_bus, None)

        spy.assert_event_occured("backend.online")

        nursery.cancel_scope.cancel()


@pytest.mark.trio
async def test_init_end_with_backend_offline_status_event(event_bus, alice):
    async with trio.open_nursery() as nursery:

        with event_bus.listen() as spy:
            await nursery.start(backend_listen_events, alice, event_bus, None)

        spy.assert_event_occured("backend.offline")

        nursery.cancel_scope.cancel()


@pytest.mark.trio
async def test_backend_switch_offline(
    mock_clock, event_bus, backend, running_backend, running_backend_listen_events, alice
):
    mock_clock.rate = 1.0

    # Switch backend offline and wait for according event

    with event_bus.listen() as spy:
        with running_backend.offline():
            await spy.wait_with_timeout("backend.offline")

        # Here backend switch back online, wait for the corresponding event

        # Backend event manager waits before retrying to connect
        mock_clock.jump(5.0)

        await spy.wait_multiple_with_timeout(["backend.online", "backend.listener.restarted"])

    # Make sure event system still works as expected

    with event_bus.listen() as spy:
        backend.event_bus.send(
            "pinged", organization_id=alice.organization_id, author="bob@test", ping="foo"
        )

        await spy.wait_with_timeout("backend.pinged", {"ping": "foo"})


@pytest.mark.trio
@pytest.mark.parametrize("type", ("folder", "file"))
@pytest.mark.parametrize("sync_root", (False, True))
async def test_realm_notif_on_new_entry_sync(
    running_backend, alice_core, alice2_user_fs, mock_clock, sync_root, type
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
        if sync_root:
            await workspace.sync()
        else:
            await workspace.sync_by_id(entry_id)
        mock_clock.rate = 1
        expected = entry_events + root_events if sync_root else entry_events
        await spy.wait_multiple_with_timeout(expected, 3)


@pytest.mark.trio
async def test_realm_notif_on_new_workspace_sync(
    mock_clock, running_backend, alice_core, alice2_user_fs
):

    # Suspend time to freeze core background tasks
    mock_clock.rate = 0
    uid = alice2_user_fs.user_manifest_id
    wid = await alice2_user_fs.workspace_create("foo")

    expected = [
        # Access to newly created realm
        ("backend.realm.roles_updated", {"realm_id": wid, "role": RealmRole.OWNER}),
        # New realm workspace manifest created
        (
            "backend.realm.vlobs_updated",
            {"realm_id": wid, "checkpoint": 1, "src_id": wid, "src_version": 1},
        ),
        # User manifest updated
        (
            "backend.realm.vlobs_updated",
            {"realm_id": uid, "checkpoint": 2, "src_id": uid, "src_version": 2},
        ),
    ]

    with alice_core.event_bus.listen() as spy:
        await alice2_user_fs.sync()
        mock_clock.rate = 1
        await spy.wait_multiple_with_timeout(expected, 3)


@pytest.mark.trio
async def test_realm_notif_maintenance(running_backend, alice_core, bob_user_fs):
    wid = await create_shared_workspace("w", bob_user_fs, alice_core)

    with alice_core.event_bus.listen() as spy:
        # Start maintenance
        job = await bob_user_fs.workspace_start_reencryption(wid)

        await spy.wait_multiple_with_timeout(
            [
                ("backend.realm.maintenance_started", {"realm_id": wid, "encryption_revision": 2}),
                # Receive and process the new key and encryption revision
                "backend.message.received",
                "sharing.updated",
            ]
        )

    with alice_core.event_bus.listen() as spy:
        # Finish maintenance
        total, done = await job.do_one_batch()
        assert total == done

        await spy.wait_with_timeout(
            "backend.realm.maintenance_finished", {"realm_id": wid, "encryption_revision": 2}
        )
