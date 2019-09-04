# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from pendulum import Pendulum
import trio

from parsec.backend.user import UserInvitation
from parsec.api.protocol import user_cancel_invitation_serializer, user_claim_serializer

from tests.common import freeze_time


@pytest.fixture
async def mallory_invitation(backend, alice, mallory):
    invitation = UserInvitation(mallory.user_id, alice.device_id, Pendulum(2000, 1, 2))
    await backend.user.create_user_invitation(alice.organization_id, invitation)
    return invitation


async def user_cancel_invitation(sock, **kwargs):
    await sock.send(
        user_cancel_invitation_serializer.req_dumps({"cmd": "user_cancel_invitation", **kwargs})
    )
    raw_rep = await sock.recv()
    rep = user_cancel_invitation_serializer.rep_loads(raw_rep)
    return rep


async def user_claim_cancelled_invitation(sock, **kwargs):
    await sock.send(user_claim_serializer.req_dumps({"cmd": "user_claim", **kwargs}))
    with trio.fail_after(1):
        raw_rep = await sock.recv()
    return user_claim_serializer.rep_loads(raw_rep)


@pytest.mark.trio
async def test_user_cancel_invitation_ok(
    alice_backend_sock, mallory_invitation, anonymous_backend_sock
):
    rep = await user_cancel_invitation(alice_backend_sock, user_id=mallory_invitation.user_id)
    assert rep == {"status": "ok"}

    # Try to use the cancelled invitation
    with freeze_time(mallory_invitation.created_on):
        rep = await user_claim_cancelled_invitation(
            anonymous_backend_sock,
            invited_user_id=mallory_invitation.user_id,
            encrypted_claim=b"<foo>",
        )
        assert rep == {"status": "not_found"}


@pytest.mark.trio
async def test_user_cancel_invitation_unknown(alice_backend_sock, mallory):
    rep = await user_cancel_invitation(alice_backend_sock, user_id=mallory.user_id)
    assert rep == {"status": "ok"}


@pytest.mark.trio
async def test_user_cancel_invitation_other_organization(
    sock_from_other_organization_factory, alice, backend, mallory_invitation
):
    # Organizations should be isolated
    async with sock_from_other_organization_factory(
        backend, mimick=alice.device_id, is_admin=True
    ) as sock:
        rep = await user_cancel_invitation(sock, user_id=mallory_invitation.user_id)
        # Cancel returns even if no invitation was found
        assert rep == {"status": "ok"}

    # Make sure the invitation still works
    invitation = await backend.user.get_user_invitation(
        alice.organization_id, mallory_invitation.user_id
    )
    assert invitation == mallory_invitation


@pytest.mark.trio
async def test_user_cancel_invitation_ok_author_not_admin(
    bob_backend_sock, mallory_invitation, anonymous_backend_sock
):
    rep = await user_cancel_invitation(bob_backend_sock, user_id=mallory_invitation.user_id)
    assert rep == {"status": "not_allowed", "reason": "User `bob` is not admin"}
