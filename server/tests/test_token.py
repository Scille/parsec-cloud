# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import base64
import hashlib

from parsec._parsec import DateTime, SecretKey
from parsec.components.auth import AccountPasswordAuthenticationToken


def gen_account_auth_token() -> tuple[AccountPasswordAuthenticationToken, SecretKey, bytes]:
    email = "foo@example.com"
    timestamp = DateTime(2025, 4, 20, 12, 12, 0)
    body = b"hello world"
    body_sha256 = hashlib.sha256(body).digest()
    key = SecretKey(base64.b64decode("zCNJOtAmCVI6FIvoPAFBJr5ehbhswvvbhkHR+dUrjaM="))
    header_and_payload = b".".join(
        (
            AccountPasswordAuthenticationToken.HEADER,
            base64.urlsafe_b64encode(email.encode()),
            str(timestamp.as_timestamp_seconds()).encode("ascii"),
        )
    )

    token = AccountPasswordAuthenticationToken(
        timestamp=timestamp,
        email=email,
        header_and_payload=header_and_payload,
        signature=key.mac_512(header_and_payload + b"." + body_sha256),
    )

    return (token, key, body)


def test_account_auth_roundtrip() -> None:
    expected, key, body = gen_account_auth_token()

    raw = AccountPasswordAuthenticationToken.generate_raw(
        timestamp=expected.timestamp,
        email=expected.email,
        body=body,
        key=key,
    )

    got = AccountPasswordAuthenticationToken.from_raw(raw)

    assert got == expected


def test_account_auth_token_verify() -> None:
    token, key, body = gen_account_auth_token()

    assert token.verify(body, key)
