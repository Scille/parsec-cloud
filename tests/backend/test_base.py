# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import pytest

from parsec.api.protocol import packb, unpackb


@pytest.mark.trio
async def test_connection(alice_ws):
    await alice_ws.send(packb({"cmd": "ping", "ping": "42"}))
    rep = await alice_ws.receive()
    assert unpackb(rep) == {"status": "ok", "pong": "42"}


@pytest.mark.trio
async def test_bad_cmd(alice_ws):
    await alice_ws.send(packb({"cmd": "dummy"}))
    rep = await alice_ws.receive()
    assert unpackb(rep) == {"status": "unknown_command", "reason": "Unknown command"}


@pytest.mark.trio
@pytest.mark.parametrize(
    "kind", ["string_message", "valid_msgpack_but_not_a_dict", "invalid_msgpack"]
)
async def test_bad_msg_format(alice_ws, kind):
    if kind == "string_message":
        await alice_ws.send("hello")  # Only websocket bytes message are accepted
    elif kind == "valid_msgpack_but_not_a_dict":
        await alice_ws.send(b"\x00")  # Encodes the number 0 as positive fixint
    else:
        assert kind == "invalid_msgpack"
        await alice_ws.send(b"\xc1")  # Never used value according to msgpack spec
    rep = await alice_ws.receive()
    assert unpackb(rep) == {"status": "invalid_msg_format", "reason": "Invalid message format"}
