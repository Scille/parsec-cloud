# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from unittest.mock import ANY

import pytest

from parsec._parsec import (
    CancelledGreetingAttemptReason,
    GreeterOrClaimer,
    GreetingAttemptID,
    PrivateKey,
    authenticated_cmds,
)
from tests.common import Backend, CoolorgRpcClients


@pytest.fixture
async def greeting_attempt(coolorg: CoolorgRpcClients, backend: Backend) -> GreetingAttemptID:
    invitation_token = coolorg.invited_alice_dev3.token

    rep = await coolorg.alice.invite_greeter_start_greeting_attempt(
        token=invitation_token,
    )
    assert isinstance(rep, authenticated_cmds.v4.invite_greeter_start_greeting_attempt.RepOk)
    return rep.greeting_attempt


@pytest.mark.skip("Not implemented yet")
async def test_authenticated_invite_greeter_step_ok(
    coolorg: CoolorgRpcClients, backend: Backend, greeting_attempt: GreetingAttemptID
) -> None:
    greeter_key = PrivateKey.generate()
    claimer_key = PrivateKey.generate()
    greeter_step = authenticated_cmds.v4.invite_greeter_step.GreeterStepWaitPeer(
        public_key=greeter_key.public_key
    )
    claimer_step = authenticated_cmds.v4.invite_greeter_step.ClaimerStepWaitPeer(
        public_key=claimer_key.public_key
    )

    # TODO: Have the claimer perform the WaitPeer step

    # Run once
    rep = await coolorg.alice.invite_greeter_step(
        greeting_attempt=greeting_attempt, greeter_step=greeter_step
    )
    assert rep == authenticated_cmds.v4.invite_greeter_step.RepOk(claimer_step=claimer_step)

    # Run once more
    rep = await coolorg.alice.invite_greeter_step(
        greeting_attempt=greeting_attempt, greeter_step=greeter_step
    )
    assert rep == authenticated_cmds.v4.invite_greeter_step.RepOk(claimer_step=claimer_step)


@pytest.mark.skip("Not implemented yet")
async def test_authenticated_invite_greeter_step_author_not_allowed(
    coolorg: CoolorgRpcClients, greeting_attempt: GreetingAttemptID
) -> None:
    rep = await coolorg.bob.invite_greeter_step(
        greeting_attempt=greeting_attempt,
        greeter_step=authenticated_cmds.v4.invite_greeter_step.GreeterStepGetNonce(),
    )

    assert rep == authenticated_cmds.v4.invite_greeter_step.RepAuthorNotAllowed()


@pytest.mark.skip("Not implemented yet")
async def test_authenticated_invite_greeter_step_invitation_cancelled(
    coolorg: CoolorgRpcClients, greeting_attempt: GreetingAttemptID
) -> None:
    await coolorg.alice.invite_cancel(token=coolorg.invited_alice_dev3.token)

    rep = await coolorg.alice.invite_greeter_step(
        greeting_attempt=greeting_attempt,
        greeter_step=authenticated_cmds.v4.invite_greeter_step.GreeterStepGetNonce(),
    )
    assert rep == authenticated_cmds.v4.invite_greeter_step.RepInvitationCancelled()


@pytest.mark.skip("Not implemented yet")
async def test_authenticated_invite_greeter_step_invitation_completed(
    coolorg: CoolorgRpcClients, greeting_attempt: GreetingAttemptID
) -> None:
    # Not implemented yet
    # await coolorg.alice.invite_complete(token=coolorg.invited_alice_dev3.token)

    rep = await coolorg.alice.invite_greeter_step(
        greeting_attempt=greeting_attempt,
        greeter_step=authenticated_cmds.v4.invite_greeter_step.GreeterStepGetNonce(),
    )

    assert rep == authenticated_cmds.v4.invite_greeter_step.RepInvitationCompleted()


@pytest.mark.skip("Not implemented yet")
async def test_authenticated_invite_greeter_step_greeting_attempt_not_found(
    coolorg: CoolorgRpcClients, greeting_attempt: GreetingAttemptID
) -> None:
    rep = await coolorg.alice.invite_greeter_step(
        greeting_attempt=GreetingAttemptID.new(),
        greeter_step=authenticated_cmds.v4.invite_greeter_step.GreeterStepGetNonce(),
    )

    assert rep == authenticated_cmds.v4.invite_greeter_step.RepGreetingAttemptNotFound()


