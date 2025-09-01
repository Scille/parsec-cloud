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
from parsec.asgi.rpc import CustomHttpStatus
from parsec.components.invite import NotReady
from parsec.events import EventGreetingAttemptReady
from tests.common import (
    Backend,
    CoolorgRpcClients,
    HttpCommonErrorsTester,
    RpcTransportError,
    ShamirOrgRpcClients,
    bob_becomes_admin_and_changes_alice,
)


@pytest.fixture
async def greeting_attempt(coolorg: CoolorgRpcClients, backend: Backend) -> GreetingAttemptID:
    outcome = await backend.invite.claimer_start_greeting_attempt(
        now=DateTime.now(),
        organization_id=coolorg.organization_id,
        greeter=coolorg.alice.user_id,
        token=coolorg.invited_alice_dev3.token,
    )
    assert isinstance(outcome, GreetingAttemptID)
    return outcome


@pytest.fixture
async def greeter_wait_peer_public_key(
    coolorg: CoolorgRpcClients,
    backend: Backend,
    greeting_attempt: GreetingAttemptID,
) -> PublicKey:
    # Greeter start greeting attempt
    outcome = await backend.invite.greeter_start_greeting_attempt(
        now=DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        token=coolorg.invited_alice_dev3.token,
        greeter=coolorg.alice.user_id,
    )
    assert outcome == greeting_attempt

    # Greeter wait peer
    greeter_key = PrivateKey.generate()
    outcome = await backend.invite.greeter_step(
        now=DateTime.now(),
        organization_id=coolorg.organization_id,
        author=coolorg.alice.device_id,
        greeter=coolorg.alice.user_id,
        greeting_attempt=greeting_attempt,
        step_index=0,
        greeter_data=greeter_key.public_key.encode(),
    )
    assert isinstance(outcome, NotReady)

    return greeter_key.public_key


async def test_invited_invite_claimer_step_ok(
    coolorg: CoolorgRpcClients,
    greeting_attempt: GreetingAttemptID,
    greeter_wait_peer_public_key: PublicKey,
) -> None:
    claimer_key = PrivateKey.generate()
    claimer_step = invited_cmds.latest.invite_claimer_step.ClaimerStepNumber0WaitPeer(
        public_key=claimer_key.public_key
    )
    expected_greeter_step = invited_cmds.latest.invite_claimer_step.GreeterStepNumber0WaitPeer(
        public_key=greeter_wait_peer_public_key
    )

    # Run once
    rep = await coolorg.invited_alice_dev3.invite_claimer_step(
        greeting_attempt=greeting_attempt, claimer_step=claimer_step
    )
    assert rep == invited_cmds.latest.invite_claimer_step.RepOk(greeter_step=expected_greeter_step)

    # Run once more
    rep = await coolorg.invited_alice_dev3.invite_claimer_step(
        greeting_attempt=greeting_attempt, claimer_step=claimer_step
    )
    assert rep == invited_cmds.latest.invite_claimer_step.RepOk(greeter_step=expected_greeter_step)


async def test_invited_invite_claimer_step_greeter_not_allowed(
    coolorg: CoolorgRpcClients, backend: Backend, greeting_attempt: GreetingAttemptID
) -> None:
    rep = await coolorg.invited_zack.invite_claimer_start_greeting_attempt(
        greeter=coolorg.alice.user_id,
    )
    assert isinstance(rep, invited_cmds.latest.invite_claimer_start_greeting_attempt.RepOk)
    greeting_attempt = rep.greeting_attempt

    await bob_becomes_admin_and_changes_alice(
        coolorg=coolorg, backend=backend, new_alice_profile=UserProfile.STANDARD
    )

    rep = await coolorg.invited_zack.invite_claimer_step(
        greeting_attempt=greeting_attempt,
        claimer_step=invited_cmds.latest.invite_claimer_step.ClaimerStepNumber2GetNonce(),
    )

    assert rep == invited_cmds.latest.invite_claimer_step.RepGreeterNotAllowed()


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
    assert rep == authenticated_cmds.latest.user_update.RepOk()

    # Revoke Alice
    now = DateTime.now()
    certif = RevokedUserCertificate(
        author=coolorg.bob.device_id,
        timestamp=now,
        user_id=coolorg.alice.user_id,
    ).dump_and_sign(coolorg.bob.signing_key)
    rep = await coolorg.bob.user_revoke(certif)
    assert rep == authenticated_cmds.latest.user_revoke.RepOk()

    rep = await coolorg.invited_alice_dev3.invite_claimer_step(
        greeting_attempt=greeting_attempt,
        claimer_step=invited_cmds.latest.invite_claimer_step.ClaimerStepNumber2GetNonce(),
    )

    assert rep == invited_cmds.latest.invite_claimer_step.RepGreeterRevoked()


