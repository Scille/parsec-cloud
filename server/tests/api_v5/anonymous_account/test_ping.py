# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from parsec._parsec import anonymous_account_cmds
from tests.common import Backend, HttpCommonErrorsTester
from tests.common.client import AccountRpcClient


async def test_anonymous_account_ping_ok(
    xfail_if_postgresql: None, account: AccountRpcClient, backend: Backend
) -> None:
    rep = await account.anonymous_account.ping(ping="hello")
    assert rep == anonymous_account_cmds.latest.ping.RepOk(pong="hello")


async def test_anonymous_account_ping_http_common_errors(
    xfail_if_postgresql: None,
    account: AccountRpcClient,
    anonymous_account_http_common_errors_tester: HttpCommonErrorsTester,
) -> None:
    async def do():
        await account.anonymous_account.ping(ping="hello")

    await anonymous_account_http_common_errors_tester(do)
