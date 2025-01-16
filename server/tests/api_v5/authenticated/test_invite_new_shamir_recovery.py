# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from unittest.mock import ANY

import pytest

from parsec._parsec import (
    InvitationStatus,
    UserID,
    authenticated_cmds,
)
from parsec.components.invite import (
    SendEmailBadOutcome,
    ShamirRecoveryInvitation,
    ShamirRecoveryRecipient,
)
from parsec.events import EventInvitation
from tests.common import Backend, CoolorgRpcClients, HttpCommonErrorsTester, ShamirOrgRpcClients


@pytest.mark.parametrize("send_email", (False, True))
async def test_authenticated_invite_new_shamir_recovery_ok_new(
    send_email: bool,
    shamirorg: ShamirOrgRpcClients,
    backend: Backend,
) -> None:
    expected_invitations = await backend.invite.test_dump_all_invitations(shamirorg.organization_id)

    with backend.event_bus.spy() as spy:
        rep = await shamirorg.mike.invite_new_shamir_recovery(
            send_email=send_email,
            claimer_user_id=shamirorg.mallory.user_id,
        )
        assert isinstance(rep, authenticated_cmds.latest.invite_new_shamir_recovery.RepOk)
        assert (
            rep.email_sent
            == authenticated_cmds.latest.invite_new_shamir_recovery.InvitationEmailSentStatus.SUCCESS
        )
        invitation_token = rep.token

        await spy.wait_event_occurred(
            EventInvitation(
                organization_id=shamirorg.organization_id,
                greeter=shamirorg.mike.user_id,
                token=invitation_token,
                status=InvitationStatus.IDLE,
            )
        )

    expected_invitations[shamirorg.mike.user_id] = [
        ShamirRecoveryInvitation(
            token=invitation_token,
            created_on=ANY,
            created_by_device_id=shamirorg.mike.device_id,
            created_by_user_id=shamirorg.mike.user_id,
            created_by_human_handle=shamirorg.mike.human_handle,
            status=InvitationStatus.IDLE,
            threshold=1,
            claimer_user_id=shamirorg.mallory.user_id,
            claimer_human_handle=shamirorg.mallory.human_handle,
            recipients=[
                ShamirRecoveryRecipient(
                    user_id=shamirorg.mike.user_id,
                    human_handle=shamirorg.mike.human_handle,
                    shares=1,
                    revoked_on=None,
                ),
            ],
            shamir_recovery_created_on=shamirorg.mallory_brief_certificate.timestamp,
            shamir_recovery_deleted_on=None,
        )
    ]
    assert (
        await backend.invite.test_dump_all_invitations(shamirorg.organization_id)
        == expected_invitations
    )


@pytest.mark.parametrize("send_email", (False, True))
async def test_authenticated_invite_new_shamir_recovery_author_not_allowed(
    send_email: bool, shamirorg: ShamirOrgRpcClients, backend: Backend
) -> None:
    # Shamir setup exists but author is not part of the recipients
    rep = await shamirorg.alice.invite_new_shamir_recovery(
        send_email=send_email,
        claimer_user_id=shamirorg.alice.user_id,
    )
    assert isinstance(rep, authenticated_cmds.latest.invite_new_shamir_recovery.RepAuthorNotAllowed)

    # No shamir setup have been created
    rep = await shamirorg.alice.invite_new_shamir_recovery(
        send_email=send_email,
        claimer_user_id=shamirorg.mike.user_id,
    )
    assert isinstance(rep, authenticated_cmds.latest.invite_new_shamir_recovery.RepAuthorNotAllowed)


@pytest.mark.parametrize("send_email", (False, True))
async def test_authenticated_invite_new_shamir_recovery_user_not_found(
    send_email: bool, shamirorg: ShamirOrgRpcClients, backend: Backend
) -> None:
    # Shamir setup exists but author is not part of the recipients
    rep = await shamirorg.alice.invite_new_shamir_recovery(
        send_email=send_email,
        claimer_user_id=UserID.new(),
    )
    assert isinstance(rep, authenticated_cmds.latest.invite_new_shamir_recovery.RepUserNotFound)


