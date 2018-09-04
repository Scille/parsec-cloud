import pytest
import trio
from trio.testing import wait_all_tasks_blocked
from async_generator import asynccontextmanager

from parsec.core.backend_events_manager import BackendEventsManager

from tests.common import connect_signal_as_event
from tests.open_tcp_stream_mock_wrapper import offline


@pytest.fixture
async def backend_event_manager(signal_ns, running_backend, alice):
    async with trio.open_nursery() as nursery:
        em = BackendEventsManager(alice, running_backend.addr, signal_ns)
        await em.init(nursery)
        try:
            yield em

        finally:
            await em.teardown()
            nursery.cancel_scope.cancel()


@asynccontextmanager
async def wait_event_subscribed(signal_ns, events):
    default_events = {("message.received",), ("device.try_claim_submitted",), ("pinged", None)}
    expected_events = default_events | set(events)

    event_subscribed = connect_signal_as_event(signal_ns, "backend.listener.restarted")

    yield

    with trio.fail_after(1.0):
        await event_subscribed.wait()
    event_subscribed.cb.assert_called_with(None, events=expected_events)


@asynccontextmanager
async def ensure_signal_not_sent(signal_ns, signal):
    def on_event(*args, **kwargs):
        raise RuntimeError("Expected listener not to be restarted !")

    with signal_ns.signal(signal).connected_to(on_event):
        yield
        await wait_all_tasks_blocked(cushion=0.01)


@pytest.mark.trio
async def test_listen_beacon_on_init(signal_ns, running_backend, alice):
    async with trio.open_nursery() as nursery:
        em = BackendEventsManager(alice, running_backend.addr, signal_ns)

        signal_ns.signal("backend.beacon.listen").send(None, beacon_id="123")

        async with wait_event_subscribed(signal_ns, {("beacon.updated", "123")}):
            await em.init(nursery)

        nursery.cancel_scope.cancel()


@pytest.mark.trio
async def test_listen_beacon(signal_ns, running_backend, alice, backend_event_manager):
    async with wait_event_subscribed(signal_ns, {("beacon.updated", "123")}):
        signal_ns.signal("backend.beacon.listen").send(None, beacon_id="123")

    event_received = connect_signal_as_event(signal_ns, "backend.beacon.updated")

    running_backend.backend.signal_ns.signal("beacon.updated").send(
        None, author="bob@test", beacon_id="123", index=1, src_id="abc", src_version=42
    )

    with trio.fail_after(1.0):
        await event_received.wait()
    event_received.cb.assert_called_with(
        None, beacon_id="123", index=1, src_id="abc", src_version=42
    )


@pytest.mark.trio
async def test_unlisten_beacon(signal_ns, running_backend, alice, backend_event_manager):

    async with wait_event_subscribed(signal_ns, {("beacon.updated", "123")}):
        signal_ns.signal("backend.beacon.listen").send(None, beacon_id="123")

    async with wait_event_subscribed(signal_ns, {}):
        signal_ns.signal("backend.beacon.unlisten").send(None, beacon_id="123")

    async with ensure_signal_not_sent(signal_ns, "backend.beacon.updated"):
        running_backend.backend.signal_ns.signal("beacon.updated").send(
            None, author="bob@test", beacon_id="123", index=1, src_id="abc", src_version=42
        )


@pytest.mark.trio
async def test_unlisten_unknown_beacon_id_does_nothing(signal_ns, alice, backend_event_manager):
    async with ensure_signal_not_sent(signal_ns, "backend.listener.restarted"):
        signal_ns.signal("backend.beacon.unlisten").send(None, beacon_id="123")


@pytest.mark.trio
async def test_listen_already_listened_beacon_id_does_nothing(
    signal_ns, alice, backend_event_manager
):
    async with wait_event_subscribed(signal_ns, {("beacon.updated", "123")}):
        signal_ns.signal("backend.beacon.listen").send(None, beacon_id="123")

    # Second subscribe is useless, event listener shouldn't be restarted

    async with ensure_signal_not_sent(signal_ns, "backend.listener.restarted"):
        signal_ns.signal("backend.beacon.listen").send(None, beacon_id="123")


@pytest.mark.trio
async def test_backend_switch_offline(
    mock_clock, signal_ns, running_backend, alice, backend_event_manager
):
    mock_clock.rate = 1.0

    async with wait_event_subscribed(signal_ns, {("beacon.updated", "123")}):
        signal_ns.signal("backend.beacon.listen").send(None, beacon_id="123")

    backend_offline = connect_signal_as_event(signal_ns, "backend.offline")

    with offline(running_backend.addr):
        with trio.fail_after(1.0):
            await backend_offline.wait()
        backend_online = connect_signal_as_event(signal_ns, "backend.online")
        event_subscribed = connect_signal_as_event(signal_ns, "backend.listener.restarted")

    # Backend event manager waits before retrying to connect
    mock_clock.jump(5.0)

    with trio.fail_after(1.0):
        await backend_online.wait()
        await event_subscribed.wait()

    # Make sure event system still works as expected
    event_received = connect_signal_as_event(signal_ns, "backend.beacon.updated")

    running_backend.backend.signal_ns.signal("beacon.updated").send(
        None, author="bob@test", beacon_id="123", index=1, src_id="abc", src_version=42
    )

    with trio.fail_after(1.0):
        await event_received.wait()
    event_received.cb.assert_called_with(
        None, beacon_id="123", index=1, src_id="abc", src_version=42
    )
