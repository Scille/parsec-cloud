# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS
# TODO
# ruff: noqa: F821
from __future__ import annotations

import pytest

from parsec._parsec import ApiVersion, DateTime
from parsec.api.protocol import (
    AuthenticatedClientHandshake,
    HandshakeType,
    RealmCreateRepBadTimestamp,
    ServerHandshake,
    packb,
)
from parsec.serde import BaseSchema, fields
from parsec.utils import BALLPARK_CLIENT_EARLY_OFFSET, BALLPARK_CLIENT_LATE_OFFSET


@pytest.mark.xfail(reason="TODO: fix this !")
def test_timestamp_out_of_ballpark_rep_schema_compatibility():
    client_timestamp = DateTime.now()
    backend_timestamp = DateTime.now().add(minutes=5)

    # Backend API >= 2.4 with older clients
    RealmCreateRepBadTimestamp(
        reason=None,
        ballpark_client_early_offset=BALLPARK_CLIENT_EARLY_OFFSET,
        ballpark_client_late_offset=BALLPARK_CLIENT_LATE_OFFSET,
        backend_timestamp=backend_timestamp,
        client_timestamp=client_timestamp,
    )

    # Backend API < 2.4 with newer clients
    RealmCreateRepBadTimestamp(
        reason=None,
        ballpark_client_early_offset=None,
        ballpark_client_late_offset=None,
        backend_timestamp=None,
        client_timestamp=None,
    )


@pytest.mark.xfail(reason="TODO: fix this !")
def test_handshake_challenge_schema_compatibility():
    # Old handshake definition
    class OlderHandshakeChallengeSchema(BaseSchema):
        handshake = fields.CheckedConstant("challenge", required=True)
        challenge = fields.Bytes(required=True)
        supported_api_versions = fields.List(ApiVersionField(), required=True)

    older_handshake_challenge_serializer = serializer_factory(OlderHandshakeChallengeSchema)

    timestamp = DateTime.now()
    old_data = {"challenge": b"123", "handshake": "challenge", "supported_api_versions": []}
    new_data = {
        **old_data,
        "ballpark_client_early_offset": 1.0,
        "ballpark_client_late_offset": 1.0,
        "backend_timestamp": timestamp,
    }
    compat_data = {
        **old_data,
        "ballpark_client_early_offset": None,
        "ballpark_client_late_offset": None,
        "backend_timestamp": None,
    }

    # Backend API >= 2.4 with older clients
    data = handshake_challenge_serializer.dumps(new_data)
    assert older_handshake_challenge_serializer.loads(data) == old_data

    # Backend API < 2.4 with newer clients
    data = older_handshake_challenge_serializer.dumps(old_data)
    assert handshake_challenge_serializer.loads(data) == {**compat_data, "client_timestamp": None}


@pytest.mark.xfail(reason="TODO: fix this !")
def test_handshake_challenge_schema_for_client_server_api_compatibility(
    mallory, alice, monkeypatch
):
    ash = ServerHandshake()

    challenge = b"1234567890"

    # Test server handshake: client API <= 2.4 server > 3.0
    client_version = ApiVersion(2, 4)

    answer = {
        "handshake": "answer",
        "type": HandshakeType.AUTHENTICATED.value,
        "client_api_version": client_version,
        "organization_id": alice.organization_id.str,
        "device_id": alice.device_id.str,
        "rvk": alice.root_verify_key.encode(),
        "answer": alice.signing_key.sign(challenge),
    }

    ash.build_challenge_req()
    ash.challenge = challenge
    ash.process_answer_req(packb(answer))
    result_req = ash.build_result_req(alice.verify_key)
    result = handshake_result_serializer.loads(result_req)
    assert result["result"] == "ok"

    # Test server handshake: client API <= 2.8 server > 3.0

    ash = ServerHandshake()
    client_version = ApiVersion(2, 8)
    answer = {
        "handshake": "answer",
        "type": HandshakeType.AUTHENTICATED.value,
        "client_api_version": client_version,
        "organization_id": alice.organization_id.str,
        "device_id": alice.device_id.str,
        "rvk": alice.root_verify_key.encode(),
        "answer": alice.signing_key.sign(answer_serializer.dumps({"answer": challenge})),
    }
    ash.build_challenge_req()
    ash.challenge = challenge
    ash.process_answer_req(packb(answer))
    result_req = ash.build_result_req(alice.verify_key)
    result = handshake_result_serializer.loads(result_req)
    assert result["result"] == "ok"

    # Authenticated client handshake: client api < 3

    bch = AuthenticatedClientHandshake(
        mallory.organization_id, mallory.device_id, mallory.signing_key, mallory.root_verify_key
    )

    client_version = ApiVersion(2, 8)

    req = {
        "handshake": "challenge",
        "challenge": challenge,
        "supported_api_versions": ServerHandshake.SUPPORTED_API_VERSIONS,
        "backend_timestamp": DateTime.now(),
        "ballpark_client_early_offset": BALLPARK_CLIENT_EARLY_OFFSET,
        "ballpark_client_late_offset": BALLPARK_CLIENT_LATE_OFFSET,
    }

    monkeypatch.setattr(bch, "SUPPORTED_API_VERSIONS", [client_version])

    answer_req = bch.process_challenge_req(packb(req))

    answer = handshake_answer_serializer.loads(answer_req)
    assert mallory.verify_key.verify(answer["answer"]) == answer_serializer.dumps(
        {"answer": challenge}
    )
