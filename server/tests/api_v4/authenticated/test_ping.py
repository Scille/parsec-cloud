# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from parsec._parsec import authenticated_cmds
from parsec.events import EventPinged
from tests.common import Backend, MinimalorgRpcClients


async def test_authenticated_ping_ok(minimalorg: MinimalorgRpcClients, backend: Backend) -> None:
    with backend.event_bus.spy() as spy:
        rep = await minimalorg.alice.ping(ping="hello")
        assert rep == authenticated_cmds.v4.ping.RepOk(pong="hello")
        await spy.wait_event_occurred(
            EventPinged(
                organization_id=minimalorg.organization_id,
                ping="hello",
            )
        )
