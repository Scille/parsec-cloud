# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from parsec._parsec import DateTime, authenticated_cmds
from parsec.components.totp import compute_totp_one_time_password
from tests.common import Backend, CoolorgRpcClients, HttpCommonErrorsTester, MinimalorgRpcClients


async def _init_totp(
    org: MinimalorgRpcClients | CoolorgRpcClients, backend: Backend
) -> tuple[bytes, str]:
    totp_secret = await backend.totp.setup_get_secret(
        organization_id=org.organization_id,
        author=org.alice.device_id,
    )
    assert isinstance(totp_secret, bytes)

    one_time_password = compute_totp_one_time_password(DateTime.now(), totp_secret)

    outcome = await backend.totp.setup_confirm(
        organization_id=org.organization_id,
        author=org.alice.device_id,
        one_time_password=one_time_password,
    )
    assert outcome is None

    return totp_secret, one_time_password


async def test_authenticated_totp_create_opaque_key_ok(
    coolorg: CoolorgRpcClients,
    backend: Backend,
) -> None:
    _, one_time_password = await _init_totp(coolorg, backend)

    rep = await coolorg.alice.totp_create_opaque_key()
    assert isinstance(rep, authenticated_cmds.latest.totp_create_opaque_key.RepOk)

    # Creating multiple opaque keys should return different IDs and keys

    rep2 = await coolorg.alice.totp_create_opaque_key()
    assert isinstance(rep2, authenticated_cmds.latest.totp_create_opaque_key.RepOk)

    assert rep.opaque_key_id != rep2.opaque_key_id
    assert rep.opaque_key != rep2.opaque_key

    outcome = await backend.totp.fetch_opaque_key(
        now=DateTime.now(),
        organization_id=coolorg.organization_id,
        user_id=coolorg.alice.user_id,
        opaque_key_id=rep.opaque_key_id,
        one_time_password=one_time_password,
    )
    assert outcome == rep.opaque_key


async def test_authenticated_totp_create_opaque_key_http_common_errors(
    coolorg: CoolorgRpcClients,
    authenticated_http_common_errors_tester: HttpCommonErrorsTester,
) -> None:
    async def do():
        await coolorg.alice.totp_create_opaque_key()

    await authenticated_http_common_errors_tester(do)
