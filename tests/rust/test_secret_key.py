# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import pytest


@pytest.mark.rust
def test_secret_key():
    from parsec.crypto import _PySecretKey, _RsSecretKey

    KEY = b"a" * 32
    MESSAGE = b"test secret key"

    rs = _RsSecretKey(KEY)
    py = _PySecretKey(KEY)

    assert isinstance(rs, _RsSecretKey)
    assert isinstance(py, _PySecretKey)

    assert rs.secret == py.secret
    assert repr(rs) == repr(py)

    ciphered_rs = rs.encrypt(MESSAGE)
    ciphered_py = py.encrypt(MESSAGE)

    # We cannot assert "ciphered_rs == ciphered_py"
    # Encryption is not idempotent (a nonce is involved), hence comparison is not possible

    # Verify with both decryption
    assert rs.decrypt(ciphered_rs) == py.decrypt(ciphered_py)
    assert rs.decrypt(ciphered_py) == py.decrypt(ciphered_rs)

    # check if we get the sane hmac
    assert rs.hmac(MESSAGE, 5) == py.hmac(MESSAGE, 5)

    # Check if generate returns the right type
    assert isinstance(_RsSecretKey.generate(), _RsSecretKey)
    assert isinstance(_PySecretKey.generate(), _PySecretKey)
