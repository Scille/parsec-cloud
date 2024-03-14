# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from parsec._parsec import InvitationToken, authenticated_cmds
from tests.common import CoolorgRpcClients


async def test_ok(coolorg: CoolorgRpcClients) -> None:
    rep = await coolorg.alice.invite_cancel(coolorg.invited_alice_dev3.token)

    assert rep == authenticated_cmds.v4.invite_cancel.RepOk()


async def test_invitation_not_found(coolorg: CoolorgRpcClients) -> None:
    rep = await coolorg.alice.invite_cancel(InvitationToken.new())

    assert rep == authenticated_cmds.v4.invite_cancel.RepInvitationNotFound()


async def test_invitation_already_deleted(coolorg: CoolorgRpcClients) -> None:
    await test_ok(coolorg)

    rep = await coolorg.alice.invite_cancel(coolorg.invited_alice_dev3.token)

    assert rep == authenticated_cmds.v4.invite_cancel.RepInvitationAlreadyDeleted()
