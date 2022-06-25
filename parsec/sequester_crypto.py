# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from functools import wraps
import oscrypto
import oscrypto.asymmetric

from oscrypto.asymmetric import PublicKey as _PublicKey
from oscrypto.asymmetric import PrivateKey as _PrivateKey
import pendulum

from parsec.api.data.certif import SequesterServiceCertificate, SequesterServiceKeyFormat

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
    @wraps(func)
    def wrapper(*args, **kwargs):
        try:
            return func(*args, **kwargs)
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


def create_service_certificate(raw_encryption_public_key: bytes, service_name: str) -> bytes:
    """
    Raises:
        SequesterCryptoError
        SequesterCryptoInvalidKeyError
    """
    now = pendulum.now()
    encryption_public_key = load_sequester_public_key(raw_encryption_public_key)
    try:
        key_format = SequesterServiceKeyFormat(
            encryption_public_key.algorithm.upper()  # type: ignore[attr-defined]
        )
    except ValueError:
        raise SequesterCryptoInvalidKeyError(
            f"Unsupported Key Format {encryption_public_key.algorithm}"  # type: ignore[attr-defined]
        )
    return SequesterServiceCertificate(
        encryption_key=raw_encryption_public_key,
        encryption_key_format=key_format,
        timestamp=now,
        service_name=service_name,
    ).dump()


def sign_service_certificate(certificate: bytes, raw_signing_key: bytes) -> bytes:
    signing_key = load_sequester_private_key(raw_signing_key)
    return sign_sequester(signing_key, certificate)
