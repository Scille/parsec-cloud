# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from unittest.mock import ANY

import pytest

from parsec._parsec import (
    CancelledGreetingAttemptReason,
    GreeterOrClaimer,
    GreetingAttemptID,
    PrivateKey,
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


async def test_invited_invite_claimer_step_ok(
    coolorg: CoolorgRpcClients, backend: Backend, greeting_attempt: GreetingAttemptID
) -> None:
    greeter_key = PrivateKey.generate()
    claimer_key = PrivateKey.generate()
    greeter_step = invited_cmds.v4.invite_claimer_step.GreeterStepWaitPeer(
        public_key=greeter_key.public_key
    )
    claimer_step = invited_cmds.v4.invite_claimer_step.ClaimerStepWaitPeer(
        public_key=claimer_key.public_key
    )

    # TODO: Have the claimer perform the WaitPeer step

    # Run once
    rep = await coolorg.invited_alice_dev3.invite_claimer_step(
        greeting_attempt=greeting_attempt, claimer_step=claimer_step
    )
    assert rep == invited_cmds.v4.invite_claimer_step.RepOk(greeter_step=greeter_step)

    # Run once more
    rep = await coolorg.invited_alice_dev3.invite_claimer_step(
        greeting_attempt=greeting_attempt, claimer_step=claimer_step
    )
    assert rep == invited_cmds.v4.invite_claimer_step.RepOk(greeter_step=greeter_step)


async def test_invited_invite_claimer_step_greeter_not_allowed(
    coolorg: CoolorgRpcClients, greeting_attempt: GreetingAttemptID
) -> None:
    # TODO: Use a user invitation and change the greeter profile to STANDARD

    rep = await coolorg.invited_alice_dev3.invite_claimer_step(
        greeting_attempt=greeting_attempt,
        claimer_step=invited_cmds.v4.invite_claimer_step.ClaimerStepGetNonce(),
    )

    assert rep == invited_cmds.v4.invite_claimer_step.RepGreeterNotAllowed()


async def test_invited_invite_claimer_step_greeter_revoked(
    coolorg: CoolorgRpcClients, greeting_attempt: GreetingAttemptID
) -> None:
    # TODO: Use a user invitation and revoked the greeter

    rep = await coolorg.invited_alice_dev3.invite_claimer_step(
        greeting_attempt=greeting_attempt,
        claimer_step=invited_cmds.v4.invite_claimer_step.ClaimerStepGetNonce(),
    )

    assert rep == invited_cmds.v4.invite_claimer_step.RepGreeterRevoked()


async def test_invited_invite_claimer_step_greeting_attempt_not_found(
    coolorg: CoolorgRpcClients, greeting_attempt: GreetingAttemptID
) -> None:
    rep = await coolorg.invited_alice_dev3.invite_claimer_step(
        greeting_attempt=GreetingAttemptID.new(),
        claimer_step=invited_cmds.v4.invite_claimer_step.ClaimerStepGetNonce(),
    )

    assert rep == invited_cmds.v4.invite_claimer_step.RepGreetingAttemptNotFound()


async def test_invited_invite_claimer_step_greeting_attempt_not_joined(
    coolorg: CoolorgRpcClients,
) -> None:
    # TODO: Have the greeter join the greeting attempt and use the corresponding ID
    greeting_attempt = GreetingAttemptID.new()

    rep = await coolorg.invited_alice_dev3.invite_claimer_step(
        greeting_attempt=greeting_attempt,
        claimer_step=invited_cmds.v4.invite_claimer_step.ClaimerStepGetNonce(),
    )

    assert rep == invited_cmds.v4.invite_claimer_step.RepGreetingAttemptNotJoined()


async def test_invited_invite_claimer_step_greeting_attempt_cancelled(
    coolorg: CoolorgRpcClients, greeting_attempt: GreetingAttemptID
) -> None:
    # Cancel the attempt
    rep = await coolorg.invited_alice_dev3.invite_claimer_cancel_greeting_attempt(
        greeting_attempt=greeting_attempt,
        reason=CancelledGreetingAttemptReason.MANUALLY_CANCELLED,
    )
    assert rep == invited_cmds.v4.invite_claimer_cancel_greeting_attempt.RepOk()

    # Try a step
    rep = await coolorg.invited_alice_dev3.invite_claimer_step(
        greeting_attempt=greeting_attempt,
        claimer_step=invited_cmds.v4.invite_claimer_step.ClaimerStepGetNonce(),
    )
    assert rep == invited_cmds.v4.invite_claimer_step.RepGreetingAttemptCancelled(
        timestamp=ANY,
        reason=CancelledGreetingAttemptReason.MANUALLY_CANCELLED,
        origin=GreeterOrClaimer.GREETER,
    )


async def test_invited_invite_claimer_step_step_mismatch(
    coolorg: CoolorgRpcClients, greeting_attempt: GreetingAttemptID
) -> None:
    greeter_key_1 = PrivateKey.generate()
    greeter_key_2 = PrivateKey.generate()
    claimer_step_1 = invited_cmds.v4.invite_claimer_step.ClaimerStepWaitPeer(
        public_key=greeter_key_1.public_key
    )
    claimer_step_2 = invited_cmds.v4.invite_claimer_step.ClaimerStepWaitPeer(
        public_key=greeter_key_2.public_key
    )
    # Run once with first public key
    rep = await coolorg.invited_alice_dev3.invite_claimer_step(
        greeting_attempt=greeting_attempt, claimer_step=claimer_step_1
    )
    assert rep == invited_cmds.v4.invite_claimer_step.RepNotReady()
    # Run once more with second public key
    rep = await coolorg.invited_alice_dev3.invite_claimer_step(
        greeting_attempt=greeting_attempt, claimer_step=claimer_step_2
    )
    assert rep == invited_cmds.v4.invite_claimer_step.RepStepMismatch()


async def test_invited_invite_claimer_step_step_too_advanced(
    coolorg: CoolorgRpcClients, greeting_attempt: GreetingAttemptID
) -> None:
    rep = await coolorg.invited_alice_dev3.invite_claimer_step(
        greeting_attempt=greeting_attempt,
        claimer_step=invited_cmds.v4.invite_claimer_step.ClaimerStepGetNonce(),
    )
    assert rep == invited_cmds.v4.invite_claimer_step.RepStepTooAdvanced()


async def test_invited_invite_claimer_step_not_ready(
    coolorg: CoolorgRpcClients, greeting_attempt: GreetingAttemptID
) -> None:
    greeter_key = PrivateKey.generate()
    claimer_step = invited_cmds.v4.invite_claimer_step.ClaimerStepWaitPeer(
        public_key=greeter_key.public_key
    )
    # Run once
    rep = await coolorg.invited_alice_dev3.invite_claimer_step(
        greeting_attempt=greeting_attempt, claimer_step=claimer_step
    )
    assert rep == invited_cmds.v4.invite_claimer_step.RepNotReady()
    # Run twice
    rep = await coolorg.invited_alice_dev3.invite_claimer_step(
        greeting_attempt=greeting_attempt,
        claimer_step=claimer_step,
    )
    assert rep == invited_cmds.v4.invite_claimer_step.RepNotReady()
