# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import pytest

from parsec.api.version import ApiVersion
from parsec.api.transport import Transport
from parsec.api.protocol import (
    packb,
    unpackb,
    ServerHandshake,
    APIV1_AnonymousClientHandshake,
    HandshakeRVKMismatch,
    HandshakeBadIdentity,
)

from tests.common import customize_fixtures


@pytest.fixture
def mock_api_versions(monkeypatch):
    default_client_version = ApiVersion(1, 11)
    default_backend_version = ApiVersion(1, 22)

    def _mock_api_versions(client_versions=None, backend_versions=None):
        if client_versions is not None:
            monkeypatch.setattr(
                APIV1_AnonymousClientHandshake, "SUPPORTED_API_VERSIONS", client_versions
            )
        if backend_versions is not None:
            monkeypatch.setattr(ServerHandshake, "SUPPORTED_API_VERSIONS", backend_versions)

    _mock_api_versions.default_client_version = default_client_version
    _mock_api_versions.default_backend_version = default_backend_version
    _mock_api_versions(
        client_versions=[default_client_version], backend_versions=[default_backend_version]
    )
    return _mock_api_versions


@pytest.mark.trio
@customize_fixtures(backend_not_populated=True)
async def test_anonymous_handshake_invalid_format(backend, server_factory):
    async with server_factory(backend.handle_client) as server:
        stream = await server.connection_factory()
        transport = await Transport.init_for_client(stream, "127.0.0.1")

        await transport.recv()  # Get challenge
        req = {
            "handshake": "foo",
            "type": "anonymous",
            "client_api_version": ApiVersion(1, 1),
            "organization_id": "zob",
        }
        await transport.send(packb(req))
        result_req = await transport.recv()
        assert unpackb(result_req) == {
            "handshake": "result",
            "result": "bad_protocol",
            "help": "{'handshake': ['Invalid value, should be `answer`']}",
        }


@pytest.mark.trio
async def test_authenticated_handshake_no_longer_supported(backend, server_factory, alice):
    async with server_factory(backend.handle_client) as server:
        stream = await server.connection_factory()
        transport = await Transport.init_for_client(stream, "127.0.0.1")

        challenge_req = await transport.recv()
        challenge = unpackb(challenge_req)["challenge"]
        answer = alice.signing_key.sign(challenge)
        answer_req = {
            "handshake": "answer",
            "client_api_version": ApiVersion(1, 3),
            "type": "AUTHENTICATED",
            "organization_id": str(alice.organization_id),
            "device_id": str(alice.device_id),
            "rvk": alice.root_verify_key.encode(),
            "answer": answer,
        }

        await transport.send(packb(answer_req))
        result_req = await transport.recv()
        assert unpackb(result_req) == {
            "handshake": "result",
            "result": "bad_protocol",
            "help": "{'type': ['Unsupported value: AUTHENTICATED']}",
        }


@pytest.mark.trio
@customize_fixtures(backend_not_populated=True)
async def test_administration_handshake_no_longer_supported(backend, server_factory):
    async with server_factory(backend.handle_client) as server:
        stream = await server.connection_factory()
        transport = await Transport.init_for_client(stream, "127.0.0.1")

        await transport.recv()
        answer_req = {
            "handshake": "answer",
            "client_api_version": ApiVersion(1, 3),
            "type": "ADMINISTRATION",
            "token": backend.config.administration_token,
        }

        await transport.send(packb(answer_req))
        result_req = await transport.recv()
        assert unpackb(result_req) == {
            "handshake": "result",
            "result": "bad_protocol",
            "help": "{'type': ['Unsupported value: ADMINISTRATION']}",
        }


@pytest.mark.trio
@pytest.mark.parametrize("check_rvk", [True, False])
async def test_anonymous_handshake_good(
    backend, server_factory, coolorg, check_rvk, mock_api_versions
):
    to_check_rvk = coolorg.root_verify_key if check_rvk else None
    ch = APIV1_AnonymousClientHandshake(coolorg.organization_id, to_check_rvk)
    async with server_factory(backend.handle_client) as server:
        stream = await server.connection_factory()
        transport = await Transport.init_for_client(stream, "127.0.0.1")

        challenge_req = await transport.recv()
        answer_req = ch.process_challenge_req(challenge_req)

        await transport.send(answer_req)
        result_req = await transport.recv()
        ch.process_result_req(result_req)

        assert ch.client_api_version == mock_api_versions.default_client_version
        assert ch.backend_api_version == mock_api_versions.default_backend_version


@pytest.mark.trio
async def test_anonymous_handshake_bad_rvk(backend, server_factory, coolorg, otherorg):
    ch = APIV1_AnonymousClientHandshake(coolorg.organization_id, otherorg.root_verify_key)
    async with server_factory(backend.handle_client) as server:
        stream = await server.connection_factory()
        transport = await Transport.init_for_client(stream, "127.0.0.1")

        challenge_req = await transport.recv()
        answer_req = ch.process_challenge_req(challenge_req)

        await transport.send(answer_req)
        result_req = await transport.recv()
        with pytest.raises(HandshakeRVKMismatch):
            ch.process_result_req(result_req)


@pytest.mark.trio
async def test_handshake_unknown_organization(backend, server_factory, organization_factory, alice):
    bad_org = organization_factory()
    ch = APIV1_AnonymousClientHandshake(bad_org.organization_id, bad_org.root_verify_key)
    async with server_factory(backend.handle_client) as server:
        stream = await server.connection_factory()
        transport = await Transport.init_for_client(stream, "127.0.0.1")

        challenge_req = await transport.recv()
        answer_req = ch.process_challenge_req(challenge_req)

        await transport.send(answer_req)
        result_req = await transport.recv()
        with pytest.raises(HandshakeBadIdentity):
            ch.process_result_req(result_req)
