# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from uuid import uuid4

from parsec.api.protocol import packb, unpackb, OrganizationID
from parsec.api.version import ApiVersion, API_VERSION
from parsec.api.transport import Transport
from parsec.api.protocol import (
    AuthenticatedClientHandshake,
    HandshakeInvitedOperation,
    InvitedClientHandshake,
    HandshakeRVKMismatch,
    HandshakeBadIdentity,
    HandshakeOrganizationExpired,
)
from parsec.backend.invite import UserInvitation, DeviceInvitation


@pytest.mark.trio
async def test_handshake_invalid_format(backend, server_factory):
    async with server_factory(backend.handle_client) as server:
        stream = server.connection_factory()
        transport = await Transport.init_for_client(stream, server.addr.hostname)

        await transport.recv()  # Get challenge
        req = {"handshake": "dummy", "client_api_version": API_VERSION}
        await transport.send(packb(req))
        result_req = await transport.recv()
        assert unpackb(result_req) == {
            "handshake": "result",
            "result": "bad_protocol",
            "help": "{'handshake': ['Invalid value, should be `answer`']}",
        }


@pytest.mark.trio
async def test_handshake_incompatible_version(backend, server_factory):
    async with server_factory(backend.handle_client) as server:
        stream = server.connection_factory()
        transport = await Transport.init_for_client(stream, server.addr.hostname)

        incompatible_version = ApiVersion(API_VERSION.version + 1, 0)
        await transport.recv()  # Get challenge
        req = {
            "handshake": "answer",
            "type": "anonymous",
            "client_api_version": incompatible_version,
            "organization_id": OrganizationID("Org"),
            "token": "whatever",
        }
        await transport.send(packb(req))
        result_req = await transport.recv()
        assert unpackb(result_req) == {
            "handshake": "result",
            "result": "bad_protocol",
            "help": "No overlap between client API versions {3.0} and backend API versions {2.0, 1.2}",
        }


@pytest.mark.trio
async def test_authenticated_handshake_good(backend, server_factory, alice):
    ch = AuthenticatedClientHandshake(
        organization_id=alice.organization_id,
        device_id=alice.device_id,
        user_signkey=alice.signing_key,
        root_verify_key=alice.root_verify_key,
    )

    async with server_factory(backend.handle_client) as server:
        stream = server.connection_factory()
        transport = await Transport.init_for_client(stream, server.addr.hostname)

        challenge_req = await transport.recv()
        answer_req = ch.process_challenge_req(challenge_req)

        await transport.send(answer_req)
        result_req = await transport.recv()
        ch.process_result_req(result_req)

        assert ch.client_api_version == API_VERSION
        assert ch.backend_api_version == API_VERSION


@pytest.mark.trio
async def test_authenticated_handshake_bad_rvk(backend, server_factory, alice, otherorg):
    ch = AuthenticatedClientHandshake(
        organization_id=alice.organization_id,
        device_id=alice.device_id,
        user_signkey=alice.signing_key,
        root_verify_key=otherorg.root_verify_key,
    )
    async with server_factory(backend.handle_client) as server:
        stream = server.connection_factory()
        transport = await Transport.init_for_client(stream, server.addr.hostname)

        challenge_req = await transport.recv()
        answer_req = ch.process_challenge_req(challenge_req)

        await transport.send(answer_req)
        result_req = await transport.recv()
        with pytest.raises(HandshakeRVKMismatch):
            ch.process_result_req(result_req)


@pytest.mark.trio
@pytest.mark.parametrize("operation", HandshakeInvitedOperation)
async def test_invited_handshake_good(backend, server_factory, alice, operation):
    if operation == HandshakeInvitedOperation.CLAIM_USER:
        invitation = UserInvitation(
            greeter_user_id=alice.user_id,
            greeter_human_handle=alice.human_handle,
            claimer_email="zack@example.com",
        )
    else:  # Claim device
        invitation = DeviceInvitation(
            greeter_user_id=alice.user_id, greeter_human_handle=alice.human_handle
        )
    await backend.invite.new(organization_id=alice.organization_id, invitation=invitation)

    ch = InvitedClientHandshake(
        organization_id=alice.organization_id, operation=operation, token=invitation.token
    )
    async with server_factory(backend.handle_client) as server:
        stream = server.connection_factory()
        transport = await Transport.init_for_client(stream, server.addr.hostname)

        challenge_req = await transport.recv()
        answer_req = ch.process_challenge_req(challenge_req)

        await transport.send(answer_req)
        result_req = await transport.recv()
        ch.process_result_req(result_req)

        assert ch.client_api_version == API_VERSION
        assert ch.backend_api_version == API_VERSION


