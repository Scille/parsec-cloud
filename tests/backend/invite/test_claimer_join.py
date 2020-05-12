# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
import trio
from pendulum import Pendulum

from parsec.api.protocol import InvitationType, InvitationStatus
from parsec.backend.invite import DeviceInvitation

from tests.backend.common import (
    events_subscribe,
    events_listen_wait,
    events_listen_nowait,
    invite_list,
)


@pytest.mark.trio
async def test_claimer_join_and_leave(
    alice, backend, bob_backend_sock, alice_backend_sock, backend_invited_sock_factory
):
    invitation = DeviceInvitation(
        greeter_user_id=alice.user_id,
        greeter_human_handle=alice.human_handle,
        created_on=Pendulum(2000, 1, 2),
    )
    await backend.invite.new(organization_id=alice.organization_id, invitation=invitation)

    await events_subscribe(alice_backend_sock)
    await events_subscribe(bob_backend_sock)

    async with backend_invited_sock_factory(
        backend,
        organization_id=alice.organization_id,
        invitation_type=InvitationType.DEVICE,
        token=invitation.token,
    ):

        # Claimer is ready, this should be notified to greeter

        with trio.fail_after(1):
            rep = await events_listen_wait(alice_backend_sock)
        assert rep == {
            "status": "ok",
            "event": "invite.status_changed",
            "token": invitation.token,
            "invitation_status": InvitationStatus.READY,
        }

        # No other authenticated users should be notified
        rep = await events_listen_nowait(bob_backend_sock)
        assert rep == {"status": "no_events"}

        rep = await invite_list(alice_backend_sock)
        assert rep == {
            "status": "ok",
            "invitations": [
                {
                    "type": InvitationType.DEVICE,
                    "token": invitation.token,
                    "created_on": Pendulum(2000, 1, 2),
                    "status": InvitationStatus.READY,
                    "deleted_on": None,
                    "deleted_reason": None,
                }
            ],
        }

    # Now claimer has left, greeter should be again notified
    with trio.fail_after(1):
        rep = await events_listen_wait(alice_backend_sock)
    assert rep == {
        "status": "ok",
        "event": "invite.status_changed",
        "token": invitation.token,
        "invitation_status": InvitationStatus.IDLE,
    }

    rep = await invite_list(alice_backend_sock)
    assert rep == {
        "status": "ok",
        "invitations": [
            {
                "type": InvitationType.DEVICE,
                "token": invitation.token,
                "created_on": Pendulum(2000, 1, 2),
                "status": InvitationStatus.IDLE,
                "deleted_on": None,
                "deleted_reason": None,
            }
        ],
    }
