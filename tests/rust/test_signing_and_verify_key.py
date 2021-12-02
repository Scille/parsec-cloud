# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import pytest


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

    assert isinstance(rs_vk, _RsVerifyKey)
    assert isinstance(py_vk, _PyVerifyKey)

    MESSAGE = b"My message"

    rs_signed = rs_sk.sign(MESSAGE)
    py_signed = py_sk.sign(MESSAGE)

    assert rs_signed == py_signed

    assert rs_vk.verify(rs_signed) == py_vk.verify(py_signed)
    assert rs_vk.verify(py_signed) == py_vk.verify(rs_signed)

    assert VerifyKey.unsecure_unwrap(rs_signed) == _PyVerifyKey.unsecure_unwrap(py_signed)
    assert VerifyKey.unsecure_unwrap(py_signed) == _PyVerifyKey.unsecure_unwrap(rs_signed)

    assert isinstance(SigningKey.generate(), SigningKey)
    assert isinstance(_PySigningKey.generate(), _PySigningKey)
