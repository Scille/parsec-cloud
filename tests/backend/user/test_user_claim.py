import pytest
from unittest.mock import ANY
from pendulum import Pendulum

from parsec.trustchain import certify_user_invitation
from parsec.backend.user import UserInvitation, USER_INVITATION_VALIDITY

from tests.common import freeze_time


@pytest.fixture
async def mallory_invitation(backend, alice, mallory):
    now = Pendulum(2000, 1, 2)
    with freeze_time(now):
        certified_invitation = certify_user_invitation(
            alice.device_id, alice.device_signkey, mallory.user_id
        )
    invitation = UserInvitation(mallory.user_id, certified_invitation, alice.device_id, now)
    await backend.user.create_user_invitation(invitation)
    return invitation


@pytest.mark.trio
async def test_user_get_invitation_creator_too_late(anonymous_backend_sock, mallory_invitation):
    with freeze_time(mallory_invitation.created_on.add(seconds=USER_INVITATION_VALIDITY + 1)):
        await anonymous_backend_sock.send(
            {"cmd": "user_get_invitation_creator", "invited_user_id": mallory_invitation.user_id}
        )
        rep = await anonymous_backend_sock.recv()
    assert rep == {"status": "not_found"}


@pytest.mark.trio
async def test_user_get_invitation_creator_unknown(anonymous_backend_sock):
    await anonymous_backend_sock.send(
        {"cmd": "user_get_invitation_creator", "invited_user_id": "zack"}
    )
    rep = await anonymous_backend_sock.recv()
    assert rep == {"status": "not_found"}


@pytest.mark.trio
async def test_user_get_invitation_creator_ok(anonymous_backend_sock, mallory_invitation):
    with freeze_time(mallory_invitation.created_on):
        await anonymous_backend_sock.send(
            {"cmd": "user_get_invitation_creator", "invited_user_id": mallory_invitation.user_id}
        )
        rep = await anonymous_backend_sock.recv()
    assert rep == {
        "status": "ok",
        "device_id": mallory_invitation.user_invitation_certifier,
        "created_on": "2000-01-01T00:00:00+00:00",
        "certified_device": ANY,
        "device_certifier": None,
        "revocated_on": None,
        "certified_revocation": None,
        "revocation_certifier": None,
    }


# TODO: user_get_invitation_creator with a creator not certified by root


@pytest.mark.trio
async def test_user_claim_invitation_ok(
    anonymous_backend_sock, alice_backend_sock, alice, mallory_invitation
):
    mallory_invitation
    with freeze_time(mallory_invitation.created_on):
        await anonymous_backend_sock.send(
            {"cmd": "user_get_invitation_creator", "invited_user_id": mallory_invitation.user_id}
        )
        rep = await anonymous_backend_sock.recv()
    assert rep == {
        "status": "ok",
        "device_id": mallory_invitation.user_invitation_certifier,
        "created_on": "2000-01-01T00:00:00+00:00",
        "certified_device": ANY,
        "device_certifier": None,
        "revocated_on": None,
        "certified_revocation": None,
        "revocation_certifier": None,
    }
