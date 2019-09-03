# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest

from parsec.api.protocol import packb, unpackb
from parsec.api.transport import Transport
from parsec.api.protocol.handshake import (
    AuthenticatedClientHandshake,
    AnonymousClientHandshake,
    AdministrationClientHandshake,
    HandshakeRVKMismatch,
    HandshakeBadIdentity,
    HandshakeBadAdministrationToken,
)

from parsec import __api_version__


@pytest.mark.trio
async def test_anonymous_handshake_invalid_format(backend, server_factory):
    async with server_factory(backend.handle_client) as server:
        stream = server.connection_factory()
        transport = await Transport.init_for_client(stream, server.addr)

        await transport.recv()  # Get challenge
        req = {
            "handshake": "answer",
            "type": "anonymous",
            "api_version": __api_version__,
            "organization_id": "zob",
            "dummy": "field",
        }
        await transport.send(packb(req))
        result_req = await transport.recv()
        assert unpackb(result_req) == {
            "handshake": "result",
            "result": "bad_format",
            "help": "{'_schema': ['Unknown field name dummy']}",
        }


@pytest.mark.trio
async def test_authenticated_handshake_good(backend, server_factory, alice):
    ch = AuthenticatedClientHandshake(
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
async def test_administration_handshake_good(backend, server_factory):
    ch = AdministrationClientHandshake(backend.config.administration_token)
    async with server_factory(backend.handle_client) as server:
        stream = server.connection_factory()
        transport = await Transport.init_for_client(stream, server.addr)

        challenge_req = await transport.recv()
        answer_req = ch.process_challenge_req(challenge_req)

        await transport.send(answer_req)
        result_req = await transport.recv()
        ch.process_result_req(result_req)


@pytest.mark.trio
async def test_admin_handshake_bad_token(backend, server_factory):
    ch = AdministrationClientHandshake("dummy token")
    async with server_factory(backend.handle_client) as server:
        stream = server.connection_factory()
        transport = await Transport.init_for_client(stream, server.addr)

        challenge_req = await transport.recv()
        answer_req = ch.process_challenge_req(challenge_req)

        await transport.send(answer_req)
        result_req = await transport.recv()
        with pytest.raises(HandshakeBadAdministrationToken):
            ch.process_result_req(result_req)


@pytest.mark.trio
@pytest.mark.parametrize("is_anonymous", [True, False])
async def test_handshake_bad_rvk(backend, server_factory, coolorg, alice, otherorg, is_anonymous):
    if is_anonymous:
        ch = AnonymousClientHandshake(coolorg.organization_id, otherorg.root_verify_key)
    else:
        ch = AuthenticatedClientHandshake(
            alice.organization_id, alice.device_id, alice.signing_key, otherorg.root_verify_key
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


@pytest.mark.trio
@pytest.mark.parametrize("check_rvk", [True, False])
async def test_anonymous_handshake_good(backend, server_factory, coolorg, check_rvk):
    to_check_rvk = coolorg.root_verify_key if check_rvk else None
    ch = AnonymousClientHandshake(coolorg.organization_id, to_check_rvk)
    async with server_factory(backend.handle_client) as server:
        stream = server.connection_factory()
        transport = await Transport.init_for_client(stream, server.addr)

        challenge_req = await transport.recv()
        answer_req = ch.process_challenge_req(challenge_req)

        await transport.send(answer_req)
        result_req = await transport.recv()
        ch.process_result_req(result_req)


@pytest.mark.trio
async def test_anonymous_handshake_bad_rvk(backend, server_factory, coolorg, otherorg):
    ch = AnonymousClientHandshake(coolorg.organization_id, otherorg.root_verify_key)
    async with server_factory(backend.handle_client) as server:
        stream = server.connection_factory()
        transport = await Transport.init_for_client(stream, server.addr)

        challenge_req = await transport.recv()
        answer_req = ch.process_challenge_req(challenge_req)

        await transport.send(answer_req)
        result_req = await transport.recv()
        with pytest.raises(HandshakeRVKMismatch):
            ch.process_result_req(result_req)


@pytest.mark.trio
@pytest.mark.parametrize("type", ["anonymous", "authenticated"])
async def test_anonymous_handshake_unknown_organization(
    backend, server_factory, organization_factory, alice, type
):
    bad_org = organization_factory()
    if type == "anonymous":
        ch = AnonymousClientHandshake(bad_org.organization_id, bad_org.root_verify_key)
    else:  # authenticated
        ch = AuthenticatedClientHandshake(
            bad_org.organization_id, alice.device_id, alice.signing_key, bad_org.root_verify_key
        )

    async with server_factory(backend.handle_client) as server:
        stream = server.connection_factory()
        transport = await Transport.init_for_client(stream, server.addr)

        challenge_req = await transport.recv()
        answer_req = ch.process_challenge_req(challenge_req)

        await transport.send(answer_req)
        result_req = await transport.recv()
        with pytest.raises(HandshakeBadIdentity):
            ch.process_result_req(result_req)


@pytest.mark.trio
async def test_authenticated_handshake_unknown_device(backend, server_factory, mallory):
    ch = AuthenticatedClientHandshake(
        mallory.organization_id, mallory.device_id, mallory.signing_key, mallory.root_verify_key
    )
    async with server_factory(backend.handle_client) as server:
        stream = server.connection_factory()
        transport = await Transport.init_for_client(stream, server.addr)

        challenge_req = await transport.recv()
        answer_req = ch.process_challenge_req(challenge_req)

        await transport.send(answer_req)
        result_req = await transport.recv()
        with pytest.raises(HandshakeBadIdentity):
            ch.process_result_req(result_req)
