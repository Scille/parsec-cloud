# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import pytest


@pytest.mark.rust
def test_private_key():
    from parsec.crypto import (
        _PyPrivateKey,
        _RsPrivateKey,
        PrivateKey,
        PublicKey,
        _PyPublicKey,
        _RsPublicKey,
    )

    assert PrivateKey is _RsPrivateKey
    assert PublicKey is _RsPublicKey

    KEY = b"a" * 32
    MESSAGE = b"test private key"

    rs_pvk = _RsPrivateKey(KEY)
    py_pvk = _PyPrivateKey(KEY)

    rs_pbk = rs_pvk.public_key
    py_pbk = py_pvk.public_key

    assert isinstance(rs_pvk, _RsPrivateKey)
    assert isinstance(py_pvk, _PyPrivateKey)

    assert isinstance(rs_pbk, _RsPublicKey)
    assert isinstance(py_pbk, _PyPublicKey)

    assert repr(rs_pvk) == repr(py_pvk)

    cyphered_rs = rs_pvk.public_key.encrypt_for_self(MESSAGE)
    cyphered_py = py_pvk.public_key.encrypt_for_self(MESSAGE)

    assert rs_pvk.decrypt_from_self(cyphered_rs) == py_pvk.decrypt_from_self(cyphered_py)
    assert rs_pvk.decrypt_from_self(cyphered_py) == py_pvk.decrypt_from_self(cyphered_rs)

    # Check if generate returns the right type
    assert isinstance(_RsPrivateKey.generate(), _RsPrivateKey)
    assert isinstance(_PyPrivateKey.generate(), _PyPrivateKey)
