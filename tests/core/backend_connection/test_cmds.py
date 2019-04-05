# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
import trio
import pendulum

from parsec.api.protocole import ServerHandshake
from parsec.crypto import build_revoked_device_certificate
from parsec.api.transport import Transport, PingReceived, PongReceived
from parsec.core.backend_connection import (
    BackendNotAvailable,
    BackendHandshakeError,
    BackendDeviceRevokedError,
    backend_cmds_pool_factory,
)

from tests.open_tcp_stream_mock_wrapper import offline


@pytest.mark.trio
async def test_backend_offline(alice):
    with pytest.raises(BackendNotAvailable):
        async with backend_cmds_pool_factory(
            alice.organization_addr, alice.device_id, alice.signing_key
        ) as cmds:
            await cmds.ping()


@pytest.mark.trio
async def test_backend_switch_offline(running_backend, alice):
    async with backend_cmds_pool_factory(
        alice.organization_addr, alice.device_id, alice.signing_key
    ) as cmds:
        with offline(running_backend.addr):
            with pytest.raises(BackendNotAvailable):
                await cmds.ping()


@pytest.mark.trio
async def test_backend_closed_cmds(running_backend, alice):
    async with backend_cmds_pool_factory(
        alice.organization_addr, alice.device_id, alice.signing_key
    ) as cmds:
        pass
    with pytest.raises(trio.ClosedResourceError):
        await cmds.ping()


@pytest.mark.trio
async def test_backend_bad_handshake(running_backend, mallory):
    with pytest.raises(BackendHandshakeError):
        async with backend_cmds_pool_factory(
            mallory.organization_addr, mallory.device_id, mallory.signing_key
        ) as cmds:
            await cmds.ping()


@pytest.mark.trio
async def test_revoked_device_handshake(running_backend, backend, alice, alice2):
    revoked_device_certificate = build_revoked_device_certificate(
        alice.device_id, alice.signing_key, alice2.device_id, pendulum.now()
    )
    await backend.user.revoke_device(
        alice2.organization_id, alice2.device_id, revoked_device_certificate, alice.device_id
    )

    with pytest.raises(BackendDeviceRevokedError):
        async with backend_cmds_pool_factory(
            alice2.organization_addr, alice2.device_id, alice2.signing_key
        ) as cmds:
            await cmds.ping()


@pytest.mark.trio
async def test_backend_disconnect_during_handshake(tcp_stream_spy, alice, backend_addr):
    async def poorly_serve_client(stream):
        transport = await Transport.init_for_client(
            stream, f"{backend_addr.host}:{backend_addr.port}"
        )
        handshake = ServerHandshake()
        await transport.send(handshake.build_challenge_req())
        await transport.recv()
        # Close connection during handshake
        await stream.aclose()

    async with trio.open_nursery() as nursery:

        async def connection_factory(*args, **kwargs):
            client_stream, server_stream = trio.testing.memory_stream_pair()
            nursery.start_soon(poorly_serve_client, server_stream)
            return client_stream

        with tcp_stream_spy.install_hook(backend_addr, connection_factory):
            with pytest.raises(BackendNotAvailable):
                async with backend_cmds_pool_factory(
                    alice.organization_addr, alice.device_id, alice.signing_key
                ) as cmds:
                    await cmds.ping()

        nursery.cancel_scope.cancel()


@pytest.mark.trio
async def test_events_listen_wait_has_watchdog(monkeypatch, mock_clock, running_backend, alice):
    # Spy on the transport events to detect the Pings/Pongs
    transport_events_sender, transport_events_receiver = trio.open_memory_channel(100)
    _vanilla_next_ws_event = Transport._next_ws_event

    async def _mocked_next_ws_event(transport):
        event = await _vanilla_next_ws_event(transport)
        await transport_events_sender.send((transport, event))
        return event

    monkeypatch.setattr(Transport, "_next_ws_event", _mocked_next_ws_event)

    async def next_ping_related_event():
        while True:
            transport, event = await transport_events_receiver.receive()
            if isinstance(event, (PingReceived, PongReceived)):
                return (transport, event)

    # Highjack the backend api to control the wait time and the final
    # event that will be returned to the client
    backend_received_cmd = trio.Event()
    backend_client_ctx = None
    vanilla_api_events_listen = running_backend.backend.logged_cmds["events_listen"]

    async def _mocked_api_events_listen(client_ctx, msg):
        nonlocal backend_client_ctx
        backend_client_ctx = client_ctx
        backend_received_cmd.set()
        return await vanilla_api_events_listen(client_ctx, msg)

    running_backend.backend.logged_cmds["events_listen"] = _mocked_api_events_listen

    event = None
    async with backend_cmds_pool_factory(
        alice.organization_addr, alice.device_id, alice.signing_key, keepalive_time=2
    ) as cmds:
        mock_clock.rate = 1
        async with trio.open_nursery() as nursery:

            async def _cmd():
                nonlocal event
                event = await cmds.events_listen(wait=True)

            nursery.start_soon(_cmd)

            # Wait for the connection to be established with the backend
            with trio.fail_after(1):
                await backend_received_cmd.wait()

            # Now advance time until ping is requested
            await trio.testing.wait_all_tasks_blocked()
            mock_clock.jump(2)
            with trio.fail_after(2):
                backend_transport, event = await next_ping_related_event()
                assert isinstance(event, PingReceived)
                client_transport, event = await next_ping_related_event()
                assert isinstance(event, PongReceived)
                assert client_transport is not backend_transport

            # Wait for another ping, just to be sure...
            await trio.testing.wait_all_tasks_blocked()
            mock_clock.jump(2)
            with trio.fail_after(1):
                backend_transport2, event = await next_ping_related_event()
                assert isinstance(event, PingReceived)
                assert backend_transport is backend_transport2
                client_transport2, event = await next_ping_related_event()
                assert isinstance(event, PongReceived)
                assert client_transport is client_transport2

            await backend_client_ctx.send_events_channel.send({"event": "pinged", "ping": "foo"})

    assert event == {"event": "pinged", "ping": "foo"}
