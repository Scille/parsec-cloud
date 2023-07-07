# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS


import json
from pathlib import Path

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
    assert unpackb(rep) == {"status": "invalid_msg_format", "reason": "Invalid message format"}


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


@pytest.mark.trio
async def test_all_api_cmds_implemented(backend):
    from parsec import _parsec

    schema_dir = (Path(__file__) / "../../../../libparsec/crates/protocol/schema/").resolve()
    for family_dir in schema_dir.iterdir():
        family_mod_name = family_dir.name
        for cmd_file in family_dir.glob("*.json5"):
            cmd_specs = json.loads(
                "\n".join(
                    [
                        x
                        for x in cmd_file.read_text(encoding="utf8").splitlines()
                        if not x.strip().startswith("//")
                    ]
                )
            )
            for cmd_spec in cmd_specs:
                family_mod = getattr(_parsec, family_mod_name)
                for version in cmd_spec["major_versions"]:
                    version_mod = getattr(family_mod, f"v{version}")
                    cmd_mod = getattr(version_mod, cmd_spec["req"]["cmd"])
                    assert cmd_mod.Req in backend.apis
