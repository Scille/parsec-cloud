import pytest
import trio
from uuid import uuid4
from trio.testing import wait_all_tasks_blocked

from parsec.core.backend_events_manager import BackendEventsManager

from tests.open_tcp_stream_mock_wrapper import offline


@pytest.fixture
async def backend_event_manager(event_bus, running_backend, alice, cert_path=None, ca_path=None):
    async with trio.open_nursery() as nursery:
        em = BackendEventsManager(alice, running_backend.addr, event_bus, cert_path, ca_path)
        await em.init(nursery)
        try:
            yield em

        finally:
            await em.teardown()
            # Don't cancel the nursery given teardown should have clean
            # the coroutines


def default_events_plus(*events):
    return {("device.try_claim_submitted",), ("message.received",), ("pinged", None)} | set(events)


@pytest.mark.trio
async def test_init_end_with_backend_online_status_event(
    event_bus, running_backend, alice, cert_path=None, ca_path=None
):
    async with trio.open_nursery() as nursery:
        em = BackendEventsManager(alice, running_backend.addr, event_bus, cert_path, ca_path)

        with event_bus.listen() as spy:
            await em.init(nursery)

        spy.assert_event_occured("backend.online")

        await em.teardown()


@pytest.mark.trio
async def test_init_end_with_backend_offline_status_event(
    unused_tcp_addr, event_bus, alice, cert_path=None, ca_path=None
):
    async with trio.open_nursery() as nursery:
        em = BackendEventsManager(alice, unused_tcp_addr, event_bus, cert_path, ca_path)

        with event_bus.listen() as spy:
            await em.init(nursery)

        spy.assert_event_occured("backend.offline")

        await em.teardown()


@pytest.mark.trio
async def test_listen_beacon_on_init(
    event_bus, running_backend, alice, cert_path=None, ca_path=None
):
    beacon_id = uuid4()
    async with trio.open_nursery() as nursery:
        em = BackendEventsManager(alice, running_backend.addr, event_bus, cert_path, ca_path)

        event_bus.send("backend.beacon.listen", beacon_id=beacon_id)

        await em.init(nursery)

        await event_bus.spy.wait(
            "backend.listener.restarted",
            kwargs={"events": default_events_plus(("beacon.updated", beacon_id))},
        )

        await em.teardown()


@pytest.mark.trio
async def test_listen_beacon_from_backend(event_bus, running_backend, alice, backend_event_manager):
    src_id = uuid4()
    beacon_id = uuid4()
    event_bus.send("backend.beacon.listen", beacon_id=beacon_id)

    with trio.fail_after(1.0):
        await event_bus.spy.wait(
            "backend.listener.restarted",
            kwargs={"events": default_events_plus(("beacon.updated", beacon_id))},
        )

    running_backend.backend.event_bus.send(
        "beacon.updated",
        author="bob@test",
        beacon_id=beacon_id,
        index=1,
        src_id=src_id,
        src_version=42,
    )

    with trio.fail_after(1.0):
        await event_bus.spy.wait(
            "backend.beacon.updated",
            kwargs={"beacon_id": beacon_id, "index": 1, "src_id": src_id, "src_version": 42},
        )


@pytest.mark.trio
async def test_unlisten_beacon(event_bus, running_backend, alice, backend_event_manager):
    src_id = uuid4()
    beacon_id = uuid4()
    event_bus.send("backend.beacon.listen", beacon_id=beacon_id)

    with trio.fail_after(1.0):
        await event_bus.spy.wait(
            "backend.listener.restarted",
            kwargs={"events": default_events_plus(("beacon.updated", beacon_id))},
        )

    # Don't use default event spy not to mix with the very first
    # `backend.listener.restarted` event
    with event_bus.listen() as spy:
        event_bus.send("backend.beacon.unlisten", beacon_id=beacon_id)

        with trio.fail_after(1.0):
            await spy.wait("backend.listener.restarted", kwargs={"events": default_events_plus()})

    # Finally make sure this event is no longer present

    with event_bus.listen() as spy:
        running_backend.backend.event_bus.send(
            "beacon.updated",
            author="bob@test",
            beacon_id=beacon_id,
            index=1,
            src_id=src_id,
            src_version=42,
        )
        await wait_all_tasks_blocked(cushion=0.01)

    assert not spy.events


@pytest.mark.trio
async def test_unlisten_unknown_beacon_id_does_nothing(event_bus, alice, backend_event_manager):
    beacon_id = uuid4()
    with event_bus.listen() as spy:
        event_bus.send("backend.beacon.unlisten", beacon_id=beacon_id)
        await wait_all_tasks_blocked(cushion=0.01)

    assert not spy.assert_events_exactly_occured(
        [("backend.beacon.unlisten", {"beacon_id": beacon_id})]
    )


@pytest.mark.trio
async def test_listen_already_listened_beacon_id_does_nothing(
    event_bus, alice, backend_event_manager
):
    beacon_id = uuid4()
    with event_bus.listen() as spy:
        event_bus.send("backend.beacon.listen", beacon_id=beacon_id)
        await spy.wait("backend.listener.restarted")

    # Second subscribe is useless, event listener shouldn't be restarted

    with event_bus.listen() as spy:
        event_bus.send("backend.beacon.listen", beacon_id=beacon_id)
        await wait_all_tasks_blocked(cushion=0.01)

    assert not spy.assert_events_exactly_occured(
        [("backend.beacon.listen", {"beacon_id": beacon_id})]
    )


@pytest.mark.trio
async def test_backend_switch_offline(
    mock_clock, event_bus, running_backend, alice, backend_event_manager
):
    beacon_id = uuid4()
    src_id = uuid4()
    mock_clock.rate = 1.0

    with event_bus.listen() as spy:
        event_bus.send("backend.beacon.listen", beacon_id=beacon_id)
        await spy.wait("backend.listener.restarted")

    # Switch backend offline and wait for according event

    with event_bus.listen() as spy:
        with offline(running_backend.addr):
            with trio.fail_after(1.0):
                await spy.wait("backend.offline")

        # Here we're backend online, wait for the event

        # Backend event manager waits before retrying to connect
        mock_clock.jump(5.0)

        with trio.fail_after(1.0):
            await spy.wait("backend.online")
            await spy.wait("backend.listener.restarted")

    # Make sure event system still works as expected

    with event_bus.listen() as spy:
        running_backend.backend.event_bus.send(
            "beacon.updated",
            author="bob@test",
            beacon_id=beacon_id,
            index=1,
            src_id=src_id,
            src_version=42,
        )

        with trio.fail_after(1.0):
            await spy.wait(
                "backend.beacon.updated",
                kwargs={"beacon_id": beacon_id, "index": 1, "src_id": src_id, "src_version": 42},
            )
