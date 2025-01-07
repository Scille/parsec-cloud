# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


import pytest

from parsec._parsec import (
    CancelledGreetingAttemptReason,
    DateTime,
    GreeterOrClaimer,
    GreetingAttemptID,
    ShamirRecoveryDeletionCertificate,
    UserProfile,
    authenticated_cmds,
    invited_cmds,
)
from tests.common import (
    Backend,
    CoolorgRpcClients,
    HttpCommonErrorsTester,
    ShamirOrgRpcClients,
    bob_becomes_admin,
    bob_becomes_admin_and_changes_alice,
)

Response = authenticated_cmds.v4.invite_greeter_cancel_greeting_attempt.Rep | None


@pytest.fixture
async def greeting_attempt(coolorg: CoolorgRpcClients, backend: Backend) -> GreetingAttemptID:
    outcome = await backend.invite.greeter_start_greeting_attempt(
        now=DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        greeter=coolorg.alice.user_id,
        token=coolorg.invited_alice_dev3.token,
    )
    assert isinstance(outcome, GreetingAttemptID)
    return outcome


@pytest.fixture
async def zack_greeting_attempt(coolorg: CoolorgRpcClients, backend: Backend) -> GreetingAttemptID:
    outcome = await backend.invite.greeter_start_greeting_attempt(
        now=DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        greeter=coolorg.alice.user_id,
        token=coolorg.invited_zack.token,
    )
    assert isinstance(outcome, GreetingAttemptID)
    return outcome


@pytest.mark.parametrize(
    "reason",
    CancelledGreetingAttemptReason.VALUES,
    ids=map(str, CancelledGreetingAttemptReason.VALUES),
)
async def test_authenticated_invite_greeter_cancel_greeting_attempt_ok(
    coolorg: CoolorgRpcClients,
    greeting_attempt: GreetingAttemptID,
    reason: CancelledGreetingAttemptReason,
) -> None:
    rep = await coolorg.alice.invite_greeter_cancel_greeting_attempt(
        greeting_attempt=greeting_attempt,
        reason=reason,
    )
    assert rep == authenticated_cmds.v4.invite_greeter_cancel_greeting_attempt.RepOk()


async def test_authenticated_invite_greeter_cancel_greeting_attempt_author_not_allowed(
    coolorg: CoolorgRpcClients,
    zack_greeting_attempt: GreetingAttemptID,
    backend: Backend,
) -> None:
    await bob_becomes_admin_and_changes_alice(coolorg, backend, UserProfile.STANDARD)

    rep = await coolorg.alice.invite_greeter_cancel_greeting_attempt(
        greeting_attempt=zack_greeting_attempt,
        reason=CancelledGreetingAttemptReason.MANUALLY_CANCELLED,
    )

    assert rep == authenticated_cmds.v4.invite_greeter_cancel_greeting_attempt.RepAuthorNotAllowed()


async def test_authenticated_invite_greeter_cancel_greeting_attempt_invitation_cancelled(
    coolorg: CoolorgRpcClients, greeting_attempt: GreetingAttemptID
) -> None:
    await coolorg.alice.invite_cancel(token=coolorg.invited_alice_dev3.token)

    rep = await coolorg.alice.invite_greeter_cancel_greeting_attempt(
        greeting_attempt=greeting_attempt,
        reason=CancelledGreetingAttemptReason.MANUALLY_CANCELLED,
    )
    assert (
        rep == authenticated_cmds.v4.invite_greeter_cancel_greeting_attempt.RepInvitationCancelled()
    )


async def test_authenticated_invite_greeter_cancel_greeting_attempt_invitation_completed(
    coolorg: CoolorgRpcClients, greeting_attempt: GreetingAttemptID
) -> None:
    await coolorg.alice.invite_complete(token=coolorg.invited_alice_dev3.token)

    rep = await coolorg.alice.invite_greeter_cancel_greeting_attempt(
        greeting_attempt=greeting_attempt,
        reason=CancelledGreetingAttemptReason.MANUALLY_CANCELLED,
    )

    assert (
        rep == authenticated_cmds.v4.invite_greeter_cancel_greeting_attempt.RepInvitationCompleted()
    )


async def test_authenticated_invite_greeter_cancel_greeting_attempt_greeting_attempt_not_found(
    coolorg: CoolorgRpcClients, zack_greeting_attempt: GreetingAttemptID, backend: Backend
) -> None:
    rep = await coolorg.alice.invite_greeter_cancel_greeting_attempt(
        greeting_attempt=GreetingAttemptID.new(),
        reason=CancelledGreetingAttemptReason.MANUALLY_CANCELLED,
    )
    assert (
        rep
        == authenticated_cmds.v4.invite_greeter_cancel_greeting_attempt.RepGreetingAttemptNotFound()
    )

    # Make Bob an admin
    await bob_becomes_admin(coolorg, backend)

    # Bob uses Alice greeting attempt
    rep = await coolorg.bob.invite_greeter_cancel_greeting_attempt(
        greeting_attempt=zack_greeting_attempt,
        reason=CancelledGreetingAttemptReason.MANUALLY_CANCELLED,
    )
    assert (
        rep
        == authenticated_cmds.v4.invite_greeter_cancel_greeting_attempt.RepGreetingAttemptNotFound()
    )


