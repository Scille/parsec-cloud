# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import pytest

from parsec._parsec import ActiveUsersLimit, authenticated_cmds
from parsec.events import EventPinged
from tests.common import Backend, MinimalorgRpcClients


async def test_events_listen_ok(
    minimalorg: MinimalorgRpcClients,
    backend: Backend,
) -> None:
    # Use `_dispatch_incoming_event` instead of `send` to ensure the event
    # has been handle by the callbacks once the function returns.
    backend.event_bus._dispatch_incoming_event(
        EventPinged(
            organization_id=minimalorg.organization_id,
            ping="event1",
        )
    )
    async with minimalorg.alice.events_listen() as alice_sse:
        backend.event_bus._dispatch_incoming_event(
            EventPinged(
                organization_id=minimalorg.organization_id,
                ping="event2",
            )
        )

        # First event is always ServiceConfig
        event = await alice_sse.next_event()
        assert event == authenticated_cmds.v4.events_listen.RepOk(
            authenticated_cmds.v4.events_listen.APIEventServerConfig(
                active_users_limit=ActiveUsersLimit.NO_LIMIT,
                user_profile_outsider_allowed=True,
            )
        )

        event = await alice_sse.next_event()
        assert event == authenticated_cmds.v4.events_listen.RepOk(
            authenticated_cmds.v4.events_listen.APIEventPinged(ping="event2")
        )


# TODO: test for each event type
# TODO: test `Last-Event-ID`
# TODO: test connection gets closed due to SseAPiEventsListenBadOutcome


@pytest.mark.timeout(2)
async def test_keep_alive(minimalorg: MinimalorgRpcClients, backend: Backend) -> None:
    # Reduce keepalive to 0.1s to speed up the test
    backend.config.sse_keepalive = 0.1
    got_keepalive = False

    async with minimalorg.alice.raw_sse_connection() as raw_sse_stream:
        async for line in raw_sse_stream.aiter_lines():
            line = line.rstrip("\n")

            if line == ":keepalive":
                got_keepalive = True
                break

    assert got_keepalive
