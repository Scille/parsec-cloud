import pytest
import trio

from parsec.core.backend_connection import (
    backend_connection_factory,
    backend_send_anonymous_cmd,
    BackendNotAvailable,
    HandshakeError,
)
from parsec.handshake import ServerHandshake
from parsec.networking import CookedSocket


@pytest.mark.trio
async def test_base(running_backend, alice):
    conn = await backend_connection_factory(running_backend.addr, alice.id, alice.device_signkey)
    await conn.send({"cmd": "ping", "ping": "hello"})
    rep = await conn.recv()
    assert rep == {"status": "ok", "pong": "hello"}


@pytest.mark.trio
async def test_backend_offline(backend_addr, alice):
    with pytest.raises(BackendNotAvailable):
        await backend_connection_factory(backend_addr, alice.id, alice.device_signkey)


@pytest.mark.trio
async def test_backend_bad_handshake(running_backend, mallory):
    with pytest.raises(HandshakeError):
        await backend_connection_factory(running_backend.addr, mallory.id, mallory.device_signkey)


@pytest.mark.trio
async def test_backend_disconnect_during_handshake(nursery, tcp_stream_spy, alice, backend_addr):
    async def poorly_serve_client(raw_sock):
        cooked_sock = CookedSocket(raw_sock)
        handshake = ServerHandshake()
        await cooked_sock.send(handshake.build_challenge_req())
        await cooked_sock.recv()
        # Close connection during handshake
        await cooked_sock.aclose()

    async def connection_factory(*args, **kwargs):
        client_sock, server_sock = trio.testing.memory_stream_pair()
        nursery.start_soon(poorly_serve_client, server_sock)
        return client_sock

    with tcp_stream_spy.install_hook(backend_addr, connection_factory):
        with pytest.raises(BackendNotAvailable):
            await backend_connection_factory(backend_addr, alice.id, alice.device_signkey)


@pytest.mark.trio
async def test_backend_send_anonymous_cmd(running_backend):
    rep = await backend_send_anonymous_cmd(running_backend.addr, {"cmd": "ping", "ping": "foo"})
    assert rep == {"status": "ok", "pong": "foo"}
