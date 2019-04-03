# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
import trio
import pendulum

from parsec.api.protocole import ServerHandshake
from parsec.crypto import build_revoked_device_certificate
from parsec.api.transport import Transport
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
    # Remember Parsec API's ping (basically a regular Parsec command)
    # and Websocket ping (executed at lower layer, can be send during
    # a Parsec command without breaking request/reply pattern) are
    # two completly different things !
    send_ping, recv_ping = trio.open_memory_channel(1)
    vanilla_ping = Transport.ping

    async def _mocked_ping(transport):
        await send_ping.send(transport)
        return await vanilla_ping(transport)

    monkeypatch.setattr(Transport, "ping", _mocked_ping)

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
        alice.organization_addr, alice.device_id, alice.signing_key
    ) as cmds:
        mock_clock.rate = 1
        with trio.fail_after(3):
            async with trio.open_nursery() as nursery:

                async def _cmd():
                    nonlocal event
                    event = await cmds.events_listen(wait=True, watchdog_time=2)

                nursery.start_soon(_cmd)

                await backend_received_cmd.wait()
                mock_clock.jump(2)
                await recv_ping.receive()

                await backend_client_ctx.send_events_channel.send(
                    {"event": "pinged", "ping": "foo"}
                )

    assert event == {"event": "pinged", "ping": "foo"}

    # TODO: this test is good (but somewhat cumbersome...) but it lacks
    # checks on what actually happened on the wire as Websocket protocol level.
