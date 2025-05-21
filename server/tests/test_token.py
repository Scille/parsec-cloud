# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import base64

import pytest

from parsec._parsec import DateTime, SecretKey
from parsec.components.auth import AccountPasswordAuthenticationToken


def gen_account_auth_token() -> tuple[bytes, AccountPasswordAuthenticationToken, SecretKey]:
    email = "foo@example.com"
    timestamp = DateTime(2025, 4, 20, 12, 12, 0)
    key = SecretKey(base64.b64decode("zCNJOtAmCVI6FIvoPAFBJr5ehbhswvvbhkHR+dUrjaM="))
    header_and_payload = b"%s.%s.%s" % (
        AccountPasswordAuthenticationToken.HEADER,
        base64.urlsafe_b64encode(email.encode()),
        str(timestamp.as_timestamp_seconds()).encode("ascii"),
    )
    expected_cooked = AccountPasswordAuthenticationToken(
        timestamp=timestamp,
        email=email,
        header_and_payload=header_and_payload,
        signature=key.mac_512(header_and_payload),
    )
    expected_raw = b"PARSEC-PASSWORD-MAC-BLAKE2B.Zm9vQGV4YW1wbGUuY29t.1745151120.ygDiZax96mrj1qad0zpFZ0VIeshWXGh5PRQmjXjA4JXF4i-02sH5nmuNSA4WBql2IFDYOaXBYhSAGkh-RP9l9Q=="

    return (expected_raw, expected_cooked, key)


def test_account_auth_roundtrip() -> None:
    expected_raw, expected_cooked, mac_key = gen_account_auth_token()

    raw = AccountPasswordAuthenticationToken.generate_raw(
        timestamp=expected_cooked.timestamp,
        email=expected_cooked.email,
        mac_key=mac_key,
    )
    assert raw == expected_raw

    got = AccountPasswordAuthenticationToken.from_raw(raw)
    assert got == expected_cooked

    # We don't support unpadded (i.e. without trailing `=`) base64, this check is
    # here to avoid silently supporting it whenever we change the implementation...
    assert raw[-2:] == b"=="  # Sanity check
    with pytest.raises(ValueError):
        AccountPasswordAuthenticationToken.from_raw(raw[:-2])


def test_account_auth_token_verify() -> None:
    _, token, mac_key = gen_account_auth_token()

    assert token.verify(mac_key)

    assert not token.verify(SecretKey.generate())
