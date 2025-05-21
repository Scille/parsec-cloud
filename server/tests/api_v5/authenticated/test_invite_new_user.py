# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from unittest.mock import ANY

import pytest

from parsec._parsec import DateTime, EmailAddress, InvitationStatus, UserProfile, authenticated_cmds
from parsec.components.invite import (
    InvitationCreatedByUser,
    SendEmailBadOutcome,
    UserGreetingAdministrator,
    UserInvitation,
    UserOnlineStatus,
)
from parsec.events import EventInvitation
from tests.common import (
    Backend,
    CoolorgRpcClients,
    HttpCommonErrorsTester,
    MinimalorgRpcClients,
    bob_becomes_admin,
    bob_becomes_admin_and_changes_alice,
)


@pytest.mark.parametrize("send_email", (False, True))
async def test_authenticated_invite_new_user_ok_new(
    send_email: bool, minimalorg: MinimalorgRpcClients, backend: Backend
) -> None:
    with backend.event_bus.spy() as spy:
        rep = await minimalorg.alice.invite_new_user(
            claimer_email=EmailAddress("new@example.invalid"),
            send_email=send_email,
        )
        assert isinstance(rep, authenticated_cmds.latest.invite_new_user.RepOk)
        assert (
            rep.email_sent
            == authenticated_cmds.latest.invite_new_user.InvitationEmailSentStatus.SUCCESS
        )
        invitation_token = rep.token

        await spy.wait_event_occurred(
            EventInvitation(
                organization_id=minimalorg.organization_id,
                possible_greeters={minimalorg.alice.user_id},
                token=invitation_token,
                status=InvitationStatus.PENDING,
            )
        )

    expected_invitations = {
        minimalorg.alice.user_id: [
            UserInvitation(
                token=invitation_token,
                created_on=ANY,
                claimer_email=EmailAddress("new@example.invalid"),
                created_by=InvitationCreatedByUser(
                    user_id=minimalorg.alice.user_id, human_handle=minimalorg.alice.human_handle
                ),
                administrators=[
                    UserGreetingAdministrator(
                        minimalorg.alice.user_id,
                        minimalorg.alice.human_handle,
                        UserOnlineStatus.UNKNOWN,
                        None,
                    )
                ],
                status=InvitationStatus.PENDING,
            )
        ]
    }
    assert (
        await backend.invite.test_dump_all_invitations(minimalorg.organization_id)
        == expected_invitations
    )


async def test_authenticated_invite_new_user_ok_already_exist(
    minimalorg: MinimalorgRpcClients, backend: Backend
) -> None:
    t1 = DateTime.now()

    with backend.event_bus.spy() as spy:
        outcome = await backend.invite.new_for_user(
            now=t1,
            organization_id=minimalorg.organization_id,
            author=minimalorg.alice.device_id,
            claimer_email=EmailAddress("new@example.invalid"),
            send_email=False,
        )
        assert isinstance(outcome, tuple)
        (invitation_token, _) = outcome

        await spy.wait_event_occurred(
            EventInvitation(
                organization_id=minimalorg.organization_id,
                possible_greeters={minimalorg.alice.user_id},
                token=invitation_token,
                status=InvitationStatus.PENDING,
            )
        )

    expected_invitations = await backend.invite.test_dump_all_invitations(
        minimalorg.organization_id
    )

    with backend.event_bus.spy() as spy:
        rep = await minimalorg.alice.invite_new_user(
            claimer_email=EmailAddress("new@example.invalid"),
            send_email=False,
        )
        assert isinstance(rep, authenticated_cmds.latest.invite_new_user.RepOk)
        assert rep.token == invitation_token

        await spy.wait_event_occurred(
            EventInvitation(
                organization_id=minimalorg.organization_id,
                possible_greeters={minimalorg.alice.user_id},
                token=invitation_token,
                status=InvitationStatus.PENDING,
            )
        )

    assert (
        await backend.invite.test_dump_all_invitations(minimalorg.organization_id)
        == expected_invitations
    )


@pytest.mark.parametrize("kind", ("never_allowed", "no_longer_allowed"))
async def test_authenticated_invite_new_user_author_not_allowed(
    coolorg: CoolorgRpcClients, backend: Backend, kind: str
):
    match kind:
        case "never_allowed":
            author = coolorg.bob

        case "no_longer_allowed":
            await bob_becomes_admin_and_changes_alice(
                coolorg=coolorg, backend=backend, new_alice_profile=UserProfile.STANDARD
            )
            author = coolorg.alice

        case unknown:
            assert False, unknown

    rep = await author.invite_new_user(
        claimer_email=EmailAddress("new@example.invalid"),
        send_email=False,
    )
    assert rep == authenticated_cmds.latest.invite_new_user.RepAuthorNotAllowed()


