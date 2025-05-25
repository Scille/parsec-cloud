# Parsec Cloud (https://parsec.cloud) Copyright (c) BUSL-1.1 2016-present Scille SAS

import base64

import pytest

from parsec._parsec import AccountAuthMethodID, DateTime, SecretKey
from parsec.components.auth import AccountAuthenticationToken


def gen_account_auth_token() -> tuple[bytes, AccountAuthenticationToken, SecretKey]:
    auth_method_id = AccountAuthMethodID.from_hex("9aae259f748045cc9fe7146eab0b132e")
    auth_method_mac_key = SecretKey(
        base64.b64decode("zCNJOtAmCVI6FIvoPAFBJr5ehbhswvvbhkHR+dUrjaM=")
    )
    timestamp = DateTime(2025, 4, 20, 12, 12, 0)
    header_and_payload = b"%s.%s.%s" % (
        AccountAuthenticationToken.HEADER,
        auth_method_id.hex.encode("ascii"),
        str(timestamp.as_timestamp_seconds()).encode("ascii"),
    )
    expected_cooked = AccountAuthenticationToken(
        auth_method_id=auth_method_id,
        timestamp=timestamp,
        header_and_payload=header_and_payload,
        signature=auth_method_mac_key.mac_512(header_and_payload),
    )
    expected_raw = b"PARSEC-MAC-BLAKE2B.9aae259f748045cc9fe7146eab0b132e.1745151120.I7b4206GeXp9qVQN8DLjoM9ul_kLhxa95X75hnGUcE9lNrlrK3PCXOhaxFi_FW5JZS504r6Xp7MMoxVZG9sMRQ=="

    return (expected_raw, expected_cooked, auth_method_mac_key)


def test_roundtrip() -> None:
    expected_raw, expected_cooked, auth_method_mac_key = gen_account_auth_token()

    raw = AccountAuthenticationToken.generate_raw(
        timestamp=expected_cooked.timestamp,
        auth_method_id=expected_cooked.auth_method_id,
        auth_method_mac_key=auth_method_mac_key,
    )
    assert raw == expected_raw

    got = AccountAuthenticationToken.from_raw(raw)
    assert got == expected_cooked

    # We don't support unpadded (i.e. without trailing `=`) base64, this check is
    # here to avoid silently supporting it whenever we change the implementation...
    assert raw[-2:] == b"=="  # Sanity check
    with pytest.raises(ValueError):
        AccountAuthenticationToken.from_raw(raw[:-2])


def test_verify() -> None:
    _, token, mac_key = gen_account_auth_token()

    assert token.verify(mac_key)

    assert not token.verify(SecretKey.generate())


def test_bad_format() -> None:
    valid_raw, _, _ = gen_account_auth_token()
    good_parts = valid_raw.split(b".")
    for i in range(len(good_parts)):
        bad_parts = [*good_parts]
        bad_parts[i] = b"<dummy>"
        bad_raw = b".".join(bad_parts)

        with pytest.raises(ValueError):
            AccountAuthenticationToken.from_raw(bad_raw)
