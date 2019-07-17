# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from pendulum import Pendulum

from parsec.api.protocole import device_cancel_invitation_serializer
from parsec.backend.user import DeviceInvitation
from parsec.types import DeviceID
from tests.common import freeze_time


@pytest.fixture
async def alice_nd_invitation(backend, alice):
    invitation = DeviceInvitation(
        DeviceID(f"{alice.user_id}@new_device"), alice.device_id, Pendulum(2000, 1, 2)
    )
    await backend.user.create_device_invitation(alice.organization_id, invitation)
    return invitation


async def device_cancel_invitation(sock, **kwargs):
    await sock.send(
        device_cancel_invitation_serializer.req_dumps({"cmd": "device_cancel_invitation", **kwargs})
    )
    raw_rep = await sock.recv()
    rep = device_cancel_invitation_serializer.rep_loads(raw_rep)
    return rep


@pytest.mark.trio
async def test_device_cancel_invitation_ok(alice_backend_sock, alice_nd_invitation):
    with freeze_time(alice_nd_invitation.created_on):
        rep = await device_cancel_invitation(
            alice_backend_sock, invited_device_name=alice_nd_invitation.device_id.device_name
        )

    assert rep == {"status": "ok"}


@pytest.mark.trio
async def test_device_cancel_invitation_unknown(alice_backend_sock, alice):
    rep = await device_cancel_invitation(alice_backend_sock, invited_device_name="foo")
    assert rep == {"status": "ok"}


@pytest.mark.trio
async def test_device_cancel_invitation_other_organization(
    sock_from_other_organization_factory, alice, backend, alice_nd_invitation
):
    # Organizations should be isolated
    async with sock_from_other_organization_factory(backend, mimick=alice.device_id) as sock:
        rep = await device_cancel_invitation(
            sock, invited_device_name=alice_nd_invitation.device_id.device_name
        )
        # Cancel returns even if no invitation was found
        assert rep == {"status": "ok"}

    # Make sure the invitation still works
    invitation = await backend.user.get_device_invitation(
        alice.organization_id, alice_nd_invitation.device_id
    )
    assert invitation == alice_nd_invitation
