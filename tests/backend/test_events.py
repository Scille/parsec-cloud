# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
import trio
from uuid import uuid4
from async_generator import asynccontextmanager

from parsec.api.protocol import (
    events_subscribe_serializer,
    events_listen_serializer,
    ping_serializer,
)


BEACON_ID = uuid4()


async def events_subscribe(sock):
    await sock.send(events_subscribe_serializer.req_dumps({"cmd": "events_subscribe"}))
    raw_rep = await sock.recv()
    rep = events_subscribe_serializer.rep_loads(raw_rep)
    assert rep == {"status": "ok"}


async def events_listen_nowait(sock):
    await sock.send(events_listen_serializer.req_dumps({"cmd": "events_listen", "wait": False}))
    raw_rep = await sock.recv()
    return events_listen_serializer.rep_loads(raw_rep)


class Listen:
    def __init__(self):
        self.rep = None


@asynccontextmanager
async def events_listen(sock):
    await sock.send(events_listen_serializer.req_dumps({"cmd": "events_listen"}))
    listen = Listen()

    yield listen

    with trio.fail_after(1):
        raw_rep = await sock.recv()
    listen.rep = events_listen_serializer.rep_loads(raw_rep)


async def ping(sock, subject="foo"):
    raw_req = ping_serializer.req_dumps({"cmd": "ping", "ping": subject})
    await sock.send(raw_req)
    raw_rep = await sock.recv()
    rep = ping_serializer.rep_loads(raw_rep)
    assert rep == {"status": "ok", "pong": subject}


@pytest.mark.trio
async def test_events_subscribe(backend, alice_backend_sock, alice2_backend_sock):
    await events_subscribe(alice_backend_sock)

    # Should ignore our own events
    with backend.event_bus.listen() as spy:
        await ping(alice_backend_sock, "bar")
        await ping(alice2_backend_sock, "foo")

        # No guarantees those events occur before the commands' return
        await spy.wait_multiple_with_timeout(["pinged", "pinged"])

    rep = await events_listen_nowait(alice_backend_sock)
    assert rep == {"status": "ok", "event": "pinged", "ping": "foo"}
    rep = await events_listen_nowait(alice_backend_sock)
    assert rep == {"status": "no_events"}


@pytest.mark.trio
async def test_event_resubscribe(backend, alice_backend_sock, alice2_backend_sock):
    await events_subscribe(alice_backend_sock)

    with backend.event_bus.listen() as spy:
        await ping(alice2_backend_sock, "foo")

        # No guarantees those events occur before the commands' return
        await spy.wait_with_timeout("pinged")

    # Resubscribing should have no effect
    await events_subscribe(alice_backend_sock)

    with backend.event_bus.listen() as spy:
        await ping(alice2_backend_sock, "bar")
        await ping(alice2_backend_sock, "spam")

        # No guarantees those events occur before the commands' return
        await spy.wait_multiple_with_timeout(["pinged", "pinged"])

    rep = await events_listen_nowait(alice_backend_sock)
    assert rep == {"status": "ok", "event": "pinged", "ping": "foo"}
    rep = await events_listen_nowait(alice_backend_sock)
    assert rep == {"status": "ok", "event": "pinged", "ping": "bar"}
    rep = await events_listen_nowait(alice_backend_sock)
    assert rep == {"status": "ok", "event": "pinged", "ping": "spam"}
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
            assert listen.rep == {"status": "ok", "event": "pinged", "ping": "foo"}

            await ping(bob_sock, "foo")

            # There is no guarantee an event is ready to be received once
            # the sender got it answer
            with trio.fail_after(1):
                while True:
                    rep = await events_listen_nowait(alice_sock)
                    if rep["status"] != "no_events":
                        break
                    await trio.sleep(0.1)
            assert rep == {"status": "ok", "event": "pinged", "ping": "foo"}

            rep = await events_listen_nowait(alice_sock)
            assert rep == {"status": "no_events"}


# TODO: test message.received and beacon.updated events
