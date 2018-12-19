import pytest

from parsec.api.protocole import message_send_serializer, message_get_serializer

from tests.backend.test_events import events_subscribe, events_listen, events_listen_nowait


async def message_send(sock, recipient, body):
    await sock.send(
        message_send_serializer.req_dump(
            {"cmd": "message_send", "recipient": recipient, "body": body}
        )
    )
    raw_rep = await sock.recv()
    return message_send_serializer.rep_load(raw_rep)


async def message_get(sock, offset=0):
    await sock.send(message_get_serializer.req_dump({"cmd": "message_get", "offset": offset}))
    raw_rep = await sock.recv()
    return message_get_serializer.rep_load(raw_rep)


@pytest.mark.trio
async def test_message_from_bob_to_alice(alice, bob, alice_backend_sock, bob_backend_sock):
    await events_subscribe(alice_backend_sock, message_received=True)
    async with events_listen(alice_backend_sock) as listen:
        await message_send(bob_backend_sock, alice.user_id, b"Hello from Bob !")

    assert listen.rep == {"status": "ok", "event": "message.received", "index": 1}

    rep = await message_get(alice_backend_sock)
    assert rep == {
        "status": "ok",
        "messages": [{"body": b"Hello from Bob !", "sender": bob.device_id, "count": 1}],
    }


@pytest.mark.trio
async def test_message_get_with_offset(alice, bob, alice_backend_sock, bob_backend_sock):
    await message_send(bob_backend_sock, alice.user_id, b"1")
    await message_send(bob_backend_sock, alice.user_id, b"2")
    await message_send(bob_backend_sock, alice.user_id, b"3")

    rep = await message_get(alice_backend_sock, 1)
    assert rep == {
        "status": "ok",
        "messages": [
            {"body": b"2", "sender": bob.device_id, "count": 2},
            {"body": b"3", "sender": bob.device_id, "count": 3},
        ],
    }


@pytest.mark.trio
@pytest.mark.postgresql
async def test_message_from_bob_to_alice_multi_backends(
    postgresql_url, alice, bob, backend_factory, backend_sock_factory
):
    async with backend_factory(
        config={"blockstore_types": ["POSTGRESQL"], "db_url": postgresql_url}
    ) as backend_1, backend_factory(
        devices=(), config={"blockstore_types": ["POSTGRESQL"], "db_url": postgresql_url}
    ) as backend_2:

        async with backend_sock_factory(
            backend_1, alice
        ) as alice_backend_sock, backend_sock_factory(backend_2, bob) as bob_backend_sock:

            await events_subscribe(alice_backend_sock, message_received=True)
            async with events_listen(alice_backend_sock) as listen:
                await message_send(bob_backend_sock, alice.user_id, b"Hello from Bob !")

            assert listen.rep == {"status": "ok", "event": "message.received", "index": 1}

            rep = await message_get(alice_backend_sock)
            assert rep == {
                "status": "ok",
                "messages": [{"body": b"Hello from Bob !", "sender": "bob@dev1", "count": 1}],
            }


@pytest.mark.trio
async def test_message_received_event(backend, alice_backend_sock, alice, bob):
    await events_subscribe(alice_backend_sock, message_received=True)

    # Good message
    await backend.message.send(bob.device_id, alice.user_id, b"Hello from bob to alice")
    await backend.message.send(bob.device_id, alice.user_id, b"Goodbye from bob to alice")
    reps = [
        await events_listen_nowait(alice_backend_sock),
        await events_listen_nowait(alice_backend_sock),
        await events_listen_nowait(alice_backend_sock),
    ]
    assert reps == [
        {"status": "ok", "event": "message.received", "index": 1},
        {"status": "ok", "event": "message.received", "index": 2},
        {"status": "no_events"},
    ]

    # Message to self is silly... and doesn't trigger event !
    await backend.message.send(alice.device_id, alice.user_id, b"Hello to myself")
    rep = await events_listen_nowait(alice_backend_sock)
    assert rep == {"status": "no_events"}
