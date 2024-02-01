# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from parsec._parsec import anonymous_cmds
from tests.common import Backend, MinimalorgRpcClients


async def test_anonymous_ping_ok(minimalorg: MinimalorgRpcClients, backend: Backend) -> None:
    rep = await minimalorg.anonymous.ping(ping="hello")
    assert rep == anonymous_cmds.v4.ping.RepOk(pong="hello")
