# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
import trio
from trio.testing import wait_all_tasks_blocked
from uuid import uuid4

from parsec.core.backend_connection import backend_listen_events

from tests.open_tcp_stream_mock_wrapper import offline


@pytest.fixture
async def running_backend_listen_events(running_backend, event_bus, alice):
    async with trio.open_nursery() as nursery:
        await nursery.start(backend_listen_events, alice, event_bus)
        yield
        nursery.cancel_scope.cancel()


@pytest.mark.trio
async def test_init_end_with_backend_online_status_event(running_backend, event_bus, alice):
    async with trio.open_nursery() as nursery:

        with event_bus.listen() as spy:
            await nursery.start(backend_listen_events, alice, event_bus)

        spy.assert_event_occured("backend.online")

        nursery.cancel_scope.cancel()


@pytest.mark.trio
async def test_init_end_with_backend_offline_status_event(event_bus, alice):
    async with trio.open_nursery() as nursery:

        with event_bus.listen() as spy:
            await nursery.start(backend_listen_events, alice, event_bus)

        spy.assert_event_occured("backend.offline")

        nursery.cancel_scope.cancel()


@pytest.mark.trio
async def test_subscribe_listen_unsubscribe_realm(
    event_bus, backend, running_backend_listen_events, alice
):
    realm_id = uuid4()
    src_id = uuid4()

    # Subscribe event

    with event_bus.listen() as spy:
        event_bus.send("backend.realm.listen", realm_id=realm_id)
        with trio.fail_after(1.0):
            await spy.wait("backend.listener.restarted")

    # Send&listen event

    backend.event_bus.send(
        "realm.vlobs_updated",
        organization_id=alice.organization_id,
        author="bob@test",
        realm_id=realm_id,
        checkpoint=1,
        src_id=src_id,
        src_version=42,
    )

    with trio.fail_after(1.0):
        await event_bus.spy.wait(
            "backend.realm.vlobs_updated",
            kwargs={"realm_id": realm_id, "checkpoint": 1, "src_id": src_id, "src_version": 42},
        )

    # Unsubscribe event

    with event_bus.listen() as spy:
        event_bus.send("backend.realm.unlisten", realm_id=realm_id)
        with trio.fail_after(1.0):
            await spy.wait("backend.listener.restarted")

    # Finally make sure this event is no longer present

    with event_bus.listen() as spy:
        backend.event_bus.send(
            "realm.vlobs_updated",
            organization_id=alice.organization_id,
            author="bob@test",
            realm_id=realm_id,
            checkpoint=1,
            src_id=src_id,
            src_version=42,
        )
        await wait_all_tasks_blocked(cushion=0.01)
        assert not spy.events


@pytest.mark.trio
async def test_unlisten_unknown_realm_does_nothing(event_bus, running_backend_listen_events):
    realm_id = uuid4()

    with event_bus.listen() as spy:
        event_bus.send("backend.realm.unlisten", realm_id=realm_id)
        await wait_all_tasks_blocked(cushion=0.01)

    assert not spy.assert_events_exactly_occured(
        [("backend.realm.unlisten", {"realm_id": realm_id})]
    )


@pytest.mark.trio
async def test_listen_already_listened_realm_does_nothing(event_bus, running_backend_listen_events):
    realm_id = uuid4()

    with event_bus.listen() as spy:
        event_bus.send("backend.realm.listen", realm_id=realm_id)
        await spy.wait("backend.listener.restarted")

    # Second subscribe is useless, event listener shouldn't be restarted
    with event_bus.listen() as spy:
        event_bus.send("backend.realm.listen", realm_id=realm_id)
        await wait_all_tasks_blocked(cushion=0.01)

    assert not spy.assert_events_exactly_occured([("backend.realm.listen", {"realm_id": realm_id})])


@pytest.mark.trio
async def test_backend_switch_offline(
    mock_clock, event_bus, backend_addr, backend, running_backend_listen_events, alice
):
    realm_id = uuid4()
    src_id = uuid4()
    mock_clock.rate = 1.0

    with event_bus.listen() as spy:
        event_bus.send("backend.realm.listen", realm_id=realm_id)
        await spy.wait("backend.listener.restarted")

    # Switch backend offline and wait for according event

    with event_bus.listen() as spy:
        with offline(backend_addr):
            with trio.fail_after(1.0):
                await spy.wait("backend.offline")

        # Here backend switch back online, wait for the corresponding event

        # Backend event manager waits before retrying to connect
        mock_clock.jump(5.0)

        with trio.fail_after(1.0):
            await spy.wait("backend.online")
            await spy.wait("backend.listener.restarted")

    # Make sure event system still works as expected

    with event_bus.listen() as spy:
        backend.event_bus.send(
            "realm.vlobs_updated",
            organization_id=alice.organization_id,
            author="bob@test",
            realm_id=realm_id,
            checkpoint=1,
            src_id=src_id,
            src_version=42,
        )

        with trio.fail_after(1.0):
            await spy.wait(
                "backend.realm.vlobs_updated",
                kwargs={"realm_id": realm_id, "checkpoint": 1, "src_id": src_id, "src_version": 42},
            )
