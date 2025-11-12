# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


from parsec._parsec import anonymous_server_cmds
from tests.common import AnonymousServerRpcClient, HttpCommonErrorsTester


async def test_anonymous_server_ping_ok(anonymous_server: AnonymousServerRpcClient) -> None:
    rep = await anonymous_server.ping(ping="hello")
    assert rep == anonymous_server_cmds.latest.ping.RepOk(pong="hello")


async def test_anonymous_server_ping_http_common_errors(
    anonymous_server: AnonymousServerRpcClient,
    anonymous_server_http_common_errors_tester: HttpCommonErrorsTester,
) -> None:
    async def do():
        await anonymous_server.ping(ping="hello")

    await anonymous_server_http_common_errors_tester(do)
