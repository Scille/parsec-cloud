# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from functools import wraps
import oscrypto
import oscrypto.asymmetric

from oscrypto.asymmetric import PublicKey as _PublicKey
from oscrypto.asymmetric import PrivateKey as _PrivateKey

SequesterPublicKey = _PublicKey
SequesterPrivateKey = _PrivateKey

hash_algorithm = "sha256"


class SequesterCryptoError(Exception):
    pass


class SequesterCryptoInvalidKeyError(SequesterCryptoError):
    pass


class SequesterCryptoWrongTypeError(SequesterCryptoError):
    pass


class SequesterCryptoOsNotCompatibleError(SequesterCryptoError):
    pass


class SequesterCryptoSignatureError(SequesterCryptoError):
    pass


def handle_loading_error(func):
    @wraps
    def wrapper(*args, **kwargs):
        try:
            func(*args, **kwargs)
        except ValueError as exc:
            raise SequesterCryptoInvalidKeyError() from exc
        except TypeError as exc:
            raise SequesterCryptoWrongTypeError() from exc
        except oscrypto.errors.AsymmetricKeyError as exc:
            raise SequesterCryptoOsNotCompatibleError() from exc
        except OSError as exc:
            raise SequesterCryptoError from exc

    return wrapper


@handle_loading_error
def load_sequester_public_key(bytes_key: bytes) -> SequesterPublicKey:
    """
    Raises:
        SequesterCryptoError
    """
    return oscrypto.asymmetric.load_public_key(bytes_key)


def dump_sequester_public_key(public_key: SequesterPublicKey) -> bytes:
    return oscrypto.asymmetric.dump_public_key(public_key)  # Dump in pem format


@handle_loading_error
def load_sequester_private_key(bytes_key: bytes) -> SequesterPrivateKey:
    """
    Raises:
        SequesterCryptoError
    """
    return oscrypto.asymmetric.load_private_key(bytes_key)


def verify_sequester(public_key: SequesterPublicKey, data: bytes, signature: bytes) -> None:
    """
    Raises:
        SequesterCryptoSignatureError
        SequesterCryptoError
    """
    try:
        oscrypto.asymmetric.rsa_pss_verify(public_key, signature, data, hash_algorithm)
    except oscrypto.errors.SignatureError:
        raise SequesterCryptoSignatureError()
    except OSError:
        raise SequesterCryptoError()


def sign_sequester(private_key: SequesterPrivateKey, data: bytes) -> bytes:
    return oscrypto.asymmetric.rsa_pss_sign(private_key, data, hash_algorithm)
