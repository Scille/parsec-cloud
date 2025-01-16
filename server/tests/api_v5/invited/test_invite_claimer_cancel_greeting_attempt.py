# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


import pytest

from parsec._parsec import (
    CancelledGreetingAttemptReason,
    DateTime,
    GreeterOrClaimer,
    GreetingAttemptID,
    RevokedUserCertificate,
    UserProfile,
    UserUpdateCertificate,
    authenticated_cmds,
    invited_cmds,
)
from tests.common import (
    Backend,
    CoolorgRpcClients,
    HttpCommonErrorsTester,
    ShamirOrgRpcClients,
    bob_becomes_admin_and_changes_alice,
)


@pytest.fixture
async def greeting_attempt(coolorg: CoolorgRpcClients, backend: Backend) -> GreetingAttemptID:
    outcome = await backend.invite.claimer_start_greeting_attempt(
        now=DateTime.now(),
        organization_id=coolorg.organization_id,
        greeter=coolorg.alice.user_id,
        token=coolorg.invited_alice_dev3.token,
    )
    assert isinstance(outcome, GreetingAttemptID)
    return outcome


@pytest.mark.parametrize(
    "reason",
    CancelledGreetingAttemptReason.VALUES,
    ids=map(str, CancelledGreetingAttemptReason.VALUES),
)
async def test_invited_invite_claimer_cancel_greeting_attempt_ok(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    greeting_attempt: GreetingAttemptID,
    reason: CancelledGreetingAttemptReason,
) -> None:
    rep = await coolorg.invited_alice_dev3.invite_claimer_cancel_greeting_attempt(
        greeting_attempt=greeting_attempt,
        reason=reason,
    )
    assert rep == invited_cmds.latest.invite_claimer_cancel_greeting_attempt.RepOk()


