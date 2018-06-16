import pytest


@pytest.mark.trio
async def test_connection(alice_backend_sock):
    await alice_backend_sock.send({"cmd": "ping", "ping": "42"})
    rep = await alice_backend_sock.recv()
    assert rep == {"status": "ok", "pong": "42"}


@pytest.mark.trio
async def test_bad_cmd(alice_backend_sock):
    await alice_backend_sock.send({"cmd": "dummy"})
    rep = await alice_backend_sock.recv()
    assert rep == {"status": "unknown_command", "reason": "Unknown command"}


@pytest.mark.trio
async def test_bad_msg_format(alice_backend_sock):
    await alice_backend_sock.sockstream.send_all(b"fooo\n")
    rep = await alice_backend_sock.recv()
    assert rep == {"status": "invalid_msg_format", "reason": "Invalid message format"}
