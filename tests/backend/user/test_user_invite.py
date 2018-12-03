import pytest

from parsec.api.user import user_invite_rep_schema
from parsec.backend.user import PEER_EVENT_MAX_WAIT


@pytest.mark.trio
async def test_user_invite(backend, alice_backend_sock, alice, mallory):
    await alice_backend_sock.send({"cmd": "user_invite", "user_id": mallory.user_id})

    # Waiting for user.claimed event
    await backend.event_bus.spy.wait("event.connected", kwargs={"event_name": "user.claimed"})

    backend.event_bus.send("user.claimed", user_id="foo", encrypted_claim=b"<dummy>")
    backend.event_bus.send("user.claimed", user_id=mallory.user_id, encrypted_claim=b"<good>")

    rep = await alice_backend_sock.recv()
    cooked_rep = user_invite_rep_schema.load(rep).data
    assert cooked_rep == {"status": "ok", "encrypted_claim": b"<good>"}


@pytest.mark.trio
async def test_user_invite_already_exists(alice_backend_sock, bob):
    await alice_backend_sock.send({"cmd": "user_invite", "user_id": bob.user_id})
    rep = await alice_backend_sock.recv()
    assert rep == {"status": "already_exists", "reason": "User `bob` already exists"}


@pytest.mark.trio
async def test_user_invite_timeout(mock_clock, backend, alice_backend_sock, alice, mallory):
    await alice_backend_sock.send({"cmd": "user_invite", "user_id": mallory.user_id})

    await backend.event_bus.spy.wait("event.connected", kwargs={"event_name": "user.claimed"})
    mock_clock.jump(PEER_EVENT_MAX_WAIT + 1)

    rep = await alice_backend_sock.recv()
    assert rep == {
        "status": "timeout",
        "reason": "Timeout while waiting for new user to be claimed.",
    }


@pytest.mark.trio
async def test_concurrent_user_invite(
    backend, alice_backend_sock, bob_backend_sock, alice, bob, mallory
):
    await alice_backend_sock.send({"cmd": "user_invite", "user_id": mallory.user_id})
    await backend.event_bus.spy.wait("event.connected", kwargs={"event_name": "user.claimed"})

    await bob_backend_sock.send({"cmd": "user_invite", "user_id": mallory.user_id})
    backend.event_bus.spy.clear()
    await backend.event_bus.spy.wait("event.connected", kwargs={"event_name": "user.claimed"})

    backend.event_bus.send("user.claimed", user_id=mallory.user_id, encrypted_claim=b"<good>")

    alice_rep = await alice_backend_sock.recv()
    cooked_alice_rep = user_invite_rep_schema.load(alice_rep).data
    assert cooked_alice_rep == {"status": "ok", "encrypted_claim": b"<good>"}

    bob_rep = await bob_backend_sock.recv()
    cooked_bob_rep = user_invite_rep_schema.load(bob_rep).data
    assert cooked_bob_rep == {"status": "ok", "encrypted_claim": b"<good>"}
