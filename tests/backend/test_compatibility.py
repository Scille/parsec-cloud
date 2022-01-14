# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import pendulum

from parsec.serde import BaseSchema, fields
from parsec.api.protocol.base import ErrorRepSchema
from parsec.api.protocol import realm_create_serializer
from parsec.api.protocol.handshake import handshake_challenge_serializer, ApiVersionField
from parsec.api.protocol.base import serializer_factory


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
