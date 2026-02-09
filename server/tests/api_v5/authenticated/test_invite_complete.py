# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from parsec._parsec import AccessToken, UserProfile, authenticated_cmds
from tests.common import (
    Backend,
    CoolorgRpcClients,
    HttpCommonErrorsTester,
    ShamirOrgRpcClients,
    bob_becomes_admin_and_changes_alice,
)


async def test_authenticated_invite_complete_ok(coolorg: CoolorgRpcClients) -> None:
    rep = await coolorg.alice.invite_complete(coolorg.invited_zack.token)
    assert rep == authenticated_cmds.latest.invite_complete.RepOk()

    rep = await coolorg.alice.invite_complete(coolorg.invited_alice_dev3.token)
    assert rep == authenticated_cmds.latest.invite_complete.RepOk()


async def test_authenticated_invite_complete_ok_with_shamir(shamirorg: ShamirOrgRpcClients) -> None:
    # For shamir recovery invitations, Alice can complete her own invitation
    rep = await shamirorg.alice.invite_complete(shamirorg.shamir_invited_alice.token)
    assert rep == authenticated_cmds.latest.invite_complete.RepOk()


async def test_authenticated_invite_complete_ok_for_non_admin_users(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    await bob_becomes_admin_and_changes_alice(
        coolorg, backend, new_alice_profile=UserProfile.STANDARD
    )
    rep = await coolorg.alice.invite_complete(coolorg.invited_alice_dev3.token)
    assert rep == authenticated_cmds.latest.invite_complete.RepOk()


async def test_authenticated_invite_complete_invitation_not_found(
    coolorg: CoolorgRpcClients,
) -> None:
    rep = await coolorg.alice.invite_complete(AccessToken.new())

    assert rep == authenticated_cmds.latest.invite_complete.RepInvitationNotFound()


async def test_authenticated_invite_complete_invitation_cancelled(
    coolorg: CoolorgRpcClients,
) -> None:
    rep = await coolorg.alice.invite_cancel(coolorg.invited_alice_dev3.token)
    assert rep == authenticated_cmds.latest.invite_cancel.RepOk()

    rep = await coolorg.alice.invite_complete(coolorg.invited_alice_dev3.token)
    assert rep == authenticated_cmds.latest.invite_complete.RepInvitationCancelled()


async def test_authenticated_invite_complete_invitation_already_completed(
    coolorg: CoolorgRpcClients,
) -> None:
    rep = await coolorg.alice.invite_complete(coolorg.invited_alice_dev3.token)
    assert rep == authenticated_cmds.latest.invite_complete.RepOk()

    rep = await coolorg.alice.invite_complete(coolorg.invited_alice_dev3.token)
    assert rep == authenticated_cmds.latest.invite_complete.RepInvitationAlreadyCompleted()


async def test_authenticated_invite_complete_author_not_allowed(
    coolorg: CoolorgRpcClients,
) -> None:
    rep = await coolorg.bob.invite_complete(coolorg.invited_alice_dev3.token)
    assert rep == authenticated_cmds.latest.invite_complete.RepAuthorNotAllowed()


async def test_authenticated_invite_complete_http_common_errors(
    coolorg: CoolorgRpcClients, authenticated_http_common_errors_tester: HttpCommonErrorsTester
) -> None:
    async def do():
        await coolorg.alice.invite_complete(coolorg.invited_alice_dev3.token)

    await authenticated_http_common_errors_tester(do)
