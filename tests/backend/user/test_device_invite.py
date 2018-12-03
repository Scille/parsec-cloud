import pytest

from parsec.api.user import device_invite_rep_schema
from parsec.backend.user import PEER_EVENT_MAX_WAIT


@pytest.fixture
def alice_nd_id(alice):
    return f"{alice.user_id}@new_device"


@pytest.mark.trio
async def test_device_invite(backend, alice_backend_sock, alice, alice_nd_id):
    await alice_backend_sock.send({"cmd": "device_invite", "device_id": alice_nd_id})

    # Waiting for device.claimed event
    await backend.event_bus.spy.wait("event.connected", kwargs={"event_name": "device.claimed"})

    backend.event_bus.send("device.claimed", device_id="foo", encrypted_claim=b"<dummy>")
    backend.event_bus.send("device.claimed", device_id=alice_nd_id, encrypted_claim=b"<good>")

    rep = await alice_backend_sock.recv()
    cooked_rep = device_invite_rep_schema.load(rep).data
    assert cooked_rep == {"status": "ok", "encrypted_claim": b"<good>"}


@pytest.mark.trio
async def test_device_invite_already_exists(alice_backend_sock, alice):
    await alice_backend_sock.send({"cmd": "device_invite", "device_id": alice.device_id})
    rep = await alice_backend_sock.recv()
    assert rep == {
        "status": "already_exists",
        "reason": f"Device `{alice.device_id}` already exists",
    }


@pytest.mark.trio
async def test_device_invite_bad_user_id(backend, alice_backend_sock, alice, bob):
    await alice_backend_sock.send({"cmd": "device_invite", "device_id": f"{bob.user_id}@foo"})
    rep = await alice_backend_sock.recv()
    assert rep == {"status": "bad_user_id", "reason": "Device must be handled by it own user."}


@pytest.mark.trio
async def test_device_invite_unknown_user(alice_backend_sock, alice):
    await alice_backend_sock.send({"cmd": "device_invite", "device_id": "zack@foo"})
    rep = await alice_backend_sock.recv()
    assert rep == {"status": "bad_user_id", "reason": "Device must be handled by it own user."}


@pytest.mark.trio
async def test_device_invite_timeout(mock_clock, backend, alice_backend_sock, alice, alice_nd_id):
    await alice_backend_sock.send({"cmd": "device_invite", "device_id": alice_nd_id})

    await backend.event_bus.spy.wait("event.connected", kwargs={"event_name": "device.claimed"})
    mock_clock.jump(PEER_EVENT_MAX_WAIT + 1)

    rep = await alice_backend_sock.recv()
    assert rep == {
        "status": "timeout",
        "reason": "Timeout while waiting for new device to be claimed.",
    }


@pytest.mark.trio
async def test_concurrent_device_invite(
    backend, alice_backend_sock, alice2_backend_sock, alice, alice_nd_id
):
    await alice_backend_sock.send({"cmd": "device_invite", "device_id": alice_nd_id})
    await backend.event_bus.spy.wait("event.connected", kwargs={"event_name": "device.claimed"})

    await alice2_backend_sock.send({"cmd": "device_invite", "device_id": alice_nd_id})
    backend.event_bus.spy.clear()
    await backend.event_bus.spy.wait("event.connected", kwargs={"event_name": "device.claimed"})

    backend.event_bus.send("device.claimed", device_id=alice_nd_id, encrypted_claim=b"<good>")

    alice_rep = await alice_backend_sock.recv()
    cooked_alice_rep = device_invite_rep_schema.load(alice_rep).data
    assert cooked_alice_rep == {"status": "ok", "encrypted_claim": b"<good>"}

    bob_rep = await alice2_backend_sock.recv()
    cooked_bob_rep = device_invite_rep_schema.load(bob_rep).data
    assert cooked_bob_rep == {"status": "ok", "encrypted_claim": b"<good>"}
