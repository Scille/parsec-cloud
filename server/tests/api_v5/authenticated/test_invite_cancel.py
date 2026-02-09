# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from parsec._parsec import AccessToken, DateTime, authenticated_cmds
from tests.common import Backend, CoolorgRpcClients, HttpCommonErrorsTester


async def test_authenticated_invite_cancel_ok(coolorg: CoolorgRpcClients) -> None:
    rep = await coolorg.alice.invite_cancel(coolorg.invited_alice_dev3.token)

    assert rep == authenticated_cmds.latest.invite_cancel.RepOk()


async def test_authenticated_invite_cancel_invitation_not_found(coolorg: CoolorgRpcClients) -> None:
    rep = await coolorg.alice.invite_cancel(AccessToken.new())

    assert rep == authenticated_cmds.latest.invite_cancel.RepInvitationNotFound()


async def test_authenticated_invite_cancel_author_not_allowed(coolorg: CoolorgRpcClients) -> None:
    rep = await coolorg.bob.invite_cancel(coolorg.invited_alice_dev3.token)

    assert rep == authenticated_cmds.latest.invite_cancel.RepAuthorNotAllowed()


async def test_authenticated_invite_cancel_invitation_already_cancelled(
    coolorg: CoolorgRpcClients,
    backend: Backend,
) -> None:
    assert (
        await backend.invite.cancel(
            now=DateTime.now(),
            organization_id=coolorg.organization_id,
            author=coolorg.alice.device_id,
            token=coolorg.invited_alice_dev3.token,
        )
        is None
    )

    rep = await coolorg.alice.invite_cancel(coolorg.invited_alice_dev3.token)

    assert rep == authenticated_cmds.latest.invite_cancel.RepInvitationAlreadyCancelled()


async def test_authenticated_invite_cancel_invitation_completed(
    coolorg: CoolorgRpcClients,
    backend: Backend,
) -> None:
    assert (
        await backend.invite.complete(
            now=DateTime.now(),
            organization_id=coolorg.organization_id,
            author=coolorg.alice.device_id,
            token=coolorg.invited_alice_dev3.token,
        )
        is None
    )

    rep = await coolorg.alice.invite_cancel(coolorg.invited_alice_dev3.token)

    assert rep == authenticated_cmds.latest.invite_cancel.RepInvitationCompleted()


async def test_authenticated_invite_cancel_http_common_errors(
    coolorg: CoolorgRpcClients, authenticated_http_common_errors_tester: HttpCommonErrorsTester
) -> None:
    async def do():
        await coolorg.alice.invite_cancel(coolorg.invited_alice_dev3.token)

    await authenticated_http_common_errors_tester(do)
