# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

from collections import deque
from collections.abc import Iterator
from contextlib import contextmanager
from unittest.mock import MagicMock

import anyio
import pytest

from parsec._parsec import (
    ActiveUsersLimit,
    DateTime,
    RevokedUserCertificate,
    SigningKey,
    authenticated_cmds,
)
from parsec.events import EventPinged
from tests.common import Backend, CoolorgRpcClients, MinimalorgRpcClients
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


async def test_missed_events(minimalorg: MinimalorgRpcClients, backend: Backend) -> None:
    backend.config.sse_events_cache_size = 2
    backend.events._last_events_cache = deque(maxlen=backend.config.sse_events_cache_size)

    # We use dispatch_incoming_event to ensure the event is processed immediately once the function return.
    # That allow to bypass the standard route of `EventBus.send` that goes through to event system of PostgreSQL.
    dispatch_event_with_no_delay = backend.event_bus._dispatch_incoming_event

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
        event = await anext(alice_sse._iter_events)
        assert event.event == "missed_events"

        # Only recent event could be received
        event = await alice_sse.next_event()
        assert event == authenticated_cmds.v4.events_listen.RepOk(
            authenticated_cmds.v4.events_listen.APIEventPinged(ping="recent_event")
        )


async def test_close_on_backpressure(minimalorg: MinimalorgRpcClients, backend: Backend) -> None:
    """
    When event are stacking up too much on the backend side, because the client is too slow to consume them,
    The connection should be closed.
    """

    async with minimalorg.alice.events_listen() as alice_sse:
        # First event is always ServiceConfig
        event = await alice_sse.next_event()
        assert event == authenticated_cmds.v4.events_listen.RepOk(
            authenticated_cmds.v4.events_listen.APIEventServerConfig(
                active_users_limit=ActiveUsersLimit.NO_LIMIT,
                user_profile_outsider_allowed=True,
            )
        )

        # Patch the backend to simulate backpressure
        # We simulate that sending an event to the client will block (this is
        # the expected behavior if the buffer is full and there are no tasks
        # waiting to receive)
        registered_clients = backend.events._registered_clients
        assert len(registered_clients) == 1
        reg_client_id: int = next(iter(registered_clients.keys()))
        mock_send_nowait = MagicMock(side_effect=anyio.WouldBlock)
        registered_clients[reg_client_id].channel_sender.send_nowait = mock_send_nowait

        await backend.event_bus.send(
            EventPinged(
                organization_id=minimalorg.organization_id,
                ping="foo",
            )
        )
        # When the server detect backpressure, it should close the connection
        with pytest.raises(StopAsyncIteration):
            await alice_sse.next_event()

        mock_send_nowait.assert_called_once()


async def test_empty_last_event_id(minimalorg: MinimalorgRpcClients, backend: Backend) -> None:
    """
    When the client sends a Last-Event-ID header with an empty value,
    the server should ignore it as if it was not provided.
    """
    async with minimalorg.alice.events_listen(last_event_id="") as alice_sse:
        backend.event_bus._dispatch_incoming_event(
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

        # Backend still send new events without processing `Last-Event-ID`
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


async def test_close_on_user_revoked(coolorg: CoolorgRpcClients, backend: Backend) -> None:
    async def send_ping(ping: str) -> None:
        await backend.event_bus.send(
            EventPinged(
                organization_id=coolorg.organization_id,
                ping=ping,
            )
        )

    async with coolorg.bob.events_listen() as bob_sse:
        await send_ping("before-revocation")

        now = DateTime.now()
        revoke_certif = RevokedUserCertificate(
            author=coolorg.alice.device_id,
            timestamp=now,
            user_id=coolorg.bob.user_id,
        )
        rep = await coolorg.alice.user_revoke(
            revoke_certif.dump_and_sign(coolorg.alice.signing_key)
        )
        assert rep == authenticated_cmds.v4.user_revoke.RepOk()

        await send_ping("after-revocation")

        # Bob still receives the ServerConfig event
        event = await bob_sse.next_event()
        assert event == authenticated_cmds.v4.events_listen.RepOk(
            authenticated_cmds.v4.events_listen.APIEventServerConfig(
                active_users_limit=ActiveUsersLimit.NO_LIMIT,
                user_profile_outsider_allowed=True,
            )
        )

        # And the events sent before the revocation
        event = await bob_sse.next_event()
        assert event == authenticated_cmds.v4.events_listen.RepOk(
            authenticated_cmds.v4.events_listen.APIEventPinged(ping="before-revocation")
        )

        # Then Bob receives a notification that he was revoked
        event = await bob_sse.next_event()
        assert event == authenticated_cmds.v4.events_listen.RepOk(
            authenticated_cmds.v4.events_listen.APIEventCommonCertificate(timestamp=now)
        )

        # And then the connection is closed
        with pytest.raises(StopAsyncIteration):
            event = await bob_sse.next_event()


async def test_close_on_organization_tos_updated(
    coolorg: CoolorgRpcClients, backend: Backend
) -> None:
    async def send_ping(ping: str) -> None:
        await backend.event_bus.send(
            EventPinged(
                organization_id=coolorg.organization_id,
                ping=ping,
            )
        )

    async with coolorg.bob.events_listen() as bob_sse:
        await send_ping("before-tos-update")

        now = DateTime.now()
        outcome = await backend.organization.update(
            now=now, id=coolorg.organization_id, tos={"fr_FR": "https://example.com/tos_fr.html"}
        )
        assert outcome is None

        await send_ping("after-tos-update")

        # Bob still receives the ServerConfig event
        event = await bob_sse.next_event()
        assert event == authenticated_cmds.v4.events_listen.RepOk(
            authenticated_cmds.v4.events_listen.APIEventServerConfig(
                active_users_limit=ActiveUsersLimit.NO_LIMIT,
                user_profile_outsider_allowed=True,
            )
        )

        # And the events sent before the tos-update
        event = await bob_sse.next_event()
        assert event == authenticated_cmds.v4.events_listen.RepOk(
            authenticated_cmds.v4.events_listen.APIEventPinged(ping="before-tos-update")
        )

        # And then the connection is closed
        with pytest.raises(StopAsyncIteration):
            event = await bob_sse.next_event()
