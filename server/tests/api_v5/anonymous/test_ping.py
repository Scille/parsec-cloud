# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from parsec._parsec import anonymous_cmds
from tests.common import Backend, CoolorgRpcClients, HttpCommonErrorsTester, MinimalorgRpcClients


async def test_anonymous_ping_ok(minimalorg: MinimalorgRpcClients, backend: Backend) -> None:
    rep = await minimalorg.anonymous.ping(ping="hello")
    assert rep == anonymous_cmds.latest.ping.RepOk(pong="hello")


async def test_anonymous_ping_http_common_errors(
    coolorg: CoolorgRpcClients, anonymous_http_common_errors_tester: HttpCommonErrorsTester
) -> None:
    async def do():
        await coolorg.anonymous.ping(ping="hello")

    await anonymous_http_common_errors_tester(do)
