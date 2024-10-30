# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from unittest.mock import ANY

import pytest

from parsec._parsec import (
    DateTime,
    InvitationStatus,
    InvitationToken,
    ShamirRecoveryBriefCertificate,
    ShamirRecoveryShareCertificate,
    UserID,
    authenticated_cmds,
)
from parsec.components.invite import (
    SendEmailBadOutcome,
    ShamirRecoveryInvitation,
    ShamirRecoveryRecipient,
)
from parsec.events import EventInvitation
from tests.common import Backend, CoolorgRpcClients, HttpCommonErrorsTester


@pytest.fixture
async def alice_shamir(backend: Backend, coolorg: CoolorgRpcClients, with_postgresql: bool) -> None:
    if with_postgresql:
        pytest.xfail("TODO: postgre not implemented yet")
    dt = DateTime.now()
    bob_share = ShamirRecoveryShareCertificate(
        author=coolorg.alice.device_id,
        user_id=coolorg.alice.user_id,
        timestamp=dt,
        recipient=coolorg.bob.user_id,
        ciphered_share=b"abc",
    )
    mallory_share = ShamirRecoveryShareCertificate(
        author=coolorg.alice.device_id,
        user_id=coolorg.alice.user_id,
        timestamp=dt,
        recipient=coolorg.mallory.user_id,
        ciphered_share=b"abc",
    )
    brief = ShamirRecoveryBriefCertificate(
        author=coolorg.alice.device_id,
        user_id=coolorg.alice.user_id,
        timestamp=dt,
        threshold=2,
        per_recipient_shares={coolorg.bob.user_id: 1, coolorg.mallory.user_id: 1},
    )

    setup = authenticated_cmds.v4.shamir_recovery_setup.ShamirRecoverySetup(
        b"abc",
        InvitationToken.new(),
        brief.dump_and_sign(coolorg.alice.signing_key),
        [
            bob_share.dump_and_sign(coolorg.alice.signing_key),
            mallory_share.dump_and_sign(coolorg.alice.signing_key),
        ],
    )
    rep = await coolorg.alice.shamir_recovery_setup(setup)
    assert rep == authenticated_cmds.v4.shamir_recovery_setup.RepOk()


@pytest.mark.parametrize("send_email", (False, True))
async def test_authenticated_invite_new_shamir_recovery_ok_new(
    send_email: bool, coolorg: CoolorgRpcClients, backend: Backend, alice_shamir: None
) -> None:
    expected_invitations = await backend.invite.test_dump_all_invitations(coolorg.organization_id)

    with backend.event_bus.spy() as spy:
        rep = await coolorg.bob.invite_new_shamir_recovery(
            send_email=send_email,
            claimer_user_id=coolorg.alice.user_id,
        )
        assert isinstance(rep, authenticated_cmds.v4.invite_new_shamir_recovery.RepOk)
        assert (
            rep.email_sent
            == authenticated_cmds.v4.invite_new_shamir_recovery.InvitationEmailSentStatus.SUCCESS
        )
        invitation_token = rep.token

        await spy.wait_event_occurred(
            EventInvitation(
                organization_id=coolorg.organization_id,
                greeter=coolorg.bob.user_id,
                token=invitation_token,
                status=InvitationStatus.IDLE,
            )
        )

    expected_invitations[coolorg.bob.user_id] = [
        ShamirRecoveryInvitation(
            token=invitation_token,
            created_on=ANY,
            created_by_device_id=coolorg.bob.device_id,
            created_by_user_id=coolorg.bob.user_id,
            created_by_human_handle=coolorg.bob.human_handle,
            status=InvitationStatus.IDLE,
            threshold=2,
            claimer_user_id=coolorg.alice.user_id,
            recipients=[
                ShamirRecoveryRecipient(
                    user_id=coolorg.bob.user_id,
                    human_handle=coolorg.bob.human_handle,
                    shares=1,
                ),
                ShamirRecoveryRecipient(
                    user_id=coolorg.mallory.user_id,
                    human_handle=coolorg.mallory.human_handle,
                    shares=1,
                ),
            ],
        )
    ]
    assert (
        await backend.invite.test_dump_all_invitations(coolorg.organization_id)
        == expected_invitations
    )


@pytest.mark.parametrize("send_email", (False, True))
async def test_authenticated_invite_new_shamir_recovery_author_not_allowed(
    send_email: bool, coolorg: CoolorgRpcClients, backend: Backend, alice_shamir: None
) -> None:
    # Shamir setup exists but author is not part of the recipients
    rep = await coolorg.alice.invite_new_shamir_recovery(
        send_email=send_email,
        claimer_user_id=coolorg.alice.user_id,
    )
    assert isinstance(rep, authenticated_cmds.v4.invite_new_shamir_recovery.RepAuthorNotAllowed)

    # No shamir setup have been created
    rep = await coolorg.alice.invite_new_shamir_recovery(
        send_email=send_email,
        claimer_user_id=coolorg.bob.user_id,
    )
    assert isinstance(rep, authenticated_cmds.v4.invite_new_shamir_recovery.RepAuthorNotAllowed)