async def test_authenticated_invite_new_shamir_recovery_ok_already_exist(
    shamirorg: ShamirOrgRpcClients, backend: Backend
) -> None:
    expected_invitations = await backend.invite.test_dump_all_invitations(shamirorg.organization_id)

    with backend.event_bus.spy() as spy:
        rep = await shamirorg.bob.invite_new_shamir_recovery(
            send_email=False,
            claimer_user_id=shamirorg.alice.user_id,
        )
        assert isinstance(rep, authenticated_cmds.latest.invite_new_shamir_recovery.RepOk)
        assert rep.token == shamirorg.shamir_invited_alice.token
        await spy.wait_event_occurred(
            EventInvitation(
                organization_id=shamirorg.organization_id,
                greeter=shamirorg.bob.user_id,
                token=rep.token,
                status=InvitationStatus.IDLE,
            )
        )

    assert (
        await backend.invite.test_dump_all_invitations(shamirorg.organization_id)
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
async def test_authenticated_invite_new_shamir_recovery_send_email_bad_outcome(
    shamirorg: ShamirOrgRpcClients,
    backend: Backend,
    bad_outcome: SendEmailBadOutcome,
    monkeypatch,
) -> None:
    async def _mocked_send_email(*args, **kwargs):
        return bad_outcome

    monkeypatch.setattr("parsec.components.invite.send_email", _mocked_send_email)

    expected_invitations = await backend.invite.test_dump_all_invitations(shamirorg.organization_id)

    with backend.event_bus.spy() as spy:
        rep = await shamirorg.mike.invite_new_shamir_recovery(
            send_email=True, claimer_user_id=shamirorg.mallory.user_id
        )
        assert isinstance(rep, authenticated_cmds.latest.invite_new_shamir_recovery.RepOk)
        invitation_token = rep.token

        match bad_outcome:
            case SendEmailBadOutcome.BAD_SMTP_CONFIG:
                expected_email_sent_status = authenticated_cmds.latest.invite_new_shamir_recovery.InvitationEmailSentStatus.SERVER_UNAVAILABLE
            case SendEmailBadOutcome.RECIPIENT_REFUSED:
                expected_email_sent_status = authenticated_cmds.latest.invite_new_shamir_recovery.InvitationEmailSentStatus.RECIPIENT_REFUSED
            case SendEmailBadOutcome.SERVER_UNAVAILABLE:
                expected_email_sent_status = authenticated_cmds.latest.invite_new_shamir_recovery.InvitationEmailSentStatus.SERVER_UNAVAILABLE
        assert rep.email_sent == expected_email_sent_status

        await spy.wait_event_occurred(
            EventInvitation(
                organization_id=shamirorg.organization_id,
                greeter=shamirorg.mike.user_id,
                token=invitation_token,
                status=InvitationStatus.IDLE,
            )
        )

    expected_invitations[shamirorg.mike.user_id] = [
        ShamirRecoveryInvitation(
            token=invitation_token,
            created_on=ANY,
            created_by_device_id=shamirorg.mike.device_id,
            created_by_user_id=shamirorg.mike.user_id,
            created_by_human_handle=shamirorg.mike.human_handle,
            status=InvitationStatus.IDLE,
            threshold=1,
            claimer_user_id=shamirorg.mallory.user_id,
            claimer_human_handle=shamirorg.mallory.human_handle,
            recipients=[
                ShamirRecoveryRecipient(
                    user_id=shamirorg.mike.user_id,
                    human_handle=shamirorg.mike.human_handle,
                    shares=1,
                    revoked_on=None,
                ),
            ],
            shamir_recovery_created_on=shamirorg.mallory_brief_certificate.timestamp,
            shamir_recovery_deleted_on=None,
        )
    ]
    assert (
        await backend.invite.test_dump_all_invitations(shamirorg.organization_id)
        == expected_invitations
    )


async def test_authenticated_invite_new_shamir_recovery_http_common_errors(
    coolorg: CoolorgRpcClients, authenticated_http_common_errors_tester: HttpCommonErrorsTester
) -> None:
    async def do():
        await coolorg.alice.invite_new_shamir_recovery(
            send_email=False,
            claimer_user_id=coolorg.bob.user_id,
        )

    await authenticated_http_common_errors_tester(do)
