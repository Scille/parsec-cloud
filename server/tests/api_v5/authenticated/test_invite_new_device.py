# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from unittest.mock import ANY

import pytest

from parsec._parsec import DateTime, InvitationStatus, authenticated_cmds
from parsec.components.invite import DeviceInvitation, SendEmailBadOutcome
from parsec.events import EventInvitation
from tests.common import Backend, CoolorgRpcClients, HttpCommonErrorsTester, MinimalorgRpcClients


@pytest.mark.parametrize("send_email", (False, True))
async def test_authenticated_invite_new_device_ok_new(
    send_email: bool, minimalorg: MinimalorgRpcClients, backend: Backend
) -> None:
    expected_invitations = await backend.invite.test_dump_all_invitations(
        minimalorg.organization_id
    )

    with backend.event_bus.spy() as spy:
        rep = await minimalorg.alice.invite_new_device(
            send_email=send_email,
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
                greeter=minimalorg.alice.user_id,
                token=invitation_token,
                status=InvitationStatus.IDLE,
            )
        )

    expected_invitations[minimalorg.alice.user_id] = [
        DeviceInvitation(
            token=invitation_token,
            created_on=ANY,
            created_by_device_id=minimalorg.alice.device_id,
            created_by_user_id=minimalorg.alice.user_id,
            created_by_human_handle=minimalorg.alice.human_handle,
            status=InvitationStatus.IDLE,
        )
    ]
    assert (
        await backend.invite.test_dump_all_invitations(minimalorg.organization_id)
        == expected_invitations
    )


async def test_authenticated_invite_new_device_ok_already_exist(
    minimalorg: MinimalorgRpcClients, backend: Backend
) -> None:
    t1 = DateTime.now()

    with backend.event_bus.spy() as spy:
        outcome = await backend.invite.new_for_device(
            now=t1,
            organization_id=minimalorg.organization_id,
            author=minimalorg.alice.device_id,
            send_email=False,
        )
        assert isinstance(outcome, tuple)
        (invitation_token, _) = outcome

        await spy.wait_event_occurred(
            EventInvitation(
                organization_id=minimalorg.organization_id,
                greeter=minimalorg.alice.user_id,
                token=invitation_token,
                status=InvitationStatus.IDLE,
            )
        )

    expected_invitations = await backend.invite.test_dump_all_invitations(
        minimalorg.organization_id
    )

    with backend.event_bus.spy() as spy:
        rep = await minimalorg.alice.invite_new_device(
            send_email=False,
        )
        assert isinstance(rep, authenticated_cmds.v4.invite_new_device.RepOk)
        assert rep.token == invitation_token

        await spy.wait_event_occurred(
            EventInvitation(
                organization_id=minimalorg.organization_id,
                greeter=minimalorg.alice.user_id,
                token=invitation_token,
                status=InvitationStatus.IDLE,
            )
        )

    assert (
        await backend.invite.test_dump_all_invitations(minimalorg.organization_id)
        == expected_invitations
    )


@pytest.mark.parametrize(
    "bad_outcome",
    (
        SendEmailBadOutcome.BAD_SMTP_CONFIG,
        SendEmailBadOutcome.RECIPIENT_REFUSED,
        SendEmailBadOutcome.SERVER_UNAVAILABLE,
    ),
)
async def test_authenticated_invite_new_device_send_email_bad_outcome(
    minimalorg: MinimalorgRpcClients,
    backend: Backend,
    bad_outcome: SendEmailBadOutcome,
    monkeypatch,
) -> None:
    async def _mocked_send_email(*args, **kwargs):
        return bad_outcome

    monkeypatch.setattr("parsec.components.invite.send_email", _mocked_send_email)

    expected_invitations = await backend.invite.test_dump_all_invitations(
        minimalorg.organization_id
    )

    with backend.event_bus.spy() as spy:
        rep = await minimalorg.alice.invite_new_device(send_email=True)
        assert isinstance(rep, authenticated_cmds.v4.invite_new_device.RepOk)
        invitation_token = rep.token

        match bad_outcome:
            case SendEmailBadOutcome.BAD_SMTP_CONFIG:
                expected_email_sent_status = authenticated_cmds.v4.invite_new_device.InvitationEmailSentStatus.SERVER_UNAVAILABLE
            case SendEmailBadOutcome.RECIPIENT_REFUSED:
                expected_email_sent_status = authenticated_cmds.v4.invite_new_device.InvitationEmailSentStatus.RECIPIENT_REFUSED
            case SendEmailBadOutcome.SERVER_UNAVAILABLE:
                expected_email_sent_status = authenticated_cmds.v4.invite_new_device.InvitationEmailSentStatus.SERVER_UNAVAILABLE
        assert rep.email_sent == expected_email_sent_status

        await spy.wait_event_occurred(
            EventInvitation(
                organization_id=minimalorg.organization_id,
                greeter=minimalorg.alice.user_id,
                token=invitation_token,
                status=InvitationStatus.IDLE,
            )
        )

    expected_invitations[minimalorg.alice.user_id] = [
        DeviceInvitation(
            token=invitation_token,
            created_on=ANY,
            created_by_device_id=minimalorg.alice.device_id,
            created_by_user_id=minimalorg.alice.user_id,
            created_by_human_handle=minimalorg.alice.human_handle,
            status=InvitationStatus.IDLE,
        )
    ]
    assert (
        await backend.invite.test_dump_all_invitations(minimalorg.organization_id)
        == expected_invitations
    )


async def test_authenticated_invite_new_device_http_common_errors(
    coolorg: CoolorgRpcClients, authenticated_http_common_errors_tester: HttpCommonErrorsTester
) -> None:
    async def do():
        await coolorg.alice.invite_new_device(
            send_email=False,
        )

    await authenticated_http_common_errors_tester(do)
