# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from parsec._parsec import anonymous_account_cmds
from tests.common import AnonymousAccountRpcClient, HttpCommonErrorsTester


async def test_anonymous_account_ping_ok(anonymous_account: AnonymousAccountRpcClient) -> None:
    rep = await anonymous_account.ping(ping="hello")
    assert rep == anonymous_account_cmds.latest.ping.RepOk(pong="hello")


async def test_anonymous_account_ping_http_common_errors(
    anonymous_account: AnonymousAccountRpcClient,
    anonymous_account_http_common_errors_tester: HttpCommonErrorsTester,
) -> None:
    async def do():
        await anonymous_account.ping(ping="hello")

    await anonymous_account_http_common_errors_tester(do)
