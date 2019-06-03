# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
import trio
from uuid import uuid4
from async_generator import asynccontextmanager

from parsec.api.protocole import (
    packb,
    events_subscribe_serializer,
    events_listen_serializer,
    ping_serializer,
)


BEACON_ID = uuid4()


async def events_subscribe(sock, **kwargs):
    # Sanity check to fail fast in case of typo
    assert not kwargs.keys() - {"ping", "realm", "message"}

    await sock.send(events_subscribe_serializer.req_dumps({"cmd": "events_subscribe", **kwargs}))
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


@pytest.mark.trio
@pytest.mark.parametrize(
    "events",
    [
        {"ping": [], "realm": [], "message": False},
        {"ping": ["foo"], "realm": [BEACON_ID], "message": True},
        {"realm": [BEACON_ID], "message": True},
        {"ping": ["foo"], "message": True},
        {"ping": ["foo"], "realm": [BEACON_ID]},
        {},
    ],
)
async def test_events_subscribe_ok(alice_backend_sock, events):
    await events_subscribe(alice_backend_sock, **events)


@pytest.mark.trio
@pytest.mark.parametrize(
    "events",
    [
        {"dummy": []},
        {"ping": [42]},
        {"realm": ["dummy"]},
        {"ping": ["a" * 100]},  # Too long
        {"message": []},
    ],
)
async def test_events_subscribe_bad_msg(alice_backend_sock, events):
    await alice_backend_sock.send(packb({"cmd": "events_subscribe", **events}))
    raw_rep = await alice_backend_sock.recv()
    rep = events_subscribe_serializer.rep_loads(raw_rep)
    assert rep["status"] == "bad_message"


async def subscribe_pinged(sock, pings):
    raw_req = events_subscribe_serializer.req_dumps({"cmd": "events_subscribe", "ping": pings})
    await sock.send(raw_req)
    raw_rep = await sock.recv()
    rep = events_subscribe_serializer.rep_loads(raw_rep)
    assert rep == {"status": "ok"}


async def ping(sock, subject="foo"):
    raw_req = ping_serializer.req_dumps({"cmd": "ping", "ping": subject})
    await sock.send(raw_req)
    raw_rep = await sock.recv()
    rep = ping_serializer.rep_loads(raw_rep)
    assert rep == {"status": "ok", "pong": subject}


@pytest.mark.trio
async def test_events_subscribe_ping(backend, alice_backend_sock, alice2_backend_sock):
    await subscribe_pinged(alice_backend_sock, ["foo", "bar"])

    # Should ignore our own events
    await ping(alice2_backend_sock, "nope")
    await ping(alice_backend_sock, "bar")
    await ping(alice2_backend_sock, "foo")

    with trio.fail_after(1):
        # No guarantees those events occur before the commands' return
        await backend.event_bus.spy.wait_multiple(["pinged", "pinged", "pinged"])

    rep = await events_listen_nowait(alice_backend_sock)
    assert rep == {"status": "ok", "event": "pinged", "ping": "foo"}
    rep = await events_listen_nowait(alice_backend_sock)
    assert rep == {"status": "no_events"}


@pytest.mark.trio
async def test_event_resubscribe(backend, alice_backend_sock, alice2_backend_sock):
    await subscribe_pinged(alice_backend_sock, ["foo", "bar"])

    with backend.event_bus.listen() as spy:
        await ping(alice2_backend_sock, "foo")

        with trio.fail_after(1):
            # No guarantees those events occur before the commands' return
            await spy.wait("pinged")

    await subscribe_pinged(alice_backend_sock, ["bar", "spam"])

    with backend.event_bus.listen() as spy:
        await ping(alice2_backend_sock, "foo")  # Should be ignored
        await ping(alice2_backend_sock, "bar")
        await ping(alice2_backend_sock, "spam")

        with trio.fail_after(1):
            # No guarantees those events occur before the commands' return
            await spy.wait_multiple(["pinged", "pinged", "pinged"])

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

            await subscribe_pinged(alice_sock, ["foo"])

            async with events_listen(alice_sock) as listen:
                await ping(bob_sock, "bar")
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
