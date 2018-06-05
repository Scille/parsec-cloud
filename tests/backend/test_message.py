import pytest

from parsec.utils import to_jsonb64

from tests.common import connect_backend


@pytest.mark.trio
async def test_message_from_bob_to_alice(backend, alice, bob):
    async with connect_backend(backend, auth_as=alice) as alice_sock, connect_backend(
        backend, auth_as=bob
    ) as bob_sock:

        await alice_sock.send({"cmd": "event_subscribe", "event": "message_arrived"})
        rep = await alice_sock.recv()
        assert rep == {"status": "ok"}

        await alice_sock.send({"cmd": "event_listen"})

        await bob_sock.send(
            {
                "cmd": "message_new",
                "recipient": alice.user_id,
                "body": to_jsonb64(b"Hello from Bob !"),
            }
        )
        rep = await bob_sock.recv()
        assert rep == {"status": "ok"}

        rep = await alice_sock.recv()
        assert rep == {"status": "ok", "event": "message_arrived", "subject": alice.user_id}

        await alice_sock.send({"cmd": "message_get"})
        rep = await alice_sock.recv()
        assert rep == {
            "status": "ok",
            "messages": [
                {"body": to_jsonb64(b"Hello from Bob !"), "sender_id": "bob@test", "count": 1}
            ],
        }
