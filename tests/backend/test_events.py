# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import pytest
import trio
from quart.testing.connections import WebsocketDisconnectError

from parsec._parsec import AuthenticatedPingRepOk, EventsListenRepNoEvents, EventsListenRepOkPinged
from parsec.backend.asgi import app_factory
from parsec.backend.backend_events import BackendEvent
from tests.backend.common import (
    authenticated_ping,
    events_listen,
    events_listen_nowait,
    events_listen_wait,
    events_subscribe,
    real_clock_timeout,
)
from tests.common import AuthenticatedRpcApiClient


@pytest.mark.trio
@pytest.mark.parametrize("revoked_during", ("idle", "listen_event"))
async def test_cancel_connection_after_events_subscribe(
    backend_asgi_app, backend_authenticated_ws_factory, bob, alice, revoked_during
):
    # The event system is also used to detect a connection should be dropped
    # given the corresponding device has been revoked.
    # Let's make sure `events_subscribe` command plays well with this behavior.

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

    async with backend_authenticated_ws_factory(backend_asgi_app, bob) as bob_ws:

        await events_subscribe(bob_ws)

        if revoked_during == "listen_event":
            with pytest.raises(WebsocketDisconnectError):
                async with events_listen(bob_ws):
                    await _do_revoke()

        else:
            await _do_revoke()
            with pytest.raises(WebsocketDisconnectError):
                async with real_clock_timeout():
                    await events_listen_wait(bob_ws)


@pytest.mark.trio
async def test_events_subscribe(backend, alice_ws, alice2_ws):
    await events_subscribe(alice_ws)

    # Should ignore our own events
    with backend.event_bus.listen() as spy:
        await authenticated_ping(alice_ws, "bar")
        await authenticated_ping(alice2_ws, "foo")

        # No guarantees those events occur before the commands' return
        await spy.wait_multiple_with_timeout([BackendEvent.PINGED, BackendEvent.PINGED])

    rep = await events_listen_nowait(alice_ws)
    assert rep == EventsListenRepOkPinged("foo")
    rep = await events_listen_nowait(alice_ws)
    assert isinstance(rep, EventsListenRepNoEvents)


@pytest.mark.trio
async def test_event_resubscribe(backend, alice_ws, alice2_ws):
    await events_subscribe(alice_ws)

    with backend.event_bus.listen() as spy:
        await authenticated_ping(alice2_ws, "foo")

        # No guarantees those events occur before the commands' return
        await spy.wait_with_timeout(BackendEvent.PINGED)

    # Resubscribing should have no effect
    await events_subscribe(alice_ws)

    with backend.event_bus.listen() as spy:
        await authenticated_ping(alice2_ws, "bar")
        await authenticated_ping(alice2_ws, "spam")

        # No guarantees those events occur before the commands' return
        await spy.wait_multiple_with_timeout([BackendEvent.PINGED, BackendEvent.PINGED])

    rep = await events_listen_nowait(alice_ws)
    assert rep == EventsListenRepOkPinged("foo")
    rep = await events_listen_nowait(alice_ws)
    assert rep == EventsListenRepOkPinged("bar")
    rep = await events_listen_nowait(alice_ws)
    assert rep == EventsListenRepOkPinged("spam")
    rep = await events_listen_nowait(alice_ws)
    assert isinstance(rep, EventsListenRepNoEvents)


@pytest.mark.trio
@pytest.mark.postgresql
async def test_cross_backend_event(backend_factory, backend_authenticated_ws_factory, alice, bob):
    async with backend_factory() as backend_1, backend_factory(populated=False) as backend_2:
        app_1 = app_factory(backend_1)
        app_2 = app_factory(backend_2)

        async with backend_authenticated_ws_factory(
            app_1, bob
        ) as bob_ws, backend_authenticated_ws_factory(app_2, alice) as alice_ws:

            await events_subscribe(alice_ws)

            async with events_listen(alice_ws) as listen:
                await authenticated_ping(bob_ws, "foo")
            assert listen.rep == EventsListenRepOkPinged("foo")

            await authenticated_ping(bob_ws, "foo")

            # There is no guarantee an event is ready to be received once
            # the sender got it answer
            async with real_clock_timeout():
                while True:
                    rep = await events_listen_nowait(alice_ws)
                    if not isinstance(rep, EventsListenRepNoEvents):
                        break
                    await trio.sleep(0.1)
            assert rep == EventsListenRepOkPinged("foo")

            rep = await events_listen_nowait(alice_ws)
            assert isinstance(rep, EventsListenRepNoEvents)


@pytest.mark.trio
async def test_events_listen_wait_cancelled(backend_asgi_app, alice_ws):
    async with events_listen(alice_ws) as listen:
        # Cancel `events_listen` by sending another command
        rep = await authenticated_ping(alice_ws, ping="foo")
        assert rep == AuthenticatedPingRepOk("foo")

        listen.rep_done = True


@pytest.mark.trio
async def test_events_close_connection_on_backpressure(
    backend_asgi_app, alice, bob_ws, monkeypatch, backend_authenticated_ws_factory
):
    # The channel has a queue of size 1, meaning it will be filled after a single command
    monkeypatch.setattr("parsec.backend.client_context.AUTHENTICATED_CLIENT_CHANNEL_SIZE", 1)
    # We need to manually initialize the websockets using the factory here to make
    # sure the monkeypatch happens before the memory channel creation and thus
    # give then a size of 1
    async with backend_authenticated_ws_factory(backend_asgi_app, alice) as alice_ws:
        await events_subscribe(alice_ws)

        await authenticated_ping(bob_ws, "foo")
        await authenticated_ping(bob_ws, "foo")

        # Alice should be disconnected at this point and exit the loop
        with trio.fail_after(1):
            while True:
                try:
                    await authenticated_ping(alice_ws, "poke")
                except WebsocketDisconnectError:
                    break


# TODO: test message.received and beacon.updated events


@pytest.mark.trio
async def test_sse_events_subscribe(
    backend, alice_rpc: AuthenticatedRpcApiClient, alice2_rpc: AuthenticatedRpcApiClient
):
    async with real_clock_timeout():
        async with alice_rpc.connect_sse_events() as sse_con:

            # Should ignore our own events
            with backend.event_bus.listen() as spy:
                await authenticated_ping(alice_rpc, "bar")
                await authenticated_ping(alice2_rpc, "foo")

                # No guarantees those events occur before the commands' return
                await spy.wait_multiple_with_timeout([BackendEvent.PINGED, BackendEvent.PINGED])

            event = await sse_con.get_next_event()
            assert sse_con.status_code == 200
            assert event == EventsListenRepOkPinged("foo")


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
                await bob_rpc.send_ping("1")

                event = await sse_con.get_next_event()
                assert event == EventsListenRepOkPinged("1")
                assert sse_con.status_code == 200

                await bob_rpc.send_ping("2")
                await bob_rpc.send_ping("3")

                event = await sse_con.get_next_event()
                assert event == EventsListenRepOkPinged("2")
                event = await sse_con.get_next_event()
                assert event == EventsListenRepOkPinged("3")


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
                BackendEvent.PINGED,
                organization_id=alice.organization_id,
                author=bob.device_id,
                ping="1",
            )
            backend.event_bus.send(
                BackendEvent.PINGED,
                organization_id=alice.organization_id,
                author=bob.device_id,
                ping="2",
            )
            backend.event_bus.send(
                BackendEvent.PINGED,
                organization_id=alice.organization_id,
                author=bob.device_id,
                ping="3",
            )

            # The connection simply gets closed without error status given nothing wrong
            # occured in practice
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
