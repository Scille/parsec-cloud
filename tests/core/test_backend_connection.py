import pytest
import trio

from parsec.core.backend_connection import (
    backend_send_anonymous_cmd,
    BackendNotAvailable,
    BackendConnectionPool,
    HandshakeBadIdentity,
    HandshakeError,
)
from parsec.backend.exceptions import AlreadyRevokedError, NotFoundError
from parsec.handshake import ServerHandshake, HandshakeRevokedDevice
from parsec.networking import CookedSocket

from tests.open_tcp_stream_mock_wrapper import offline


@pytest.mark.trio
async def test_base(connection_pool):
    async with connection_pool.connection() as conn:
        await conn.send({"cmd": "ping", "ping": "hello"})
        rep = await conn.recv()
        assert rep == {"status": "ok", "pong": "hello"}


@pytest.mark.trio
async def test_backend_offline(tcp_stream_spy, backend_addr, connection_pool):
    # Using tcp_stream_spy make us avoid long wait for time
    with offline(backend_addr):
        with pytest.raises(BackendNotAvailable):
            async with connection_pool.connection():
                pass


@pytest.mark.trio
async def test_backend_bad_handshake(running_backend, mallory):
    connection_pool = BackendConnectionPool(
        running_backend.addr, mallory.id, mallory.device_signkey
    )
    with pytest.raises(BackendNotAvailable):
        async with connection_pool.connection():
            pass


@pytest.mark.trio
async def test_backend_disconnect_during_handshake(tcp_stream_spy, alice, backend_addr):
    async def poorly_serve_client(raw_sock):
        cooked_sock = CookedSocket(raw_sock)
        handshake = ServerHandshake()
        await cooked_sock.send(handshake.build_challenge_req())
        await cooked_sock.recv()
        # Close connection during handshake
        await cooked_sock.aclose()

    async with trio.open_nursery() as nursery:

        async def connection_factory(*args, **kwargs):
            client_sock, server_sock = trio.testing.memory_stream_pair()
            nursery.start_soon(poorly_serve_client, server_sock)
            return client_sock

        with tcp_stream_spy.install_hook(backend_addr, connection_factory):
            with pytest.raises(BackendNotAvailable):
                connection_pool = BackendConnectionPool(
                    backend_addr, alice.id, alice.device_signkey
                )
                with pytest.raises(HandshakeError):
                    async with connection_pool.connection():
                        pass

        nursery.cancel_scope.cancel()


@pytest.mark.trio
async def test_revoked_device_handshake(alice, backend, backend_sock_factory):
    async with backend_sock_factory(backend, alice):
        pass

    await backend.user.revoke_device(alice.user_id, alice.device_name)

    with pytest.raises(AlreadyRevokedError):
        await backend.user.revoke_device(alice.user_id, alice.device_name)

    with pytest.raises(NotFoundError):
        await backend.user.revoke_device(alice.user_id, "bar")

    with pytest.raises(HandshakeRevokedDevice):
        async with backend_sock_factory(backend, alice):
            pass


@pytest.mark.trio
async def test_backend_send_anonymous_cmd(running_backend):
    rep = await backend_send_anonymous_cmd(running_backend.addr, {"cmd": "ping", "ping": "foo"})
    assert rep == {"status": "ok", "pong": "foo"}


@pytest.mark.trio
async def test_connection_pool_max_connection(running_backend, alice):
    max_connections = 2
    connection_pool = BackendConnectionPool(
        running_backend.addr, alice.id, alice.device_signkey, max_connections
    )
    assert connection_pool.size() == 0

    async with connection_pool.connection():
        assert connection_pool.size() == 1
        async with connection_pool.connection():
            assert connection_pool.size() == 2
            with trio.move_on_after(0.2) as cancel_scope:
                async with connection_pool.connection():
                    pass
            assert cancel_scope.cancelled_caught


@pytest.mark.trio
async def test_connection_pool_reuse_released_connection(running_backend, alice):
    connection_pool = BackendConnectionPool(running_backend.addr, alice.id, alice.device_signkey)
    assert connection_pool.size() == 0
    async with connection_pool.connection():
        assert connection_pool.size() == 1
        async with connection_pool.connection():
            assert connection_pool.size() == 2
    assert connection_pool.size() == 2
    async with connection_pool.connection():
        assert connection_pool.size() == 2


@pytest.mark.trio
async def test_connection_pool_disconnect(running_backend, alice):
    connection_pool = BackendConnectionPool(running_backend.addr, alice.id, alice.device_signkey)

    async with connection_pool.connection():
        assert connection_pool.size() == 1
        await connection_pool.disconnect()
        assert connection_pool.size() == 0
    assert connection_pool.size() == 0
