# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from parsec._parsec import InvitationToken, authenticated_cmds
from tests.common import CoolorgRpcClients, HttpCommonErrorsTester


async def test_authenticated_invite_cancel_ok(coolorg: CoolorgRpcClients) -> None:
    rep = await coolorg.alice.invite_cancel(coolorg.invited_alice_dev3.token)

    assert rep == authenticated_cmds.v4.invite_cancel.RepOk()


async def test_authenticated_invite_cancel_invitation_not_found(coolorg: CoolorgRpcClients) -> None:
    rep = await coolorg.alice.invite_cancel(InvitationToken.new())

    assert rep == authenticated_cmds.v4.invite_cancel.RepInvitationNotFound()


async def test_authenticated_invite_cancel_invitation_already_deleted(
    coolorg: CoolorgRpcClients,
) -> None:
    await test_authenticated_invite_cancel_ok(coolorg)

    rep = await coolorg.alice.invite_cancel(coolorg.invited_alice_dev3.token)

    assert rep == authenticated_cmds.v4.invite_cancel.RepInvitationAlreadyDeleted()


async def test_authenticated_invite_cancel_http_common_errors(
    coolorg: CoolorgRpcClients, authenticated_http_common_errors_tester: HttpCommonErrorsTester
) -> None:
    async def do():
        await coolorg.alice.invite_cancel(coolorg.invited_alice_dev3.token)

    await authenticated_http_common_errors_tester(do)