async def test_authenticated_invite_new_user_claimer_email_already_enrolled(
    coolorg: CoolorgRpcClients,
):
    rep = await coolorg.alice.invite_new_user(
        claimer_email=coolorg.bob.human_handle.email,
        send_email=False,
    )
    assert rep == authenticated_cmds.latest.invite_new_user.RepClaimerEmailAlreadyEnrolled()


@pytest.mark.parametrize(
    "bad_outcome",
    (
        SendEmailBadOutcome.BAD_SMTP_CONFIG,
        SendEmailBadOutcome.RECIPIENT_REFUSED,
        SendEmailBadOutcome.SERVER_UNAVAILABLE,
    ),
)
async def test_authenticated_invite_new_user_send_email_bad_outcome(
    minimalorg: MinimalorgRpcClients,
    backend: Backend,
    bad_outcome: SendEmailBadOutcome,
    monkeypatch,
) -> None:
    async def _mocked_send_email(*args, **kwargs):
        return bad_outcome

    monkeypatch.setattr("parsec.components.invite.send_email", _mocked_send_email)

    with backend.event_bus.spy() as spy:
        rep = await minimalorg.alice.invite_new_user(
            claimer_email=EmailAddress("new@example.invalid"),
            send_email=True,
        )
        assert isinstance(rep, authenticated_cmds.latest.invite_new_user.RepOk)
        invitation_token = rep.token

        match bad_outcome:
            case SendEmailBadOutcome.BAD_SMTP_CONFIG:
                expected_email_sent_status = authenticated_cmds.latest.invite_new_user.InvitationEmailSentStatus.SERVER_UNAVAILABLE
            case SendEmailBadOutcome.RECIPIENT_REFUSED:
                expected_email_sent_status = authenticated_cmds.latest.invite_new_user.InvitationEmailSentStatus.RECIPIENT_REFUSED
            case SendEmailBadOutcome.SERVER_UNAVAILABLE:
                expected_email_sent_status = authenticated_cmds.latest.invite_new_user.InvitationEmailSentStatus.SERVER_UNAVAILABLE
        assert rep.email_sent == expected_email_sent_status

        await spy.wait_event_occurred(
            EventInvitation(
                organization_id=minimalorg.organization_id,
                possible_greeters={minimalorg.alice.user_id},
                token=invitation_token,
                status=InvitationStatus.PENDING,
            )
        )

    expected_invitations = {
        minimalorg.alice.user_id: [
            UserInvitation(
                token=invitation_token,
                created_on=ANY,
                claimer_email=EmailAddress("new@example.invalid"),
                created_by=InvitationCreatedByUser(
                    user_id=minimalorg.alice.user_id, human_handle=minimalorg.alice.human_handle
                ),
                administrators=[
                    UserGreetingAdministrator(
                        minimalorg.alice.user_id,
                        minimalorg.alice.human_handle,
                        UserOnlineStatus.UNKNOWN,
                        None,
                    )
                ],
                status=InvitationStatus.PENDING,
            )
        ]
    }
    assert (
        await backend.invite.test_dump_all_invitations(minimalorg.organization_id)
        == expected_invitations
    )


async def test_authenticated_invite_new_user_with_shared_user_invitations(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    # Bob becomes admin
    await bob_becomes_admin(coolorg, backend)

    # Save invitations for reference
    excepted_invitations = await backend.invite.test_dump_all_invitations(coolorg.organization_id)

    with backend.event_bus.spy() as spy:
        # Bob invites zack, although an existing invitation created by Alice already exists
        rep = await coolorg.bob.invite_new_user(
            claimer_email=EmailAddress("zack@example.invalid"),
            send_email=False,
        )

        # The same token as the existing invitation has been returned
        assert isinstance(rep, authenticated_cmds.latest.invite_new_user.RepOk)
        assert rep.token == coolorg.invited_zack.token

        # The corresponding event has been generated
        await spy.wait_event_occurred(
            EventInvitation(
                organization_id=coolorg.organization_id,
                possible_greeters={coolorg.alice.user_id, coolorg.bob.user_id},
                token=coolorg.invited_zack.token,
                status=InvitationStatus.PENDING,
            )
        )

    # No new invitation has been created
    assert (
        await backend.invite.test_dump_all_invitations(coolorg.organization_id)
        == excepted_invitations
    )


async def test_authenticated_invite_new_user_http_common_errors(
    coolorg: CoolorgRpcClients, authenticated_http_common_errors_tester: HttpCommonErrorsTester
) -> None:
    async def do():
        await coolorg.alice.invite_new_user(
            claimer_email=EmailAddress("new@example.invalid"),
            send_email=False,
        )

    await authenticated_http_common_errors_tester(do)
