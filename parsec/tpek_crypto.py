# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import oscrypto
import oscrypto.asymmetric

from oscrypto.asymmetric import PublicKey as _PublicKey
from oscrypto.asymmetric import PrivateKey as _PrivateKey
from pathlib import Path

DerPublicKey = _PublicKey
DerPrivateKey = _PrivateKey

hash_algorithm = "sha256"


class TpekCryptoError(Exception):
    pass


class TpekCryptoSignatureError(TpekCryptoError):
    pass


def load_der_public_key(bytes_key: bytes) -> DerPublicKey:
    return oscrypto.asymmetric.load_public_key(bytes_key)


def load_der_private_key(path: Path) -> DerPrivateKey:
    return oscrypto.asymmetric.load_private_key(path)


def verify_tpek(public_key: DerPublicKey, data: bytes, signature: bytes) -> None:
    """
    Raises:
        TpekCryptoSignatureError
        TpekCryptoError
    """
    try:
        oscrypto.asymmetric.rsa_pss_verify(public_key, signature, data, hash_algorithm)
    except oscrypto.errors.SignatureError:
        raise TpekCryptoSignatureError()
    except OSError:
        raise TpekCryptoError()


def sign_tpek(private_key: DerPrivateKey, data: bytes) -> bytes:
    return oscrypto.asymmetric.rsa_pss_sign(private_key, data, hash_algorithm)
