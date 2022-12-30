# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from typing import Optional

import pytest
import trio

from parsec._parsec import (
    AuthenticatedPingRepOk,
    ClientType,
    CoreEvent,
    EventsListenRepOkRealmRolesUpdated,
    EventsListenRepOkRealmVlobsUpdated,
    OrganizationConfig,
    RealmID,
)
from parsec.api.data import EntryName
from parsec.api.protocol import RealmRole, VlobID
from parsec.backend.backend_events import BackendEvent
from parsec.core.backend_connection import (
    BackendAuthenticatedConn,
    BackendConnectionRefused,
    BackendConnStatus,
    BackendNotAvailable,
)
from parsec.core.fs.userfs.userfs import UserFS
from parsec.event_bus import EventBus
from tests.common import real_clock_timeout


@pytest.fixture
async def alice_backend_conn(alice, event_bus_factory, running_backend_ready):
    await running_backend_ready()
    event_bus = event_bus_factory()
    conn = BackendAuthenticatedConn(alice, event_bus)
    with event_bus.listen() as spy:
        async with conn.run():
            await spy.wait_with_timeout(
                CoreEvent.BACKEND_CONNECTION_CHANGED,
                {"status": BackendConnStatus.READY, "status_exc": None},
            )
            yield conn


@pytest.mark.trio
@pytest.mark.parametrize("apiv22_organization_cmd_supported", (True, False))
async def test_init_with_backend_online(
    running_backend, event_bus, alice, apiv22_organization_cmd_supported
):
    monitor_initialized = False
    monitor_teardown = False

    # Mock organization config command
    async def _mocked_organization_config(client_ctx, msg):
        if apiv22_organization_cmd_supported:
            return {
                "user_profile_outsider_allowed": True,
                "active_users_limit": None,
                "status": "ok",
            }
        else:
            # Backend with API support <2.2, the client should be able to fallback
            return {"status": "unknown_command"}

    _mocked_organization_config._api_info = running_backend.backend.apis[ClientType.AUTHENTICATED][
        "organization_config"
    ]._api_info

    running_backend.backend.apis[ClientType.AUTHENTICATED][
        "organization_config"
    ] = _mocked_organization_config

    async def _monitor(
        task_status=trio.TASK_STATUS_IGNORED,
        user_fs: Optional[UserFS] = None,
        event_bus: Optional[EventBus] = None,
    ):
        nonlocal monitor_initialized, monitor_teardown
        monitor_initialized = True
        try:
            task_status.started()
            await trio.sleep_forever()
        finally:
            monitor_teardown = True

    conn = BackendAuthenticatedConn(alice, event_bus)
    assert conn.status == BackendConnStatus.LOST
    default_organization_config = OrganizationConfig(
        user_profile_outsider_allowed=False,
        active_users_limit=None,
        sequester_authority=None,
        sequester_services=None,
    )
    assert conn.get_organization_config() == default_organization_config

    conn.register_monitor(_monitor)
    with event_bus.listen() as spy:
        async with conn.run():
            await spy.wait_multiple_with_timeout(
                [
                    (
                        CoreEvent.BACKEND_CONNECTION_CHANGED,
                        {"status": BackendConnStatus.INITIALIZING, "status_exc": None},
                    ),
                    (
                        CoreEvent.BACKEND_CONNECTION_CHANGED,
                        {"status": BackendConnStatus.READY, "status_exc": None},
                    ),
                ]
            )
            assert conn.status == BackendConnStatus.READY
            assert monitor_initialized
            assert not monitor_teardown

            # Test organization config retrieval
            if apiv22_organization_cmd_supported:
                assert conn.get_organization_config() == OrganizationConfig(
                    user_profile_outsider_allowed=True,
                    active_users_limit=None,
                    sequester_authority=None,
                    sequester_services=None,
                )
            else:
                # Default value
                assert conn.get_organization_config() == default_organization_config

            # Test command
            rep = await conn.cmds.ping("foo")
            assert rep == AuthenticatedPingRepOk("foo")

            # Test events
            running_backend.backend.event_bus.send(
                BackendEvent.PINGED,
                organization_id=alice.organization_id,
                author="bob@test",
                ping="foo",
            )
            await spy.wait_with_timeout(CoreEvent.BACKEND_PINGED, {"ping": "foo"})

        assert monitor_teardown