@pytest.mark.parametrize("send_email", (False, True))
async def test_authenticated_invite_new_shamir_recovery_user_not_found(
    send_email: bool, coolorg: CoolorgRpcClients, backend: Backend, alice_shamir: None
) -> None:
    # Shamir setup exists but author is not part of the recipients
    rep = await coolorg.alice.invite_new_shamir_recovery(
        send_email=send_email,
        claimer_user_id=UserID.new(),
    )
    assert isinstance(rep, authenticated_cmds.v4.invite_new_shamir_recovery.RepUserNotFound)


async def test_authenticated_invite_new_shamir_recovery_ok_already_exist(
    coolorg: CoolorgRpcClients, backend: Backend, alice_shamir: None
) -> None:
    t1 = DateTime.now()

    outcome = await backend.invite.new_for_shamir_recovery(
        now=t1,
        organization_id=coolorg.organization_id,
        author=coolorg.bob.device_id,
        send_email=False,
        claimer_user_id=coolorg.alice.user_id,
    )
    assert isinstance(outcome, tuple)
    (invitation_token, _) = outcome

    expected_invitations = await backend.invite.test_dump_all_invitations(coolorg.organization_id)

    with backend.event_bus.spy() as spy:
        rep = await coolorg.bob.invite_new_shamir_recovery(
            send_email=False,
            claimer_user_id=coolorg.alice.user_id,
        )
        assert isinstance(rep, authenticated_cmds.v4.invite_new_shamir_recovery.RepOk)
        assert rep.token == invitation_token

        await spy.wait_event_occurred(
            EventInvitation(
                organization_id=coolorg.organization_id,
                greeter=coolorg.bob.user_id,
                token=invitation_token,
                status=InvitationStatus.IDLE,
            )
        )

    assert (
        await backend.invite.test_dump_all_invitations(coolorg.organization_id)
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
    coolorg: CoolorgRpcClients,
    backend: Backend,
    bad_outcome: SendEmailBadOutcome,
    alice_shamir: None,
    monkeypatch,
) -> None:
    async def _mocked_send_email(*args, **kwargs):
        return bad_outcome

    monkeypatch.setattr("parsec.components.invite.send_email", _mocked_send_email)

    expected_invitations = await backend.invite.test_dump_all_invitations(coolorg.organization_id)

    with backend.event_bus.spy() as spy:
        rep = await coolorg.bob.invite_new_shamir_recovery(
            send_email=True, claimer_user_id=coolorg.alice.user_id
        )
        assert isinstance(rep, authenticated_cmds.v4.invite_new_shamir_recovery.RepOk)
        invitation_token = rep.token

        match bad_outcome:
            case SendEmailBadOutcome.BAD_SMTP_CONFIG:
                expected_email_sent_status = authenticated_cmds.v4.invite_new_shamir_recovery.InvitationEmailSentStatus.SERVER_UNAVAILABLE
            case SendEmailBadOutcome.RECIPIENT_REFUSED:
                expected_email_sent_status = authenticated_cmds.v4.invite_new_shamir_recovery.InvitationEmailSentStatus.RECIPIENT_REFUSED
            case SendEmailBadOutcome.SERVER_UNAVAILABLE:
                expected_email_sent_status = authenticated_cmds.v4.invite_new_shamir_recovery.InvitationEmailSentStatus.SERVER_UNAVAILABLE
        assert rep.email_sent == expected_email_sent_status

        await spy.wait_event_occurred(
            EventInvitation(
                organization_id=coolorg.organization_id,
                greeter=coolorg.bob.user_id,
                token=invitation_token,
                status=InvitationStatus.IDLE,
            )
        )

    expected_invitations[coolorg.bob.user_id] = [
        ShamirRecoveryInvitation(
            token=invitation_token,
            created_on=ANY,
            created_by_device_id=coolorg.bob.device_id,
            created_by_user_id=coolorg.bob.user_id,
            created_by_human_handle=coolorg.bob.human_handle,
            status=InvitationStatus.IDLE,
            threshold=2,
            claimer_user_id=coolorg.alice.user_id,
            recipients=[
                ShamirRecoveryRecipient(
                    user_id=coolorg.bob.user_id,
                    human_handle=coolorg.bob.human_handle,
                    shares=1,
                ),
                ShamirRecoveryRecipient(
                    user_id=coolorg.mallory.user_id,
                    human_handle=coolorg.mallory.human_handle,
                    shares=1,
                ),
            ],
        )
    ]
    assert (
        await backend.invite.test_dump_all_invitations(coolorg.organization_id)
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
