# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import (
    CancelledGreetingAttemptReason,
    DateTime,
    GreeterOrClaimer,
    GreetingAttemptID,
    PrivateKey,
    PublicKey,
    ShamirRecoveryDeletionCertificate,
    UserProfile,
    authenticated_cmds,
    invited_cmds,
)
from parsec.components.invite import NotReady
from parsec.events import EventGreetingAttemptJoined
from tests.common import (
    Backend,
    CoolorgRpcClients,
    HttpCommonErrorsTester,
    ShamirOrgRpcClients,
    bob_becomes_admin,
    bob_becomes_admin_and_changes_alice,
)


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


@pytest.fixture
async def zack_greeting_attempt(coolorg: CoolorgRpcClients, backend: Backend) -> GreetingAttemptID:
    outcome = await backend.invite.greeter_start_greeting_attempt(
        now=DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        greeter=coolorg.alice.user_id,
        token=coolorg.invited_zack.token,
    )
    assert isinstance(outcome, GreetingAttemptID)
    return outcome


@pytest.fixture
async def claimer_wait_peer_public_key(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    greeting_attempt: GreetingAttemptID,
) -> PublicKey:
    # Claimer start greeting attempt
    outcome = await backend.invite.claimer_start_greeting_attempt(
        now=DateTime.now(),
        organization_id=coolorg.organization_id,
        token=coolorg.invited_alice_dev3.token,
        greeter=coolorg.alice.user_id,
    )
    assert outcome == greeting_attempt

    # Claimer wait peer
    claimer_key = PrivateKey.generate()
    outcome = await backend.invite.claimer_step(
        now=DateTime.now(),
        organization_id=coolorg.organization_id,
        token=coolorg.invited_alice_dev3.token,
        greeting_attempt=greeting_attempt,
        step_index=0,
        claimer_data=claimer_key.public_key.encode(),
    )
    assert isinstance(outcome, NotReady)

    return claimer_key.public_key


async def test_authenticated_invite_greeter_step_ok(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    greeting_attempt: GreetingAttemptID,
    claimer_wait_peer_public_key: PublicKey,
) -> None:
    greeter_key = PrivateKey.generate()
    greeter_step = authenticated_cmds.latest.invite_greeter_step.GreeterStepNumber0WaitPeer(
        public_key=greeter_key.public_key
    )
    expected_claimer_step = (
        authenticated_cmds.latest.invite_greeter_step.ClaimerStepNumber0WaitPeer(
            public_key=claimer_wait_peer_public_key
        )
    )

    # Run once
    rep = await coolorg.alice.invite_greeter_step(
        greeting_attempt=greeting_attempt, greeter_step=greeter_step
    )
    assert rep == authenticated_cmds.latest.invite_greeter_step.RepOk(
        claimer_step=expected_claimer_step
    )

    # Run once more
    rep = await coolorg.alice.invite_greeter_step(
        greeting_attempt=greeting_attempt, greeter_step=greeter_step
    )
    assert rep == authenticated_cmds.latest.invite_greeter_step.RepOk(
        claimer_step=expected_claimer_step
    )


async def test_authenticated_invite_greeter_step_author_not_allowed(
    coolorg: CoolorgRpcClients, zack_greeting_attempt: GreetingAttemptID, backend
) -> None:
    await bob_becomes_admin_and_changes_alice(
        coolorg=coolorg, backend=backend, new_alice_profile=UserProfile.STANDARD
    )

    rep = await coolorg.alice.invite_greeter_step(
        greeting_attempt=zack_greeting_attempt,
        greeter_step=authenticated_cmds.latest.invite_greeter_step.GreeterStepNumber3GetNonce(),
    )

    assert rep == authenticated_cmds.latest.invite_greeter_step.RepAuthorNotAllowed()


async def test_authenticated_invite_greeter_step_invitation_cancelled(
    coolorg: CoolorgRpcClients, greeting_attempt: GreetingAttemptID
) -> None:
    await coolorg.alice.invite_cancel(token=coolorg.invited_alice_dev3.token)

    rep = await coolorg.alice.invite_greeter_step(
        greeting_attempt=greeting_attempt,
        greeter_step=authenticated_cmds.latest.invite_greeter_step.GreeterStepNumber3GetNonce(),
    )
    assert rep == authenticated_cmds.latest.invite_greeter_step.RepInvitationCancelled()


