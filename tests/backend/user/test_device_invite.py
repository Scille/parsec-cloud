# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
import trio
from async_generator import asynccontextmanager

from parsec.api.protocol import DeviceID, device_invite_serializer
from parsec.backend.user import PEER_EVENT_MAX_WAIT


@pytest.fixture
def alice_nd_id(alice):
    return DeviceID(f"{alice.user_id}@new_device")


@asynccontextmanager
async def device_invite(sock, **kwargs):
    reps = []
    await sock.send(device_invite_serializer.req_dumps({"cmd": "device_invite", **kwargs}))
    yield reps
    raw_rep = await sock.recv()
    rep = device_invite_serializer.rep_loads(raw_rep)
    reps.append(rep)


@pytest.mark.trio
async def test_device_invite(backend, alice_backend_sock, alice, alice_nd_id):
    with backend.event_bus.listen() as spy, trio.fail_after(1):
        async with device_invite(
            alice_backend_sock, invited_device_name=alice_nd_id.device_name
        ) as prep:

            # Waiting for device.claimed event
            await spy.wait("event.connected", {"event_name": "device.claimed"})

            backend.event_bus.send(
                "device.claimed",
                organization_id=alice.organization_id,
                device_id="foo",
                encrypted_claim=b"<dummy>",
            )
            backend.event_bus.send(
                "device.claimed",
                organization_id=alice.organization_id,
                device_id=alice_nd_id,
                encrypted_claim=b"<good>",
            )

        assert prep[0] == {"status": "ok", "encrypted_claim": b"<good>"}


@pytest.mark.trio
async def test_device_invite_already_exists(alice_backend_sock, alice):
    with trio.fail_after(1):
        async with device_invite(alice_backend_sock, invited_device_name=alice.device_name) as prep:
            pass
    assert prep[0] == {
        "status": "already_exists",
        "reason": f"Device `{alice.device_id}` already exists",
    }


@pytest.mark.trio
async def test_device_invite_timeout(mock_clock, backend, alice_backend_sock, alice, alice_nd_id):
    with backend.event_bus.listen() as spy:
        async with device_invite(
            alice_backend_sock, invited_device_name=alice_nd_id.device_name
        ) as prep:
            await spy.wait_with_timeout("event.connected", {"event_name": "device.claimed"})
            mock_clock.jump(PEER_EVENT_MAX_WAIT + 1)

    assert prep[0] == {
        "status": "timeout",
        "reason": "Timeout while waiting for new device to be claimed.",
    }


@pytest.mark.trio
async def test_concurrent_device_invite(
    backend, alice_backend_sock, alice2_backend_sock, alice, alice_nd_id
):
    with backend.event_bus.listen() as spy, trio.fail_after(1):
        async with device_invite(
            alice_backend_sock, invited_device_name=alice_nd_id.device_name
        ) as prep:

            await spy.wait("event.connected", {"event_name": "device.claimed"})
            spy.clear()

            async with device_invite(
                alice2_backend_sock, invited_device_name=alice_nd_id.device_name
            ) as prep2:

                await spy.wait("event.connected", {"event_name": "device.claimed"})

                backend.event_bus.send(
                    "device.claimed",
                    organization_id=alice.organization_id,
                    device_id=alice_nd_id,
                    encrypted_claim=b"<good>",
                )

    assert prep[0] == {"status": "ok", "encrypted_claim": b"<good>"}
    assert prep2[0] == {"status": "ok", "encrypted_claim": b"<good>"}


@pytest.mark.trio
async def test_device_invite_same_name_different_organizations(
    backend, alice_backend_sock, otheralice_backend_sock, alice, otheralice, alice_nd_id
):
    with backend.event_bus.listen() as spy, trio.fail_after(1):
        async with device_invite(
            alice_backend_sock, invited_device_name=alice_nd_id.device_name
        ) as prep:

            await spy.wait("event.connected", {"event_name": "device.claimed"})

            backend.event_bus.send(
                "device.claimed",
                organization_id=otheralice.organization_id,
                device_id=alice_nd_id,
                encrypted_claim=b"<from OtherOrg>",
            )
            await trio.sleep(0)
            backend.event_bus.send(
                "device.claimed",
                organization_id=alice.organization_id,
                device_id=alice_nd_id,
                encrypted_claim=b"<from CoolOrg>",
            )

    assert prep[0] == {"status": "ok", "encrypted_claim": b"<from CoolOrg>"}
