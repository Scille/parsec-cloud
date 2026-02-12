# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from parsec._parsec import AccessToken, DateTime, UserID, anonymous_cmds, authenticated_cmds
from parsec.components.totp import compute_totp_one_time_password
from parsec.components.user import UserInfo
from tests.common import (
    Backend,
    CoolorgRpcClients,
    HttpCommonErrorsTester,
    MinimalorgRpcClients,
    bob_becomes_admin_and_changes_alice,
)


async def _reset_alice_totp_setup(
    org: MinimalorgRpcClients | CoolorgRpcClients, backend: Backend
) -> tuple[AccessToken, bytes, str]:
    outcome = await backend.totp.reset(org.organization_id, org.alice.user_id)
    assert isinstance(outcome, tuple)
    _, _, totp_reset_addr, _ = outcome

    totp_secret = await backend.totp.setup_get_secret_with_token(
        org.organization_id, org.alice.user_id, totp_reset_addr.token
    )
    assert isinstance(totp_secret, bytes)

    one_time_password = compute_totp_one_time_password(now=DateTime.now(), secret=totp_secret)

    return totp_reset_addr.token, totp_secret, one_time_password


async def test_anonymous_totp_setup_confirm_ok(
    minimalorg: MinimalorgRpcClients, backend: Backend
) -> None:
    reset_token, _, one_time_password = await _reset_alice_totp_setup(minimalorg, backend)

    # Confirm with valid OTP

    rep = await minimalorg.anonymous.totp_setup_confirm(
        user_id=minimalorg.alice.user_id,
        token=reset_token,
        one_time_password=one_time_password,
    )
    assert rep == anonymous_cmds.latest.totp_setup_confirm.RepOk()

    # Now we cannot get the TOTP secret anymore...

    outcome = await minimalorg.alice.totp_setup_get_secret()
    assert outcome == authenticated_cmds.latest.totp_setup_get_secret.RepAlreadySetup()

    outcome = await minimalorg.anonymous.totp_setup_get_secret(
        user_id=minimalorg.alice.user_id,
        token=reset_token,
    )
    assert outcome == anonymous_cmds.latest.totp_setup_get_secret.RepBadToken()

    # ...or re-confirm

    rep = await minimalorg.anonymous.totp_setup_confirm(
        user_id=minimalorg.alice.user_id,
        token=reset_token,
        one_time_password=one_time_password,
    )
    assert rep == anonymous_cmds.latest.totp_setup_confirm.RepBadToken()


async def test_anonymous_totp_setup_confirm_invalid_one_time_password(
    minimalorg: MinimalorgRpcClients, backend: Backend
) -> None:
    outcome = await backend.totp.reset(minimalorg.organization_id, minimalorg.alice.user_id)
    assert isinstance(outcome, tuple)
    _, _, totp_reset_addr, _ = outcome

    rep = await minimalorg.anonymous.totp_setup_confirm(
        user_id=minimalorg.alice.user_id,
        token=totp_reset_addr.token,
        one_time_password="00",  # Always wrong given our one-time-password is supposed to be 6 digits
    )
    assert rep == anonymous_cmds.latest.totp_setup_confirm.RepInvalidOneTimePassword()


async def test_anonymous_totp_setup_confirm_bad_token(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    # Unknown token

    rep = await coolorg.anonymous.totp_setup_confirm(
        user_id=coolorg.alice.user_id,
        token=AccessToken.new(),
        one_time_password="000000",
    )
    assert rep == anonymous_cmds.latest.totp_setup_confirm.RepBadToken()

    # Good token, but wrong user

    reset_token, _, one_time_password = await _reset_alice_totp_setup(coolorg, backend)

    rep = await coolorg.anonymous.totp_setup_confirm(
        user_id=coolorg.bob.user_id,
        token=reset_token,
        one_time_password=one_time_password,
    )
    assert rep == anonymous_cmds.latest.totp_setup_confirm.RepBadToken()

    # Good token, but user doesn't exist

    rep = await coolorg.anonymous.totp_setup_confirm(
        user_id=UserID.new(),
        token=reset_token,
        one_time_password=one_time_password,
    )
    assert rep == anonymous_cmds.latest.totp_setup_confirm.RepBadToken()

    # Good token, but user is frozen

    outcome = await backend.user.freeze_user(
        organization_id=coolorg.organization_id,
        user_id=coolorg.alice.user_id,
        user_email=None,
        frozen=True,
    )
    assert isinstance(outcome, UserInfo)

    rep = await coolorg.anonymous.totp_setup_confirm(
        user_id=coolorg.alice.user_id,
        token=reset_token,
        one_time_password=one_time_password,
    )
    assert rep == anonymous_cmds.latest.totp_setup_confirm.RepBadToken()

    outcome = await backend.user.freeze_user(
        organization_id=coolorg.organization_id,
        user_id=coolorg.alice.user_id,
        user_email=None,
        frozen=False,
    )
    assert isinstance(outcome, UserInfo)

    # Good token, but user is revoked

    await bob_becomes_admin_and_changes_alice(coolorg, backend, new_alice_profile=None)

    rep = await coolorg.anonymous.totp_setup_confirm(
        user_id=coolorg.alice.user_id,
        token=reset_token,
        one_time_password=one_time_password,
    )
    assert rep == anonymous_cmds.latest.totp_setup_confirm.RepBadToken()


async def test_anonymous_totp_setup_confirm_http_common_errors(
    coolorg: CoolorgRpcClients,
    anonymous_http_common_errors_tester: HttpCommonErrorsTester,
) -> None:
    async def do():
        await coolorg.anonymous.totp_setup_confirm(
            user_id=coolorg.alice.user_id,
            token=AccessToken.new(),
            one_time_password="000000",
        )

    await anonymous_http_common_errors_tester(do)
