# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import (
    DateTime,
    GreetingAttemptID,
    RevokedUserCertificate,
    UserID,
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


async def test_invited_invite_claimer_start_greeting_attempt_ok(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    rep = await coolorg.invited_alice_dev3.invite_claimer_start_greeting_attempt(
        greeter=coolorg.alice.user_id,
    )
    assert isinstance(rep, invited_cmds.v4.invite_claimer_start_greeting_attempt.RepOk)
    assert isinstance(rep.greeting_attempt, GreetingAttemptID)


async def test_invited_invite_claimer_start_greeting_attempt_greeter_not_found(
    coolorg: CoolorgRpcClients,
) -> None:
    rep = await coolorg.invited_alice_dev3.invite_claimer_start_greeting_attempt(
        greeter=UserID.new(),
    )

    assert rep == invited_cmds.v4.invite_claimer_start_greeting_attempt.RepGreeterNotFound()


async def test_invited_invite_claimer_start_greeting_attempt_greeter_not_allowed(
    coolorg: CoolorgRpcClients,
) -> None:
    rep = await coolorg.invited_alice_dev3.invite_claimer_start_greeting_attempt(
        greeter=coolorg.bob.user_id,
    )

    assert rep == invited_cmds.v4.invite_claimer_start_greeting_attempt.RepGreeterNotAllowed()


async def test_invited_invite_claimer_start_greeting_attempt_greeter_revoked(
    coolorg: CoolorgRpcClients,
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

    # Try to invite
    rep = await coolorg.invited_alice_dev3.invite_claimer_start_greeting_attempt(
        greeter=coolorg.alice.user_id,
    )

    assert rep == invited_cmds.v4.invite_claimer_start_greeting_attempt.RepGreeterRevoked()
