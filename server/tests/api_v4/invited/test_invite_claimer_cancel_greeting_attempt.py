# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from unittest.mock import ANY

import pytest

from parsec._parsec import (
    CancelledGreetingAttemptReason,
    GreeterOrClaimer,
    GreetingAttemptID,
    invited_cmds,
)
from tests.common import Backend, CoolorgRpcClients


@pytest.fixture
async def greeting_attempt(coolorg: CoolorgRpcClients, backend: Backend) -> GreetingAttemptID:
    invitation_token = coolorg.invited_alice_dev3.token

    rep = await coolorg.invited_alice_dev3.invite_claimer_start_greeting_attempt(
        token=invitation_token,
        greeter=coolorg.alice.user_id,
    )
    assert isinstance(rep, invited_cmds.v4.invite_claimer_start_greeting_attempt.RepOk)
    return rep.greeting_attempt


@pytest.mark.skip("Not implemented yet")
async def test_invited_invite_claimer_cancel_greeting_attempt_ok(
    coolorg: CoolorgRpcClients, backend: Backend, greeting_attempt: GreetingAttemptID
) -> None:
    rep = await coolorg.invited_alice_dev3.invite_claimer_cancel_greeting_attempt(
        greeting_attempt=greeting_attempt,
        reason=CancelledGreetingAttemptReason.MANUALLY_CANCELLED,
    )
    assert rep == invited_cmds.v4.invite_claimer_cancel_greeting_attempt.RepOk()


@pytest.mark.skip("Not implemented yet")
async def test_invited_invite_claimer_cancel_greeting_attempt_greeter_not_allowed(
    coolorg: CoolorgRpcClients, greeting_attempt: GreetingAttemptID
) -> None:
    # TODO: Use a user invitation and change the greeter profile to STANDARD

    rep = await coolorg.invited_alice_dev3.invite_claimer_cancel_greeting_attempt(
        greeting_attempt=greeting_attempt,
        reason=CancelledGreetingAttemptReason.MANUALLY_CANCELLED,
    )

    assert rep == invited_cmds.v4.invite_claimer_cancel_greeting_attempt.RepGreeterNotAllowed()


@pytest.mark.skip("Not implemented yet")
async def test_invited_invite_claimer_cancel_greeting_attempt_greeter_revoked(
    coolorg: CoolorgRpcClients, greeting_attempt: GreetingAttemptID
) -> None:
    # TODO: Revoke the greeter

    rep = await coolorg.invited_alice_dev3.invite_claimer_cancel_greeting_attempt(
        greeting_attempt=greeting_attempt,
        reason=CancelledGreetingAttemptReason.MANUALLY_CANCELLED,
    )
    assert rep == invited_cmds.v4.invite_claimer_cancel_greeting_attempt.RepGreeterRevoked()


@pytest.mark.skip("Not implemented yet")
async def test_invited_invite_claimer_cancel_greeting_attempt_greeting_attempt_not_found(
    coolorg: CoolorgRpcClients,
) -> None:
    rep = await coolorg.invited_alice_dev3.invite_claimer_cancel_greeting_attempt(
        greeting_attempt=GreetingAttemptID.new(),
        reason=CancelledGreetingAttemptReason.MANUALLY_CANCELLED,
    )

    assert (
        rep == invited_cmds.v4.invite_claimer_cancel_greeting_attempt.RepGreetingAttemptNotFound()
    )


@pytest.mark.skip("Not implemented yet")
async def test_invited_invite_claimer_cancel_greeting_attempt_greeting_attempt_not_joined(
    coolorg: CoolorgRpcClients,
) -> None:
    # TODO: Have the greeter join the greeting attempt and use the corresponding ID
    greeting_attempt = GreetingAttemptID.new()

    rep = await coolorg.invited_alice_dev3.invite_claimer_cancel_greeting_attempt(
        greeting_attempt=greeting_attempt,
        reason=CancelledGreetingAttemptReason.MANUALLY_CANCELLED,
    )

    assert (
        rep == invited_cmds.v4.invite_claimer_cancel_greeting_attempt.RepGreetingAttemptNotJoined()
    )


@pytest.mark.skip("Not implemented yet")
async def test_invited_invite_claimer_cancel_greeting_attempt_greeting_attempt_already_cancelled(
    coolorg: CoolorgRpcClients, greeting_attempt: GreetingAttemptID
) -> None:
    # Cancel once
    rep = await coolorg.invited_alice_dev3.invite_claimer_cancel_greeting_attempt(
        greeting_attempt=greeting_attempt,
        reason=CancelledGreetingAttemptReason.MANUALLY_CANCELLED,
    )
    assert rep == invited_cmds.v4.invite_claimer_cancel_greeting_attempt.RepOk()

    # Cancel again
    rep = await coolorg.invited_alice_dev3.invite_claimer_cancel_greeting_attempt(
        greeting_attempt=greeting_attempt,
        reason=CancelledGreetingAttemptReason.MANUALLY_CANCELLED,
    )
    assert (
        rep
        == invited_cmds.v4.invite_claimer_cancel_greeting_attempt.RepGreetingAttemptAlreadyCancelled(
            timestamp=ANY,
            reason=CancelledGreetingAttemptReason.MANUALLY_CANCELLED,
            origin=GreeterOrClaimer.CLAIMER,
        )
    )
