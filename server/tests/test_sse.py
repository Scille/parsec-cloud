# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from collections import deque

import pytest

from parsec._parsec import ActiveUsersLimit, SigningKey, authenticated_cmds
from parsec.events import EventPinged
from tests.common import Backend, MinimalorgRpcClients


@pytest.mark.parametrize("initial_good_auth", (False, True))
async def test_events_listen_auth_then_not_allowed(
    initial_good_auth: bool,
    minimalorg: MinimalorgRpcClients,
) -> None:
    if initial_good_auth:
        # First authentication goes fine (which setups authentication cache)...
        async with minimalorg.alice.events_listen() as alice_sse:
            await alice_sse.next_event()

    # ...no we force alice to use the wrong signing key...
    minimalorg.alice.signing_key = SigningKey.generate()

    # ...which cause authentication failure

    async with minimalorg.alice.raw_sse_connection() as response:
        assert response.status_code == 403, response.content


# TODO: Here put generic tests on the `/authenticated/<raw_organization_id>/events` route:
# TODO: - test keepalive (must be done by starting an actual uvicorn server)
# TODO: - test sending empty `Last-Event-ID` is different from not sending it (should trigger `event:missed_events`)
# TODO: - test close on user revoked
# TODO: - test close on backpressure (too many events pilling up)


async def test_missed_events(minimalorg: MinimalorgRpcClients, backend: Backend) -> None:
    backend.config.sse_events_cache_size = 2
    backend.events._last_events_cache = deque(maxlen=backend.config.sse_events_cache_size)  # pyright: ignore[reportPrivateUsage]

    # We use dispatch_incoming_event to ensure the event is processed immediately once the function return.
    # That allow to bypass the standard route of `EventBus.send` that goes through to event system of PostgreSQL.
    dispatch_event_with_no_delay = backend.event_bus._dispatch_incoming_event  # pyright: ignore[reportPrivateUsage]

    first_event = EventPinged(
        organization_id=minimalorg.organization_id,
        ping="event0",
    )
    dispatch_event_with_no_delay(first_event)
    for ping in range(1, 5):
        dispatch_event_with_no_delay(
            EventPinged(
                organization_id=minimalorg.organization_id,
                ping=f"event{ping}",
            )
        )

    async with minimalorg.alice.events_listen(last_event_id=str(first_event.event_id)) as alice_sse:
        dispatch_event_with_no_delay(
            EventPinged(
                organization_id=minimalorg.organization_id,
                ping="recent_event",
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

        # Backend should inform that some events were missed and we could not retrieve them
        event = await anext(alice_sse._iter_events)  # pyright: ignore[reportPrivateUsage]
        assert event.event == "missed_events"

        # Only recent event could be received
        event = await alice_sse.next_event()
        assert event == authenticated_cmds.v4.events_listen.RepOk(
            authenticated_cmds.v4.events_listen.APIEventPinged(ping="recent_event")
        )
