import pytest
import trio

from parsec.core.backend_events_manager import BackendEventsManager


@pytest.fixture
async def backend_event_manager(nursery, running_backend, signal_ns, alice):
    em = BackendEventsManager(alice, running_backend.addr, signal_ns)
    await em.init(nursery)
    try:
        yield em

    finally:
        await em.teardown()


def connect_as_event(signal_ns, signal_name):
    event = trio.Event()

    def set_event(*args):
        event.set()

    event._cb = set_event  # Prevent wearef destruction
    signal_ns.signal(signal_name).connect(set_event)
    return event


@pytest.mark.trio
async def test_subscribe_backend_event(running_backend, signal_ns, backend_event_manager):
    backend_ready = connect_as_event(signal_ns, "backend_event_manager_listener_started")
    # Dummy event (not provided by backend)
    await backend_event_manager.subscribe_backend_event("ping")
    await backend_ready.wait()

    ping_received = connect_as_event(signal_ns, "ping")

    with trio.fail_after(1.0):
        running_backend.backend.signal_ns.signal("ping").send()
        await ping_received.wait()


@pytest.mark.trio
async def test_unsubscribe_unknown_backend_event(running_backend, backend_event_manager):
    with pytest.raises(KeyError):
        await backend_event_manager.unsubscribe_backend_event("dummy")


@pytest.mark.trio
async def test_subscribe_already_subscribed_backend_event(running_backend, backend_event_manager):
    await backend_event_manager.subscribe_backend_event("ping")
    with pytest.raises(KeyError):
        await backend_event_manager.subscribe_backend_event("ping")


@pytest.mark.trio
async def test_unsbuscribe_backend_event(running_backend, signal_ns, backend_event_manager):
    backend_ready = connect_as_event(signal_ns, "backend_event_manager_listener_started")
    await backend_event_manager.subscribe_backend_event("ping")
    await backend_ready.wait()

    backend_ready.clear()
    await backend_event_manager.unsubscribe_backend_event("ping")
    await backend_ready.wait()

    def on_ping(*args):
        raise RuntimeError("Expected not to receive this event !")

    signal_ns.signal("ping").connect(on_ping)

    running_backend.backend.signal_ns.signal("ping").send()
    # Nothing occured ? Then we're good !
    await trio.sleep(0.01)
