import pytest
import trio

from parsec.api.protocole import ServerHandshake
from parsec.trustchain import certify_device_revocation
from parsec.api.transport import WebsocketTransport
from parsec.core.backend_connection import (
    BackendNotAvailable,
    BackendHandshakeError,
    BackendDeviceRevokedError,
    backend_cmds_factory,
)

from tests.open_tcp_stream_mock_wrapper import offline


@pytest.mark.trio
async def test_backend_offline(backend_addr, alice):
    with pytest.raises(BackendNotAvailable):
        async with backend_cmds_factory(backend_addr, alice.device_id, alice.signing_key):
            pass


@pytest.mark.trio
async def test_backend_switch_offline(running_backend, alice):
    async with backend_cmds_factory(
        running_backend.addr, alice.device_id, alice.signing_key
    ) as cmds:
        with offline(running_backend.addr):
            with pytest.raises(BackendNotAvailable):
                await cmds.ping("Whatever")


@pytest.mark.trio
async def test_backend_closed_cmds(running_backend, alice):
    async with backend_cmds_factory(
        running_backend.addr, alice.device_id, alice.signing_key
    ) as cmds:
        pass
    with pytest.raises(trio.ClosedResourceError):
        await cmds.ping("Whatever")


@pytest.mark.trio
async def test_backend_bad_handshake(running_backend, mallory):
    with pytest.raises(BackendHandshakeError):
        async with backend_cmds_factory(
            running_backend.addr, mallory.device_id, mallory.signing_key
        ):
            pass


@pytest.mark.trio
async def test_revoked_device_handshake(running_backend, backend, alice, alice2):
    certified_revocation = certify_device_revocation(
        alice.device_id, alice.signing_key, alice2.device_id
    )
    await backend.user.revoke_device(alice2.device_id, certified_revocation, alice.device_id)

    with pytest.raises(BackendDeviceRevokedError):
        async with backend_cmds_factory(running_backend.addr, alice2.device_id, alice2.signing_key):
            pass


@pytest.mark.trio
async def test_backend_disconnect_during_handshake(tcp_stream_spy, alice, backend_addr):
    async def poorly_serve_client(stream):
        transport = await WebsocketTransport.init_for_client(stream, "foo", "bar")
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
                async with backend_cmds_factory(backend_addr, alice.device_id, alice.signing_key):
                    pass

        nursery.cancel_scope.cancel()
