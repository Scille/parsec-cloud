# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from parsec._parsec import authenticated_account_cmds
from tests.common import Backend, CoolorgRpcClients, HttpCommonErrorsTester, MinimalorgRpcClients


async def test_authenticated_account_ping_ok(
    minimalorg: MinimalorgRpcClients, backend: Backend
) -> None:
    rep = await minimalorg.authenticated_account.ping(ping="hello")
    assert rep == authenticated_account_cmds.latest.ping.RepOk(pong="hello")


async def test_authenticated_account_ping_http_common_errors(
    coolorg: CoolorgRpcClients,
    authenticated_account_http_common_errors_tester: HttpCommonErrorsTester,
) -> None:
    async def do():
        await coolorg.authenticated_account.ping(ping="hello")

    await authenticated_account_http_common_errors_tester(do)
