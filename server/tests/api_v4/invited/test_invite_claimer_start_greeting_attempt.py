# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from parsec._parsec import (
    CancelledGreetingAttemptReason,
    DateTime,
    GreeterOrClaimer,
    GreetingAttemptID,
    RevokedUserCertificate,
    UserID,
    UserProfile,
    UserUpdateCertificate,
    authenticated_cmds,
    invited_cmds,
)
from tests.common import Backend, CoolorgRpcClients, HttpCommonErrorsTester


async def test_invited_invite_claimer_start_greeting_attempt_ok(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    # This is a scenario where the claimer starts first.
    # See `test_authenticated_invite_greeter_start_greeting_attempt_ok` for the opposite scenario.
    invitation_token = coolorg.invited_alice_dev3.token

    # Claimer goes first
    rep = await coolorg.invited_alice_dev3.invite_claimer_start_greeting_attempt(
        greeter=coolorg.alice.user_id,
    )
    assert isinstance(rep, invited_cmds.v4.invite_claimer_start_greeting_attempt.RepOk)
    assert isinstance(rep.greeting_attempt, GreetingAttemptID)
    first_greeting_attempt = rep.greeting_attempt

    # Greeter goes second, getting the same greeting attempt
    rep = await coolorg.alice.invite_greeter_start_greeting_attempt(
        token=invitation_token,
    )
    assert isinstance(rep, authenticated_cmds.v4.invite_greeter_start_greeting_attempt.RepOk)
    assert rep.greeting_attempt == first_greeting_attempt

    # Claimer starts again, getting a second greeting attempt
    rep = await coolorg.invited_alice_dev3.invite_claimer_start_greeting_attempt(
        greeter=coolorg.alice.user_id,
    )
    assert isinstance(rep, invited_cmds.v4.invite_claimer_start_greeting_attempt.RepOk)
    assert isinstance(rep.greeting_attempt, GreetingAttemptID)
    second_greeting_attempt = rep.greeting_attempt
    assert second_greeting_attempt != first_greeting_attempt

    # First attempt has been properly cancelled
    rep = await coolorg.alice.invite_greeter_cancel_greeting_attempt(
        first_greeting_attempt,
        reason=CancelledGreetingAttemptReason.MANUALLY_CANCELLED,
    )
    assert isinstance(
        rep,
        authenticated_cmds.v4.invite_greeter_cancel_greeting_attempt.RepGreetingAttemptAlreadyCancelled,
    )
    assert (
        rep
        == authenticated_cmds.v4.invite_greeter_cancel_greeting_attempt.RepGreetingAttemptAlreadyCancelled(
            origin=GreeterOrClaimer.CLAIMER,
            reason=CancelledGreetingAttemptReason.AUTOMATICALLY_CANCELLED,
            timestamp=rep.timestamp,
        )
    )

    # Greeter follows, getting the second greeting attempt
    rep = await coolorg.alice.invite_greeter_start_greeting_attempt(
        token=invitation_token,
    )
    assert isinstance(rep, authenticated_cmds.v4.invite_greeter_start_greeting_attempt.RepOk)
    assert rep.greeting_attempt == second_greeting_attempt

    # Greeter starts again, getting a third greeting attempt
    rep = await coolorg.alice.invite_greeter_start_greeting_attempt(
        token=invitation_token,
    )
    assert isinstance(rep, authenticated_cmds.v4.invite_greeter_start_greeting_attempt.RepOk)
    assert isinstance(rep.greeting_attempt, GreetingAttemptID)
    third_greeting_attempt = rep.greeting_attempt
    assert third_greeting_attempt != first_greeting_attempt
    assert third_greeting_attempt != second_greeting_attempt

    # Second attempt has been properly cancelled
    rep = await coolorg.invited_alice_dev3.invite_claimer_cancel_greeting_attempt(
        second_greeting_attempt,
        reason=CancelledGreetingAttemptReason.MANUALLY_CANCELLED,
    )
    assert isinstance(
        rep,
        invited_cmds.v4.invite_claimer_cancel_greeting_attempt.RepGreetingAttemptAlreadyCancelled,
    )
    assert (
        rep
        == invited_cmds.v4.invite_claimer_cancel_greeting_attempt.RepGreetingAttemptAlreadyCancelled(
            origin=GreeterOrClaimer.GREETER,
            reason=CancelledGreetingAttemptReason.AUTOMATICALLY_CANCELLED,
            timestamp=rep.timestamp,
        )
    )

    # Claimer follows, getting the third greeting attempt
    rep = await coolorg.invited_alice_dev3.invite_claimer_start_greeting_attempt(
        greeter=coolorg.alice.user_id,
    )
    assert isinstance(rep, invited_cmds.v4.invite_claimer_start_greeting_attempt.RepOk)
    assert rep.greeting_attempt == third_greeting_attempt


async def test_invited_invite_claimer_start_greeting_attempt_greeter_not_found(
    coolorg: CoolorgRpcClients,
) -> None:
    rep = await coolorg.invited_alice_dev3.invite_claimer_start_greeting_attempt(
        greeter=UserID.new(),
    )

    assert rep == invited_cmds.v4.invite_claimer_start_greeting_attempt.RepGreeterNotFound()


async def test_invited_invite_claimer_start_greeting_attempt_greeter_not_allowed(
    coolorg: CoolorgRpcClients,
) -> None:
    rep = await coolorg.invited_alice_dev3.invite_claimer_start_greeting_attempt(
        greeter=coolorg.bob.user_id,
    )

    assert rep == invited_cmds.v4.invite_claimer_start_greeting_attempt.RepGreeterNotAllowed()


async def test_invited_invite_claimer_start_greeting_attempt_greeter_revoked(
    coolorg: CoolorgRpcClients,
) -> None:
    # Make Bob an admin
    certif = UserUpdateCertificate(
        author=coolorg.alice.device_id,
        new_profile=UserProfile.ADMIN,
        user_id=coolorg.bob.user_id,
        timestamp=DateTime.now(),
    ).dump_and_sign(coolorg.alice.signing_key)
    rep = await coolorg.alice.user_update(certif)
    assert rep == authenticated_cmds.v4.user_update.RepOk()

    # Revoke Alice
    now = DateTime.now()
    certif = RevokedUserCertificate(
        author=coolorg.bob.device_id,
        timestamp=now,
        user_id=coolorg.alice.user_id,
    ).dump_and_sign(coolorg.bob.signing_key)
    rep = await coolorg.bob.user_revoke(certif)
    assert rep == authenticated_cmds.v4.user_revoke.RepOk()

    # Try to invite
    rep = await coolorg.invited_alice_dev3.invite_claimer_start_greeting_attempt(
        greeter=coolorg.alice.user_id,
    )

    assert rep == invited_cmds.v4.invite_claimer_start_greeting_attempt.RepGreeterRevoked()


async def test_invited_invite_claimer_start_greeting_attempt_http_common_errors(
    coolorg: CoolorgRpcClients, invited_http_common_errors_tester: HttpCommonErrorsTester
) -> None:
    async def do():
        await coolorg.invited_alice_dev3.invite_claimer_start_greeting_attempt(
            greeter=coolorg.alice.user_id,
        )

    await invited_http_common_errors_tester(do)
