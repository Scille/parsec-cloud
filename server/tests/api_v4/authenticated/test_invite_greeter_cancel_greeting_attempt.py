# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from unittest.mock import ANY

import pytest

from parsec._parsec import (
    CancelledGreetingAttemptReason,
    GreeterOrClaimer,
    GreetingAttemptID,
    authenticated_cmds,
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


@pytest.mark.skip("Not implemented yet")
async def test_authenticated_invite_greeter_cancel_greeting_attempt_ok(
    coolorg: CoolorgRpcClients, backend: Backend, greeting_attempt: GreetingAttemptID
) -> None:
    rep = await coolorg.alice.invite_greeter_cancel_greeting_attempt(
        greeting_attempt=greeting_attempt,
        reason=CancelledGreetingAttemptReason.MANUALLY_CANCELLED,
    )
    assert rep == authenticated_cmds.v4.invite_greeter_cancel_greeting_attempt.RepOk()


@pytest.mark.skip("Not implemented yet")
async def test_authenticated_invite_greeter_cancel_greeting_attempt_author_not_allowed(
    coolorg: CoolorgRpcClients, greeting_attempt: GreetingAttemptID
) -> None:
    rep = await coolorg.bob.invite_greeter_cancel_greeting_attempt(
        greeting_attempt=greeting_attempt,
        reason=CancelledGreetingAttemptReason.MANUALLY_CANCELLED,
    )

    assert rep == authenticated_cmds.v4.invite_greeter_cancel_greeting_attempt.RepAuthorNotAllowed()


@pytest.mark.skip("Not implemented yet")
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


@pytest.mark.skip("Not implemented yet")
async def test_authenticated_invite_greeter_cancel_greeting_attempt_invitation_completed(
    coolorg: CoolorgRpcClients, greeting_attempt: GreetingAttemptID
) -> None:
    # Not implemented yet
    # await coolorg.alice.invite_complete(token=coolorg.invited_alice_dev3.token)

    rep = await coolorg.alice.invite_greeter_cancel_greeting_attempt(
        greeting_attempt=greeting_attempt,
        reason=CancelledGreetingAttemptReason.MANUALLY_CANCELLED,
    )

    assert (
        rep == authenticated_cmds.v4.invite_greeter_cancel_greeting_attempt.RepInvitationCompleted()
    )


@pytest.mark.skip("Not implemented yet")
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


@pytest.mark.skip("Not implemented yet")
async def test_authenticated_invite_greeter_cancel_greeting_attempt_greeting_attempt_not_joined(
    coolorg: CoolorgRpcClients,
) -> None:
    # TODO: Have the claimer join the greeting attempt and use the corresponding ID
    greeting_attempt = GreetingAttemptID.new()

    rep = await coolorg.alice.invite_greeter_cancel_greeting_attempt(
        greeting_attempt=greeting_attempt,
        reason=CancelledGreetingAttemptReason.MANUALLY_CANCELLED,
    )

    assert (
        rep
        == authenticated_cmds.v4.invite_greeter_cancel_greeting_attempt.RepGreetingAttemptNotJoined()
    )


@pytest.mark.skip("Not implemented yet")
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
    assert (
        rep
        == authenticated_cmds.v4.invite_greeter_cancel_greeting_attempt.RepGreetingAttemptAlreadyCancelled(
            timestamp=ANY,
            reason=CancelledGreetingAttemptReason.MANUALLY_CANCELLED,
            origin=GreeterOrClaimer.GREETER,
        )
    )
