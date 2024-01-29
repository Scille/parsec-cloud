# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from unittest.mock import ANY

import pytest

from parsec._parsec import InvitationStatus, authenticated_cmds
from parsec.components.invite import DeviceInvitation
from parsec.events import EventInvitation
from tests.common import Backend, MinimalorgRpcClients


@pytest.mark.parametrize("send_email", (False, True))
async def test_authenticated_invite_new_device_ok(
    send_email: bool, minimalorg: MinimalorgRpcClients, backend: Backend
) -> None:
    if send_email:
        pytest.xfail(reason="TODO")

    expected_invitations = await backend.invite.test_dump_all_invitations(
        minimalorg.organization_id
    )

    with backend.event_bus.spy() as spy:
        rep = await minimalorg.alice.invite_new_device(
            send_email=False,
        )
        assert isinstance(rep, authenticated_cmds.v4.invite_new_device.RepOk)
        assert (
            rep.email_sent
            == authenticated_cmds.v4.invite_new_device.InvitationEmailSentStatus.SUCCESS
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

    expected_invitations[minimalorg.alice.device_id.user_id] = [
        DeviceInvitation(
            token=invitation_token,
            created_on=ANY,
            greeter_user_id=minimalorg.alice.device_id.user_id,
            greeter_human_handle=minimalorg.alice.human_handle,
            status=InvitationStatus.IDLE,
        )
    ]
    assert (
        await backend.invite.test_dump_all_invitations(minimalorg.organization_id)
        == expected_invitations
    )
