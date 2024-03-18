# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from parsec._parsec import ActiveUsersLimit, authenticated_cmds
from parsec.events import EventPinged
from tests.common import Backend, MinimalorgRpcClients


async def test_user_not_receive_event_before_listen(
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
# TODO: test keepalive
# TODO: test `Last-Event-ID`
# TODO: test connection gets closed due to SseAPiEventsListenBadOutcome
