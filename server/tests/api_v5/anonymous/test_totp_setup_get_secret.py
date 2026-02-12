# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from parsec._parsec import AccessToken, UserID, anonymous_cmds
from parsec.components.user import UserInfo
from tests.common import (
    Backend,
    CoolorgRpcClients,
    HttpCommonErrorsTester,
    MinimalorgRpcClients,
    alice_gives_profile,
)


async def test_anonymous_totp_setup_get_secret_ok(
    minimalorg: MinimalorgRpcClients, backend: Backend
) -> None:
    outcome = await backend.totp.reset(minimalorg.organization_id, minimalorg.alice.user_id)
    assert isinstance(outcome, tuple)
    _, _, totp_reset_addr, _ = outcome

    rep = await minimalorg.anonymous.totp_setup_get_secret(
        user_id=minimalorg.alice.user_id,
        token=totp_reset_addr.token,
    )
    assert isinstance(rep, anonymous_cmds.latest.totp_setup_get_secret.RepOk)
    assert isinstance(rep.totp_secret, bytes)
    assert len(rep.totp_secret) > 0

    # Check idempotence
    rep2 = await minimalorg.anonymous.totp_setup_get_secret(
        user_id=minimalorg.alice.user_id,
        token=totp_reset_addr.token,
    )
    assert rep2 == anonymous_cmds.latest.totp_setup_get_secret.RepOk(totp_secret=rep.totp_secret)

    # Note we check if `totp_secret` can be use to produce the right
    # one-time-password in `test_totp_setup_confirm.py`


async def test_anonymous_totp_setup_get_secret_bad_token(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    # Unknown token

    rep = await coolorg.anonymous.totp_setup_get_secret(
        user_id=coolorg.bob.user_id, token=AccessToken.new()
    )
    assert rep == anonymous_cmds.latest.totp_setup_get_secret.RepBadToken()

    # Good token, but wrong user

    outcome = await backend.totp.reset(coolorg.organization_id, coolorg.bob.user_id)
    assert isinstance(outcome, tuple)
    _, _, totp_reset_addr, _ = outcome

    rep = await coolorg.anonymous.totp_setup_get_secret(
        user_id=coolorg.alice.user_id, token=totp_reset_addr.token
    )
    assert rep == anonymous_cmds.latest.totp_setup_get_secret.RepBadToken()

    # Good token, but user doesn't exist

    rep = await coolorg.anonymous.totp_setup_get_secret(
        user_id=UserID.new(), token=totp_reset_addr.token
    )
    assert rep == anonymous_cmds.latest.totp_setup_get_secret.RepBadToken()

    # Good token, but user is frozen

    outcome = await backend.user.freeze_user(
        organization_id=coolorg.organization_id,
        user_id=coolorg.bob.user_id,
        user_email=None,
        frozen=True,
    )
    assert isinstance(outcome, UserInfo)

    rep = await coolorg.anonymous.totp_setup_get_secret(
        user_id=coolorg.bob.user_id, token=totp_reset_addr.token
    )
    assert rep == anonymous_cmds.latest.totp_setup_get_secret.RepBadToken()

    outcome = await backend.user.freeze_user(
        organization_id=coolorg.organization_id,
        user_id=coolorg.bob.user_id,
        user_email=None,
        frozen=False,
    )
    assert isinstance(outcome, UserInfo)

    # Good token, but user is revoked

    await alice_gives_profile(coolorg, backend, recipient=coolorg.bob.user_id, new_profile=None)

    rep = await coolorg.anonymous.totp_setup_get_secret(
        user_id=coolorg.bob.user_id, token=totp_reset_addr.token
    )
    assert rep == anonymous_cmds.latest.totp_setup_get_secret.RepBadToken()

    # Note we check that `bad_token` is returned after `totp_setup_confirm`
    # has been successfully called in `test_totp_setup_confirm.py`


async def test_anonymous_totp_setup_get_secret_http_common_errors(
    coolorg: CoolorgRpcClients,
    anonymous_http_common_errors_tester: HttpCommonErrorsTester,
) -> None:
    async def do():
        await coolorg.anonymous.totp_setup_get_secret(
            user_id=coolorg.alice.user_id,
            token=AccessToken.new(),
        )

    await anonymous_http_common_errors_tester(do)
