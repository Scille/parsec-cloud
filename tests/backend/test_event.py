import pytest
import trio


@pytest.mark.trio
async def test_event_subscribe(alice_backend_sock):
    sock = alice_backend_sock

    await sock.send({"cmd": "event_subscribe", "event": "pinged", "ping": "foo"})
    rep = await sock.recv()
    assert rep == {"status": "ok"}


@pytest.mark.trio
async def test_event_subscribe_unkown_event(alice_backend_sock):
    sock = alice_backend_sock

    await sock.send({"cmd": "event_subscribe", "event": "foo"})
    rep = await sock.recv()
    assert rep == {"status": "bad_message", "errors": {"event": ["Unsupported value: foo"]}}


async def subscribe_ping(sock, ping):
    await sock.send({"cmd": "event_subscribe", "event": "pinged", "ping": ping})
    rep = await sock.recv()
    assert rep == {"status": "ok"}


async def ping(sock, subject):
    await sock.send({"cmd": "ping", "ping": subject})
    rep = await sock.recv()
    assert rep == {"status": "ok", "pong": subject}


@pytest.mark.trio
async def test_event_unsubscribe(alice_backend_sock):
    sock = alice_backend_sock

    await subscribe_ping(sock, "foo")

    await sock.send({"cmd": "event_unsubscribe", "event": "pinged", "ping": "foo"})
    rep = await sock.recv()
    assert rep == {"status": "ok"}


@pytest.mark.trio
async def test_event_unsubscribe_ping_bad_msg(alice_backend_sock):
    sock = alice_backend_sock

    await subscribe_ping(sock, "foo")
    await sock.send({"cmd": "event_unsubscribe", "event": "pinged", "ping": "bar"})
    rep = await sock.recv()
    assert rep == {"status": "not_subscribed", "reason": "Not subscribed to ('pinged', 'bar')"}


@pytest.mark.trio
async def test_event_unsubscribe_bad_event(alice_backend_sock):
    sock = alice_backend_sock

    await sock.send({"cmd": "event_unsubscribe", "event": "message.received"})
    rep = await sock.recv()
    assert rep == {"status": "not_subscribed", "reason": "Not subscribed to 'message.received'"}


@pytest.mark.trio
async def test_event_unsubscribe_unknown_event(alice_backend_sock):
    sock = alice_backend_sock

    await sock.send({"cmd": "event_unsubscribe", "event": "unknown"})
    rep = await sock.recv()
    assert rep == {"status": "bad_message", "errors": {"event": ["Unsupported value: unknown"]}}


@pytest.mark.trio
async def test_ignore_own_events(alice_backend_sock):
    sock = alice_backend_sock

    await subscribe_ping(sock, "foo")

    await ping(sock, "foo")

    await sock.send({"cmd": "event_listen", "wait": False})
    rep = await sock.recv()
    assert rep == {"status": "no_events"}


@pytest.mark.trio
async def test_event_listen(alice_backend_sock, bob_backend_sock):
    alice_sock, bob_sock = alice_backend_sock, bob_backend_sock

    await alice_sock.send({"cmd": "event_listen", "wait": False})
    rep = await alice_sock.recv()
    assert rep == {"status": "no_events"}

    await subscribe_ping(alice_sock, "foo")

    await alice_sock.send({"cmd": "event_listen"})

    await ping(bob_sock, "bar")
    await ping(bob_sock, "foo")

    with trio.fail_after(1):
        rep = await alice_sock.recv()
    assert rep == {"status": "ok", "event": "pinged", "ping": "foo"}

    await ping(bob_sock, "foo")

    # There is no guarantee an event is ready to be received once
    # the sender got it answer
    with trio.fail_after(1):
        while True:
            await alice_sock.send({"cmd": "event_listen", "wait": False})
            rep = await alice_sock.recv()
            if rep["status"] != "no_events":
                break
            await trio.sleep(0.1)
    assert rep == {"status": "ok", "event": "pinged", "ping": "foo"}

    await alice_sock.send({"cmd": "event_listen", "wait": False})
    rep = await alice_sock.recv()
    assert rep == {"status": "no_events"}


@pytest.mark.trio
@pytest.mark.postgresql
async def test_cross_backend_event(backend_factory, backend_sock_factory, alice, bob):
    async with backend_factory() as backend_1, backend_factory(devices=()) as backend_2:
        async with backend_sock_factory(backend_1, alice) as alice_sock, backend_sock_factory(
            backend_2, bob
        ) as bob_sock:

            await subscribe_ping(alice_sock, "foo")

            await alice_sock.send({"cmd": "event_listen"})

            await ping(bob_sock, "bar")
            await ping(bob_sock, "foo")

            with trio.fail_after(1):
                rep = await alice_sock.recv()
            assert rep == {"status": "ok", "event": "pinged", "ping": "foo"}

            await ping(bob_sock, "foo")

            # There is no guarantee an event is ready to be received once
            # the sender got it answer
            with trio.fail_after(1):
                while True:
                    await alice_sock.send({"cmd": "event_listen", "wait": False})
                    rep = await alice_sock.recv()
                    if rep["status"] != "no_events":
                        break
                    await trio.sleep(0.1)
            assert rep == {"status": "ok", "event": "pinged", "ping": "foo"}

            await alice_sock.send({"cmd": "event_listen", "wait": False})
            rep = await alice_sock.recv()
            assert rep == {"status": "no_events"}


# TODO: test private events
