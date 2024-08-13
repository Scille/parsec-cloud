# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from unittest.mock import ANY

import pytest

from parsec._parsec import UserID, invited_cmds
from tests.common import Backend, CoolorgRpcClients


@pytest.mark.skip("Not implemented yet")
async def test_invited_invite_claimer_start_greeting_attempt_ok(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    invitation_token = coolorg.invited_alice_dev3.token

    rep = await coolorg.invited_alice_dev3.invite_claimer_start_greeting_attempt(
        token=invitation_token,
        greeter=coolorg.alice.user_id,
    )
    assert rep == invited_cmds.v4.invite_claimer_start_greeting_attempt.RepOk(greeting_attempt=ANY)


@pytest.mark.skip("Not implemented yet")
async def test_invited_invite_claimer_start_greeting_attempt_greeter_not_found(
    coolorg: CoolorgRpcClients,
) -> None:
    invitation_token = coolorg.invited_alice_dev3.token

    rep = await coolorg.invited_alice_dev3.invite_claimer_start_greeting_attempt(
        token=invitation_token,
        greeter=UserID.new(),
    )

    assert rep == invited_cmds.v4.invite_claimer_start_greeting_attempt.RepGreeterNotFound()


@pytest.mark.skip("Not implemented yet")
async def test_invited_invite_claimer_start_greeting_attempt_greeter_not_allowed(
    coolorg: CoolorgRpcClients,
) -> None:
    rep = await coolorg.invited_alice_dev3.invite_claimer_start_greeting_attempt(
        token=coolorg.invited_alice_dev3.token,
        greeter=coolorg.bob.user_id,
    )

    assert rep == invited_cmds.v4.invite_claimer_start_greeting_attempt.RepGreeterNotAllowed()


@pytest.mark.skip("Not implemented yet")
async def test_invited_invite_claimer_start_greeting_attempt_greeter_revoked(
    coolorg: CoolorgRpcClients,
) -> None:
    # TODO
    # await coolorg.bob.user_revoke(alice_revoked_user_certificate)

    rep = await coolorg.invited_alice_dev3.invite_claimer_start_greeting_attempt(
        token=coolorg.invited_alice_dev3.token,
        greeter=coolorg.alice.user_id,
    )

    assert rep == invited_cmds.v4.invite_claimer_start_greeting_attempt.RepGreeterRevoked()
