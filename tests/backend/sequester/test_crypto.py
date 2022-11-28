# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import oscrypto.asymmetric
import pytest

from parsec._parsec import SequesterPrivateKeyDer, SequesterPublicKeyDer, SequesterVerifyKeyDer


def test_only_rsa_is_supported():
    # Unsupported format for service encryption key (only RSA is currently supported)
    unsupported_public_key, _ = oscrypto.asymmetric.generate_pair("dsa", bit_size=1024)
    with pytest.raises(ValueError):
        SequesterPublicKeyDer(
            oscrypto.asymmetric.dump_public_key(unsupported_public_key, encoding="der")
        ),
    with pytest.raises(ValueError):
        SequesterVerifyKeyDer(
            oscrypto.asymmetric.dump_public_key(unsupported_public_key, encoding="der")
        ),


def test_encryption():
    private_key = SequesterPrivateKeyDer.generate()
    public_key = private_key.public_key

    encrypted = public_key.encrypt(b"foo")
    decrypted = private_key.decrypt(encrypted)
    assert decrypted == b"foo"


def test_signature():
    private_key = SequesterPrivateKeyDer.generate()
    verify_key = private_key.public_key.verify_key
    signing_key = private_key.signing_key

    signed = signing_key.sign(b"foo")
    verified = verify_key.verify(signed)
    assert verified == b"foo"
