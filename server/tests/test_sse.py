# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from collections import deque
from collections.abc import Iterator
from contextlib import contextmanager

import pytest

from parsec._parsec import ActiveUsersLimit, SigningKey, authenticated_cmds
from parsec.events import EventPinged
from tests.common import Backend, MinimalorgRpcClients
from tests.common.backend import AsgiApp


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


@contextmanager
def fork_process_to_run_asgi_server(app: AsgiApp) -> Iterator[tuple[str, int]]:
    """
    Uvicorn is not designed to be run programmatically like we are going to do here
    (it contains calls to `sys.exit`, captures signal, and spawn tasks in the event loop).
    So instead we have to spawn it in it own sub-process...

    Note this comes with is own issues given it means the our test process gets
    forked and hence the `app`/`backend` objects diverge between the two processes
    (i.e. cannot send a request to the server then use the `backend` object to check
    how the server internal state has changed).
    """
    from multiprocessing import Process

    from parsec.asgi import serve_parsec_asgi_app

    # TODO: This is a hack to get a unique-enough port for the server.
    # It would be cleaner to pass port=0, then parse the sub-process
    # stdout to retrieve the actual port in use.
    def get_local_port(host: str) -> int:
        import socket

        with socket.socket(socket.AF_INET, socket.SOCK_STREAM) as s:
            s.bind((host, 0))
            return s.getsockname()[1]

    HOST = "localhost"
    PORT = get_local_port(HOST)

    proc = Process(
        target=serve_parsec_asgi_app,
        kwargs={
            "app": app,
            "host": HOST,
            "port": PORT,
        },
    )

    proc.start()
    yield (HOST, PORT)
    proc.terminate()


@pytest.mark.timeout(2)
async def test_keep_alive_real_server(minimalorg: MinimalorgRpcClients, app: AsgiApp) -> None:
    assert isinstance(app.state.backend, Backend)
    app.state.backend.config.sse_keepalive = 0.1

    got_keepalive = False

    alice = minimalorg.alice
    with fork_process_to_run_asgi_server(app) as (host, port):
        alice.url = f"http://{host}:{port}/authenticated/{alice.organization_id}"

        async with alice.raw_sse_connection() as raw_sse_stream:
            async for line in raw_sse_stream.aiter_lines():
                line = line.rstrip("\n")

                if line == ":keepalive":
                    got_keepalive = True
                    break

    assert got_keepalive
