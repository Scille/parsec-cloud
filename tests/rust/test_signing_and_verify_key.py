# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import pytest
import nacl


@pytest.mark.rust
def test_signing_key():
    from parsec.crypto import (
        _PySigningKey,
        _RsSigningKey,
        SigningKey,
        VerifyKey,
        _RsVerifyKey,
        _PyVerifyKey,
    )

    assert SigningKey is _RsSigningKey
    assert VerifyKey is _RsVerifyKey

    KEY = b"a" * 32

    rs_sk = SigningKey(KEY)
    py_sk = _PySigningKey(KEY)

    rs_vk = rs_sk.verify_key
    py_vk = py_sk.verify_key

    assert SigningKey(KEY) == SigningKey(KEY)
    assert SigningKey(KEY) != SigningKey(b"b" * 32)

    assert isinstance(rs_vk, _RsVerifyKey)
    assert isinstance(py_vk, _PyVerifyKey)

    # Sign a message with both, check if the signed message is the same
    MESSAGE = b"My message"

    rs_signed = rs_sk.sign(MESSAGE)
    py_signed = py_sk.sign(MESSAGE)

    assert rs_signed == py_signed

    # Verify with both
    assert rs_vk.verify(rs_signed) == py_vk.verify(py_signed)
    assert rs_vk.verify(py_signed) == py_vk.verify(rs_signed)

    # Check if unsecure_unwrap is the same
    assert VerifyKey.unsecure_unwrap(rs_signed) == _PyVerifyKey.unsecure_unwrap(py_signed)
    assert VerifyKey.unsecure_unwrap(py_signed) == _PyVerifyKey.unsecure_unwrap(rs_signed)

    # Check if generate returns the right type
    assert isinstance(SigningKey.generate(), SigningKey)
    assert isinstance(_PySigningKey.generate(), _PySigningKey)

    # Check if they both react in a similar manner with incorrect data
    assert rs_vk.unsecure_unwrap(b"random_data") == py_vk.unsecure_unwrap(b"random_data")

    with pytest.raises(nacl.exceptions.CryptoError):
        py_vk.verify(b"random_data")
    with pytest.raises(nacl.exceptions.CryptoError):
        rs_vk.verify(b"random data")
