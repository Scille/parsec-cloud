# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


import pytest

from parsec._parsec import (
    CancelledGreetingAttemptReason,
    GreeterOrClaimer,
    GreetingAttemptID,
    authenticated_cmds,
    invited_cmds,
)
from tests.common import Backend, CoolorgRpcClients

Response = authenticated_cmds.v4.invite_greeter_cancel_greeting_attempt.Rep | None


@pytest.fixture
async def greeting_attempt(coolorg: CoolorgRpcClients, backend: Backend) -> GreetingAttemptID:
    invitation_token = coolorg.invited_alice_dev3.token

    rep = await coolorg.alice.invite_greeter_start_greeting_attempt(
        token=invitation_token,
    )
    assert isinstance(rep, authenticated_cmds.v4.invite_greeter_start_greeting_attempt.RepOk)
    return rep.greeting_attempt


async def test_authenticated_invite_greeter_cancel_greeting_attempt_ok(
    coolorg: CoolorgRpcClients, backend: Backend, greeting_attempt: GreetingAttemptID
) -> None:
    rep = await coolorg.alice.invite_greeter_cancel_greeting_attempt(
        greeting_attempt=greeting_attempt,
        reason=CancelledGreetingAttemptReason.MANUALLY_CANCELLED,
    )
    assert rep == authenticated_cmds.v4.invite_greeter_cancel_greeting_attempt.RepOk()


async def test_authenticated_invite_greeter_cancel_greeting_attempt_author_not_allowed(
    coolorg: CoolorgRpcClients, greeting_attempt: GreetingAttemptID
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
    coolorg: CoolorgRpcClients, greeting_attempt: GreetingAttemptID
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
        token=coolorg.invited_alice_dev3.token,
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