async def test_authenticated_invite_greeter_step_invitation_completed(
    coolorg: CoolorgRpcClients, greeting_attempt: GreetingAttemptID
) -> None:
    await coolorg.alice.invite_complete(token=coolorg.invited_alice_dev3.token)

    rep = await coolorg.alice.invite_greeter_step(
        greeting_attempt=greeting_attempt,
        greeter_step=authenticated_cmds.latest.invite_greeter_step.GreeterStepNumber3GetNonce(),
    )

    assert rep == authenticated_cmds.latest.invite_greeter_step.RepInvitationCompleted()


async def test_authenticated_invite_greeter_step_greeting_attempt_not_found(
    coolorg: CoolorgRpcClients, zack_greeting_attempt: GreetingAttemptID, backend: Backend
) -> None:
    rep = await coolorg.alice.invite_greeter_step(
        greeting_attempt=GreetingAttemptID.new(),
        greeter_step=authenticated_cmds.latest.invite_greeter_step.GreeterStepNumber3GetNonce(),
    )
    assert rep == authenticated_cmds.latest.invite_greeter_step.RepGreetingAttemptNotFound()

    # Make Bob an admin
    await bob_becomes_admin(coolorg=coolorg, backend=backend)

    # Bob uses Alice greeting attempt
    rep = await coolorg.bob.invite_greeter_step(
        greeting_attempt=zack_greeting_attempt,
        greeter_step=authenticated_cmds.latest.invite_greeter_step.GreeterStepNumber3GetNonce(),
    )
    assert rep == authenticated_cmds.latest.invite_greeter_step.RepGreetingAttemptNotFound()


async def test_authenticated_invite_greeter_step_greeting_attempt_not_joined(
    coolorg: CoolorgRpcClients,
) -> None:
    # Claimer start greeting attempt
    rep = await coolorg.invited_alice_dev3.invite_claimer_start_greeting_attempt(
        greeter=coolorg.alice.user_id,
    )
    assert isinstance(rep, invited_cmds.latest.invite_claimer_start_greeting_attempt.RepOk)
    greeting_attempt = rep.greeting_attempt

    rep = await coolorg.alice.invite_greeter_step(
        greeting_attempt=greeting_attempt,
        greeter_step=authenticated_cmds.latest.invite_greeter_step.GreeterStepNumber3GetNonce(),
    )

    assert rep == authenticated_cmds.latest.invite_greeter_step.RepGreetingAttemptNotJoined()


async def test_authenticated_invite_greeter_step_greeting_attempt_cancelled(
    coolorg: CoolorgRpcClients, greeting_attempt: GreetingAttemptID
) -> None:
    # Cancel the attempt
    rep = await coolorg.alice.invite_greeter_cancel_greeting_attempt(
        greeting_attempt=greeting_attempt,
        reason=CancelledGreetingAttemptReason.MANUALLY_CANCELLED,
    )
    assert rep == authenticated_cmds.latest.invite_greeter_cancel_greeting_attempt.RepOk()

    # Try a step
    rep = await coolorg.alice.invite_greeter_step(
        greeting_attempt=greeting_attempt,
        greeter_step=authenticated_cmds.latest.invite_greeter_step.GreeterStepNumber3GetNonce(),
    )
    assert isinstance(
        rep, authenticated_cmds.latest.invite_greeter_step.RepGreetingAttemptCancelled
    )
    assert rep == authenticated_cmds.latest.invite_greeter_step.RepGreetingAttemptCancelled(
        timestamp=rep.timestamp,
        reason=CancelledGreetingAttemptReason.MANUALLY_CANCELLED,
        origin=GreeterOrClaimer.GREETER,
    )


async def test_authenticated_invite_greeter_step_step_mismatch(
    coolorg: CoolorgRpcClients, greeting_attempt: GreetingAttemptID
) -> None:
    greeter_key_1 = PrivateKey.generate()
    greeter_key_2 = PrivateKey.generate()
    greeter_step_1 = authenticated_cmds.latest.invite_greeter_step.GreeterStepNumber0WaitPeer(
        public_key=greeter_key_1.public_key
    )
    greeter_step_2 = authenticated_cmds.latest.invite_greeter_step.GreeterStepNumber0WaitPeer(
        public_key=greeter_key_2.public_key
    )
    # Run once with first public key
    rep = await coolorg.alice.invite_greeter_step(
        greeting_attempt=greeting_attempt, greeter_step=greeter_step_1
    )
    assert rep == authenticated_cmds.latest.invite_greeter_step.RepNotReady()
    # Run once more with second public key
    rep = await coolorg.alice.invite_greeter_step(
        greeting_attempt=greeting_attempt, greeter_step=greeter_step_2
    )
    assert rep == authenticated_cmds.latest.invite_greeter_step.RepStepMismatch()


