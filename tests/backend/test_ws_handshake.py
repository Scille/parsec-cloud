# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import logging
from unittest.mock import ANY

import pytest

from parsec.api.protocol import (
    AuthenticatedClientHandshake,
    HandshakeBadIdentity,
    HandshakeOrganizationExpired,
    HandshakeRVKMismatch,
    InvitationToken,
    InvitationType,
    InvitedClientHandshake,
    packb,
    unpackb,
)
from parsec.api.protocol.handshake import ServerHandshake
from parsec.api.version import API_VERSION, ApiVersion
from parsec.backend.backend_events import BackendEvent


@pytest.mark.trio
@pytest.mark.parametrize(
    "kind",
    ["bad_handshake_type", "irrelevant_dict", "valid_msgpack_but_not_a_dict", "invalid_msgpack"],
)
async def test_handshake_send_invalid_answer_data(backend_asgi_app, kind):
    if kind == "bad_handshake_type":
        bad_req = packb({"handshake": "dummy", "client_api_version": API_VERSION})
    elif kind == "irrelevant_dict":
        bad_req = packb({"foo": "bar"})
    elif kind == "valid_msgpack_but_not_a_dict":
        bad_req = b"\x00"  # Encodes the number 0 as positive fixint
    else:
        assert kind == "invalid_msgpack"
        bad_req = b"\xc1"  # Never used value according to msgpack spec

    client = backend_asgi_app.test_client()
    async with client.websocket("/ws") as ws:
        await ws.receive()  # Get challenge
        await ws.send(bad_req)
        result_req = await ws.receive()
        assert unpackb(result_req) == {"handshake": "result", "result": "bad_protocol", "help": ANY}


@pytest.mark.trio
async def test_handshake_incompatible_version(backend_asgi_app):
    client = backend_asgi_app.test_client()
    async with client.websocket("/ws") as ws:
        incompatible_version = ApiVersion(API_VERSION.version + 1, 0)
        await ws.receive()  # Get challenge
        req = {
            "handshake": "answer",
            "type": "anonymous",
            "client_api_version": incompatible_version,
            "organization_id": "Org",
            "token": "whatever",
        }
        await ws.send(packb(req))
        result_req = await ws.receive()
        assert unpackb(result_req) == {
            "handshake": "result",
            "result": "bad_protocol",
            "help": f"No overlap between client API versions {{{incompatible_version}}} and backend API versions {{{', '.join(map(str, ServerHandshake.SUPPORTED_API_VERSIONS))}}}",
        }


@pytest.mark.trio
async def test_authenticated_handshake_good(backend_asgi_app, alice):
    ch = AuthenticatedClientHandshake(
        organization_id=alice.organization_id,
        device_id=alice.device_id,
        user_signkey=alice.signing_key,
        root_verify_key=alice.root_verify_key,
    )

    client = backend_asgi_app.test_client()
    async with client.websocket("/ws") as ws:
        challenge_req = await ws.receive()
        answer_req = ch.process_challenge_req(challenge_req)

        await ws.send(answer_req)
        result_req = await ws.receive()
        ch.process_result_req(result_req)

        assert ch.client_api_version == API_VERSION
        assert ch.backend_api_version == API_VERSION


@pytest.mark.trio
async def test_authenticated_handshake_bad_rvk(backend_asgi_app, alice, otherorg):
    ch = AuthenticatedClientHandshake(
        organization_id=alice.organization_id,
        device_id=alice.device_id,
        user_signkey=alice.signing_key,
        root_verify_key=otherorg.root_verify_key,
    )
    client = backend_asgi_app.test_client()
    async with client.websocket("/ws") as ws:
        challenge_req = await ws.receive()
        answer_req = ch.process_challenge_req(challenge_req)

        await ws.send(answer_req)
        result_req = await ws.receive()
        with pytest.raises(HandshakeRVKMismatch):
            ch.process_result_req(result_req)


@pytest.mark.trio
@pytest.mark.parametrize("invitation_type", (InvitationType.USER, InvitationType.DEVICE))
async def test_invited_handshake_good(backend_asgi_app, backend, alice, invitation_type):
    if invitation_type == InvitationType.USER:
        invitation = await backend.invite.new_for_user(
            organization_id=alice.organization_id,
            greeter_user_id=alice.user_id,
            claimer_email="zack@example.com",
        )
    else:  # Claim device
        invitation = await backend.invite.new_for_device(
            organization_id=alice.organization_id, greeter_user_id=alice.user_id
        )

    ch = InvitedClientHandshake(
        organization_id=alice.organization_id,
        invitation_type=invitation_type,
        token=invitation.token,
    )
    client = backend_asgi_app.test_client()
    async with client.websocket("/ws") as ws:
        challenge_req = await ws.receive()
        answer_req = ch.process_challenge_req(challenge_req)

        await ws.send(answer_req)
        result_req = await ws.receive()
        ch.process_result_req(result_req)

        assert ch.client_api_version == API_VERSION
        assert ch.backend_api_version == API_VERSION


