# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from unittest.mock import ANY

import pytest

from parsec._parsec import InvitationToken, authenticated_cmds
from tests.common import Backend, CoolorgRpcClients

Response = authenticated_cmds.v4.invite_greeter_start_greeting_attempt.Rep | None


@pytest.mark.skip("Not implemented yet")
async def test_authenticated_invite_greeter_start_greeting_attempt_ok(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    invitation_token = coolorg.invited_alice_dev3.token

    rep = await coolorg.alice.invite_greeter_start_greeting_attempt(
        token=invitation_token,
    )
    assert rep == authenticated_cmds.v4.invite_greeter_start_greeting_attempt.RepOk(
        greeting_attempt=ANY
    )


@pytest.mark.skip("Not implemented yet")
async def test_authenticated_invite_greeter_start_greeting_attempt_invitation_not_found(
    coolorg: CoolorgRpcClients,
) -> None:
    rep = await coolorg.alice.invite_greeter_start_greeting_attempt(
        token=InvitationToken.new(),
    )

    assert (
        rep == authenticated_cmds.v4.invite_greeter_start_greeting_attempt.RepInvitationNotFound()
    )


@pytest.mark.skip("Not implemented yet")
async def test_authenticated_invite_greeter_start_greeting_attempt_invitation_cancelled(
    coolorg: CoolorgRpcClients,
) -> None:
    await coolorg.alice.invite_cancel(token=coolorg.invited_alice_dev3.token)

    rep = await coolorg.alice.invite_greeter_start_greeting_attempt(
        token=coolorg.invited_alice_dev3.token,
    )

    assert (
        rep == authenticated_cmds.v4.invite_greeter_start_greeting_attempt.RepInvitationCancelled()
    )


@pytest.mark.skip("Not implemented yet")
async def test_authenticated_invite_greeter_start_greeting_attempt_invitation_completed(
    coolorg: CoolorgRpcClients,
) -> None:
    # Not implemented yet
    # await coolorg.alice.invite_complete(token=coolorg.invited_alice_dev3.token)

    rep = await coolorg.alice.invite_greeter_start_greeting_attempt(
        token=coolorg.invited_alice_dev3.token,
    )

    assert (
        rep == authenticated_cmds.v4.invite_greeter_start_greeting_attempt.RepInvitationCompleted()
    )


@pytest.mark.skip("Not implemented yet")
async def test_authenticated_invite_greeter_start_greeting_attempt_author_not_allowed(
    coolorg: CoolorgRpcClients,
) -> None:
    rep = await coolorg.bob.invite_greeter_start_greeting_attempt(
        token=coolorg.invited_alice_dev3.token,
    )

    assert rep == authenticated_cmds.v4.invite_greeter_start_greeting_attempt.RepAuthorNotAllowed()
