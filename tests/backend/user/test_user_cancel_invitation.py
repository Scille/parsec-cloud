import pytest
from pendulum import Pendulum

from parsec.backend.user import UserInvitation

from tests.common import freeze_time


@pytest.fixture
async def mallory_invitation(backend, alice, mallory):
    invitation = UserInvitation(mallory.user_id, alice.device_id, Pendulum(2000, 1, 2))
    await backend.user.create_user_invitation(invitation)
    return invitation


@pytest.mark.trio
async def test_user_cancel_invitation_ok(alice_backend_sock, mallory_invitation):
    with freeze_time(mallory_invitation.created_on):
        await alice_backend_sock.send(
            {"cmd": "user_cancel_invitation", "user_id": mallory_invitation.user_id}
        )
        rep = await alice_backend_sock.recv()
    assert rep == {"status": "ok"}


@pytest.mark.trio
async def test_user_cancel_invitation_unknown(alice_backend_sock):
    await alice_backend_sock.send({"cmd": "user_cancel_invitation", "user_id": "zack"})
    rep = await alice_backend_sock.recv()
    assert rep == {"status": "ok"}
