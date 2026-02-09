# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from parsec._parsec import AccessToken, invited_cmds
from tests.common import CoolorgRpcClients, HttpCommonErrorsTester, ShamirOrgRpcClients


async def test_invited_invite_shamir_recovery_reveal_ok(shamirorg: ShamirOrgRpcClients) -> None:
    token = shamirorg.alice_shamir_reveal_token
    ciphered_data = shamirorg.alice_shamir_ciphered_data
    rep = await shamirorg.shamir_invited_alice.invite_shamir_recovery_reveal(reveal_token=token)
    assert rep == invited_cmds.latest.invite_shamir_recovery_reveal.RepOk(
        ciphered_data=ciphered_data
    )


async def test_invited_invite_shamir_recovery_reveal_bad_reveal_token(
    shamirorg: ShamirOrgRpcClients,
) -> None:
    token = AccessToken.new()
    rep = await shamirorg.shamir_invited_alice.invite_shamir_recovery_reveal(token)
    assert rep == invited_cmds.latest.invite_shamir_recovery_reveal.RepBadRevealToken()


async def test_invited_invite_shamir_recovery_reveal_bad_invitation_type(
    coolorg: CoolorgRpcClients,
) -> None:
    token = AccessToken.new()

    rep = await coolorg.invited_zack.invite_shamir_recovery_reveal(token)
    assert rep == invited_cmds.latest.invite_shamir_recovery_reveal.RepBadInvitationType()

    rep = await coolorg.invited_alice_dev3.invite_shamir_recovery_reveal(token)
    assert rep == invited_cmds.latest.invite_shamir_recovery_reveal.RepBadInvitationType()


async def test_invited_invite_shamir_recovery_reveal_http_common_errors(
    coolorg: CoolorgRpcClients, invited_http_common_errors_tester: HttpCommonErrorsTester
) -> None:
    token = AccessToken.new()

    async def do():
        await coolorg.invited_alice_dev3.invite_shamir_recovery_reveal(reveal_token=token)

    await invited_http_common_errors_tester(do)
