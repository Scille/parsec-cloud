import pytest
from pendulum import Pendulum

from parsec.types import DeviceID
from parsec.backend.user import DeviceInvitation
from parsec.api.protocole import device_cancel_invitation_serializer

from tests.common import freeze_time


@pytest.fixture
async def alice_nd_invitation(backend, alice):
    invitation = DeviceInvitation(
        DeviceID(f"{alice.user_id}@new_device"), alice.device_id, Pendulum(2000, 1, 2)
    )
    await backend.user.create_device_invitation(invitation)
    return invitation


async def device_cancel_invitation(sock, **kwargs):
    await sock.send(
        device_cancel_invitation_serializer.req_dump({"cmd": "device_cancel_invitation", **kwargs})
    )
    raw_rep = await sock.recv()
    rep = device_cancel_invitation_serializer.rep_load(raw_rep)
    return rep


@pytest.mark.trio
async def test_device_cancel_invitation_ok(alice_backend_sock, alice_nd_invitation):
    with freeze_time(alice_nd_invitation.created_on):
        rep = await device_cancel_invitation(
            alice_backend_sock, device_id=alice_nd_invitation.device_id
        )

    assert rep == {"status": "ok"}


@pytest.mark.trio
async def test_device_cancel_invitation_unknown(alice_backend_sock, alice):
    rep = await device_cancel_invitation(alice_backend_sock, device_id=f"{alice.user_id}@foo")
    assert rep == {"status": "ok"}


@pytest.mark.trio
async def test_device_cancel_invitation_bad_user_id(alice_backend_sock, mallory):
    rep = await device_cancel_invitation(alice_backend_sock, device_id=mallory.device_id)
    assert rep == {"status": "bad_user_id", "reason": "Device must be handled by it own user."}
