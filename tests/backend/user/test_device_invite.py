import pytest

from parsec.backend.user import PEER_EVENT_MAX_WAIT
from parsec.api.protocole import device_invite_serializer


@pytest.fixture
def alice_nd_id(alice):
    return f"{alice.user_id}@new_device"


@pytest.mark.trio
async def test_device_invite(backend, alice_backend_sock, alice, alice_nd_id):
    await alice_backend_sock.send(
        device_invite_serializer.req_dump({"cmd": "device_invite", "device_id": alice_nd_id})
    )

    # Waiting for device.claimed event
    await backend.event_bus.spy.wait("event.connected", kwargs={"event_name": "device.claimed"})

    backend.event_bus.send("device.claimed", device_id="foo", encrypted_claim=b"<dummy>")
    backend.event_bus.send("device.claimed", device_id=alice_nd_id, encrypted_claim=b"<good>")

    raw_rep = await alice_backend_sock.recv()
    rep = device_invite_serializer.rep_load(raw_rep)
    assert rep == {"status": "ok", "encrypted_claim": b"<good>"}


@pytest.mark.trio
async def test_device_invite_already_exists(alice_backend_sock, alice):
    await alice_backend_sock.send(
        device_invite_serializer.req_dump({"cmd": "device_invite", "device_id": alice.device_id})
    )
    raw_rep = await alice_backend_sock.recv()
    rep = device_invite_serializer.rep_load(raw_rep)
    assert rep == {
        "status": "already_exists",
        "reason": f"Device `{alice.device_id}` already exists",
    }


@pytest.mark.trio
async def test_device_invite_bad_user_id(backend, alice_backend_sock, alice, bob):
    await alice_backend_sock.send(
        device_invite_serializer.req_dump(
            {"cmd": "device_invite", "device_id": f"{bob.user_id}@foo"}
        )
    )
    raw_rep = await alice_backend_sock.recv()
    rep = device_invite_serializer.rep_load(raw_rep)
    assert rep == {"status": "bad_user_id", "reason": "Device must be handled by it own user."}


@pytest.mark.trio
async def test_device_invite_unknown_user(alice_backend_sock, alice):
    await alice_backend_sock.send({"cmd": "device_invite", "device_id": "zack@foo"})
    raw_req = await alice_backend_sock.recv()
    rep = device_invite_serializer.rep_load(raw_req)
    assert rep == {"status": "bad_user_id", "reason": "Device must be handled by it own user."}


@pytest.mark.trio
async def test_device_invite_timeout(mock_clock, backend, alice_backend_sock, alice, alice_nd_id):
    await alice_backend_sock.send({"cmd": "device_invite", "device_id": alice_nd_id})

    await backend.event_bus.spy.wait("event.connected", kwargs={"event_name": "device.claimed"})
    mock_clock.jump(PEER_EVENT_MAX_WAIT + 1)

    raw_req = await alice_backend_sock.recv()
    rep = device_invite_serializer.rep_load(raw_req)
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

    raw_req = await alice_backend_sock.recv()
    rep = device_invite_serializer.rep_load(raw_req)
    assert rep == {"status": "ok", "encrypted_claim": b"<good>"}

    raw_req = await alice2_backend_sock.recv()
    rep = device_invite_serializer.rep_load(raw_req)
    assert rep == {"status": "ok", "encrypted_claim": b"<good>"}