@pytest.mark.skip("Not implemented yet")
async def test_authenticated_invite_greeter_step_greeting_attempt_not_joined(
    coolorg: CoolorgRpcClients,
) -> None:
    # TODO: Have the claimer join the greeting attempt and use the corresponding ID
    greeting_attempt = GreetingAttemptID.new()

    rep = await coolorg.alice.invite_greeter_step(
        greeting_attempt=greeting_attempt,
        greeter_step=authenticated_cmds.v4.invite_greeter_step.GreeterStepGetNonce(),
    )

    assert rep == authenticated_cmds.v4.invite_greeter_step.RepGreetingAttemptNotJoined()


@pytest.mark.skip("Not implemented yet")
async def test_authenticated_invite_greeter_step_greeting_attempt_cancelled(
    coolorg: CoolorgRpcClients, greeting_attempt: GreetingAttemptID
) -> None:
    # Cancel the attempt
    rep = await coolorg.alice.invite_greeter_cancel_greeting_attempt(
        greeting_attempt=greeting_attempt,
        reason=CancelledGreetingAttemptReason.MANUALLY_CANCELLED,
    )
    assert rep == authenticated_cmds.v4.invite_greeter_cancel_greeting_attempt.RepOk()

    # Try a step
    rep = await coolorg.alice.invite_greeter_step(
        greeting_attempt=greeting_attempt,
        greeter_step=authenticated_cmds.v4.invite_greeter_step.GreeterStepGetNonce(),
    )
    assert rep == authenticated_cmds.v4.invite_greeter_step.RepGreetingAttemptCancelled(
        timestamp=ANY,
        reason=CancelledGreetingAttemptReason.MANUALLY_CANCELLED,
        origin=GreeterOrClaimer.GREETER,
    )


@pytest.mark.skip("Not implemented yet")
async def test_authenticated_invite_greeter_step_step_mismatch(
    coolorg: CoolorgRpcClients, greeting_attempt: GreetingAttemptID
) -> None:
    greeter_key_1 = PrivateKey.generate()
    greeter_key_2 = PrivateKey.generate()
    greeter_step_1 = authenticated_cmds.v4.invite_greeter_step.GreeterStepWaitPeer(
        public_key=greeter_key_1.public_key
    )
    greeter_step_2 = authenticated_cmds.v4.invite_greeter_step.GreeterStepWaitPeer(
        public_key=greeter_key_2.public_key
    )
    # Run once with first public key
    rep = await coolorg.alice.invite_greeter_step(
        greeting_attempt=greeting_attempt, greeter_step=greeter_step_1
    )
    assert rep == authenticated_cmds.v4.invite_greeter_step.RepNotReady()
    # Run once more with second public key
    rep = await coolorg.alice.invite_greeter_step(
        greeting_attempt=greeting_attempt, greeter_step=greeter_step_2
    )
    assert rep == authenticated_cmds.v4.invite_greeter_step.RepStepMismatch()


@pytest.mark.skip("Not implemented yet")
async def test_authenticated_invite_greeter_step_step_too_advanced(
    coolorg: CoolorgRpcClients, greeting_attempt: GreetingAttemptID
) -> None:
    rep = await coolorg.alice.invite_greeter_step(
        greeting_attempt=greeting_attempt,
        greeter_step=authenticated_cmds.v4.invite_greeter_step.GreeterStepGetNonce(),
    )
    assert rep == authenticated_cmds.v4.invite_greeter_step.RepStepTooAdvanced()


@pytest.mark.skip("Not implemented yet")
async def test_authenticated_invite_greeter_step_not_ready(
    coolorg: CoolorgRpcClients, greeting_attempt: GreetingAttemptID
) -> None:
    greeter_key = PrivateKey.generate()
    greeter_step = authenticated_cmds.v4.invite_greeter_step.GreeterStepWaitPeer(
        public_key=greeter_key.public_key
    )
    # Run once
    rep = await coolorg.alice.invite_greeter_step(
        greeting_attempt=greeting_attempt, greeter_step=greeter_step
    )
    assert rep == authenticated_cmds.v4.invite_greeter_step.RepNotReady()
    # Run twice
    rep = await coolorg.alice.invite_greeter_step(
        greeting_attempt=greeting_attempt, greeter_step=greeter_step
    )
    assert rep == authenticated_cmds.v4.invite_greeter_step.RepNotReady()
