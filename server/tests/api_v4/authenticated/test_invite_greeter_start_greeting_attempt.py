# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from parsec._parsec import (
    CancelledGreetingAttemptReason,
    GreeterOrClaimer,
    GreetingAttemptID,
    InvitationToken,
    authenticated_cmds,
    invited_cmds,
)
from tests.common import CoolorgRpcClients, HttpCommonErrorsTester

Response = authenticated_cmds.v4.invite_greeter_start_greeting_attempt.Rep | None


async def test_authenticated_invite_greeter_start_greeting_attempt_ok(
    coolorg: CoolorgRpcClients,
) -> None:
    # This is a scenario where the greeter starts first.
    # See `test_invited_invite_claimer_start_greeting_attempt_ok` for the opposite scenario.
    invitation_token = coolorg.invited_alice_dev3.token

    # Greeter goes first
    rep = await coolorg.alice.invite_greeter_start_greeting_attempt(
        token=invitation_token,
    )
    assert isinstance(rep, authenticated_cmds.v4.invite_greeter_start_greeting_attempt.RepOk)
    assert isinstance(rep.greeting_attempt, GreetingAttemptID)
    first_greeting_attempt = rep.greeting_attempt

    # Claimer goes second, getting the same greeting attempt
    rep = await coolorg.invited_alice_dev3.invite_claimer_start_greeting_attempt(
        greeter=coolorg.alice.user_id,
    )
    assert isinstance(rep, invited_cmds.v4.invite_claimer_start_greeting_attempt.RepOk)
    assert rep.greeting_attempt == first_greeting_attempt

    # Greeter starts again, getting a second greeting attempt
    rep = await coolorg.alice.invite_greeter_start_greeting_attempt(
        token=invitation_token,
    )
    assert isinstance(rep, authenticated_cmds.v4.invite_greeter_start_greeting_attempt.RepOk)
    assert isinstance(rep.greeting_attempt, GreetingAttemptID)
    second_greeting_attempt = rep.greeting_attempt
    assert second_greeting_attempt != first_greeting_attempt

    # First attempt has been properly cancelled
    rep = await coolorg.invited_alice_dev3.invite_claimer_cancel_greeting_attempt(
        first_greeting_attempt,
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

    # Claimer follows, getting the second greeting attempt
    rep = await coolorg.invited_alice_dev3.invite_claimer_start_greeting_attempt(
        greeter=coolorg.alice.user_id,
    )
    assert isinstance(rep, invited_cmds.v4.invite_claimer_start_greeting_attempt.RepOk)
    assert rep.greeting_attempt == second_greeting_attempt

    # Claimer starts again, getting a third greeting attempt
    rep = await coolorg.invited_alice_dev3.invite_claimer_start_greeting_attempt(
        greeter=coolorg.alice.user_id,
    )
    assert isinstance(rep, invited_cmds.v4.invite_claimer_start_greeting_attempt.RepOk)
    assert isinstance(rep.greeting_attempt, GreetingAttemptID)
    third_greeting_attempt = rep.greeting_attempt
    assert third_greeting_attempt != first_greeting_attempt
    assert third_greeting_attempt != second_greeting_attempt

    # Second attempt has been properly cancelled
    rep = await coolorg.alice.invite_greeter_cancel_greeting_attempt(
        second_greeting_attempt,
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

    # Greeter follows, getting the third greeting attempt
    rep = await coolorg.alice.invite_greeter_start_greeting_attempt(
        token=invitation_token,
    )
    assert isinstance(rep, authenticated_cmds.v4.invite_greeter_start_greeting_attempt.RepOk)
    assert rep.greeting_attempt == third_greeting_attempt


async def test_authenticated_invite_greeter_start_greeting_attempt_invitation_not_found(
    coolorg: CoolorgRpcClients,
) -> None:
    rep = await coolorg.alice.invite_greeter_start_greeting_attempt(
        token=InvitationToken.new(),
    )

    assert (
        rep == authenticated_cmds.v4.invite_greeter_start_greeting_attempt.RepInvitationNotFound()
    )


async def test_authenticated_invite_greeter_start_greeting_attempt_invitation_cancelled(
    coolorg: CoolorgRpcClients,
) -> None:
    await coolorg.alice.invite_cancel(token=coolorg.invited_alice_dev3.token)

    rep = await coolorg.alice.invite_greeter_start_greeting_attempt(
        token=coolorg.invited_alice_dev3.token,
    )

    assert (
        rep == authenticated_cmds.v4.invite_greeter_start_greeting_attempt.RepInvitationCancelled()
    )


async def test_authenticated_invite_greeter_start_greeting_attempt_invitation_completed(
    coolorg: CoolorgRpcClients,
) -> None:
    await coolorg.alice.invite_complete(token=coolorg.invited_alice_dev3.token)

    rep = await coolorg.alice.invite_greeter_start_greeting_attempt(
        token=coolorg.invited_alice_dev3.token,
    )

    assert (
        rep == authenticated_cmds.v4.invite_greeter_start_greeting_attempt.RepInvitationCompleted()
    )


async def test_authenticated_invite_greeter_start_greeting_attempt_author_not_allowed(
    coolorg: CoolorgRpcClients,
) -> None:
    rep = await coolorg.bob.invite_greeter_start_greeting_attempt(
        token=coolorg.invited_alice_dev3.token,
    )

    assert rep == authenticated_cmds.v4.invite_greeter_start_greeting_attempt.RepAuthorNotAllowed()


async def test_authenticated_invite_greeter_start_greeting_attempt_http_common_errors(
    coolorg: CoolorgRpcClients, authenticated_http_common_errors_tester: HttpCommonErrorsTester
) -> None:
    async def do():
        await coolorg.alice.invite_greeter_start_greeting_attempt(
            token=coolorg.invited_alice_dev3.token,
        )

    await authenticated_http_common_errors_tester(do)