async def test_authenticated_invite_greeter_cancel_greeting_attempt_greeting_attempt_not_joined(
    coolorg: CoolorgRpcClients,
) -> None:
    rep = await coolorg.invited_alice_dev3.invite_claimer_start_greeting_attempt(
        greeter=coolorg.alice.user_id,
    )
    assert isinstance(rep, invited_cmds.v4.invite_claimer_start_greeting_attempt.RepOk)

    rep = await coolorg.alice.invite_greeter_cancel_greeting_attempt(
        greeting_attempt=rep.greeting_attempt,
        reason=CancelledGreetingAttemptReason.MANUALLY_CANCELLED,
    )

    assert (
        rep
        == authenticated_cmds.v4.invite_greeter_cancel_greeting_attempt.RepGreetingAttemptNotJoined()
    )


@pytest.mark.parametrize(
    "reason",
    CancelledGreetingAttemptReason.VALUES,
    ids=map(str, CancelledGreetingAttemptReason.VALUES),
)
async def test_authenticated_invite_greeter_cancel_greeting_attempt_greeting_attempt_already_cancelled(
    coolorg: CoolorgRpcClients,
    greeting_attempt: GreetingAttemptID,
    reason: CancelledGreetingAttemptReason,
) -> None:
    # Cancel once
    rep = await coolorg.alice.invite_greeter_cancel_greeting_attempt(
        greeting_attempt=greeting_attempt,
        reason=reason,
    )
    assert rep == authenticated_cmds.v4.invite_greeter_cancel_greeting_attempt.RepOk()

    # Cancel again
    rep = await coolorg.alice.invite_greeter_cancel_greeting_attempt(
        greeting_attempt=greeting_attempt,
        reason=CancelledGreetingAttemptReason.AUTOMATICALLY_CANCELLED,
    )

    assert isinstance(
        rep,
        authenticated_cmds.v4.invite_greeter_cancel_greeting_attempt.RepGreetingAttemptAlreadyCancelled,
    )
    assert (
        rep
        == authenticated_cmds.v4.invite_greeter_cancel_greeting_attempt.RepGreetingAttemptAlreadyCancelled(
            timestamp=rep.timestamp,
            reason=reason,
            origin=GreeterOrClaimer.GREETER,
        )
    )


async def test_authenticated_invite_greeter_cancel_greeting_attempt_with_deleted_shamir(
    shamirorg: ShamirOrgRpcClients,
) -> None:
    rep = await shamirorg.bob.invite_greeter_start_greeting_attempt(
        token=shamirorg.shamir_invited_alice.token,
    )
    assert isinstance(rep, authenticated_cmds.v4.invite_greeter_start_greeting_attempt.RepOk)
    greeting_attempt = rep.greeting_attempt

    # Delete Alice shamir recovery
    dt = DateTime.now()
    author = shamirorg.alice
    brief = shamirorg.alice_brief_certificate
    deletion = ShamirRecoveryDeletionCertificate(
        author=author.device_id,
        timestamp=dt,
        setup_to_delete_timestamp=brief.timestamp,
        setup_to_delete_user_id=brief.user_id,
        share_recipients=set(brief.per_recipient_shares.keys()),
    ).dump_and_sign(author.signing_key)
    rep = await shamirorg.alice.shamir_recovery_delete(deletion)
    assert rep == authenticated_cmds.v4.shamir_recovery_delete.RepOk()

    rep = await shamirorg.bob.invite_greeter_cancel_greeting_attempt(
        greeting_attempt=greeting_attempt,
        reason=CancelledGreetingAttemptReason.MANUALLY_CANCELLED,
    )
    assert (
        rep == authenticated_cmds.v4.invite_greeter_cancel_greeting_attempt.RepInvitationCancelled()
    )


async def test_authenticated_invite_greeter_cancel_greeting_attempt_http_common_errors(
    coolorg: CoolorgRpcClients,
    greeting_attempt: GreetingAttemptID,
    authenticated_http_common_errors_tester: HttpCommonErrorsTester,
) -> None:
    async def do():
        await coolorg.alice.invite_greeter_cancel_greeting_attempt(
            greeting_attempt=greeting_attempt,
            reason=CancelledGreetingAttemptReason.MANUALLY_CANCELLED,
        )

    await authenticated_http_common_errors_tester(do)
