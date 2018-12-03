import pytest
from unittest.mock import ANY
from pendulum import Pendulum

from parsec.backend.user import User, UserInvitation, USER_INVITATION_VALIDITY, PEER_EVENT_MAX_WAIT

from tests.common import freeze_time


@pytest.fixture
async def mallory_invitation(backend, alice, mallory):
    invitation = UserInvitation(mallory.user_id, alice.device_id, Pendulum(2000, 1, 2))
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
        "device_id": mallory_invitation.creator,
        "created_on": "2000-01-01T00:00:00+00:00",
        "certified_device": ANY,
        "device_certifier": None,
        "revocated_on": None,
        "certified_revocation": None,
        "revocation_certifier": None,
    }


# TODO: user_get_invitation_creator with a creator not certified by root


@pytest.mark.trio
async def test_user_claim_ok(backend, anonymous_backend_sock, mallory_invitation):
    with freeze_time(mallory_invitation.created_on):
        await anonymous_backend_sock.send(
            {
                "cmd": "user_claim",
                "invited_user_id": mallory_invitation.user_id,
                "encrypted_claim": b"<foo>",
            }
        )

        await backend.event_bus.spy.wait("event.connected", kwargs={"event_name": "user.created"})
        backend.event_bus.send("user.created", user_id="dummy")
        backend.event_bus.send("user.created", user_id=mallory_invitation.user_id)

        rep = await anonymous_backend_sock.recv()
    assert rep == {"status": "ok"}


@pytest.mark.trio
async def test_user_claim_timeout(mock_clock, backend, anonymous_backend_sock, mallory_invitation):
    with freeze_time(mallory_invitation.created_on):
        await anonymous_backend_sock.send(
            {
                "cmd": "user_claim",
                "invited_user_id": mallory_invitation.user_id,
                "encrypted_claim": b"<foo>",
            }
        )

        await backend.event_bus.spy.wait("event.connected", kwargs={"event_name": "user.created"})
        mock_clock.jump(PEER_EVENT_MAX_WAIT + 1)

        rep = await anonymous_backend_sock.recv()
    assert rep == {
        "status": "timeout",
        "reason": "Timeout while waiting for invitation creator to answer.",
    }


@pytest.mark.trio
async def test_user_claim_denied(backend, anonymous_backend_sock, mallory_invitation):
    with freeze_time(mallory_invitation.created_on):
        await anonymous_backend_sock.send(
            {
                "cmd": "user_claim",
                "invited_user_id": mallory_invitation.user_id,
                "encrypted_claim": b"<foo>",
            }
        )

        await backend.event_bus.spy.wait(
            "event.connected", kwargs={"event_name": "user.invitation.cancelled"}
        )
        backend.event_bus.send("user.created", user_id="dummy")
        backend.event_bus.send("user.invitation.cancelled", user_id=mallory_invitation.user_id)

        rep = await anonymous_backend_sock.recv()
    assert rep == {"status": "denied", "reason": "Invitation creator rejected us."}


@pytest.mark.trio
async def test_user_claim_unknown(anonymous_backend_sock):
    await anonymous_backend_sock.send(
        {"cmd": "user_claim", "invited_user_id": "dummy", "encrypted_claim": b"<foo>"}
    )
    rep = await anonymous_backend_sock.recv()
    assert rep == {"status": "not_found"}


@pytest.mark.trio
async def test_user_claim_already_exists(
    mock_clock, backend, anonymous_backend_sock, alice, mallory_invitation
):
    await backend.user.create_user(
        User(
            user_id=mallory_invitation.user_id,
            certified_user=b"<foo>",
            user_certifier=alice.device_id,
        )
    )

    with freeze_time(mallory_invitation.created_on):
        await anonymous_backend_sock.send(
            {
                "cmd": "user_claim",
                "invited_user_id": mallory_invitation.user_id,
                "encrypted_claim": b"<foo>",
            }
        )
        rep = await anonymous_backend_sock.recv()
    assert rep == {"status": "not_found"}