async def test_invited_invite_claimer_step_greeting_attempt_not_found(
    coolorg: CoolorgRpcClients, greeting_attempt: GreetingAttemptID
) -> None:
    rep = await coolorg.invited_alice_dev3.invite_claimer_step(
        greeting_attempt=GreetingAttemptID.new(),
        claimer_step=invited_cmds.latest.invite_claimer_step.ClaimerStepNumber2GetNonce(),
    )

    assert rep == invited_cmds.latest.invite_claimer_step.RepGreetingAttemptNotFound()

    # Zack uses Alice greeting attempt
    rep = await coolorg.invited_zack.invite_claimer_step(
        greeting_attempt=greeting_attempt,
        claimer_step=invited_cmds.latest.invite_claimer_step.ClaimerStepNumber2GetNonce(),
    )
    assert rep == invited_cmds.latest.invite_claimer_step.RepGreetingAttemptNotFound()


async def test_invited_invite_claimer_step_greeting_attempt_not_joined(
    coolorg: CoolorgRpcClients,
) -> None:
    rep = await coolorg.alice.invite_greeter_start_greeting_attempt(
        token=coolorg.invited_alice_dev3.token,
    )
    assert isinstance(rep, authenticated_cmds.latest.invite_greeter_start_greeting_attempt.RepOk)

    rep = await coolorg.invited_alice_dev3.invite_claimer_step(
        greeting_attempt=rep.greeting_attempt,
        claimer_step=invited_cmds.latest.invite_claimer_step.ClaimerStepNumber2GetNonce(),
    )

    assert rep == invited_cmds.latest.invite_claimer_step.RepGreetingAttemptNotJoined()


async def test_invited_invite_claimer_step_greeting_attempt_cancelled(
    coolorg: CoolorgRpcClients, greeting_attempt: GreetingAttemptID
) -> None:
    # Cancel the attempt
    rep = await coolorg.invited_alice_dev3.invite_claimer_cancel_greeting_attempt(
        greeting_attempt=greeting_attempt,
        reason=CancelledGreetingAttemptReason.MANUALLY_CANCELLED,
    )
    assert rep == invited_cmds.latest.invite_claimer_cancel_greeting_attempt.RepOk()

    # Try a step
    rep = await coolorg.invited_alice_dev3.invite_claimer_step(
        greeting_attempt=greeting_attempt,
        claimer_step=invited_cmds.latest.invite_claimer_step.ClaimerStepNumber2GetNonce(),
    )
    assert isinstance(rep, invited_cmds.latest.invite_claimer_step.RepGreetingAttemptCancelled)
    assert rep == invited_cmds.latest.invite_claimer_step.RepGreetingAttemptCancelled(
        timestamp=rep.timestamp,
        reason=CancelledGreetingAttemptReason.MANUALLY_CANCELLED,
        origin=GreeterOrClaimer.CLAIMER,
    )


async def test_invited_invite_claimer_step_step_mismatch(
    coolorg: CoolorgRpcClients, greeting_attempt: GreetingAttemptID
) -> None:
    greeter_key_1 = PrivateKey.generate()
    greeter_key_2 = PrivateKey.generate()
    claimer_step_1 = invited_cmds.latest.invite_claimer_step.ClaimerStepNumber0WaitPeer(
        public_key=greeter_key_1.public_key
    )
    claimer_step_2 = invited_cmds.latest.invite_claimer_step.ClaimerStepNumber0WaitPeer(
        public_key=greeter_key_2.public_key
    )
    # Run once with first public key
    rep = await coolorg.invited_alice_dev3.invite_claimer_step(
        greeting_attempt=greeting_attempt, claimer_step=claimer_step_1
    )
    assert rep == invited_cmds.latest.invite_claimer_step.RepNotReady()
    # Run once more with second public key
    rep = await coolorg.invited_alice_dev3.invite_claimer_step(
        greeting_attempt=greeting_attempt, claimer_step=claimer_step_2
    )
    assert rep == invited_cmds.latest.invite_claimer_step.RepStepMismatch()


async def test_invited_invite_claimer_step_step_too_advanced(
    coolorg: CoolorgRpcClients, greeting_attempt: GreetingAttemptID
) -> None:
    rep = await coolorg.invited_alice_dev3.invite_claimer_step(
        greeting_attempt=greeting_attempt,
        claimer_step=invited_cmds.latest.invite_claimer_step.ClaimerStepNumber2GetNonce(),
    )
    assert rep == invited_cmds.latest.invite_claimer_step.RepStepTooAdvanced()


