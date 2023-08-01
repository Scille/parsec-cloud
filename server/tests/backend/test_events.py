# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import pytest
import trio

from parsec._parsec import BackendEventPinged
from parsec.api.protocol import (
    APIEventPinged,
    EventsListenRepOk,
)
from parsec.backend.asgi import app_factory
from tests.backend.common import (
    authenticated_ping,
    real_clock_timeout,
)
from tests.common import AuthenticatedRpcApiClient

# TODO: also test connection is cancelled when the organization gets expired


@pytest.mark.trio
async def test_sse_events_connection_closed_on_user_revoke(
    backend_asgi_app, bob_rpc: AuthenticatedRpcApiClient, bob, alice
):
    async def _do_revoke():
        await backend_asgi_app.backend.user.revoke_user(
            organization_id=bob.organization_id,
            user_id=bob.user_id,
            revoked_user_certificate=b"wathever",
            revoked_user_certifier=alice.device_id,
        )
        # connection cancellation is handled through events, so wait
        # for things to settle down to make sure there is no pending event
        await trio.testing.wait_all_tasks_blocked()

    async with bob_rpc.connect_sse_events() as sse_con:
        assert sse_con.status_code == 200

        await _do_revoke()
        async with real_clock_timeout():
            while True:
                try:
                    await sse_con.connection.receive()
                except trio.EndOfChannel:
                    # Connection is finally closed
                    break


@pytest.mark.trio
async def test_sse_events_subscribe(
    backend, alice_rpc: AuthenticatedRpcApiClient, alice2_rpc: AuthenticatedRpcApiClient
):
    async with real_clock_timeout():
        async with alice_rpc.connect_sse_events() as sse_con:
            assert sse_con.status_code == 200

            # Should ignore our own events
            with backend.event_bus.listen() as spy:
                await authenticated_ping(alice_rpc, "event1 (ignored)")
                await authenticated_ping(alice2_rpc, "event2")

                # No guarantees those events occur before the commands' return
                await spy.wait_multiple_with_timeout([BackendEventPinged, BackendEventPinged])

            last_event_id, event = await sse_con.get_next_event_and_id()
            assert event == EventsListenRepOk(APIEventPinged("event2"))

    # Also test last-event-id feature: we miss those events...
    await authenticated_ping(alice2_rpc, "event3")
    await authenticated_ping(alice_rpc, "event4 (ignored)")
    await authenticated_ping(alice2_rpc, "event5")
    async with real_clock_timeout():
        async with alice_rpc.connect_sse_events(last_event_id=last_event_id) as sse_con:
            async with alice_rpc.connect_sse_events(
                last_event_id="<unknown event id>"
            ) as sse_con_bad_last_event_id:
                assert sse_con.status_code == 200
                assert sse_con_bad_last_event_id.status_code == 200

                event = await sse_con.get_next_event()
                assert event == EventsListenRepOk(APIEventPinged("event3"))

                await authenticated_ping(alice2_rpc, "event6")

                event = await sse_con.get_next_event()
                assert event == EventsListenRepOk(APIEventPinged("event5"))

                event = await sse_con.get_next_event()
                assert event == EventsListenRepOk(APIEventPinged("event6"))

                # If an unknown event is provided, we get notified about it...
                with pytest.raises(RuntimeError) as exc:
                    await sse_con_bad_last_event_id.get_next_event()
                assert str(exc.value) == "missed events !"

                # ...and then can read the events since we arrived normally
                event = await sse_con_bad_last_event_id.get_next_event()
                assert event == EventsListenRepOk(APIEventPinged("event6"))


@pytest.mark.trio
async def test_sse_events_bad_auth(alice_rpc: AuthenticatedRpcApiClient):
    async with real_clock_timeout():

        def _before_send_hook(args):
            args["headers"]["Signature"] = "AAAA"

        async with alice_rpc.connect_sse_events(before_send_hook=_before_send_hook) as sse_con:
            response = await sse_con.connection.as_response()
            assert response.status_code == 401


