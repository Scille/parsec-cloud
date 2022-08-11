# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import pytest
from pendulum import datetime

from parsec.api.protocol import InvitationType, InvitationStatus, APIEvent

from tests.common import real_clock_timeout
from tests.backend.common import (
    events_subscribe,
    events_listen_wait,
    events_listen_nowait,
    invite_list,
)


@pytest.mark.trio
async def test_greeter_event_on_claimer_join_and_leave(
    alice, backend_asgi_app, bob_ws, alice_ws, backend_invited_ws_factory
):
    invitation = await backend_asgi_app.backend.invite.new_for_device(
        organization_id=alice.organization_id,
        greeter_user_id=alice.user_id,
        created_on=datetime(2000, 1, 2),
    )

    await events_subscribe(alice_ws)
    await events_subscribe(bob_ws)

    async with backend_invited_ws_factory(
        backend_asgi_app,
        organization_id=alice.organization_id,
        invitation_type=InvitationType.DEVICE,
        token=invitation.token,
    ):

        # Claimer is ready, this should be notified to greeter

        async with real_clock_timeout():
            rep = await events_listen_wait(alice_ws)
            # PostgreSQL event dispatching might be lagging behind and return
            # the IDLE event first
            if rep.get("invitation_status") == InvitationStatus.IDLE:
                rep = await events_listen_wait(alice_ws)
        assert rep == {
            "status": "ok",
            "event": APIEvent.INVITE_STATUS_CHANGED,
            "token": invitation.token,
            "invitation_status": InvitationStatus.READY,
        }

        # No other authenticated users should be notified
        rep = await events_listen_nowait(bob_ws)
        assert rep == {"status": "no_events"}

        rep = await invite_list(alice_ws)
        assert rep == {
            "status": "ok",
            "invitations": [
                {
                    "type": InvitationType.DEVICE,
                    "token": invitation.token,
                    "created_on": datetime(2000, 1, 2),
                    "status": InvitationStatus.READY,
                }
            ],
        }

    # Now claimer has left, greeter should be again notified
    async with real_clock_timeout():
        rep = await events_listen_wait(alice_ws)
    assert rep == {
        "status": "ok",
        "event": APIEvent.INVITE_STATUS_CHANGED,
        "token": invitation.token,
        "invitation_status": InvitationStatus.IDLE,
    }

    rep = await invite_list(alice_ws)
    assert rep == {
        "status": "ok",
        "invitations": [
            {
                "type": InvitationType.DEVICE,
                "token": invitation.token,
                "created_on": datetime(2000, 1, 2),
                "status": InvitationStatus.IDLE,
            }
        ],
    }
