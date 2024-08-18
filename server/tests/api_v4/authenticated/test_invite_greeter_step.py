# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import (
    CancelledGreetingAttemptReason,
    GreeterOrClaimer,
    GreetingAttemptID,
    PrivateKey,
    PublicKey,
    authenticated_cmds,
    invited_cmds,
)
from tests.common import Backend, CoolorgRpcClients


# TODO: Remove once PostgreSQL is supported
@pytest.fixture(autouse=True)
def _skip_if_postgresql(skip_if_postgresql: None) -> None:  # type: ignore
    pass


@pytest.fixture
async def greeting_attempt(coolorg: CoolorgRpcClients, backend: Backend) -> GreetingAttemptID:
    invitation_token = coolorg.invited_alice_dev3.token

    rep = await coolorg.alice.invite_greeter_start_greeting_attempt(
        token=invitation_token,
    )
    assert isinstance(rep, authenticated_cmds.v4.invite_greeter_start_greeting_attempt.RepOk)
    return rep.greeting_attempt


@pytest.fixture
async def claimer_wait_peer_public_key(
    coolorg: CoolorgRpcClients, greeting_attempt: GreetingAttemptID
) -> PublicKey:
    # Claimer start greeting attempt
    rep = await coolorg.invited_alice_dev3.invite_claimer_start_greeting_attempt(
        token=coolorg.invited_alice_dev3.token,
        greeter=coolorg.alice.user_id,
    )
    assert rep == invited_cmds.v4.invite_claimer_start_greeting_attempt.RepOk(
        greeting_attempt=greeting_attempt
    )

    # Claimer wait peer
    claimer_key = PrivateKey.generate()
    claimer_step = invited_cmds.v4.invite_claimer_step.ClaimerStepWaitPeer(
        public_key=claimer_key.public_key
    )
    rep = await coolorg.invited_alice_dev3.invite_claimer_step(
        greeting_attempt=greeting_attempt, claimer_step=claimer_step
    )
    assert rep == invited_cmds.v4.invite_claimer_step.RepNotReady()

    return claimer_key.public_key


async def test_authenticated_invite_greeter_step_ok(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    greeting_attempt: GreetingAttemptID,
    claimer_wait_peer_public_key: PublicKey,
) -> None:
    greeter_key = PrivateKey.generate()
    greeter_step = authenticated_cmds.v4.invite_greeter_step.GreeterStepWaitPeer(
        public_key=greeter_key.public_key
    )
    expected_claimer_step = authenticated_cmds.v4.invite_greeter_step.ClaimerStepWaitPeer(
        public_key=claimer_wait_peer_public_key
    )

    # Run once
    rep = await coolorg.alice.invite_greeter_step(
        greeting_attempt=greeting_attempt, greeter_step=greeter_step
    )
    assert rep == authenticated_cmds.v4.invite_greeter_step.RepOk(
        claimer_step=expected_claimer_step
    )

    # Run once more
    rep = await coolorg.alice.invite_greeter_step(
        greeting_attempt=greeting_attempt, greeter_step=greeter_step
    )
    assert rep == authenticated_cmds.v4.invite_greeter_step.RepOk(
        claimer_step=expected_claimer_step
    )


async def test_authenticated_invite_greeter_step_author_not_allowed(
    coolorg: CoolorgRpcClients, greeting_attempt: GreetingAttemptID
) -> None:
    rep = await coolorg.bob.invite_greeter_step(
        greeting_attempt=greeting_attempt,
        greeter_step=authenticated_cmds.v4.invite_greeter_step.GreeterStepGetNonce(),
    )

    assert rep == authenticated_cmds.v4.invite_greeter_step.RepAuthorNotAllowed()


async def test_authenticated_invite_greeter_step_invitation_cancelled(
    coolorg: CoolorgRpcClients, greeting_attempt: GreetingAttemptID
) -> None:
    await coolorg.alice.invite_cancel(token=coolorg.invited_alice_dev3.token)

    rep = await coolorg.alice.invite_greeter_step(
        greeting_attempt=greeting_attempt,
        greeter_step=authenticated_cmds.v4.invite_greeter_step.GreeterStepGetNonce(),
    )
    assert rep == authenticated_cmds.v4.invite_greeter_step.RepInvitationCancelled()


async def test_authenticated_invite_greeter_step_invitation_completed(
    coolorg: CoolorgRpcClients, greeting_attempt: GreetingAttemptID
) -> None:
    await coolorg.alice.invite_complete(token=coolorg.invited_alice_dev3.token)

    rep = await coolorg.alice.invite_greeter_step(
        greeting_attempt=greeting_attempt,
        greeter_step=authenticated_cmds.v4.invite_greeter_step.GreeterStepGetNonce(),
    )

    assert rep == authenticated_cmds.v4.invite_greeter_step.RepInvitationCompleted()


async def test_authenticated_invite_greeter_step_greeting_attempt_not_found(
    coolorg: CoolorgRpcClients, greeting_attempt: GreetingAttemptID
) -> None:
    rep = await coolorg.alice.invite_greeter_step(
        greeting_attempt=GreetingAttemptID.new(),
        greeter_step=authenticated_cmds.v4.invite_greeter_step.GreeterStepGetNonce(),
    )

    assert rep == authenticated_cmds.v4.invite_greeter_step.RepGreetingAttemptNotFound()


async def test_authenticated_invite_greeter_step_greeting_attempt_not_joined(
    coolorg: CoolorgRpcClients,
) -> None:
    # Claimer start greeting attempt
    rep = await coolorg.invited_alice_dev3.invite_claimer_start_greeting_attempt(
        token=coolorg.invited_alice_dev3.token,
        greeter=coolorg.alice.user_id,
    )
    assert isinstance(rep, invited_cmds.v4.invite_claimer_start_greeting_attempt.RepOk)
    greeting_attempt = rep.greeting_attempt

    rep = await coolorg.alice.invite_greeter_step(
        greeting_attempt=greeting_attempt,
        greeter_step=authenticated_cmds.v4.invite_greeter_step.GreeterStepGetNonce(),
    )

    assert rep == authenticated_cmds.v4.invite_greeter_step.RepGreetingAttemptNotJoined()


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
    assert isinstance(rep, authenticated_cmds.v4.invite_greeter_step.RepGreetingAttemptCancelled)
    assert rep == authenticated_cmds.v4.invite_greeter_step.RepGreetingAttemptCancelled(
        timestamp=rep.timestamp,
        reason=CancelledGreetingAttemptReason.MANUALLY_CANCELLED,
        origin=GreeterOrClaimer.GREETER,
    )


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


async def test_authenticated_invite_greeter_step_step_too_advanced(
    coolorg: CoolorgRpcClients, greeting_attempt: GreetingAttemptID
) -> None:
    rep = await coolorg.alice.invite_greeter_step(
        greeting_attempt=greeting_attempt,
        greeter_step=authenticated_cmds.v4.invite_greeter_step.GreeterStepGetNonce(),
    )
    assert rep == authenticated_cmds.v4.invite_greeter_step.RepStepTooAdvanced()


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
