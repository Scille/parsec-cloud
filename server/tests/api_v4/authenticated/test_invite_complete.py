# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import InvitationToken, authenticated_cmds
from tests.common import CoolorgRpcClients


@pytest.mark.skip("Not implemented yet")
async def test_authenticated_invite_complete_ok(coolorg: CoolorgRpcClients) -> None:
    rep = await coolorg.alice.invite_complete(coolorg.invited_alice_dev3.token)
    assert rep == authenticated_cmds.v4.invite_complete.RepOk()


@pytest.mark.skip("Not implemented yet")
async def test_authenticated_invite_complete_invitation_not_found(
    coolorg: CoolorgRpcClients,
) -> None:
    rep = await coolorg.alice.invite_complete(InvitationToken.new())

    assert rep == authenticated_cmds.v4.invite_complete.RepInvitationNotFound()


@pytest.mark.skip("Not implemented yet")
async def test_authenticated_invite_complete_invitation_cancelled(
    coolorg: CoolorgRpcClients,
) -> None:
    rep = await coolorg.alice.invite_cancel(coolorg.invited_alice_dev3.token)
    assert rep == authenticated_cmds.v4.invite_cancel.RepOk()

    rep = await coolorg.alice.invite_complete(coolorg.invited_alice_dev3.token)
    assert rep == authenticated_cmds.v4.invite_complete.RepInvitationCancelled()


@pytest.mark.skip("Not implemented yet")
async def test_authenticated_invite_complete_invitation_already_completed(
    coolorg: CoolorgRpcClients,
) -> None:
    rep = await coolorg.alice.invite_complete(coolorg.invited_alice_dev3.token)
    assert rep == authenticated_cmds.v4.invite_complete.RepOk()

    rep = await coolorg.alice.invite_complete(coolorg.invited_alice_dev3.token)
    assert rep == authenticated_cmds.v4.invite_complete.RepInvitationCancelled()


@pytest.mark.skip("Not implemented yet")
async def test_authenticated_invite_complete_author_not_allowed(
    coolorg: CoolorgRpcClients,
) -> None:
    # TODO: Use a user invite and change the author profile to standard
    rep = await coolorg.alice.invite_complete(coolorg.invited_alice_dev3.token)
    assert rep == authenticated_cmds.v4.invite_complete.RepAuthorNotAllowed()
