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


@pytest.mark.trio
async def test_subscribe_backend_event(running_backend, signal_ns, backend_event_manager):
    # Dummy event (not provided by backend)
    await backend_event_manager.subscribe_backend_event("ping")

    ping_received = trio.Event()

    def on_ping(*args):
        ping_received.set()

    signal_ns.signal("ping").connect(on_ping)

    # It's possible the signal is sent before the connection with the backend
    # has been setup, in such case we wait a bit an retry.
    for tries in range(1, 4):
        with trio.move_on_after(0.01):
            running_backend.backend.signal_ns.signal("ping").send()
            await ping_received.wait()
            break

    else:
        raise RuntimeError("Not signal received after %s tentatives" % tries)


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
    await backend_event_manager.subscribe_backend_event("ping")
    await trio.sleep(0.01)
    await backend_event_manager.unsubscribe_backend_event("ping")
    await trio.sleep(0.01)

    def on_ping(*args):
        raise RuntimeError("Expected not to receive this event !")

    signal_ns.signal("ping").connect(on_ping)

    running_backend.backend.signal_ns.signal("ping").send()
    # Nothing occured ? Then we're good !
    await trio.sleep(0.01)
