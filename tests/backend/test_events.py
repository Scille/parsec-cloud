# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import pytest
import trio

from quart.testing.connections import WebsocketDisconnectError

from parsec.api.protocol import APIEvent
from parsec.backend.asgi import app_factory
from parsec.backend.backend_events import BackendEvent

from tests.backend.common import (
    events_subscribe,
    events_listen,
    events_listen_wait,
    events_listen_nowait,
    ping,
    real_clock_timeout,
)


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
        await ping(alice_ws, "bar")
        await ping(alice2_ws, "foo")

        # No guarantees those events occur before the commands' return
        await spy.wait_multiple_with_timeout([BackendEvent.PINGED, BackendEvent.PINGED])

    rep = await events_listen_nowait(alice_ws)
    assert rep == {"status": "ok", "event": APIEvent.PINGED, "ping": "foo"}
    rep = await events_listen_nowait(alice_ws)
    assert rep == {"status": "no_events"}


@pytest.mark.trio
async def test_event_resubscribe(backend, alice_ws, alice2_ws):
    await events_subscribe(alice_ws)

    with backend.event_bus.listen() as spy:
        await ping(alice2_ws, "foo")

        # No guarantees those events occur before the commands' return
        await spy.wait_with_timeout(BackendEvent.PINGED)

    # Resubscribing should have no effect
    await events_subscribe(alice_ws)

    with backend.event_bus.listen() as spy:
        await ping(alice2_ws, "bar")
        await ping(alice2_ws, "spam")

        # No guarantees those events occur before the commands' return
        await spy.wait_multiple_with_timeout([BackendEvent.PINGED, BackendEvent.PINGED])

    rep = await events_listen_nowait(alice_ws)
    assert rep == {"status": "ok", "event": APIEvent.PINGED, "ping": "foo"}
    rep = await events_listen_nowait(alice_ws)
    assert rep == {"status": "ok", "event": APIEvent.PINGED, "ping": "bar"}
    rep = await events_listen_nowait(alice_ws)
    assert rep == {"status": "ok", "event": APIEvent.PINGED, "ping": "spam"}
    rep = await events_listen_nowait(alice_ws)
    assert rep == {"status": "no_events"}


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
                await ping(bob_ws, "foo")
            assert listen.rep == {"status": "ok", "event": APIEvent.PINGED, "ping": "foo"}

            await ping(bob_ws, "foo")

            # There is no guarantee an event is ready to be received once
            # the sender got it answer
            async with real_clock_timeout():
                while True:
                    rep = await events_listen_nowait(alice_ws)
                    if rep["status"] != "no_events":
                        break
                    await trio.sleep(0.1)
            assert rep == {"status": "ok", "event": APIEvent.PINGED, "ping": "foo"}

            rep = await events_listen_nowait(alice_ws)
            assert rep == {"status": "no_events"}


@pytest.mark.trio
async def test_events_listen_wait_cancelled(backend_asgi_app, alice_ws):
    async with events_listen(alice_ws) as listen:
        # Cancel `events_listen` by sending another command
        rep = await ping(alice_ws, ping="foo")
        assert rep == {"status": "ok", "pong": "foo"}

        listen.rep_done = True


# TODO: test message.received and beacon.updated events
