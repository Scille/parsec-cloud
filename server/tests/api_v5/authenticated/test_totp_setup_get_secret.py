# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from parsec._parsec import authenticated_cmds
from tests.common import Backend, CoolorgRpcClients, HttpCommonErrorsTester, MinimalorgRpcClients

from .test_totp_create_opaque_key import _init_totp


async def test_authenticated_totp_setup_get_secret_ok(minimalorg: MinimalorgRpcClients) -> None:
    rep = await minimalorg.alice.totp_setup_get_secret()
    assert isinstance(rep, authenticated_cmds.latest.totp_setup_get_secret.RepOk)
    assert isinstance(rep.totp_secret, bytes)
    assert len(rep.totp_secret) > 0

    # Check idempotence
    rep2 = await minimalorg.alice.totp_setup_get_secret()
    assert rep2 == authenticated_cmds.latest.totp_setup_get_secret.RepOk(
        totp_secret=rep.totp_secret
    )

    # Note we check if `totp_secret` can be use to produce the right
    # one-time-password in `test_totp_setup_confirm.py`


async def test_authenticated_totp_setup_get_secret_already_setup(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    await _init_totp(coolorg, backend)

    rep = await coolorg.alice.totp_setup_get_secret()
    assert rep == authenticated_cmds.latest.totp_setup_get_secret.RepAlreadySetup()


async def test_authenticated_totp_setup_get_secret_http_common_errors(
    coolorg: CoolorgRpcClients,
    authenticated_http_common_errors_tester: HttpCommonErrorsTester,
) -> None:
    async def do():
        await coolorg.alice.totp_setup_get_secret()

    await authenticated_http_common_errors_tester(do)
