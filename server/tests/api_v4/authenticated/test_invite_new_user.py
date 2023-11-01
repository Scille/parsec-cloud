# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from unittest.mock import ANY

import pytest

from parsec._parsec import InvitationStatus, authenticated_cmds
from parsec.components.invite import UserInvitation
from parsec.events import EventInvitation
from tests.common import Backend, MinimalorgRpcClients


@pytest.mark.parametrize("send_email", (False, True))
async def test_authenticated_invite_new_user_ok(
    send_email: bool, minimalorg: MinimalorgRpcClients, backend: Backend
) -> None:
    if send_email:
        pytest.xfail(reason="TODO")

    with backend.event_bus.spy() as spy:
        rep = await minimalorg.alice.invite_new_user(
            claimer_email="new@example.invalid",
            send_email=False,
        )
        assert isinstance(rep, authenticated_cmds.v4.invite_new_user.RepOk)
        assert (
            rep.email_sent
            == authenticated_cmds.v4.invite_new_user.InvitationEmailSentStatus.SUCCESS
        )
        invitation_token = rep.token

        await spy.wait_event_occurred(
            EventInvitation(
                organization_id=minimalorg.organization_id,
                greeter=minimalorg.alice.device_id.user_id,
                token=invitation_token,
                status=InvitationStatus.IDLE,
            )
        )

    expected_invitations = {
        minimalorg.alice.device_id.user_id: [
            UserInvitation(
                token=invitation_token,
                created_on=ANY,
                claimer_email="new@example.invalid",
                greeter_human_handle=minimalorg.alice.human_handle,
                greeter_user_id=minimalorg.alice.device_id.user_id,
                status=InvitationStatus.IDLE,
            )
        ]
    }
    assert (
        await backend.invite.test_dump_all_invitations(minimalorg.organization_id)
        == expected_invitations
    )
