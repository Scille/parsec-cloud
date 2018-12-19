import pytest
from async_generator import asynccontextmanager

from parsec.backend.user import PEER_EVENT_MAX_WAIT
from parsec.api.protocole import user_invite_serializer


@asynccontextmanager
async def user_invite(sock, **kwargs):
    reps = []
    await sock.send(user_invite_serializer.req_dump({"cmd": "user_invite", **kwargs}))
    yield reps
    raw_rep = await sock.recv()
    rep = user_invite_serializer.rep_load(raw_rep)
    reps.append(rep)


@pytest.mark.trio
async def test_user_invite(backend, alice_backend_sock, alice, mallory):
    async with user_invite(alice_backend_sock, user_id=mallory.user_id) as prep:

        # Waiting for user.claimed event
        await backend.event_bus.spy.wait("event.connected", kwargs={"event_name": "user.claimed"})

        backend.event_bus.send("user.claimed", user_id="foo", encrypted_claim=b"<dummy>")
        backend.event_bus.send("user.claimed", user_id=mallory.user_id, encrypted_claim=b"<good>")

    assert prep[0] == {"status": "ok", "encrypted_claim": b"<good>"}


@pytest.mark.trio
async def test_user_invite_already_exists(alice_backend_sock, bob):
    async with user_invite(alice_backend_sock, user_id=bob.user_id) as prep:
        pass
    assert prep[0] == {"status": "already_exists", "reason": "User `bob` already exists"}


@pytest.mark.trio
async def test_user_invite_timeout(mock_clock, backend, alice_backend_sock, alice, mallory):
    async with user_invite(alice_backend_sock, user_id=mallory.user_id) as prep:

        await backend.event_bus.spy.wait("event.connected", kwargs={"event_name": "user.claimed"})
        mock_clock.jump(PEER_EVENT_MAX_WAIT + 1)

    assert prep[0] == {
        "status": "timeout",
        "reason": "Timeout while waiting for new user to be claimed.",
    }


@pytest.mark.trio
async def test_concurrent_user_invite(
    backend, alice_backend_sock, bob_backend_sock, alice, bob, mallory
):
    async with user_invite(alice_backend_sock, user_id=mallory.user_id) as prep1:

        await backend.event_bus.spy.wait("event.connected", kwargs={"event_name": "user.claimed"})
        async with user_invite(bob_backend_sock, user_id=mallory.user_id) as prep2:

            backend.event_bus.spy.clear()
            await backend.event_bus.spy.wait(
                "event.connected", kwargs={"event_name": "user.claimed"}
            )

            backend.event_bus.send(
                "user.claimed", user_id=mallory.user_id, encrypted_claim=b"<good>"
            )

    assert prep1[0] == {"status": "ok", "encrypted_claim": b"<good>"}
    assert prep2[0] == {"status": "ok", "encrypted_claim": b"<good>"}