@pytest.mark.trio
async def test_api_version_in_logs_on_handshake(backend_asgi_app, backend, alice, caplog):
    invitation = await backend.invite.new_for_user(
        organization_id=alice.organization_id,
        greeter_user_id=alice.user_id,
        claimer_email="zack@example.com",
    )

    ch = InvitedClientHandshake(
        organization_id=alice.organization_id,
        invitation_type=InvitationType.USER,
        token=invitation.token,
    )
    client_api_version = ApiVersion(3, 99)
    ch.SUPPORTED_API_VERSIONS = [client_api_version]
    client = backend_asgi_app.test_client()
    with caplog.at_level(logging.INFO):
        async with client.websocket("/ws") as ws:
            challenge_req = await ws.receive()
            answer_req = ch.process_challenge_req(challenge_req)

            await ws.send(answer_req)
            result_req = await ws.receive()
            ch.process_result_req(result_req)

            # Sanity checks
            assert ch.client_api_version == client_api_version
            assert ch.backend_api_version == API_VERSION

        assert f"(client/server API version: {client_api_version}/{API_VERSION})" in caplog.text


@pytest.mark.trio
@pytest.mark.parametrize("invitation_type", (InvitationType.USER, InvitationType.DEVICE))
async def test_invited_handshake_bad_token(backend_asgi_app, coolorg, invitation_type):
    ch = InvitedClientHandshake(
        organization_id=coolorg.organization_id,
        invitation_type=invitation_type,
        token=InvitationToken.new(),
    )
    client = backend_asgi_app.test_client()
    async with client.websocket("/ws") as ws:
        challenge_req = await ws.receive()
        answer_req = ch.process_challenge_req(challenge_req)

        await ws.send(answer_req)
        result_req = await ws.receive()
        with pytest.raises(HandshakeBadIdentity):
            ch.process_result_req(result_req)


@pytest.mark.trio
async def test_invited_handshake_bad_token_type(backend_asgi_app, backend, alice):
    invitation = await backend.invite.new_for_device(
        organization_id=alice.organization_id, greeter_user_id=alice.user_id
    )

    ch = InvitedClientHandshake(
        organization_id=alice.organization_id,
        invitation_type=InvitationType.USER,
        token=invitation.token,
    )
    client = backend_asgi_app.test_client()
    async with client.websocket("/ws") as ws:
        challenge_req = await ws.receive()
        answer_req = ch.process_challenge_req(challenge_req)

        await ws.send(answer_req)
        result_req = await ws.receive()
        with pytest.raises(HandshakeBadIdentity):
            ch.process_result_req(result_req)


@pytest.mark.trio
@pytest.mark.parametrize("type", ["invited", "authenticated"])
async def test_handshake_unknown_organization(backend_asgi_app, organization_factory, alice, type):
    bad_org = organization_factory()
    if type == "invited":
        ch = InvitedClientHandshake(
            organization_id=bad_org.organization_id,
            invitation_type=InvitationType.USER,
            token=InvitationToken.new(),
        )
    else:  # authenticated
        ch = AuthenticatedClientHandshake(
            organization_id=bad_org.organization_id,
            device_id=alice.device_id,
            user_signkey=alice.signing_key,
            root_verify_key=bad_org.root_verify_key,
        )

    client = backend_asgi_app.test_client()
    async with client.websocket("/ws") as ws:
        challenge_req = await ws.receive()
        answer_req = ch.process_challenge_req(challenge_req)

        await ws.send(answer_req)
        result_req = await ws.receive()
        with pytest.raises(HandshakeBadIdentity):
            ch.process_result_req(result_req)


@pytest.mark.trio
@pytest.mark.parametrize("type", ["invited", "authenticated"])
async def test_handshake_expired_organization(backend_asgi_app, backend, expiredorg, alice, type):
    if type == "invited":
        ch = InvitedClientHandshake(
            organization_id=expiredorg.organization_id,
            invitation_type=InvitationType.USER,
            token=InvitationToken.new(),
        )
    else:  # authenticated
        ch = AuthenticatedClientHandshake(
            organization_id=expiredorg.organization_id,
            device_id=alice.device_id,
            user_signkey=alice.signing_key,
            root_verify_key=expiredorg.root_verify_key,
        )

    with backend.event_bus.listen() as spy:
        client = backend_asgi_app.test_client()
        async with client.websocket("/ws") as ws:
            challenge_req = await ws.receive()
            answer_req = ch.process_challenge_req(challenge_req)

            await ws.send(answer_req)
            result_req = await ws.receive()
            with pytest.raises(HandshakeOrganizationExpired):
                ch.process_result_req(result_req)
            await spy.wait_with_timeout(BackendEvent.ORGANIZATION_EXPIRED)


@pytest.mark.trio
async def test_authenticated_handshake_unknown_device(backend_asgi_app, mallory):
    ch = AuthenticatedClientHandshake(
        organization_id=mallory.organization_id,
        device_id=mallory.device_id,
        user_signkey=mallory.signing_key,
        root_verify_key=mallory.root_verify_key,
    )
    client = backend_asgi_app.test_client()
    async with client.websocket("/ws") as ws:
        challenge_req = await ws.receive()
        answer_req = ch.process_challenge_req(challenge_req)

        await ws.send(answer_req)
        result_req = await ws.receive()
        with pytest.raises(HandshakeBadIdentity):
            ch.process_result_req(result_req)


@pytest.mark.trio
async def test_handshake_string_websocket_message(backend_asgi_app, mallory):
    client = backend_asgi_app.test_client()
    async with client.websocket("/ws") as ws:
        await ws.receive()  # Get the challenge
        await ws.send("hello")

        result_req = await ws.receive()
        assert unpackb(result_req) == {
            "result": "bad_protocol",
            "handshake": "result",
            "help": "Expected bytes message in websocket",
        }
