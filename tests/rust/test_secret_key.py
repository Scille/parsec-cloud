# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import pytest


@pytest.mark.rust
def test_secret_key():
    from parsec.crypto import _PySecretKey, _RsSecretKey, SecretKey

    assert SecretKey is _RsSecretKey

    KEY = b"a" * 32
    MESSAGE = b"test secret key"

    rst = SecretKey(KEY)
    py = _PySecretKey(KEY)

    assert isinstance(rst, SecretKey)
    assert isinstance(py, _PySecretKey)

    assert rst.secret == py.secret
    assert repr(rst) == repr(py)

    ciphered_rst = rst.encrypt(MESSAGE)
    ciphered_py = py.encrypt(MESSAGE)

    # We cannot assert "ciphered_rst == ciphered_py" because the encryption is not exactly the same

    # Verify with both decryption
    assert rst.decrypt(ciphered_rst) == py.decrypt(ciphered_py)
    assert rst.decrypt(ciphered_py) == py.decrypt(ciphered_rst)

    # check if we get the sane hmac
    assert rst.hmac(MESSAGE) == py.hmac(MESSAGE)

    # Check if generate returns the right type
    assert isinstance(SecretKey.generate(), SecretKey)
    assert isinstance(_PySecretKey.generate(), _PySecretKey)
