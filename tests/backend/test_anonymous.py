import pytest


@pytest.mark.trio
async def test_connect_as_anonymous(anonymous_backend_sock):
    await anonymous_backend_sock.send({"cmd": "ping", "ping": "foo"})
    rep = await anonymous_backend_sock.recv()
    assert rep == {"status": "ok", "pong": "foo"}


@pytest.mark.trio
async def test_anonymous_has_limited_access(anonymous_backend_sock):
    for cmd in [
        "user_get",
        "user_create",
        "blockstore_post",
        "blockstore_get",
        "vlob_create",
        "vlob_read",
        "vlob_update",
        "user_vlob_read",
        "user_vlob_update",
        "message_get",
        "message_new",
        "pubkey_get",
    ]:
        await anonymous_backend_sock.send({"cmd": cmd})
        rep = await anonymous_backend_sock.recv()
        assert rep == {"status": "unknown_command", "reason": "Unknown command"}
