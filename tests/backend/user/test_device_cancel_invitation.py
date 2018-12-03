import pytest
from pendulum import Pendulum

from parsec.types import DeviceID
from parsec.backend.user import DeviceInvitation

from tests.common import freeze_time


@pytest.fixture
async def alice_nd_invitation(backend, alice):
    invitation = DeviceInvitation(
        DeviceID(f"{alice.user_id}@new_device"), alice.device_id, Pendulum(2000, 1, 2)
    )
    await backend.user.create_device_invitation(invitation)
    return invitation


@pytest.mark.trio
async def test_device_cancel_invitation_ok(alice_backend_sock, alice_nd_invitation):
    with freeze_time(alice_nd_invitation.created_on):
        await alice_backend_sock.send(
            {"cmd": "device_cancel_invitation", "device_id": alice_nd_invitation.device_id}
        )
        rep = await alice_backend_sock.recv()
    assert rep == {"status": "ok"}


@pytest.mark.trio
async def test_device_cancel_invitation_unknown(alice_backend_sock, alice):
    await alice_backend_sock.send(
        {"cmd": "device_cancel_invitation", "device_id": f"{alice.user_id}@foo"}
    )
    rep = await alice_backend_sock.recv()
    assert rep == {"status": "ok"}


@pytest.mark.trio
async def test_device_cancel_invitation_bad_user_id(alice_backend_sock):
    await alice_backend_sock.send({"cmd": "device_cancel_invitation", "device_id": "zack@foo"})
    rep = await alice_backend_sock.recv()
    assert rep == {"status": "bad_user_id", "reason": "Device must be handled by it own user."}
