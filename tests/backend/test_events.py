# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import pytest
import trio

from parsec.api.protocol import APIEvent
from parsec.api.transport import TransportError
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
    backend, backend_sock_factory, bob, alice, revoked_during
):
    # The event system is also used to detect a connection should be dropped
    # given the corresponding device has been revoked.
    # Let's make sure `events_subscribe` command plays well with this behavior.

    async def _do_revoke():
        await backend.user.revoke_user(
            organization_id=bob.organization_id,
            user_id=bob.user_id,
            revoked_user_certificate=b"wathever",
            revoked_user_certifier=alice.device_id,
        )
        # connection cancellation is handled through events, so wait
        # for things to settle down to make sure there is no pending event
        await trio.testing.wait_all_tasks_blocked()

    async with backend_sock_factory(
        backend, bob, freeze_on_transport_error=False
    ) as bob_backend_sock:

        await events_subscribe(bob_backend_sock)

        if revoked_during == "listen_event":
            with pytest.raises(TransportError):
                async with events_listen(bob_backend_sock):
                    await _do_revoke()

        else:
            await _do_revoke()
            with pytest.raises(TransportError):
                async with real_clock_timeout():
                    await events_listen_wait(bob_backend_sock)


@pytest.mark.trio
async def test_events_subscribe(backend, alice_backend_sock, alice2_backend_sock):
    await events_subscribe(alice_backend_sock)

    # Should ignore our own events
    with backend.event_bus.listen() as spy:
        await ping(alice_backend_sock, "bar")
        await ping(alice2_backend_sock, "foo")

        # No guarantees those events occur before the commands' return
        await spy.wait_multiple_with_timeout([BackendEvent.PINGED, BackendEvent.PINGED])

    rep = await events_listen_nowait(alice_backend_sock)
    assert rep == {"status": "ok", "event": APIEvent.PINGED, "ping": "foo"}
    rep = await events_listen_nowait(alice_backend_sock)
    assert rep == {"status": "no_events"}


@pytest.mark.trio
async def test_event_resubscribe(backend, alice_backend_sock, alice2_backend_sock):
    await events_subscribe(alice_backend_sock)

    with backend.event_bus.listen() as spy:
        await ping(alice2_backend_sock, "foo")

        # No guarantees those events occur before the commands' return
        await spy.wait_with_timeout(BackendEvent.PINGED)

    # Resubscribing should have no effect
    await events_subscribe(alice_backend_sock)

    with backend.event_bus.listen() as spy:
        await ping(alice2_backend_sock, "bar")
        await ping(alice2_backend_sock, "spam")

        # No guarantees those events occur before the commands' return
        await spy.wait_multiple_with_timeout([BackendEvent.PINGED, BackendEvent.PINGED])

    rep = await events_listen_nowait(alice_backend_sock)
    assert rep == {"status": "ok", "event": APIEvent.PINGED, "ping": "foo"}
    rep = await events_listen_nowait(alice_backend_sock)
    assert rep == {"status": "ok", "event": APIEvent.PINGED, "ping": "bar"}
    rep = await events_listen_nowait(alice_backend_sock)
    assert rep == {"status": "ok", "event": APIEvent.PINGED, "ping": "spam"}
    rep = await events_listen_nowait(alice_backend_sock)
    assert rep == {"status": "no_events"}


@pytest.mark.trio
@pytest.mark.postgresql
async def test_cross_backend_event(backend_factory, backend_sock_factory, alice, bob):
    async with backend_factory() as backend_1, backend_factory(populated=False) as backend_2:
        async with backend_sock_factory(backend_1, alice) as alice_sock, backend_sock_factory(
            backend_2, bob
        ) as bob_sock:

            await events_subscribe(alice_sock)

            async with events_listen(alice_sock) as listen:
                await ping(bob_sock, "foo")
            assert listen.rep == {"status": "ok", "event": APIEvent.PINGED, "ping": "foo"}

            await ping(bob_sock, "foo")

            # There is no guarantee an event is ready to be received once
            # the sender got it answer
            async with real_clock_timeout():
                while True:
                    rep = await events_listen_nowait(alice_sock)
                    if rep["status"] != "no_events":
                        break
                    await trio.sleep(0.1)
            assert rep == {"status": "ok", "event": APIEvent.PINGED, "ping": "foo"}

            rep = await events_listen_nowait(alice_sock)
            assert rep == {"status": "no_events"}


# TODO: test message.received and beacon.updated events
