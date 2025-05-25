# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from parsec._parsec import authenticated_account_cmds
from tests.common import AuthenticatedAccountRpcClient, HttpCommonErrorsTester


async def test_authenticated_account_ping_ok(
    xfail_if_postgresql: None,
    alice_account: AuthenticatedAccountRpcClient,
) -> None:
    rep = await alice_account.ping(ping="hello")
    assert rep == authenticated_account_cmds.latest.ping.RepOk(pong="hello")


async def test_authenticated_account_ping_http_common_errors(
    xfail_if_postgresql: None,
    alice_account: AuthenticatedAccountRpcClient,
    authenticated_account_http_common_errors_tester: HttpCommonErrorsTester,
) -> None:
    async def do():
        await alice_account.ping(ping="hello")

    await authenticated_account_http_common_errors_tester(do)
