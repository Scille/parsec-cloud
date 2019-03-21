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
    backend_cmds_factory,
)

from tests.open_tcp_stream_mock_wrapper import offline


@pytest.mark.trio
async def test_backend_offline(alice):
    with pytest.raises(BackendNotAvailable):
        async with backend_cmds_factory(
            alice.organization_addr, alice.device_id, alice.signing_key
        ) as cmds:
            await cmds.ping()


@pytest.mark.trio
async def test_backend_switch_offline(running_backend, alice):
    async with backend_cmds_factory(
        alice.organization_addr, alice.device_id, alice.signing_key
    ) as cmds:
        with offline(running_backend.addr):
            with pytest.raises(BackendNotAvailable):
                await cmds.ping()


@pytest.mark.trio
async def test_backend_closed_cmds(running_backend, alice):
    async with backend_cmds_factory(
        alice.organization_addr, alice.device_id, alice.signing_key
    ) as cmds:
        pass
    with pytest.raises(trio.ClosedResourceError):
        await cmds.ping()


@pytest.mark.trio
async def test_backend_bad_handshake(running_backend, mallory):
    with pytest.raises(BackendHandshakeError):
        async with backend_cmds_factory(
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
        async with backend_cmds_factory(
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
                async with backend_cmds_factory(
                    alice.organization_addr, alice.device_id, alice.signing_key
                ) as cmds:
                    await cmds.ping()

        nursery.cancel_scope.cancel()
