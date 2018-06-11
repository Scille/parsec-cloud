import pytest

from tests.common import connect_backend


@pytest.mark.trio
async def test_connection(backend, alice):
    async with connect_backend(backend, auth_as=alice) as sock:
        await sock.send({"cmd": "ping", "ping": "42"})
        rep = await sock.recv()
        assert rep == {"status": "ok", "pong": "42"}


@pytest.mark.trio
async def test_bad_cmd(backend, alice):
    async with connect_backend(backend, auth_as=alice) as sock:
        await sock.send({"cmd": "dummy"})
        rep = await sock.recv()
        assert rep == {"status": "unknown_command", "reason": "Unknown command"}


@pytest.mark.trio
async def test_bad_msg_format(backend, alice):
    async with connect_backend(backend, auth_as=alice) as sock:
        await sock.sockstream.send_all(b"fooo\n")
        rep = await sock.recv()
        assert rep == {"status": "invalid_msg_format", "reason": "Invalid message format"}
