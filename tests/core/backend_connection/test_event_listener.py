import pytest
import trio

from parsec.core.backend_connection2 import backend_listen_events


@pytest.mark.trio
async def test_init_end_with_backend_online_status_event(running_backend, event_bus, alice):
    async with trio.open_nursery() as nursery:

        with event_bus.listen() as spy:
            await nursery.start(backend_listen_events, running_backend.addr, alice, event_bus)

        spy.assert_event_occured("backend.online")

        nursery.cancel_scope.cancel()


@pytest.mark.trio
async def test_init_end_with_backend_offline_status_event(unused_tcp_addr, event_bus, alice):
    async with trio.open_nursery() as nursery:

        with event_bus.listen() as spy:
            await nursery.start(backend_listen_events, unused_tcp_addr, alice, event_bus)

        spy.assert_event_occured("backend.offline")

        nursery.cancel_scope.cancel()