async def test_invited_invite_claimer_cancel_greeting_attempt_greeter_not_allowed(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    rep = await coolorg.invited_zack.invite_claimer_start_greeting_attempt(
        greeter=coolorg.alice.user_id,
    )
    assert isinstance(rep, invited_cmds.latest.invite_claimer_start_greeting_attempt.RepOk)
    greeting_attempt = rep.greeting_attempt

    await bob_becomes_admin_and_changes_alice(
        coolorg=coolorg, backend=backend, new_alice_profile=UserProfile.STANDARD
    )

    # Cancel the greeting attempt
    rep = await coolorg.invited_zack.invite_claimer_cancel_greeting_attempt(
        greeting_attempt=greeting_attempt,
        reason=CancelledGreetingAttemptReason.MANUALLY_CANCELLED,
    )

    assert rep == invited_cmds.latest.invite_claimer_cancel_greeting_attempt.RepGreeterNotAllowed()


async def test_invited_invite_claimer_cancel_greeting_attempt_greeter_revoked(
    coolorg: CoolorgRpcClients, greeting_attempt: GreetingAttemptID
) -> None:
    # Make Bob an admin
    certif = UserUpdateCertificate(
        author=coolorg.alice.device_id,
        new_profile=UserProfile.ADMIN,
        user_id=coolorg.bob.user_id,
        timestamp=DateTime.now(),
    ).dump_and_sign(coolorg.alice.signing_key)
    rep = await coolorg.alice.user_update(certif)
    assert rep == authenticated_cmds.latest.user_update.RepOk()

    # Revoke Alice
    now = DateTime.now()
    certif = RevokedUserCertificate(
        author=coolorg.bob.device_id,
        timestamp=now,
        user_id=coolorg.alice.user_id,
    ).dump_and_sign(coolorg.bob.signing_key)
    rep = await coolorg.bob.user_revoke(certif)
    assert rep == authenticated_cmds.latest.user_revoke.RepOk()

    rep = await coolorg.invited_alice_dev3.invite_claimer_cancel_greeting_attempt(
        greeting_attempt=greeting_attempt,
        reason=CancelledGreetingAttemptReason.MANUALLY_CANCELLED,
    )
    assert rep == invited_cmds.latest.invite_claimer_cancel_greeting_attempt.RepGreeterRevoked()


async def test_invited_invite_claimer_cancel_greeting_attempt_greeting_attempt_not_found(
    coolorg: CoolorgRpcClients, backend: Backend, greeting_attempt: GreetingAttemptID
) -> None:
    rep = await coolorg.invited_alice_dev3.invite_claimer_cancel_greeting_attempt(
        greeting_attempt=GreetingAttemptID.new(),
        reason=CancelledGreetingAttemptReason.MANUALLY_CANCELLED,
    )
    assert (
        rep
        == invited_cmds.latest.invite_claimer_cancel_greeting_attempt.RepGreetingAttemptNotFound()
    )

    # Zack uses Alice greeting attempt
    rep = await coolorg.invited_zack.invite_claimer_cancel_greeting_attempt(
        greeting_attempt=greeting_attempt,
        reason=CancelledGreetingAttemptReason.MANUALLY_CANCELLED,
    )
    assert (
        rep
        == invited_cmds.latest.invite_claimer_cancel_greeting_attempt.RepGreetingAttemptNotFound()
    )


async def test_invited_invite_claimer_cancel_greeting_attempt_greeting_attempt_not_joined(
    coolorg: CoolorgRpcClients,
) -> None:
    rep = await coolorg.alice.invite_greeter_start_greeting_attempt(
        token=coolorg.invited_alice_dev3.token,
    )
    assert isinstance(rep, authenticated_cmds.latest.invite_greeter_start_greeting_attempt.RepOk)

    rep = await coolorg.invited_alice_dev3.invite_claimer_cancel_greeting_attempt(
        greeting_attempt=rep.greeting_attempt,
        reason=CancelledGreetingAttemptReason.MANUALLY_CANCELLED,
    )

    assert (
        rep
        == invited_cmds.latest.invite_claimer_cancel_greeting_attempt.RepGreetingAttemptNotJoined()
    )


@pytest.mark.parametrize(
    "reason",
    CancelledGreetingAttemptReason.VALUES,
    ids=map(str, CancelledGreetingAttemptReason.VALUES),
)
async def test_invited_invite_claimer_cancel_greeting_attempt_greeting_attempt_already_cancelled(
    coolorg: CoolorgRpcClients,
    greeting_attempt: GreetingAttemptID,
    reason: CancelledGreetingAttemptReason,
) -> None:
    # Cancel once
    rep = await coolorg.invited_alice_dev3.invite_claimer_cancel_greeting_attempt(
        greeting_attempt=greeting_attempt,
        reason=reason,
    )
    assert rep == invited_cmds.latest.invite_claimer_cancel_greeting_attempt.RepOk()

    # Cancel again
    rep = await coolorg.invited_alice_dev3.invite_claimer_cancel_greeting_attempt(
        greeting_attempt=greeting_attempt,
        reason=CancelledGreetingAttemptReason.AUTOMATICALLY_CANCELLED,
    )
    assert isinstance(
        rep,
        invited_cmds.latest.invite_claimer_cancel_greeting_attempt.RepGreetingAttemptAlreadyCancelled,
    )
    assert (
        rep
        == invited_cmds.latest.invite_claimer_cancel_greeting_attempt.RepGreetingAttemptAlreadyCancelled(
            timestamp=rep.timestamp,
            reason=reason,
            origin=GreeterOrClaimer.CLAIMER,
        )
    )


async def test_invited_invite_claimer_cancel_greeting_attempt_with_shamir_deleted(
    shamirorg: ShamirOrgRpcClients,
    invited_greeting_with_deleted_shamir_tester: HttpCommonErrorsTester,
) -> None:
    rep = await shamirorg.shamir_invited_alice.invite_claimer_start_greeting_attempt(
        greeter=shamirorg.bob.user_id,
    )
    assert isinstance(rep, invited_cmds.latest.invite_claimer_start_greeting_attempt.RepOk)

    async def do():
        await shamirorg.shamir_invited_alice.invite_claimer_cancel_greeting_attempt(
            greeting_attempt=rep.greeting_attempt,
            reason=CancelledGreetingAttemptReason.MANUALLY_CANCELLED,
        )

    await invited_greeting_with_deleted_shamir_tester(do)


async def test_invited_invite_claimer_cancel_greeting_attempt_http_common_errors(
    coolorg: CoolorgRpcClients,
    greeting_attempt: GreetingAttemptID,
    invited_http_common_errors_tester: HttpCommonErrorsTester,
) -> None:
    async def do():
        await coolorg.invited_alice_dev3.invite_claimer_cancel_greeting_attempt(
            greeting_attempt=greeting_attempt,
            reason=CancelledGreetingAttemptReason.MANUALLY_CANCELLED,
        )

    await invited_http_common_errors_tester(do)
