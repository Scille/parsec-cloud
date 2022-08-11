# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import pendulum

from parsec.serde import BaseSchema, fields
from parsec.api.protocol.base import ErrorRepSchema, packb
from parsec.api.protocol import realm_create_serializer
from parsec.api.protocol.handshake import (
    AuthenticatedClientHandshake,
    ServerHandshake,
    handshake_challenge_serializer,
    handshake_answer_serializer,
    handshake_result_serializer,
    ApiVersionField,
    HandshakeType,
)
from parsec.api.protocol.base import serializer_factory

from parsec.api.version import ApiVersion
from parsec.utils import BALLPARK_CLIENT_EARLY_OFFSET, BALLPARK_CLIENT_LATE_OFFSET


def test_timestamp_out_of_ballpark_rep_schema_compatibility():
    client_timestamp = pendulum.now()
    backend_timestamp = pendulum.now().add(minutes=5)

    # Backend API >= 2.4 with older clients
    data = realm_create_serializer.timestamp_out_of_ballpark_rep_dump(
        backend_timestamp=backend_timestamp, client_timestamp=client_timestamp
    )
    assert ErrorRepSchema().load(data).data == {"status": "bad_timestamp"}

    # Backend API < 2.4 with newer clients
    data = (
        ErrorRepSchema()
        .dump({"status": "bad_timestamp", "reason": "Timestamp is out of date."})
        .data
    )
    assert realm_create_serializer.rep_load(data) == {
        "status": "bad_timestamp",
        "client_timestamp": None,
        "backend_timestamp": None,
        "ballpark_client_early_offset": None,
        "ballpark_client_late_offset": None,
    }


def test_handshake_challenge_schema_compatibility():

    # Old handshake definition
    class OlderHandshakeChallengeSchema(BaseSchema):
        handshake = fields.CheckedConstant("challenge", required=True)
        challenge = fields.Bytes(required=True)
        supported_api_versions = fields.List(ApiVersionField(), required=True)

    older_handshake_challenge_serializer = serializer_factory(OlderHandshakeChallengeSchema)

    timestamp = pendulum.now()
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
    assert handshake_challenge_serializer.loads(data) == compat_data


def test_handshake_challenge_schema_for_client_server_api_compatibility(
    mallory, alice, monkeypatch
):
    ash = ServerHandshake()

    bch = AuthenticatedClientHandshake(
        mallory.organization_id, mallory.device_id, mallory.signing_key, mallory.root_verify_key
    )

    challenge = b"1234567890"

    # Backend API >= 2.5 and Client API < 2.5
    client_version = ApiVersion(2, 4)
    backend_version = ApiVersion(2, 5)

    answer = {
        "handshake": "answer",
        "type": HandshakeType.AUTHENTICATED.value,
        "client_api_version": client_version,
        "organization_id": str(alice.organization_id),
        "device_id": str(alice.device_id),
        "rvk": alice.root_verify_key.encode(),
        "answer": alice.signing_key.sign(challenge),
    }

    ash.build_challenge_req()
    ash.challenge = challenge

    ash.process_answer_req(packb(answer))
    result_req = ash.build_result_req(alice.verify_key)

    result = handshake_result_serializer.loads(result_req)
    assert result["result"] == "ok"

    # Backend API < 2.5 and Client API >= 2.5
    client_version = ApiVersion(2, 5)
    backend_version = ApiVersion(2, 4)

    req = {
        "handshake": "challenge",
        "challenge": challenge,
        "supported_api_versions": [backend_version],
        "backend_timestamp": pendulum.now(),
        "ballpark_client_early_offset": BALLPARK_CLIENT_EARLY_OFFSET,
        "ballpark_client_late_offset": BALLPARK_CLIENT_LATE_OFFSET,
    }

    monkeypatch.setattr(bch, "SUPPORTED_API_VERSIONS", [client_version])

    answer_req = bch.process_challenge_req(packb(req))

    answer = handshake_answer_serializer.loads(answer_req)
    assert mallory.verify_key.verify(answer["answer"]) == challenge