async def test_invited_invite_claimer_step_not_ready(
    coolorg: CoolorgRpcClients,
    greeting_attempt: GreetingAttemptID,
) -> None:
    greeter_key = PrivateKey.generate()
    claimer_step = invited_cmds.latest.invite_claimer_step.ClaimerStepNumber0WaitPeer(
        public_key=greeter_key.public_key
    )
    # Run once
    rep = await coolorg.invited_alice_dev3.invite_claimer_step(
        greeting_attempt=greeting_attempt, claimer_step=claimer_step
    )
    assert rep == invited_cmds.latest.invite_claimer_step.RepNotReady()
    # Run twice
    rep = await coolorg.invited_alice_dev3.invite_claimer_step(
        greeting_attempt=greeting_attempt,
        claimer_step=claimer_step,
    )
    assert rep == invited_cmds.latest.invite_claimer_step.RepNotReady()


async def test_invited_invite_claimer_step_with_shamir_deleted(
    shamirorg: ShamirOrgRpcClients,
    invited_greeting_with_deleted_shamir_tester: HttpCommonErrorsTester,
) -> None:
    rep = await shamirorg.shamir_invited_alice.invite_claimer_start_greeting_attempt(
        greeter=shamirorg.bob.user_id,
    )
    assert isinstance(rep, invited_cmds.latest.invite_claimer_start_greeting_attempt.RepOk)

    async def do():
        await shamirorg.shamir_invited_alice.invite_claimer_step(
            greeting_attempt=rep.greeting_attempt,
            claimer_step=invited_cmds.latest.invite_claimer_step.ClaimerStepNumber2GetNonce(),
        )

    await invited_greeting_with_deleted_shamir_tester(do)


async def test_invited_invite_claimer_step_greeting_attempt_ready_event(
    coolorg: CoolorgRpcClients,
    greeting_attempt: GreetingAttemptID,
    backend: Backend,
) -> None:
    greeter_key = PrivateKey.generate()
    claimer_step = invited_cmds.latest.invite_claimer_step.ClaimerStepNumber0WaitPeer(
        public_key=greeter_key.public_key
    )
    # Run once

    with backend.event_bus.spy() as spy:
        rep = await coolorg.invited_alice_dev3.invite_claimer_step(
            greeting_attempt=greeting_attempt, claimer_step=claimer_step
        )
        assert rep == invited_cmds.latest.invite_claimer_step.RepNotReady()

        await spy.wait_event_occurred(
            EventGreetingAttemptReady(
                organization_id=coolorg.organization_id,
                greeter=coolorg.alice.user_id,
                token=coolorg.invited_alice_dev3.token,
                greeting_attempt=greeting_attempt,
            )
        )

    # Run twice

    with backend.event_bus.spy() as spy:
        rep = await coolorg.invited_alice_dev3.invite_claimer_step(
            greeting_attempt=greeting_attempt,
            claimer_step=claimer_step,
        )
        assert rep == invited_cmds.latest.invite_claimer_step.RepNotReady()

        await spy.wait_event_occurred(
            EventGreetingAttemptReady(
                organization_id=coolorg.organization_id,
                greeter=coolorg.alice.user_id,
                token=coolorg.invited_alice_dev3.token,
                greeting_attempt=greeting_attempt,
            )
        )


async def test_invited_invite_claimer_step_http_common_errors(
    coolorg: CoolorgRpcClients,
    greeting_attempt: GreetingAttemptID,
    invited_http_common_errors_tester: HttpCommonErrorsTester,
) -> None:
    async def do():
        await coolorg.invited_alice_dev3.invite_claimer_step(
            greeting_attempt=greeting_attempt,
            claimer_step=invited_cmds.latest.invite_claimer_step.ClaimerStepNumber2GetNonce(),
        )

    await invited_http_common_errors_tester(do)


async def test_invited_invite_claimer_client_uses_legacy_sas_code_algorithm(
    coolorg: CoolorgRpcClients,
    greeting_attempt: GreetingAttemptID,
    greeter_wait_peer_public_key: PublicKey,
) -> None:
    claimer_key = PrivateKey.generate()
    claimer_step = invited_cmds.latest.invite_claimer_step.ClaimerStepNumber0WaitPeer(
        public_key=claimer_key.public_key
    )
    expected_greeter_step = invited_cmds.latest.invite_claimer_step.GreeterStepNumber0WaitPeer(
        public_key=greeter_wait_peer_public_key
    )

    coolorg.invited_alice_dev3.headers["Api-Version"] = "5.1"
    with pytest.raises(RpcTransportError) as ctx:
        rep = await coolorg.invited_alice_dev3.invite_claimer_step(
            greeting_attempt=greeting_attempt, claimer_step=claimer_step
        )
    (response,) = ctx.value.args
    assert response.status_code == CustomHttpStatus.UnsupportedApiVersion.value

    coolorg.invited_alice_dev3.headers["Api-Version"] = "5.2"
    rep = await coolorg.invited_alice_dev3.invite_claimer_step(
        greeting_attempt=greeting_attempt, claimer_step=claimer_step
    )
    assert rep == invited_cmds.latest.invite_claimer_step.RepOk(greeter_step=expected_greeter_step)
