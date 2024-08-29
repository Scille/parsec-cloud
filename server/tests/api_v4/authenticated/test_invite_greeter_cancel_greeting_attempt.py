# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


import pytest

from parsec._parsec import (
    CancelledGreetingAttemptReason,
    DateTime,
    GreeterOrClaimer,
    GreetingAttemptID,
    authenticated_cmds,
    invited_cmds,
)
from tests.common import Backend, CoolorgRpcClients, HttpCommonErrorsTester

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


async def test_authenticated_invite_greeter_cancel_greeting_attempt_ok(
    coolorg: CoolorgRpcClients, greeting_attempt: GreetingAttemptID
) -> None:
    rep = await coolorg.alice.invite_greeter_cancel_greeting_attempt(
        greeting_attempt=greeting_attempt,
        reason=CancelledGreetingAttemptReason.MANUALLY_CANCELLED,
    )
    assert rep == authenticated_cmds.v4.invite_greeter_cancel_greeting_attempt.RepOk()


async def test_authenticated_invite_greeter_cancel_greeting_attempt_author_not_allowed(
    coolorg: CoolorgRpcClients,
    greeting_attempt: GreetingAttemptID,
) -> None:
    rep = await coolorg.bob.invite_greeter_cancel_greeting_attempt(
        greeting_attempt=greeting_attempt,
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
    coolorg: CoolorgRpcClients,
) -> None:
    rep = await coolorg.alice.invite_greeter_cancel_greeting_attempt(
        greeting_attempt=GreetingAttemptID.new(),
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


async def test_authenticated_invite_greeter_cancel_greeting_attempt_greeting_attempt_already_cancelled(
    coolorg: CoolorgRpcClients, greeting_attempt: GreetingAttemptID
) -> None:
    # Cancel once
    rep = await coolorg.alice.invite_greeter_cancel_greeting_attempt(
        greeting_attempt=greeting_attempt,
        reason=CancelledGreetingAttemptReason.MANUALLY_CANCELLED,
    )
    assert rep == authenticated_cmds.v4.invite_greeter_cancel_greeting_attempt.RepOk()

    # Cancel again
    rep = await coolorg.alice.invite_greeter_cancel_greeting_attempt(
        greeting_attempt=greeting_attempt,
        reason=CancelledGreetingAttemptReason.MANUALLY_CANCELLED,
    )

    assert isinstance(
        rep,
        authenticated_cmds.v4.invite_greeter_cancel_greeting_attempt.RepGreetingAttemptAlreadyCancelled,
    )
    assert (
        rep
        == authenticated_cmds.v4.invite_greeter_cancel_greeting_attempt.RepGreetingAttemptAlreadyCancelled(
            timestamp=rep.timestamp,
            reason=CancelledGreetingAttemptReason.MANUALLY_CANCELLED,
            origin=GreeterOrClaimer.GREETER,
        )
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
