# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from parsec._parsec import GreetingAttemptID, InvitationToken, authenticated_cmds
from tests.common import Backend, CoolorgRpcClients

Response = authenticated_cmds.v4.invite_greeter_start_greeting_attempt.Rep | None


async def test_authenticated_invite_greeter_start_greeting_attempt_ok(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    invitation_token = coolorg.invited_alice_dev3.token

    rep = await coolorg.alice.invite_greeter_start_greeting_attempt(
        token=invitation_token,
    )
    assert isinstance(rep, authenticated_cmds.v4.invite_greeter_start_greeting_attempt.RepOk)
    assert isinstance(rep.greeting_attempt, GreetingAttemptID)


async def test_authenticated_invite_greeter_start_greeting_attempt_invitation_not_found(
    coolorg: CoolorgRpcClients,
) -> None:
    rep = await coolorg.alice.invite_greeter_start_greeting_attempt(
        token=InvitationToken.new(),
    )

    assert (
        rep == authenticated_cmds.v4.invite_greeter_start_greeting_attempt.RepInvitationNotFound()
    )


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


async def test_authenticated_invite_greeter_start_greeting_attempt_invitation_completed(
    coolorg: CoolorgRpcClients,
) -> None:
    await coolorg.alice.invite_complete(token=coolorg.invited_alice_dev3.token)

    rep = await coolorg.alice.invite_greeter_start_greeting_attempt(
        token=coolorg.invited_alice_dev3.token,
    )

    assert (
        rep == authenticated_cmds.v4.invite_greeter_start_greeting_attempt.RepInvitationCompleted()
    )


async def test_authenticated_invite_greeter_start_greeting_attempt_author_not_allowed(
    coolorg: CoolorgRpcClients,
) -> None:
    rep = await coolorg.bob.invite_greeter_start_greeting_attempt(
        token=coolorg.invited_alice_dev3.token,
    )

    assert rep == authenticated_cmds.v4.invite_greeter_start_greeting_attempt.RepAuthorNotAllowed()
