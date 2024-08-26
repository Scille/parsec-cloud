# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from parsec._parsec import invited_cmds
from tests.common import Backend, CoolorgRpcClients, HttpCommonErrorsTester


async def test_invited_ping_ok(coolorg: CoolorgRpcClients, backend: Backend) -> None:
    rep = await coolorg.invited_zack.ping(ping="hello")
    assert rep == invited_cmds.v4.ping.RepOk(pong="hello")


async def test_invited_ping_http_common_errors(
    coolorg: CoolorgRpcClients, invited_http_common_errors_tester: HttpCommonErrorsTester
) -> None:
    async def do():
        await coolorg.invited_alice_dev3.ping(ping="hello")

    await invited_http_common_errors_tester(do)
