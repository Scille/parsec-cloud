# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import trio
import pytest
from wsproto.events import BytesMessage, Ping, Pong, CloseConnection, AcceptConnection, Request

from parsec.api.transport import Transport
from parsec.api.protocol import events_listen_serializer, ping_serializer


@pytest.mark.trio
async def test_events_listen_wait_has_watchdog(
    monkeypatch, frozen_clock, running_backend, backend_sock_factory, alice
):
    KEEPALIVE_TIME = 30  # Autojump clock, so we won't wait for that long
    # Spy on the transport events to detect the wsproto events
    transport_events_sender, transport_events_receiver = trio.open_memory_channel(100)
    _vanilla_next_ws_event = Transport._next_ws_event

    async def _mocked_next_ws_event(transport):
        event = await _vanilla_next_ws_event(transport)
        await transport_events_sender.send((transport, event))
        return event

    monkeypatch.setattr(Transport, "_next_ws_event", _mocked_next_ws_event)

    async def next_ws_proto_related_event(expected_event_type=None, expected_transport=None):
        async with frozen_clock.real_clock_timeout():
            transport, event = await transport_events_receiver.receive()
        if expected_event_type is not None:
            assert isinstance(event, expected_event_type)
        if expected_transport is not None:
            assert transport is expected_transport
        return (transport, event)

    async def _events_listen_cmd(client_transport):
        req = {"cmd": "events_listen", "wait": True}
        raw_req = events_listen_serializer.req_dumps(req)
        await client_transport.send(raw_req)
        # The backend won't have any event for us, so this is going to block
        # and send Ping events from time to time
        await client_transport.recv()

    async with backend_sock_factory(
        running_backend.backend, alice, keepalive=KEEPALIVE_TIME, freeze_on_transport_error=False
    ) as client_transport:

        # The backend should upgrade the websocket connection
        backend_transport, event = await next_ws_proto_related_event(expected_event_type=Request)
        assert backend_transport is not client_transport

        # The client accept the new websocket connection
        _, event = await next_ws_proto_related_event(
            expected_event_type=AcceptConnection, expected_transport=client_transport
        )

        # Now is the time for the Parsec-level hanshake
        _, event = await next_ws_proto_related_event(
            expected_event_type=BytesMessage, expected_transport=client_transport
        )
        assert b"handshake" in event.data  # Quick sanity check
        _, event = await next_ws_proto_related_event(
            expected_event_type=BytesMessage, expected_transport=backend_transport
        )
        assert b"handshake" in event.data  # Quick sanity check
        _, event = await next_ws_proto_related_event(
            expected_event_type=BytesMessage, expected_transport=client_transport
        )
        assert b"handshake" in event.data  # Quick sanity check

        # Keepalive is handled by `Transport.recv`, hence we must have a concurrent
        # coroutine waiting on the `events_listen` response
        async with trio.open_nursery() as nursery:

            # Connection is ready now, client sends the `events_listen` command
            nursery.start_soon(_events_listen_cmd, client_transport)
            _, event = await next_ws_proto_related_event(
                expected_event_type=BytesMessage, expected_transport=backend_transport
            )
            assert b"events_listen" in event.data  # Quick sanity check

            # Now advance time until ping websocket is requested
            frozen_clock.jump(KEEPALIVE_TIME)
            await next_ws_proto_related_event(
                expected_event_type=Ping, expected_transport=backend_transport
            )
            await next_ws_proto_related_event(
                expected_event_type=Pong, expected_transport=client_transport
            )
            # Wait for another ping websocket, just to be sure...
            frozen_clock.jump(KEEPALIVE_TIME)
            await next_ws_proto_related_event(
                expected_event_type=Ping, expected_transport=backend_transport
            )
            await next_ws_proto_related_event(
                expected_event_type=Pong, expected_transport=client_transport
            )

            # Stop the wait on `events_listen` answer
            nursery.cancel_scope.cancel()

        # Cancel `events_listen` by sending another command
        # Note: The Parsec ping command is not related to Ping/Pong events in websocket
        raw_req = ping_serializer.req_dumps({"cmd": "ping", "ping": "foo"})
        await client_transport.send(raw_req)
        _, event = await next_ws_proto_related_event(
            expected_event_type=BytesMessage, expected_transport=backend_transport
        )
        assert b"ping" in event.data  # Quick sanity check
        async with trio.open_nursery() as nursery:
            nursery.start_soon(client_transport.recv)
            _, event = await next_ws_proto_related_event(
                expected_event_type=BytesMessage, expected_transport=client_transport
            )
            assert ping_serializer.rep_loads(event.data) == {"status": "ok", "pong": "foo"}

        # Test the CloseConnection event
        await client_transport.aclose()
        await next_ws_proto_related_event(
            expected_event_type=CloseConnection, expected_transport=backend_transport
        )
