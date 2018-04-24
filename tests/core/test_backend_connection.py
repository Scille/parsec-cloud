import pytest

from parsec.core.backend_connection import (
    backend_connection_factory, backend_send_anonymous_cmd, BackendNotAvailable, HandshakeError
)


@pytest.mark.trio
async def test_base(running_backend, alice):
    conn = await backend_connection_factory(running_backend.addr, alice)
    await conn.send({"cmd": "ping", "ping": "hello"})
    rep = await conn.recv()
    assert rep == {"status": "ok", "pong": "hello"}


@pytest.mark.trio
async def test_backend_offline(backend_addr, alice):
    with pytest.raises(BackendNotAvailable):
        await backend_connection_factory(backend_addr, alice)


@pytest.mark.trio
async def test_backend_bad_handshake(running_backend, mallory):
    with pytest.raises(HandshakeError):
        await backend_connection_factory(running_backend.addr, mallory)


@pytest.mark.xfail
@pytest.mark.trio
async def test_backend_disconnect_during_handshake(running_backend, alice):
    # TODO
    raise NotImplementedError()


@pytest.mark.trio
async def test_backend_send_anonymous_cmd(running_backend):
    rep = await backend_send_anonymous_cmd(running_backend.addr, {"cmd": "ping", "ping": "foo"})
    assert rep == {"status": "ok", "pong": "foo"}
