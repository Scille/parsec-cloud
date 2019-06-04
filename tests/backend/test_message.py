# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
import trio
from pendulum import Pendulum, now as pendulum_now

from parsec.api.protocole import message_send_serializer, message_get_serializer

from tests.common import freeze_time
from tests.backend.test_events import events_subscribe, events_listen, events_listen_nowait


async def message_send(sock, recipient, body, timestamp=None, check_rep=True):
    timestamp = timestamp or pendulum_now()
    await sock.send(
        message_send_serializer.req_dumps(
            {"cmd": "message_send", "recipient": recipient, "timestamp": timestamp, "body": body}
        )
    )
    raw_rep = await sock.recv()
    rep = message_send_serializer.rep_loads(raw_rep)
    if check_rep:
        assert rep["status"] == "ok"
    return rep


async def message_get(sock, offset=0):
    await sock.send(message_get_serializer.req_dumps({"cmd": "message_get", "offset": offset}))
    raw_rep = await sock.recv()
    return message_get_serializer.rep_loads(raw_rep)


@pytest.mark.trio
async def test_message_from_bob_to_alice(alice, bob, alice_backend_sock, bob_backend_sock):
    await events_subscribe(alice_backend_sock)
    d1 = Pendulum(2000, 1, 1)
    async with events_listen(alice_backend_sock) as listen:
        with freeze_time(d1):
            await message_send(bob_backend_sock, alice.user_id, b"Hello from Bob !")

    assert listen.rep == {"status": "ok", "event": "message.received", "index": 1}

    rep = await message_get(alice_backend_sock)
    assert rep == {
        "status": "ok",
        "messages": [
            {"body": b"Hello from Bob !", "sender": bob.device_id, "timestamp": d1, "count": 1}
        ],
    }


@pytest.mark.trio
async def test_message_send_bad_timestamp(alice, bob_backend_sock):
    d1 = Pendulum(2000, 1, 1)
    with freeze_time(d1):
        d2 = d1.add(seconds=3600)
        rep = await message_send(
            bob_backend_sock, alice.user_id, b"Hello from Bob !", timestamp=d2, check_rep=False
        )
    assert rep == {"status": "bad_timestamp", "reason": "Timestamp is out of date."}


@pytest.mark.trio
async def test_message_get_with_offset(alice, bob, alice_backend_sock, bob_backend_sock):
    d1 = Pendulum(2000, 1, 1)
    d2 = Pendulum(2000, 1, 2)
    with freeze_time(d1):
        await message_send(bob_backend_sock, alice.user_id, b"1")
        await message_send(bob_backend_sock, alice.user_id, b"2")
    with freeze_time(d2):
        await message_send(bob_backend_sock, alice.user_id, b"3")

    rep = await message_get(alice_backend_sock, 1)
    assert rep == {
        "status": "ok",
        "messages": [
            {"body": b"2", "sender": bob.device_id, "timestamp": d1, "count": 2},
            {"body": b"3", "sender": bob.device_id, "timestamp": d2, "count": 3},
        ],
    }


@pytest.mark.trio
@pytest.mark.postgresql
async def test_message_from_bob_to_alice_multi_backends(
    postgresql_url, alice, bob, backend_factory, backend_sock_factory
):
    d1 = Pendulum(2000, 1, 1)
    async with backend_factory(
        config={"blockstore_types": ["POSTGRESQL"], "db_url": postgresql_url}
    ) as backend_1, backend_factory(
        populated=False, config={"blockstore_types": ["POSTGRESQL"], "db_url": postgresql_url}
    ) as backend_2:

        async with backend_sock_factory(
            backend_1, alice
        ) as alice_backend_sock, backend_sock_factory(backend_2, bob) as bob_backend_sock:

            await events_subscribe(alice_backend_sock)
            async with events_listen(alice_backend_sock) as listen:
                with freeze_time(d1):
                    await message_send(bob_backend_sock, alice.user_id, b"Hello from Bob !")

            assert listen.rep == {"status": "ok", "event": "message.received", "index": 1}

            rep = await message_get(alice_backend_sock)
            assert rep == {
                "status": "ok",
                "messages": [
                    {
                        "body": b"Hello from Bob !",
                        "sender": bob.device_id,
                        "timestamp": d1,
                        "count": 1,
                    }
                ],
            }


@pytest.mark.trio
async def test_message_received_event(backend, alice_backend_sock, alice, bob):
    d1 = Pendulum(2000, 1, 1)
    await events_subscribe(alice_backend_sock)

    # Good message
    await backend.message.send(
        bob.organization_id, bob.device_id, alice.user_id, d1, b"Hello from bob to alice"
    )
    await backend.message.send(
        bob.organization_id, bob.device_id, alice.user_id, d1, b"Goodbye from bob to alice"
    )

    with trio.fail_after(1):
        # No guarantees those events occur before the commands' return
        await backend.event_bus.spy.wait_multiple(["message.received", "message.received"])

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
    await backend.message.send(
        alice.organization_id, alice.device_id, alice.user_id, d1, b"Hello to myself"
    )
    rep = await events_listen_nowait(alice_backend_sock)
    assert rep == {"status": "no_events"}
