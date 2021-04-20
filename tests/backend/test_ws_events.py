# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio
import pytest

from wsproto.events import Ping, Pong, CloseConnection, AcceptConnection, Request
from parsec.api.transport import Transport
from parsec.api.protocol import events_listen_serializer, ping_serializer


@pytest.mark.trio
async def test_events_listen_wait_has_watchdog(
    monkeypatch, mock_clock, running_backend, backend_sock_factory, alice
):
    KEEPALIVE_TIME = 2
    # Spy on the transport events to detect the wsproto events
    transport_events_sender, transport_events_receiver = trio.open_memory_channel(100)
    _vanilla_next_ws_event = Transport._next_ws_event

    async def _mocked_next_ws_event(transport):
        event = await _vanilla_next_ws_event(transport)
        await transport_events_sender.send((transport, event))
        return event

    monkeypatch.setattr(Transport, "_next_ws_event", _mocked_next_ws_event)

    async def next_ws_proto_related_event():
        while True:
            transport, event = await transport_events_receiver.receive()
            if isinstance(event, (Ping, Pong, AcceptConnection, Request, CloseConnection)):
                return (transport, event)

    events_listen_rep = None

    async def _cmd():
        nonlocal events_listen_rep
        req = {"cmd": "events_listen", "wait": True}
        raw_req = events_listen_serializer.req_dumps(req)
        await transport.send(raw_req)
        # The events_listen response will be triggered by the next command on the stream
        # so we will be blocked here until the next command will be sent.
        # We may use the same transport for the next request because the websocket
        # protocol allow the duplex operations.
        events_listen_rep = await transport.recv()

    async with backend_sock_factory(
        running_backend.backend, alice, keepalive=KEEPALIVE_TIME
    ) as transport:
        mock_clock.rate = 1

        await trio.testing.wait_all_tasks_blocked()
        # The backend should upgrade the websocket connection
        backend_transport, event = await next_ws_proto_related_event()
        assert isinstance(event, Request)

        # The client accept the new websocket connection
        client_transport, event = await next_ws_proto_related_event()
        assert isinstance(event, AcceptConnection)

        async with trio.open_service_nursery() as nursery:

            nursery.start_soon(_cmd)
            # Now advance time until ping websocket is requested
            await trio.testing.wait_all_tasks_blocked()
            mock_clock.jump(KEEPALIVE_TIME)
            with trio.fail_after(1):
                backend_transport1, event = await next_ws_proto_related_event()
                assert isinstance(event, Ping)
                client_transport1, event = await next_ws_proto_related_event()
                assert isinstance(event, Pong)
                assert client_transport1 is client_transport
                assert client_transport1 is not backend_transport
                assert backend_transport is backend_transport1

            # Wait for another ping websocket, just to be sure...
            await trio.testing.wait_all_tasks_blocked()
            mock_clock.jump(KEEPALIVE_TIME)
            with trio.fail_after(1):
                backend_transport2, event = await next_ws_proto_related_event()
                assert isinstance(event, Ping)
                assert backend_transport1 is backend_transport2
                client_transport2, event = await next_ws_proto_related_event()
                assert isinstance(event, Pong)
                assert client_transport1 is client_transport2

            # Send another request in the stream to be sure that will be managed by the backend
            raw_req = ping_serializer.req_dumps({"cmd": "ping", "ping": "foo"})
            await transport.send(raw_req)

        # The ping request should trigger the event_listen's receive method
        assert ping_serializer.rep_loads(events_listen_rep) == {"status": "ok", "pong": "foo"}

        # Test the CloseConnection event
        await transport.transport.aclose()
        transport, event = await next_ws_proto_related_event()
        assert transport == backend_transport
        assert isinstance(event, CloseConnection)