@pytest.mark.trio
async def test_init_with_backend_offline(event_bus, alice):
    conn = BackendAuthenticatedConn(alice, event_bus)
    assert conn.status == BackendConnStatus.LOST
    default_organization_config = OrganizationConfig(
        user_profile_outsider_allowed=False,
        active_users_limit=None,
        sequester_authority=None,
        sequester_services=None,
    )
    assert conn.get_organization_config() == default_organization_config

    with event_bus.listen() as spy:
        async with conn.run():
            await spy.wait_with_timeout(
                CoreEvent.BACKEND_CONNECTION_CHANGED,
                {"status": BackendConnStatus.LOST, "status_exc": spy.ANY},
            )
            assert conn.status == BackendConnStatus.LOST
            assert conn.get_organization_config() == default_organization_config

            # Test command not possible
            with pytest.raises(BackendNotAvailable):
                await conn.cmds.ping("foo")


@pytest.mark.trio
@pytest.mark.parametrize("during_bootstrap", (True, False))
async def test_monitor_crash(caplog, running_backend, event_bus, alice, during_bootstrap):
    async def _bad_monitor(*, task_status=trio.TASK_STATUS_IGNORED):
        if during_bootstrap:
            raise RuntimeError("D'oh !")
        task_status.started()
        await trio.sleep(0)
        raise RuntimeError("D'oh !")

    conn = BackendAuthenticatedConn(alice, event_bus)
    with event_bus.listen() as spy:
        conn.register_monitor(_bad_monitor)
        async with conn.run():
            await spy.wait_with_timeout(
                CoreEvent.BACKEND_CONNECTION_CHANGED,
                {"status": BackendConnStatus.CRASHED, "status_exc": spy.ANY},
            )
            assert conn.status == BackendConnStatus.CRASHED
            caplog.assert_occurred_once(
                "[error    ] Unhandled exception            [parsec.core.backend_connection.authenticated]"
            )

            # Test command not possible
            with pytest.raises(BackendNotAvailable) as exc:
                await conn.cmds.ping()
            assert str(exc.value) == "Backend connection manager has crashed: D'oh !"


@pytest.mark.trio
async def test_switch_offline(frozen_clock, running_backend, event_bus, alice):
    conn = BackendAuthenticatedConn(alice, event_bus)
    with event_bus.listen() as spy:
        async with conn.run():

            # Wait for the connection to be initialized
            await spy.wait_with_timeout(
                CoreEvent.BACKEND_CONNECTION_CHANGED,
                {"status": BackendConnStatus.READY, "status_exc": None},
            )

            # Switch backend offline and wait for according event
            spy.clear()
            with running_backend.offline():
                await spy.wait_with_timeout(
                    CoreEvent.BACKEND_CONNECTION_CHANGED,
                    {"status": BackendConnStatus.LOST, "status_exc": spy.ANY},
                )
                assert conn.status == BackendConnStatus.LOST

            # Here backend switch back online, wait for the corresponding event
            spy.clear()

            # Backend event manager waits before retrying to connect
            await frozen_clock.sleep_with_autojump(5)
            await spy.wait_with_timeout(
                CoreEvent.BACKEND_CONNECTION_CHANGED,
                {"status": BackendConnStatus.READY, "status_exc": None},
            )
            assert conn.status == BackendConnStatus.READY

            # Make sure event system still works as expected
            spy.clear()
            running_backend.backend.event_bus.send(
                BackendEvent.PINGED,
                organization_id=alice.organization_id,
                author="bob@test",
                ping="foo",
            )
            await spy.wait_with_timeout(CoreEvent.BACKEND_PINGED, {"ping": "foo"})


