# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from parsec._parsec import DateTime, authenticated_cmds
from parsec.components.totp import compute_totp_one_time_password
from tests.common import Backend, CoolorgRpcClients, HttpCommonErrorsTester, MinimalorgRpcClients

from .test_totp_create_opaque_key import _init_totp


async def test_authenticated_totp_setup_confirm_ok(
    minimalorg: MinimalorgRpcClients, backend: Backend
) -> None:
    totp_secret = await backend.totp.setup_get_secret(
        minimalorg.organization_id, minimalorg.alice.device_id
    )
    assert isinstance(totp_secret, bytes)

    # Confirm with valid OTP

    one_time_password = compute_totp_one_time_password(now=DateTime.now(), secret=totp_secret)
    rep = await minimalorg.alice.totp_setup_confirm(
        one_time_password=one_time_password,
    )
    assert rep == authenticated_cmds.latest.totp_setup_confirm.RepOk()

    # Now we cannot get the TOTP secret anymore

    outcome = await minimalorg.alice.totp_setup_get_secret()
    assert outcome == authenticated_cmds.latest.totp_setup_get_secret.RepAlreadySetup()


async def test_authenticated_totp_setup_confirm_invalid_one_time_password(
    minimalorg: MinimalorgRpcClients,
) -> None:
    rep = await minimalorg.alice.totp_setup_confirm(
        one_time_password="00",  # Always wrong given our one-time-password is supposed to be 6 digits
    )
    assert rep == authenticated_cmds.latest.totp_setup_confirm.RepInvalidOneTimePassword()


async def test_authenticated_totp_setup_confirm_already_setup(
    minimalorg: MinimalorgRpcClients, backend: Backend
) -> None:
    _, one_time_password = await _init_totp(minimalorg, backend)

    rep = await minimalorg.alice.totp_setup_confirm(
        one_time_password=one_time_password,
    )
    assert rep == authenticated_cmds.latest.totp_setup_confirm.RepAlreadySetup()


async def test_authenticated_totp_setup_confirm_http_common_errors(
    coolorg: CoolorgRpcClients,
    authenticated_http_common_errors_tester: HttpCommonErrorsTester,
) -> None:
    async def do():
        await coolorg.alice.totp_setup_confirm(one_time_password="000000")

    await authenticated_http_common_errors_tester(do)
