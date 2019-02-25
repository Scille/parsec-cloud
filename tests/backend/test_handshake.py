# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest

from parsec.api.protocole import packb, unpackb
from parsec.api.transport import Transport
from parsec.api.protocole.handshake import (
    ClientHandshake,
    AnonymousClientHandshake,
    HandshakeRVKMismatch,
)


# @pytest.mark.trio
# @pytest.mark.parametrize("bad_part", ["user_id", "device_name"])
# async def test_handshake_unknown_device(bad_part, backend, server_factory, alice):
#     async with server_factory(backend.handle_client) as server:
#         stream = server.connection_factory()
#         transport = await Transport.init_for_client(stream, server.addr)
#         if bad_part == "user_id":
#             identity = "zack@dummy"
#         else:
#             identity = f"{alice.user_id}@dummy"

#         await transport.recv()  # Get challenge
#         await transport.send(
#             packb({"handshake": "answer",
#                 "identity": identity,
#                 "organization":
#                 "answer": b"fooo"})
#         )
#         result_req = await transport.recv()
#         assert unpackb(result_req) == {"handshake": "result", "result": "bad_identity"}


@pytest.mark.trio
async def test_handshake_invalid_format(backend, server_factory):
    async with server_factory(backend.handle_client) as server:
        stream = server.connection_factory()
        transport = await Transport.init_for_client(stream, server.addr)

        await transport.recv()  # Get challenge
        req = {"handshake": "answer", "organization_id": "zob", "dummy": "field"}
        await transport.send(packb(req))
        result_req = await transport.recv()
        assert unpackb(result_req) == {
            "handshake": "result",
            "result": "bad_format",
            "help": "{'_schema': ['Unknown field name dummy']}",
        }


@pytest.mark.trio
async def test_handshake_good(backend, server_factory, alice):
    ch = ClientHandshake(
        alice.organization_id, alice.device_id, alice.signing_key, alice.root_verify_key
    )
    async with server_factory(backend.handle_client) as server:
        stream = server.connection_factory()
        transport = await Transport.init_for_client(stream, server.addr)

        challenge_req = await transport.recv()
        answer_req = ch.process_challenge_req(challenge_req)

        await transport.send(answer_req)
        result_req = await transport.recv()
        ch.process_result_req(result_req)


@pytest.mark.trio
@pytest.mark.parametrize("is_anonymous", [True, False])
async def test_handshake_mismatch_rvc(
    backend, server_factory, coolorg, alice, organization_factory, is_anonymous
):
    badorg = organization_factory()
    if is_anonymous:
        ch = AnonymousClientHandshake(coolorg.organization_id, badorg.root_verify_key)
    else:
        ch = ClientHandshake(
            alice.organization_id, alice.device_id, alice.signing_key, badorg.root_verify_key
        )
    async with server_factory(backend.handle_client) as server:
        stream = server.connection_factory()
        transport = await Transport.init_for_client(stream, server.addr)

        challenge_req = await transport.recv()
        answer_req = ch.process_challenge_req(challenge_req)

        await transport.send(answer_req)
        result_req = await transport.recv()
        with pytest.raises(HandshakeRVKMismatch):
            ch.process_result_req(result_req)
