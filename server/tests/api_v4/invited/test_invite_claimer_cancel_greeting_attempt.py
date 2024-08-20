# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


import pytest

from parsec._parsec import (
    CancelledGreetingAttemptReason,
    DateTime,
    GreeterOrClaimer,
    GreetingAttemptID,
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
    invitation_token = coolorg.invited_alice_dev3.token

    rep = await coolorg.invited_alice_dev3.invite_claimer_start_greeting_attempt(
        token=invitation_token,
        greeter=coolorg.alice.user_id,
    )
    assert isinstance(rep, invited_cmds.v4.invite_claimer_start_greeting_attempt.RepOk)
    return rep.greeting_attempt


async def test_invited_invite_claimer_cancel_greeting_attempt_ok(
    coolorg: CoolorgRpcClients, backend: Backend, greeting_attempt: GreetingAttemptID
) -> None:
    rep = await coolorg.invited_alice_dev3.invite_claimer_cancel_greeting_attempt(
        greeting_attempt=greeting_attempt,
        reason=CancelledGreetingAttemptReason.MANUALLY_CANCELLED,
    )
    assert rep == invited_cmds.v4.invite_claimer_cancel_greeting_attempt.RepOk()


async def test_invited_invite_claimer_cancel_greeting_attempt_greeter_not_allowed(
    coolorg: CoolorgRpcClients,
) -> None:
    invitation_token = coolorg.invited_zack.token

    rep = await coolorg.invited_zack.invite_claimer_start_greeting_attempt(
        token=invitation_token,
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

    # Cancel the greeting attempt
    rep = await coolorg.invited_zack.invite_claimer_cancel_greeting_attempt(
        greeting_attempt=greeting_attempt,
        reason=CancelledGreetingAttemptReason.MANUALLY_CANCELLED,
    )

    assert rep == invited_cmds.v4.invite_claimer_cancel_greeting_attempt.RepGreeterNotAllowed()


async def test_invited_invite_claimer_cancel_greeting_attempt_greeter_revoked(
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

    rep = await coolorg.invited_alice_dev3.invite_claimer_cancel_greeting_attempt(
        greeting_attempt=greeting_attempt,
        reason=CancelledGreetingAttemptReason.MANUALLY_CANCELLED,
    )
    assert rep == invited_cmds.v4.invite_claimer_cancel_greeting_attempt.RepGreeterRevoked()


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


async def test_invited_invite_claimer_cancel_greeting_attempt_greeting_attempt_not_joined(
    coolorg: CoolorgRpcClients,
) -> None:
    rep = await coolorg.alice.invite_greeter_start_greeting_attempt(
        token=coolorg.invited_alice_dev3.token,
    )
    assert isinstance(rep, authenticated_cmds.v4.invite_greeter_start_greeting_attempt.RepOk)

    rep = await coolorg.invited_alice_dev3.invite_claimer_cancel_greeting_attempt(
        greeting_attempt=rep.greeting_attempt,
        reason=CancelledGreetingAttemptReason.MANUALLY_CANCELLED,
    )

    assert (
        rep == invited_cmds.v4.invite_claimer_cancel_greeting_attempt.RepGreetingAttemptNotJoined()
    )


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
    assert isinstance(
        rep,
        invited_cmds.v4.invite_claimer_cancel_greeting_attempt.RepGreetingAttemptAlreadyCancelled,
    )
    assert (
        rep
        == invited_cmds.v4.invite_claimer_cancel_greeting_attempt.RepGreetingAttemptAlreadyCancelled(
            timestamp=rep.timestamp,
            reason=CancelledGreetingAttemptReason.MANUALLY_CANCELLED,
            origin=GreeterOrClaimer.CLAIMER,
        )
    )