@pytest.mark.trio
@pytest.mark.parametrize("operation", HandshakeInvitedOperation)
async def test_invited_handshake_bad_token(backend, server_factory, coolorg, operation):
    ch = InvitedClientHandshake(
        organization_id=coolorg.organization_id, operation=operation, token=uuid4()
    )
    async with server_factory(backend.handle_client) as server:
        stream = server.connection_factory()
        transport = await Transport.init_for_client(stream, server.addr.hostname)

        challenge_req = await transport.recv()
        answer_req = ch.process_challenge_req(challenge_req)

        await transport.send(answer_req)
        result_req = await transport.recv()
        with pytest.raises(HandshakeBadIdentity):
            ch.process_result_req(result_req)


@pytest.mark.trio
async def test_invited_handshake_bad_token_type(backend, server_factory, alice):
    invitation = DeviceInvitation(
        greeter_user_id=alice.user_id, greeter_human_handle=alice.human_handle
    )
    await backend.invite.new(organization_id=alice.organization_id, invitation=invitation)

    ch = InvitedClientHandshake(
        organization_id=alice.organization_id,
        operation=HandshakeInvitedOperation.CLAIM_USER,
        token=invitation.token,
    )
    async with server_factory(backend.handle_client) as server:
        stream = server.connection_factory()
        transport = await Transport.init_for_client(stream, server.addr.hostname)

        challenge_req = await transport.recv()
        answer_req = ch.process_challenge_req(challenge_req)

        await transport.send(answer_req)
        result_req = await transport.recv()
        with pytest.raises(HandshakeBadIdentity):
            ch.process_result_req(result_req)


@pytest.mark.trio
@pytest.mark.parametrize("type", ["invited", "authenticated"])
async def test_handshake_unknown_organization(
    backend, server_factory, organization_factory, alice, type
):
    bad_org = organization_factory()
    if type == "invited":
        ch = InvitedClientHandshake(
            organization_id=bad_org.organization_id,
            operation=HandshakeInvitedOperation.CLAIM_USER,
            token=uuid4(),
        )
    else:  # authenticated
        ch = AuthenticatedClientHandshake(
            organization_id=bad_org.organization_id,
            device_id=alice.device_id,
            user_signkey=alice.signing_key,
            root_verify_key=bad_org.root_verify_key,
        )

    async with server_factory(backend.handle_client) as server:
        stream = server.connection_factory()
        transport = await Transport.init_for_client(stream, server.addr.hostname)

        challenge_req = await transport.recv()
        answer_req = ch.process_challenge_req(challenge_req)

        await transport.send(answer_req)
        result_req = await transport.recv()
        with pytest.raises(HandshakeBadIdentity):
            ch.process_result_req(result_req)


@pytest.mark.trio
@pytest.mark.parametrize("type", ["invited", "authenticated"])
async def test_handshake_expired_organization(backend, server_factory, expiredorg, alice, type):
    if type == "invited":
        ch = InvitedClientHandshake(
            organization_id=expiredorg.organization_id,
            operation=HandshakeInvitedOperation.CLAIM_USER,
            token=uuid4(),
        )
    else:  # authenticated
        ch = AuthenticatedClientHandshake(
            organization_id=expiredorg.organization_id,
            device_id=alice.device_id,
            user_signkey=alice.signing_key,
            root_verify_key=expiredorg.root_verify_key,
        )

    async with server_factory(backend.handle_client) as server:
        stream = server.connection_factory()
        transport = await Transport.init_for_client(stream, server.addr.hostname)

        challenge_req = await transport.recv()
        answer_req = ch.process_challenge_req(challenge_req)

        await transport.send(answer_req)
        result_req = await transport.recv()
        with pytest.raises(HandshakeOrganizationExpired):
            ch.process_result_req(result_req)


@pytest.mark.trio
async def test_authenticated_handshake_unknown_device(backend, server_factory, mallory):
    ch = AuthenticatedClientHandshake(
        organization_id=mallory.organization_id,
        device_id=mallory.device_id,
        user_signkey=mallory.signing_key,
        root_verify_key=mallory.root_verify_key,
    )
    async with server_factory(backend.handle_client) as server:
        stream = server.connection_factory()
        transport = await Transport.init_for_client(stream, server.addr.hostname)

        challenge_req = await transport.recv()
        answer_req = ch.process_challenge_req(challenge_req)

        await transport.send(answer_req)
        result_req = await transport.recv()
        with pytest.raises(HandshakeBadIdentity):
            ch.process_result_req(result_req)