async def test_authenticated_invite_greeter_step_step_too_advanced(
    coolorg: CoolorgRpcClients, greeting_attempt: GreetingAttemptID
) -> None:
    rep = await coolorg.alice.invite_greeter_step(
        greeting_attempt=greeting_attempt,
        greeter_step=authenticated_cmds.latest.invite_greeter_step.GreeterStepNumber3GetNonce(),
    )
    assert rep == authenticated_cmds.latest.invite_greeter_step.RepStepTooAdvanced()


async def test_authenticated_invite_greeter_step_not_ready(
    coolorg: CoolorgRpcClients, greeting_attempt: GreetingAttemptID
) -> None:
    greeter_key = PrivateKey.generate()
    greeter_step = authenticated_cmds.latest.invite_greeter_step.GreeterStepNumber0WaitPeer(
        public_key=greeter_key.public_key
    )
    # Run once
    rep = await coolorg.alice.invite_greeter_step(
        greeting_attempt=greeting_attempt, greeter_step=greeter_step
    )
    assert rep == authenticated_cmds.latest.invite_greeter_step.RepNotReady()
    # Run twice
    rep = await coolorg.alice.invite_greeter_step(
        greeting_attempt=greeting_attempt, greeter_step=greeter_step
    )
    assert rep == authenticated_cmds.latest.invite_greeter_step.RepNotReady()


async def test_authenticated_invite_greeter_step_with_deleted_shamir(
    shamirorg: ShamirOrgRpcClients,
) -> None:
    rep = await shamirorg.bob.invite_greeter_start_greeting_attempt(
        token=shamirorg.shamir_invited_alice.token,
    )
    assert isinstance(rep, authenticated_cmds.latest.invite_greeter_start_greeting_attempt.RepOk)
    greeting_attempt = rep.greeting_attempt

    # Delete Alice shamir recovery
    dt = DateTime.now()
    author = shamirorg.alice
    brief = shamirorg.alice_brief_certificate
    deletion = ShamirRecoveryDeletionCertificate(
        author=author.device_id,
        timestamp=dt,
        setup_to_delete_timestamp=brief.timestamp,
        setup_to_delete_user_id=brief.user_id,
        share_recipients=set(brief.per_recipient_shares.keys()),
    ).dump_and_sign(author.signing_key)
    rep = await shamirorg.alice.shamir_recovery_delete(deletion)
    assert rep == authenticated_cmds.latest.shamir_recovery_delete.RepOk()

    rep = await shamirorg.bob.invite_greeter_step(
        greeting_attempt=greeting_attempt,
        greeter_step=authenticated_cmds.latest.invite_greeter_step.GreeterStepNumber3GetNonce(),
    )
    assert rep == authenticated_cmds.latest.invite_greeter_step.RepInvitationCancelled()


async def test_authenticated_invite_greeter_step_greeting_attempt_joined_event(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    greeting_attempt: GreetingAttemptID,
    claimer_wait_peer_public_key: PublicKey,
) -> None:
    greeter_key = PrivateKey.generate()
    greeter_step = authenticated_cmds.latest.invite_greeter_step.GreeterStepNumber0WaitPeer(
        public_key=greeter_key.public_key
    )
    expected_claimer_step = (
        authenticated_cmds.latest.invite_greeter_step.ClaimerStepNumber0WaitPeer(
            public_key=claimer_wait_peer_public_key
        )
    )

    with backend.event_bus.spy() as spy:
        rep = await coolorg.alice.invite_greeter_step(
            greeting_attempt=greeting_attempt, greeter_step=greeter_step
        )
        assert rep == authenticated_cmds.latest.invite_greeter_step.RepOk(
            claimer_step=expected_claimer_step
        )

        await spy.wait_event_occurred(
            EventGreetingAttemptJoined(
                organization_id=coolorg.organization_id,
                greeter=coolorg.alice.user_id,
                token=coolorg.invited_alice_dev3.token,
                greeting_attempt=greeting_attempt,
            )
        )


async def test_authenticated_invite_greeter_step_http_common_errors(
    coolorg: CoolorgRpcClients,
    greeting_attempt: GreetingAttemptID,
    authenticated_http_common_errors_tester: HttpCommonErrorsTester,
) -> None:
    async def do():
        greeter_key = PrivateKey.generate()
        greeter_step = authenticated_cmds.latest.invite_greeter_step.GreeterStepNumber0WaitPeer(
            public_key=greeter_key.public_key
        )
        await coolorg.alice.invite_greeter_step(
            greeting_attempt=greeting_attempt, greeter_step=greeter_step
        )

    await authenticated_http_common_errors_tester(do)
