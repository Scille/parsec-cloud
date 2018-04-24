import pytest

from parsec.core.backend_connection import (
    backend_connection_factory, BackendNotAvailable, HandshakeError
)

from tests.open_tcp_stream_mock_wrapper import offline


@pytest.mark.trio
async def test_base(running_backend, backend_addr, alice):
    conn = await backend_connection_factory(backend_addr, alice)
    await conn.send({"cmd": "ping", "ping": "hello"})
    rep = await conn.recv()
    assert rep == {"status": "ok", "pong": "hello"}


@pytest.mark.trio
async def test_backend_offline(backend_addr, alice):
    with pytest.raises(BackendNotAvailable):
        await backend_connection_factory(backend_addr, alice)


@pytest.mark.trio
async def test_backend_bad_handshake(running_backend, backend_addr, mallory):
    with pytest.raises(HandshakeError):
        await backend_connection_factory(backend_addr, mallory)


@pytest.mark.xfail
@pytest.mark.trio
async def test_backend_disconnect_during_handshake(running_backend, backend_addr, alice):
    # TODO
    raise NotImplementedError()