@pytest.mark.trio
async def test_sse_events_bad_accept_type(alice_rpc: AuthenticatedRpcApiClient):
    async with real_clock_timeout():

        def _before_send_hook(args):
            args["headers"]["Accept"] = "application/json"

        async with alice_rpc.connect_sse_events(before_send_hook=_before_send_hook) as sse_con:
            response = await sse_con.connection.as_response()
            assert response.status_code == 406


@pytest.mark.trio
@pytest.mark.postgresql
async def test_sse_cross_backend_event(backend_factory, alice, bob):
    async with backend_factory() as backend_1, backend_factory(populated=False) as backend_2:
        app_1 = app_factory(backend_1)
        app_2 = app_factory(backend_2)

        bob_rpc = AuthenticatedRpcApiClient(app_1.test_client(), bob)
        alice_rpc = AuthenticatedRpcApiClient(app_2.test_client(), alice)

        async with real_clock_timeout():
            async with alice_rpc.connect_sse_events() as sse_con:
                await authenticated_ping(bob_rpc, ping="1")

                event = await sse_con.get_next_event()
                assert event == EventsListenRepOk(APIEventPinged("1"))
                assert sse_con.status_code == 200

                await authenticated_ping(bob_rpc, ping="2")
                await authenticated_ping(bob_rpc, ping="3")

                event = await sse_con.get_next_event()
                assert event == EventsListenRepOk(APIEventPinged("2"))
                event = await sse_con.get_next_event()
                assert event == EventsListenRepOk(APIEventPinged("3"))


@pytest.mark.trio
async def test_sse_events_close_connection_on_backpressure(
    monkeypatch, backend, alice_rpc: AuthenticatedRpcApiClient, alice, bob
):
    # The channel has a queue of size 1, meaning it will be filled after a single command
    monkeypatch.setattr("parsec.backend.client_context.AUTHENTICATED_CLIENT_CHANNEL_SIZE", 1)
    # `alice_rpc` fixture lazily intiate connection with the server, hence the
    # monkeypatch of the queue size will be taken into account when creating client context

    async with real_clock_timeout():
        async with alice_rpc.connect_sse_events() as sse_con:
            # In SSE, our code pops the events from the client context without waiting
            # peer acknowledgement. Hence the TCP layer must saturate first before
            # the client context has to actually piled up events in it queue.
            # So here we use directly the event bus to send events synchronously,
            # hence having the events pile up without a chance for the coroutine running
            # the SSE handler to pop them.
            # But there is a trick on top of that ! Trio event queue first looks for
            # tasks to wakeup before actually queuing the event. So under certain
            # concurrency 2 events is not enough (1st event triggers the wakeup for the
            # SSE task, 2nd event gets queued), hence we dispatch 3 events here !
            backend.event_bus.send(
                BackendEventPinged,
                event_id="1",
                payload=BackendEventPinged(
                    organization_id=alice.organization_id,
                    author=bob.device_id,
                    ping="1",
                ),
            )
            backend.event_bus.send(
                BackendEventPinged,
                event_id="2",
                payload=BackendEventPinged(
                    organization_id=alice.organization_id,
                    author=bob.device_id,
                    ping="2",
                ),
            )
            backend.event_bus.send(
                BackendEventPinged,
                event_id="3",
                payload=BackendEventPinged(
                    organization_id=alice.organization_id,
                    author=bob.device_id,
                    ping="3",
                ),
            )

            # The connection simply gets closed without error status given nothing wrong
            # occurred in practice
            response = await sse_con.connection.as_response()
            # Status code is 200 given it was provided with the very first event (and at
            # that time the server didn't know the client will become non-responsive !)
            assert response.status_code == 200
            # Note we don't check the response's body, this is because it is possible we
            # receive some events before the connection is actually closed, typically the
            # SSE code was waiting on the event memory channel so it receives one event,
            # then gets the Cancelled exception next await.


@pytest.mark.trio
async def test_sse_events_keepalive(frozen_clock, alice_rpc: AuthenticatedRpcApiClient):
    async with real_clock_timeout():
        async with alice_rpc.connect_sse_events() as sse_con:
            for _ in range(3):
                await frozen_clock.sleep_with_autojump(31)
                raw = await sse_con.connection.receive()
                assert raw == b":keepalive\n\n"
