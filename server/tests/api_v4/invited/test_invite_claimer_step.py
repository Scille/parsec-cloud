# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


import pytest

from parsec._parsec import (
    CancelledGreetingAttemptReason,
    DateTime,
    GreeterOrClaimer,
    GreetingAttemptID,
    PrivateKey,
    PublicKey,
    RevokedUserCertificate,
    UserProfile,
    UserUpdateCertificate,
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
    rep = await coolorg.invited_alice_dev3.invite_claimer_start_greeting_attempt(
        greeter=coolorg.alice.user_id,
    )
    assert isinstance(rep, invited_cmds.v4.invite_claimer_start_greeting_attempt.RepOk)
    return rep.greeting_attempt


@pytest.fixture
async def greeter_wait_peer_public_key(
    coolorg: CoolorgRpcClients, greeting_attempt: GreetingAttemptID
) -> PublicKey:
    # Greeter start greeting attempt
    rep = await coolorg.alice.invite_greeter_start_greeting_attempt(
        token=coolorg.invited_alice_dev3.token,
    )
    assert rep == authenticated_cmds.v4.invite_greeter_start_greeting_attempt.RepOk(
        greeting_attempt=greeting_attempt
    )

    # Greeter wait peer
    greeter_key = PrivateKey.generate()
    greeter_step = authenticated_cmds.v4.invite_greeter_step.GreeterStepNumber0WaitPeer(
        public_key=greeter_key.public_key
    )
    rep = await coolorg.alice.invite_greeter_step(
        greeting_attempt=greeting_attempt, greeter_step=greeter_step
    )
    assert rep == authenticated_cmds.v4.invite_greeter_step.RepNotReady()

    return greeter_key.public_key


async def test_invited_invite_claimer_step_ok(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    greeting_attempt: GreetingAttemptID,
    greeter_wait_peer_public_key: PublicKey,
) -> None:
    claimer_key = PrivateKey.generate()
    claimer_step = invited_cmds.v4.invite_claimer_step.ClaimerStepNumber0WaitPeer(
        public_key=claimer_key.public_key
    )
    expected_greeter_step = invited_cmds.v4.invite_claimer_step.GreeterStepNumber0WaitPeer(
        public_key=greeter_wait_peer_public_key
    )

    # Run once
    rep = await coolorg.invited_alice_dev3.invite_claimer_step(
        greeting_attempt=greeting_attempt, claimer_step=claimer_step
    )
    assert rep == invited_cmds.v4.invite_claimer_step.RepOk(greeter_step=expected_greeter_step)

    # Run once more
    rep = await coolorg.invited_alice_dev3.invite_claimer_step(
        greeting_attempt=greeting_attempt, claimer_step=claimer_step
    )
    assert rep == invited_cmds.v4.invite_claimer_step.RepOk(greeter_step=expected_greeter_step)


async def test_invited_invite_claimer_step_greeter_not_allowed(
    coolorg: CoolorgRpcClients, greeting_attempt: GreetingAttemptID
) -> None:
    rep = await coolorg.invited_zack.invite_claimer_start_greeting_attempt(
        greeter=coolorg.alice.user_id,
    )
    assert isinstance(rep, invited_cmds.v4.invite_claimer_start_greeting_attempt.RepOk)
    greeting_attempt = rep.greeting_attempt

    # Make Bob an admin
    certif = UserUpdateCertificate(
        author=coolorg.alice.device_id,
        new_profile=UserProfile.ADMIN,
        user_id=coolorg.bob.user_id,
        timestamp=DateTime.now(),
    ).dump_and_sign(coolorg.alice.signing_key)
    rep = await coolorg.alice.user_update(certif)
    assert rep == authenticated_cmds.v4.user_update.RepOk()

    # Alice is no longer an admin
    certif = UserUpdateCertificate(
        author=coolorg.bob.device_id,
        new_profile=UserProfile.STANDARD,
        user_id=coolorg.alice.user_id,
        timestamp=DateTime.now(),
    ).dump_and_sign(coolorg.bob.signing_key)
    rep = await coolorg.bob.user_update(certif)
    assert rep == authenticated_cmds.v4.user_update.RepOk()

    rep = await coolorg.invited_zack.invite_claimer_step(
        greeting_attempt=greeting_attempt,
        claimer_step=invited_cmds.v4.invite_claimer_step.ClaimerStepNumber2GetNonce(),
    )

    assert rep == invited_cmds.v4.invite_claimer_step.RepGreeterNotAllowed()


async def test_invited_invite_claimer_step_greeter_revoked(
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

    rep = await coolorg.invited_alice_dev3.invite_claimer_step(
        greeting_attempt=greeting_attempt,
        claimer_step=invited_cmds.v4.invite_claimer_step.ClaimerStepNumber2GetNonce(),
    )

    assert rep == invited_cmds.v4.invite_claimer_step.RepGreeterRevoked()


async def test_invited_invite_claimer_step_greeting_attempt_not_found(
    coolorg: CoolorgRpcClients, greeting_attempt: GreetingAttemptID
) -> None:
    rep = await coolorg.invited_alice_dev3.invite_claimer_step(
        greeting_attempt=GreetingAttemptID.new(),
        claimer_step=invited_cmds.v4.invite_claimer_step.ClaimerStepNumber2GetNonce(),
    )

    assert rep == invited_cmds.v4.invite_claimer_step.RepGreetingAttemptNotFound()


async def test_invited_invite_claimer_step_greeting_attempt_not_joined(
    coolorg: CoolorgRpcClients,
) -> None:
    rep = await coolorg.alice.invite_greeter_start_greeting_attempt(
        token=coolorg.invited_alice_dev3.token,
    )
    assert isinstance(rep, authenticated_cmds.v4.invite_greeter_start_greeting_attempt.RepOk)

    rep = await coolorg.invited_alice_dev3.invite_claimer_step(
        greeting_attempt=rep.greeting_attempt,
        claimer_step=invited_cmds.v4.invite_claimer_step.ClaimerStepNumber2GetNonce(),
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
        claimer_step=invited_cmds.v4.invite_claimer_step.ClaimerStepNumber2GetNonce(),
    )
    assert isinstance(rep, invited_cmds.v4.invite_claimer_step.RepGreetingAttemptCancelled)
    assert rep == invited_cmds.v4.invite_claimer_step.RepGreetingAttemptCancelled(
        timestamp=rep.timestamp,
        reason=CancelledGreetingAttemptReason.MANUALLY_CANCELLED,
        origin=GreeterOrClaimer.CLAIMER,
    )


async def test_invited_invite_claimer_step_step_mismatch(
    coolorg: CoolorgRpcClients, greeting_attempt: GreetingAttemptID
) -> None:
    greeter_key_1 = PrivateKey.generate()
    greeter_key_2 = PrivateKey.generate()
    claimer_step_1 = invited_cmds.v4.invite_claimer_step.ClaimerStepNumber0WaitPeer(
        public_key=greeter_key_1.public_key
    )
    claimer_step_2 = invited_cmds.v4.invite_claimer_step.ClaimerStepNumber0WaitPeer(
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
        claimer_step=invited_cmds.v4.invite_claimer_step.ClaimerStepNumber2GetNonce(),
    )
    assert rep == invited_cmds.v4.invite_claimer_step.RepStepTooAdvanced()


async def test_invited_invite_claimer_step_not_ready(
    coolorg: CoolorgRpcClients, greeting_attempt: GreetingAttemptID
) -> None:
    greeter_key = PrivateKey.generate()
    claimer_step = invited_cmds.v4.invite_claimer_step.ClaimerStepNumber0WaitPeer(
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
