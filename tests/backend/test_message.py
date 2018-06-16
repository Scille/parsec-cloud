import pytest
import trio

from parsec.utils import to_jsonb64

from parsec.backend.drivers import postgresql as pg_driver


@pytest.mark.trio
async def test_message_from_bob_to_alice(alice, bob, alice_backend_sock, bob_backend_sock):
    await alice_backend_sock.send({"cmd": "event_subscribe", "event": "message.received"})
    rep = await alice_backend_sock.recv()
    assert rep == {"status": "ok"}

    await alice_backend_sock.send({"cmd": "event_listen"})

    await bob_backend_sock.send(
        {"cmd": "message_new", "recipient": alice.user_id, "body": to_jsonb64(b"Hello from Bob !")}
    )
    rep = await bob_backend_sock.recv()
    assert rep == {"status": "ok"}

    with trio.fail_after(1):
        rep = await alice_backend_sock.recv()
    assert rep == {"status": "ok", "event": "message.received", "index": 1}

    await alice_backend_sock.send({"cmd": "message_get"})
    rep = await alice_backend_sock.recv()
    assert rep == {
        "status": "ok",
        "messages": [{"body": to_jsonb64(b"Hello from Bob !"), "sender_id": bob.id, "count": 1}],
    }


@pytest.mark.trio
async def test_message_from_bob_to_alice_multi_backends(
    asyncio_loop, postgresql_url, alice, bob, backend_factory, backend_sock_factory
):
    await pg_driver.handler.init_db(postgresql_url, True)
    backend_1 = await backend_factory(
        config={"blockstore_postgresql": True, "dburl": postgresql_url}
    )
    backend_2 = await backend_factory(
        config={"blockstore_postgresql": True, "dburl": postgresql_url}
    )

    await backend_1.user.create(
        author="<backend-fixture>",
        user_id=alice.user_id,
        broadcast_key=alice.user_pubkey.encode(),
        devices=[(alice.device_name, alice.device_verifykey.encode())],
    )

    await backend_1.user.create(
        author="<backend-fixture>",
        user_id=bob.user_id,
        broadcast_key=bob.user_pubkey.encode(),
        devices=[(bob.device_name, bob.device_verifykey.encode())],
    )

    alice_backend_sock = backend_sock_factory(backend_1, alice)
    bob_backend_sock = backend_sock_factory(backend_2, bob)

    await alice_backend_sock.send({"cmd": "event_subscribe", "event": "message.received"})
    rep = await alice_backend_sock.recv()
    assert rep == {"status": "ok"}

    await alice_backend_sock.send({"cmd": "event_listen"})

    await bob_backend_sock.send(
        {"cmd": "message_new", "recipient": alice.user_id, "body": to_jsonb64(b"Hello from Bob !")}
    )
    rep = await bob_backend_sock.recv()
    assert rep == {"status": "ok"}

    with trio.fail_after(1):
        rep = await alice_backend_sock.recv()
    assert rep == {"status": "ok", "event": "message.received", "index": 1}

    await alice_backend_sock.send({"cmd": "message_get"})
    rep = await alice_backend_sock.recv()
    assert rep == {
        "status": "ok",
        "messages": [
            {"body": to_jsonb64(b"Hello from Bob !"), "sender_id": "bob@test", "count": 1}
        ],
    }
