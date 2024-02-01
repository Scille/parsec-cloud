# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from parsec._parsec import invited_cmds
from tests.common import Backend, CoolorgRpcClients


async def test_invited_ping_ok(coolorg: CoolorgRpcClients, backend: Backend) -> None:
    rep = await coolorg.invited_zack.ping(ping="hello")
    assert rep == invited_cmds.v4.ping.RepOk(pong="hello")
