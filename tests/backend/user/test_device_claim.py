# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from pendulum import Pendulum
from async_generator import asynccontextmanager

from parsec.types import DeviceID
from parsec.api.protocole import device_get_invitation_creator_serializer, device_claim_serializer
from parsec.backend.user import Device, DeviceInvitation, PEER_EVENT_MAX_WAIT

from tests.common import freeze_time


@pytest.fixture
async def alice_nd_invitation(backend, alice):
    invitation = DeviceInvitation(
        DeviceID(f"{alice.user_id}@new_device"), alice.device_id, Pendulum(2000, 1, 2)
    )
    await backend.user.create_device_invitation(alice.organization_id, invitation)
    return invitation


async def device_get_invitation_creator(sock, **kwargs):
    await sock.send(
        device_get_invitation_creator_serializer.req_dumps(
            {"cmd": "device_get_invitation_creator", **kwargs}
        )
    )
    raw_rep = await sock.recv()
    rep = device_get_invitation_creator_serializer.rep_loads(raw_rep)
    return rep


@asynccontextmanager
async def device_claim(sock, **kwargs):
    reps = []
    await sock.send(device_claim_serializer.req_dumps({"cmd": "device_claim", **kwargs}))
    yield reps
    raw_rep = await sock.recv()
    rep = device_claim_serializer.rep_loads(raw_rep)
    reps.append(rep)


@pytest.mark.trio
async def test_device_claim_ok(backend, anonymous_backend_sock, alice, alice_nd_invitation):
    with freeze_time(alice_nd_invitation.created_on):
        async with device_claim(
            anonymous_backend_sock,
            invited_device_id=alice_nd_invitation.device_id,
            encrypted_claim=b"<foo>",
        ) as prep:

            await backend.event_bus.spy.wait(
                "event.connected", kwargs={"event_name": "device.created"}
            )
            backend.event_bus.send(
                "device.created",
                organization_id=alice.organization_id,
                device_id="dummy@foo",
                encrypted_answer=b"<dummy>",
            )
            backend.event_bus.send(
                "device.created",
                organization_id=alice.organization_id,
                device_id=alice_nd_invitation.device_id,
                encrypted_answer=b"<good>",
            )
    assert prep[0] == {"status": "ok", "encrypted_answer": b"<good>"}


@pytest.mark.trio
async def test_device_claim_timeout(
    mock_clock, backend, anonymous_backend_sock, alice_nd_invitation
):
    with freeze_time(alice_nd_invitation.created_on):
        async with device_claim(
            anonymous_backend_sock,
            invited_device_id=alice_nd_invitation.device_id,
            encrypted_claim=b"<foo>",
        ) as prep:

            await backend.event_bus.spy.wait(
                "event.connected", kwargs={"event_name": "device.created"}
            )
            mock_clock.jump(PEER_EVENT_MAX_WAIT + 1)

    assert prep[0] == {
        "status": "timeout",
        "reason": "Timeout while waiting for invitation creator to answer.",
    }


@pytest.mark.trio
async def test_device_claim_denied(backend, anonymous_backend_sock, alice, alice_nd_invitation):
    with freeze_time(alice_nd_invitation.created_on):
        async with device_claim(
            anonymous_backend_sock,
            invited_device_id=alice_nd_invitation.device_id,
            encrypted_claim=b"<foo>",
        ) as prep:

            await backend.event_bus.spy.wait(
                "event.connected", kwargs={"event_name": "device.invitation.cancelled"}
            )
            backend.event_bus.send(
                "device.created", organization_id=alice.organization_id, device_id="dummy"
            )
            backend.event_bus.send(
                "device.invitation.cancelled",
                organization_id=alice.organization_id,
                device_id=alice_nd_invitation.device_id,
            )

    assert prep[0] == {"status": "denied", "reason": "Invitation creator rejected us."}


@pytest.mark.trio
async def test_device_claim_unknown(anonymous_backend_sock, mallory):
    async with device_claim(
        anonymous_backend_sock, invited_device_id=mallory.device_id, encrypted_claim=b"<foo>"
    ) as prep:

        pass

    assert prep[0] == {"status": "not_found"}


@pytest.mark.trio
async def test_device_claim_already_exists(
    mock_clock, backend, anonymous_backend_sock, alice, alice_nd_invitation
):
    await backend.user.create_device(
        alice.organization_id,
        Device(
            device_id=alice_nd_invitation.device_id,
            device_certificate=b"<foo>",
            device_certifier=alice.device_id,
        ),
    )

    with freeze_time(alice_nd_invitation.created_on):
        async with device_claim(
            anonymous_backend_sock,
            invited_device_id=alice_nd_invitation.device_id,
            encrypted_claim=b"<foo>",
        ) as prep:

            pass

    assert prep[0] == {"status": "not_found"}


@pytest.mark.trio
async def test_device_claim_other_organization(
    backend, sock_from_other_organization_factory, alice, alice_nd_invitation
):
    # Organizations should be isolated
    async with sock_from_other_organization_factory(backend, anonymous=True) as sock:
        async with device_claim(
            sock, invited_device_id=alice_nd_invitation.device_id, encrypted_claim=b"<foo>"
        ) as prep:
            pass
    assert prep[0] == {"status": "not_found"}
