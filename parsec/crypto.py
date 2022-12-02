# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

from nacl.exceptions import CryptoError
from nacl.pwhash import argon2i

from parsec._parsec import HashDigest, PrivateKey, PublicKey, SecretKey, SigningKey, VerifyKey

# Note to simplify things, we adopt `nacl.CryptoError` as our root error cls

__all__ = (
    # Exceptions
    "CryptoError",
    # Types
    "SecretKey",
    "HashDigest",
    "PrivateKey",
    "PublicKey",
    "SigningKey",
    "VerifyKey",
    "argon2i",
)
