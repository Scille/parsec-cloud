import pytest

from tests.common import connect_backend


@pytest.mark.trio
async def test_event_subscribe(backend, alice):
    async with connect_backend(backend, auth_as=alice) as sock:
        await sock.send({"cmd": "event_subscribe", "event": "ping", "subject": "foo"})
        rep = await sock.recv()
        assert rep == {"status": "ok"}


@pytest.mark.trio
async def test_event_subscribe_unkown_event(backend, alice):
    async with connect_backend(backend, auth_as=alice) as sock:
        await sock.send({"cmd": "event_subscribe", "event": "foo", "subject": "foo"})
        rep = await sock.recv()
        assert rep == {"status": "bad_message", "errors": {"event": ["Not a valid choice."]}}


async def subscribe(sock, event, subject):
    await sock.send({"cmd": "event_subscribe", "event": event, "subject": subject})
    rep = await sock.recv()
    assert rep == {"status": "ok"}


@pytest.mark.trio
async def test_event_unsubscribe(backend, alice):
    async with connect_backend(backend, auth_as=alice) as sock:
        await subscribe(sock, "ping", "foo")
        await sock.send({"cmd": "event_unsubscribe", "event": "ping", "subject": "foo"})
        rep = await sock.recv()
        assert rep == {"status": "ok"}


@pytest.mark.trio
async def test_event_unsubscribe_bad_subject(backend, alice):
    async with connect_backend(backend, auth_as=alice) as sock:
        await subscribe(sock, "ping", "foo")
        await sock.send({"cmd": "event_unsubscribe", "event": "ping", "subject": "bar"})
        rep = await sock.recv()
        assert rep == {
            "status": "not_subscribed",
            "reason": "Not subscribed to this event/subject couple",
        }


@pytest.mark.trio
async def test_event_unsubscribe_bad_event(backend, alice):
    async with connect_backend(backend, auth_as=alice) as sock:
        await sock.send({"cmd": "event_unsubscribe", "event": "ping", "subject": "bar"})
        rep = await sock.recv()
        assert rep == {
            "status": "not_subscribed",
            "reason": "Not subscribed to this event/subject couple",
        }


@pytest.mark.trio
async def test_event_unsubscribe_unknown_event(backend, alice):
    async with connect_backend(backend, auth_as=alice) as sock:
        await sock.send({"cmd": "event_unsubscribe", "event": "unknown", "subject": "bar"})
        rep = await sock.recv()
        assert rep == {"status": "bad_message", "errors": {"event": ["Not a valid choice."]}}


@pytest.mark.trio
async def test_event_listen(backend, alice, bob):
    async with connect_backend(backend, auth_as=alice) as alice_sock, connect_backend(
        backend, auth_as=bob
    ) as bob_sock:

        await alice_sock.send({"cmd": "event_listen", "wait": False})
        rep = await alice_sock.recv()
        assert rep == {"status": "ok"}

        await subscribe(alice_sock, "ping", "foo")

        await alice_sock.send({"cmd": "event_listen"})

        await bob_sock.send({"cmd": "ping", "ping": "bar"})
        await bob_sock.recv()
        await bob_sock.send({"cmd": "ping", "ping": "foo"})
        await bob_sock.recv()

        rep = await alice_sock.recv()
        assert rep == {"status": "ok", "event": "ping", "subject": "foo"}

        await bob_sock.send({"cmd": "ping", "ping": "foo"})
        await bob_sock.recv()
        await alice_sock.send({"cmd": "event_listen", "wait": False})
        rep = await alice_sock.recv()
        assert rep == {"status": "ok", "event": "ping", "subject": "foo"}


# TODO: test private events
