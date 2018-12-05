import pytest

from parsec.backend.user import PEER_EVENT_MAX_WAIT
from parsec.api.protocole import user_invite_serializer


@pytest.mark.trio
async def test_user_invite(backend, alice_backend_sock, alice, mallory):
    await alice_backend_sock.send(
        user_invite_serializer.req_dump({"cmd": "user_invite", "user_id": mallory.user_id})
    )

    # Waiting for user.claimed event
    await backend.event_bus.spy.wait("event.connected", kwargs={"event_name": "user.claimed"})

    backend.event_bus.send("user.claimed", user_id="foo", encrypted_claim=b"<dummy>")
    backend.event_bus.send("user.claimed", user_id=mallory.user_id, encrypted_claim=b"<good>")

    raw_rep = await alice_backend_sock.recv()
    rep = user_invite_serializer.rep_load(raw_rep)
    assert rep == {"status": "ok", "encrypted_claim": b"<good>"}


@pytest.mark.trio
async def test_user_invite_already_exists(alice_backend_sock, bob):
    await alice_backend_sock.send(
        user_invite_serializer.req_dump({"cmd": "user_invite", "user_id": bob.user_id})
    )
    raw_rep = await alice_backend_sock.recv()
    rep = user_invite_serializer.rep_load(raw_rep)
    assert rep == {"status": "already_exists", "reason": "User `bob` already exists"}


@pytest.mark.trio
async def test_user_invite_timeout(mock_clock, backend, alice_backend_sock, alice, mallory):
    await alice_backend_sock.send(
        user_invite_serializer.req_dump({"cmd": "user_invite", "user_id": mallory.user_id})
    )

    await backend.event_bus.spy.wait("event.connected", kwargs={"event_name": "user.claimed"})
    mock_clock.jump(PEER_EVENT_MAX_WAIT + 1)

    raw_rep = await alice_backend_sock.recv()
    rep = user_invite_serializer.rep_load(raw_rep)
    assert rep == {
        "status": "timeout",
        "reason": "Timeout while waiting for new user to be claimed.",
    }


@pytest.mark.trio
async def test_concurrent_user_invite(
    backend, alice_backend_sock, bob_backend_sock, alice, bob, mallory
):
    await alice_backend_sock.send(
        user_invite_serializer.req_dump({"cmd": "user_invite", "user_id": mallory.user_id})
    )
    await backend.event_bus.spy.wait("event.connected", kwargs={"event_name": "user.claimed"})

    await bob_backend_sock.send(
        user_invite_serializer.req_dump({"cmd": "user_invite", "user_id": mallory.user_id})
    )
    backend.event_bus.spy.clear()
    await backend.event_bus.spy.wait("event.connected", kwargs={"event_name": "user.claimed"})

    backend.event_bus.send("user.claimed", user_id=mallory.user_id, encrypted_claim=b"<good>")

    raw_rep = await alice_backend_sock.recv()
    rep = user_invite_serializer.rep_load(raw_rep)
    assert rep == {"status": "ok", "encrypted_claim": b"<good>"}

    raw_rep = await bob_backend_sock.recv()
    rep = user_invite_serializer.rep_load(raw_rep)
    assert rep == {"status": "ok", "encrypted_claim": b"<good>"}
