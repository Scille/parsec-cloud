import pytest
from async_generator import asynccontextmanager

from parsec.backend.user import PEER_EVENT_MAX_WAIT
from parsec.api.protocole import device_invite_serializer


@pytest.fixture
def alice_nd_id(alice):
    return f"{alice.user_id}@new_device"


@asynccontextmanager
async def device_invite(sock, **kwargs):
    reps = []
    await sock.send(device_invite_serializer.req_dump({"cmd": "device_invite", **kwargs}))
    yield reps
    raw_rep = await sock.recv()
    rep = device_invite_serializer.rep_load(raw_rep)
    reps.append(rep)


@pytest.mark.trio
async def test_device_invite(backend, alice_backend_sock, alice, alice_nd_id):
    async with device_invite(alice_backend_sock, device_id=alice_nd_id) as prep:

        # Waiting for device.claimed event
        await backend.event_bus.spy.wait("event.connected", kwargs={"event_name": "device.claimed"})

        backend.event_bus.send("device.claimed", device_id="foo", encrypted_claim=b"<dummy>")
        backend.event_bus.send("device.claimed", device_id=alice_nd_id, encrypted_claim=b"<good>")

    assert prep[0] == {"status": "ok", "encrypted_claim": b"<good>"}


@pytest.mark.trio
async def test_device_invite_already_exists(alice_backend_sock, alice):
    async with device_invite(alice_backend_sock, device_id=alice.device_id) as prep:
        pass
    assert prep[0] == {
        "status": "already_exists",
        "reason": f"Device `{alice.device_id}` already exists",
    }


@pytest.mark.trio
async def test_device_invite_bad_user_id(backend, alice_backend_sock, alice, bob):
    async with device_invite(alice_backend_sock, device_id=f"{bob.user_id}@foo") as prep:
        pass
    assert prep[0] == {"status": "bad_user_id", "reason": "Device must be handled by it own user."}


@pytest.mark.trio
async def test_device_invite_unknown_user(alice_backend_sock, alice, mallory):
    async with device_invite(alice_backend_sock, device_id=mallory.device_id) as prep:
        pass
    assert prep[0] == {"status": "bad_user_id", "reason": "Device must be handled by it own user."}


@pytest.mark.trio
async def test_device_invite_timeout(mock_clock, backend, alice_backend_sock, alice, alice_nd_id):
    async with device_invite(alice_backend_sock, device_id=alice_nd_id) as prep:
        await backend.event_bus.spy.wait("event.connected", kwargs={"event_name": "device.claimed"})
        mock_clock.jump(PEER_EVENT_MAX_WAIT + 1)
    assert prep[0] == {
        "status": "timeout",
        "reason": "Timeout while waiting for new device to be claimed.",
    }


@pytest.mark.trio
async def test_concurrent_device_invite(
    backend, alice_backend_sock, alice2_backend_sock, alice, alice_nd_id
):
    async with device_invite(alice_backend_sock, device_id=alice_nd_id) as prep:

        await backend.event_bus.spy.wait("event.connected", kwargs={"event_name": "device.claimed"})

        async with device_invite(alice2_backend_sock, device_id=alice_nd_id) as prep2:

            backend.event_bus.spy.clear()
            await backend.event_bus.spy.wait(
                "event.connected", kwargs={"event_name": "device.claimed"}
            )

            backend.event_bus.send(
                "device.claimed", device_id=alice_nd_id, encrypted_claim=b"<good>"
            )

    assert prep[0] == {"status": "ok", "encrypted_claim": b"<good>"}
    assert prep2[0] == {"status": "ok", "encrypted_claim": b"<good>"}
