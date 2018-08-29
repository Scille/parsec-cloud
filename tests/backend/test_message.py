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
@pytest.mark.postgresql
async def test_message_from_bob_to_alice_multi_backends(
    postgresql_url, alice, bob, backend_factory, backend_sock_factory
):
    async with backend_factory(
        config={"blockstore_url": "POSTGRESQL", "db_url": postgresql_url}
    ) as backend_1, backend_factory(
        devices=(), config={"blockstore_url": "POSTGRESQL", "db_url": postgresql_url}
    ) as backend_2:

        async with backend_sock_factory(
            backend_1, alice
        ) as alice_backend_sock, backend_sock_factory(backend_2, bob) as bob_backend_sock:

            await alice_backend_sock.send({"cmd": "event_subscribe", "event": "message.received"})
            rep = await alice_backend_sock.recv()
            assert rep == {"status": "ok"}

            await alice_backend_sock.send({"cmd": "event_listen"})

            await bob_backend_sock.send(
                {
                    "cmd": "message_new",
                    "recipient": alice.user_id,
                    "body": to_jsonb64(b"Hello from Bob !"),
                }
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
                    {"body": to_jsonb64(b"Hello from Bob !"), "sender_id": "bob@dev1", "count": 1}
                ],
            }
