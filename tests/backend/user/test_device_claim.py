import pytest
from unittest.mock import ANY
from pendulum import Pendulum

from parsec.types import DeviceID
from parsec.api.user import device_claim_rep_schema
from parsec.backend.user import Device, DeviceInvitation, INVITATION_VALIDITY, PEER_EVENT_MAX_WAIT

from tests.common import freeze_time


@pytest.fixture
async def alice_nd_invitation(backend, alice):
    invitation = DeviceInvitation(
        DeviceID(f"{alice.user_id}@new_device"), alice.device_id, Pendulum(2000, 1, 2)
    )
    await backend.user.create_device_invitation(invitation)
    return invitation


@pytest.mark.trio
async def test_device_get_invitation_creator_too_late(anonymous_backend_sock, alice_nd_invitation):
    with freeze_time(alice_nd_invitation.created_on.add(seconds=INVITATION_VALIDITY + 1)):
        await anonymous_backend_sock.send(
            {
                "cmd": "device_get_invitation_creator",
                "invited_device_id": alice_nd_invitation.device_id,
            }
        )
        rep = await anonymous_backend_sock.recv()
    assert rep == {"status": "not_found"}


@pytest.mark.trio
async def test_device_get_invitation_creator_unknown(anonymous_backend_sock):
    await anonymous_backend_sock.send(
        {"cmd": "device_get_invitation_creator", "invited_device_id": "zack@foo"}
    )
    rep = await anonymous_backend_sock.recv()
    assert rep == {"status": "not_found"}


@pytest.mark.trio
async def test_device_get_invitation_creator_ok(anonymous_backend_sock, alice_nd_invitation):
    with freeze_time(alice_nd_invitation.created_on):
        await anonymous_backend_sock.send(
            {
                "cmd": "device_get_invitation_creator",
                "invited_device_id": alice_nd_invitation.device_id,
            }
        )
        rep = await anonymous_backend_sock.recv()
    assert rep == {
        "status": "ok",
        "device_id": alice_nd_invitation.creator,
        "created_on": "2000-01-01T00:00:00+00:00",
        "certified_device": ANY,
        "device_certifier": None,
        "revocated_on": None,
        "certified_revocation": None,
        "revocation_certifier": None,
    }


# TODO: device_get_invitation_creator with a creator not certified by root


@pytest.mark.trio
async def test_device_claim_ok(backend, anonymous_backend_sock, alice_nd_invitation):
    with freeze_time(alice_nd_invitation.created_on):
        await anonymous_backend_sock.send(
            {
                "cmd": "device_claim",
                "invited_device_id": alice_nd_invitation.device_id,
                "encrypted_claim": b"<foo>",
            }
        )

        await backend.event_bus.spy.wait("event.connected", kwargs={"event_name": "device.created"})
        backend.event_bus.send("device.created", device_id="dummy@foo", encrypted_answer=b"<dummy>")
        backend.event_bus.send(
            "device.created", device_id=alice_nd_invitation.device_id, encrypted_answer=b"<good>"
        )

        rep = await anonymous_backend_sock.recv()
    cooked_rep, errors = device_claim_rep_schema.load(rep)
    assert not errors
    assert cooked_rep == {"status": "ok", "encrypted_answer": b"<good>"}


@pytest.mark.trio
async def test_device_claim_timeout(
    mock_clock, backend, anonymous_backend_sock, alice_nd_invitation
):
    with freeze_time(alice_nd_invitation.created_on):
        await anonymous_backend_sock.send(
            {
                "cmd": "device_claim",
                "invited_device_id": alice_nd_invitation.device_id,
                "encrypted_claim": b"<foo>",
            }
        )

        await backend.event_bus.spy.wait("event.connected", kwargs={"event_name": "device.created"})
        mock_clock.jump(PEER_EVENT_MAX_WAIT + 1)

        rep = await anonymous_backend_sock.recv()
    assert rep == {
        "status": "timeout",
        "reason": "Timeout while waiting for invitation creator to answer.",
    }


@pytest.mark.trio
async def test_device_claim_denied(backend, anonymous_backend_sock, alice_nd_invitation):
    with freeze_time(alice_nd_invitation.created_on):
        await anonymous_backend_sock.send(
            {
                "cmd": "device_claim",
                "invited_device_id": alice_nd_invitation.device_id,
                "encrypted_claim": b"<foo>",
            }
        )

        await backend.event_bus.spy.wait(
            "event.connected", kwargs={"event_name": "device.invitation.cancelled"}
        )
        backend.event_bus.send("device.created", device_id="dummy")
        backend.event_bus.send(
            "device.invitation.cancelled", device_id=alice_nd_invitation.device_id
        )

        rep = await anonymous_backend_sock.recv()
    assert rep == {"status": "denied", "reason": "Invitation creator rejected us."}


@pytest.mark.trio
async def test_device_claim_unknown(anonymous_backend_sock):
    await anonymous_backend_sock.send(
        {"cmd": "device_claim", "invited_device_id": "dummy@foo", "encrypted_claim": b"<foo>"}
    )
    rep = await anonymous_backend_sock.recv()
    assert rep == {"status": "not_found"}


@pytest.mark.trio
async def test_device_claim_already_exists(
    mock_clock, backend, anonymous_backend_sock, alice, alice_nd_invitation
):
    await backend.user.create_device(
        Device(
            device_id=alice_nd_invitation.device_id,
            certified_device=b"<foo>",
            device_certifier=alice.device_id,
        )
    )

    with freeze_time(alice_nd_invitation.created_on):
        await anonymous_backend_sock.send(
            {
                "cmd": "device_claim",
                "invited_device_id": alice_nd_invitation.device_id,
                "encrypted_claim": b"<foo>",
            }
        )
        rep = await anonymous_backend_sock.recv()
    assert rep == {"status": "not_found"}
