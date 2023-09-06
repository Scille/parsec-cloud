# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
from __future__ import annotations

import pytest
import trio
from quart.testing.connections import WebsocketDisconnectError

from parsec._parsec import ApiVersion, BackendEventPinged, authenticated_cmds
from parsec.backend.asgi import app_factory
from tests.backend.common import (
    apiv2v3_events_listen,
    apiv2v3_events_listen_nowait,
    apiv2v3_events_listen_wait,
    apiv2v3_events_subscribe,
    authenticated_ping,
    real_clock_timeout,
)
from tests.common import AuthenticatedRpcApiClient

APIEventPinged = authenticated_cmds.v3.events_listen.APIEventPinged
AuthenticatedPingRepOk = authenticated_cmds.v3.ping.RepOk
EventsListenRepNoEvents = authenticated_cmds.v3.events_listen.RepNoEvents
EventsListenRepOk = authenticated_cmds.v3.events_listen.RepOk


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
            revoked_user_certificate=b"whatever",
            revoked_user_certifier=alice.device_id,
        )
        # connection cancellation is handled through events, so wait
        # for things to settle down to make sure there is no pending event
        await trio.testing.wait_all_tasks_blocked()

    async with backend_authenticated_ws_factory(backend_asgi_app, bob) as bob_ws:
        await apiv2v3_events_subscribe(bob_ws)

        if revoked_during == "listen_event":
            with pytest.raises(WebsocketDisconnectError):
                async with apiv2v3_events_listen(bob_ws):
                    await _do_revoke()

        else:
            await _do_revoke()
            with pytest.raises(WebsocketDisconnectError):
                async with real_clock_timeout():
                    await apiv2v3_events_listen_wait(bob_ws)


@pytest.mark.trio
async def test_events_subscribe(backend, alice_ws, alice2_ws):
    await apiv2v3_events_subscribe(alice_ws)

    # Should ignore our own events
    with backend.event_bus.listen() as spy:
        await authenticated_ping(alice_ws, "bar")
        await authenticated_ping(alice2_ws, "foo")

        # No guarantees those events occur before the commands' return
        await spy.wait_multiple_with_timeout([BackendEventPinged, BackendEventPinged])

    rep = await apiv2v3_events_listen_nowait(alice_ws)
    assert rep == EventsListenRepOk(APIEventPinged("foo"))
    rep = await apiv2v3_events_listen_nowait(alice_ws)
    assert isinstance(rep, EventsListenRepNoEvents)


@pytest.mark.trio
async def test_event_resubscribe(backend, alice_ws, alice2_ws):
    await apiv2v3_events_subscribe(alice_ws)

    with backend.event_bus.listen() as spy:
        await authenticated_ping(alice2_ws, "foo")

        # No guarantees those events occur before the commands' return
        await spy.wait_with_timeout(BackendEventPinged)

    # Resubscribing should have no effect
    await apiv2v3_events_subscribe(alice_ws)

    with backend.event_bus.listen() as spy:
        await authenticated_ping(alice2_ws, "bar")
        await authenticated_ping(alice2_ws, "spam")

        # No guarantees those events occur before the commands' return
        await spy.wait_multiple_with_timeout([BackendEventPinged, BackendEventPinged])

    rep = await apiv2v3_events_listen_nowait(alice_ws)
    assert rep == EventsListenRepOk(APIEventPinged("foo"))
    rep = await apiv2v3_events_listen_nowait(alice_ws)
    assert rep == EventsListenRepOk(APIEventPinged("bar"))
    rep = await apiv2v3_events_listen_nowait(alice_ws)
    assert rep == EventsListenRepOk(APIEventPinged("spam"))
    rep = await apiv2v3_events_listen_nowait(alice_ws)
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
            await apiv2v3_events_subscribe(alice_ws)

            async with apiv2v3_events_listen(alice_ws) as listen:
                await authenticated_ping(bob_ws, "foo")
            assert listen.rep == EventsListenRepOk(APIEventPinged("foo"))

            await authenticated_ping(bob_ws, "foo")

            # There is no guarantee an event is ready to be received once
            # the sender got it answer
            async with real_clock_timeout():
                while True:
                    rep = await apiv2v3_events_listen_nowait(alice_ws)
                    if not isinstance(rep, EventsListenRepNoEvents):
                        break
                    await trio.sleep(0.1)
            assert rep == EventsListenRepOk(APIEventPinged("foo"))

            rep = await apiv2v3_events_listen_nowait(alice_ws)
            assert isinstance(rep, EventsListenRepNoEvents)


@pytest.mark.trio
async def test_events_listen_wait_cancelled(backend_asgi_app, alice_ws):
    async with apiv2v3_events_listen(alice_ws) as listen:
        # Cancel `apiv2v3_events_listen` by sending another command
        rep = await authenticated_ping(alice_ws, ping="foo")
        assert rep == AuthenticatedPingRepOk("foo")

        listen.rep_done = True


@pytest.mark.trio
async def test_events_close_connection_on_back_pressure(
    backend_asgi_app, alice, bob_ws, monkeypatch, backend_authenticated_ws_factory
):
    # The channel has a queue of size 1, meaning it will be filled after a single command
    monkeypatch.setattr("parsec.backend.client_context.AUTHENTICATED_CLIENT_CHANNEL_SIZE", 1)
    # We need to manually initialize the websockets using the factory here to make
    # sure the monkeypatch happens before the memory channel creation and thus
    # give then a size of 1
    async with backend_authenticated_ws_factory(backend_asgi_app, alice) as alice_ws:
        await apiv2v3_events_subscribe(alice_ws)

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
async def test_sse_not_support_by_apiv3(backend, alice_rpc: AuthenticatedRpcApiClient):
    def _use_api_v3(args: dict) -> None:
        args["headers"]["Api-Version"] = str(ApiVersion.API_V3_VERSION)

    async with alice_rpc.connect_sse_events(before_send_hook=_use_api_v3) as sse_con:
        assert sse_con.status_code == 415
