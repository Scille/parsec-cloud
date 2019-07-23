# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest

from parsec.api.protocol import packb, unpackb


@pytest.mark.trio
async def test_connection(alice_backend_sock):
    await alice_backend_sock.send(packb({"cmd": "ping", "ping": "42"}))
    rep = await alice_backend_sock.recv()
    assert unpackb(rep) == {"status": "ok", "pong": "42"}


@pytest.mark.trio
async def test_bad_cmd(alice_backend_sock):
    await alice_backend_sock.send(packb({"cmd": "dummy"}))
    rep = await alice_backend_sock.recv()
    assert unpackb(rep) == {"status": "unknown_command", "reason": "Unknown command"}


# @pytest.mark.trio
# async def test_bad_msg_format(alice_backend_sock):
#     await alice_backend_sock.stream.send_all(b"\x00\x00\x00\x04fooo")
#     rep = await alice_backend_sock.recv()
#     assert unpackb(rep) == {"status": "invalid_msg_format", "reason": "Invalid message format"}