@pytest.mark.trio
async def test_concurrency_sends(running_backend, alice, event_bus):
    CONCURRENCY = 10
    work_done_counter = 0
    work_all_done = trio.Event()

    async def sender(cmds, x):
        nonlocal work_done_counter
        rep = await cmds.ping(x)
        assert rep == AuthenticatedPingRepOk(str(x))
        work_done_counter += 1
        if work_done_counter == CONCURRENCY:
            work_all_done.set()

    conn = BackendAuthenticatedConn(
        alice,
        event_bus,
        max_pool=CONCURRENCY // 2,
    )
    async with conn.run():

        async with trio.open_service_nursery() as nursery:
            for x in range(CONCURRENCY):
                nursery.start_soon(sender, conn.cmds, str(x))

        async with real_clock_timeout():
            await work_all_done.wait()


@pytest.mark.trio
async def test_realm_notif_on_new_entry_sync(running_backend, alice_backend_conn, alice2_user_fs):
    wid = await alice2_user_fs.workspace_create(EntryName("foo"))
    workspace = alice2_user_fs.get_workspace(wid)

    await workspace.touch("/foo")
    entry_id = await workspace.path_id("/foo")

    with alice_backend_conn.event_bus.listen() as spy:
        await workspace.sync()
        await spy.wait_multiple_with_timeout(
            [
                # File manifest creation
                (
                    CoreEvent.BACKEND_REALM_VLOBS_UPDATED,
                    {
                        "realm_id": wid,
                        "checkpoint": 1,
                        "src_id": entry_id,
                        "src_version": 1,
                    },
                ),
                # Workspace manifest creation containing the file entry
                (
                    CoreEvent.BACKEND_REALM_VLOBS_UPDATED,
                    {
                        "realm_id": wid,
                        "checkpoint": 2,
                        "src_id": wid,
                        "src_version": 1,
                    },
                ),
            ]
        )


@pytest.mark.trio
async def test_realm_notif_on_new_workspace_sync(
    running_backend, alice_backend_conn, alice2_user_fs
):
    uid = alice2_user_fs.user_manifest_id
    wid = await alice2_user_fs.workspace_create(EntryName("foo"))

    with alice_backend_conn.event_bus.listen() as spy:
        await alice2_user_fs.sync()
        await spy.wait_multiple_with_timeout(
            [
                # Access to newly created realm
                EventsListenRepOkRealmRolesUpdated(RealmID.from_bytes(wid.bytes), RealmRole.OWNER),
                EventsListenRepOkRealmVlobsUpdated(
                    RealmID.from_bytes(wid.bytes), 1, VlobID.from_bytes(wid.bytes), 1
                ),
                EventsListenRepOkRealmVlobsUpdated(
                    RealmID.from_bytes(uid.bytes), 2, VlobID.from_bytes(uid.bytes), 2
                ),
            ]
        )


@pytest.mark.trio
async def test_realm_notif_maintenance(running_backend, alice_backend_conn, alice2_user_fs):
    wid = await alice2_user_fs.workspace_create(EntryName("foo"))
    await alice2_user_fs.sync()

    with alice_backend_conn.event_bus.listen() as spy:
        # Start maintenance
        job = await alice2_user_fs.workspace_start_reencryption(wid)

        await spy.wait_multiple_with_timeout(
            [
                (
                    CoreEvent.BACKEND_REALM_MAINTENANCE_STARTED,
                    {"realm_id": wid, "encryption_revision": 2},
                ),
                # Receive the message containing the new key and encryption revision
                CoreEvent.BACKEND_MESSAGE_RECEIVED,
            ]
        )

    with alice_backend_conn.event_bus.listen() as spy:
        # Finish maintenance
        total, done = await job.do_one_batch()
        assert total == done

        await spy.wait_with_timeout(
            CoreEvent.BACKEND_REALM_MAINTENANCE_FINISHED,
            {"realm_id": wid, "encryption_revision": 2},
        )


@pytest.mark.trio
async def test_connection_refused(running_backend, event_bus, mallory):
    conn = BackendAuthenticatedConn(mallory, event_bus)
    with event_bus.listen() as spy:
        async with conn.run():

            # Wait for the connection to be initialized
            await spy.wait_with_timeout(
                CoreEvent.BACKEND_CONNECTION_CHANGED,
                {"status": BackendConnStatus.REFUSED, "status_exc": spy.ANY},
            )

            # Trying to use the connection should end up with an exception
            with pytest.raises(BackendConnectionRefused):
                await conn.cmds.ping()
