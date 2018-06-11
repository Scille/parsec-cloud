import pytest

from parsec.utils import to_jsonb64

from parsec.backend.exceptions import AlreadyExistsError as UserAlreadyExistsError
from parsec.backend.drivers import postgresql as pg_driver
from tests.common import connect_backend
from tests.conftest import backend, backend_factory, postgresql_url


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


@pytest.mark.trio
async def test_message_from_bob_to_alice_multi_backends(asyncio_loop, default_devices, alice, bob):
    url = postgresql_url()
    await pg_driver.handler.init_db(url, True)
    async with backend_factory(
        **{"blockstore_postgresql": True, "dburl": url}
    ) as backend_1, backend_factory(**{"blockstore_postgresql": True, "dburl": url}) as backend_2:

        for device in default_devices:
            try:
                await backend_1.user.create(
                    author="<backend-fixture>",
                    user_id=device.user_id,
                    broadcast_key=device.user_pubkey.encode(),
                    devices=[(device.device_name, device.device_verifykey.encode())],
                )
            except UserAlreadyExistsError:
                await backend_1.user.create_device(
                    user_id=device.user_id,
                    device_name=device.device_name,
                    verify_key=device.device_verifykey.encode(),
                )

        async with connect_backend(backend_1, auth_as=alice) as alice_sock, connect_backend(
            backend_2, auth_as=bob
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
