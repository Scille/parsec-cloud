# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from typing import Union
from enum import Enum
import oscrypto.asymmetric
import oscrypto.errors

from parsec.crypto import CryptoError, SecretKey


class SequesterKeyAlgorithm(Enum):
    RSA = "RSA"


def _enforce_rsa_output_has_key_size(key: oscrypto.asymmetric.PrivateKey, output: bytes) -> bytes:
    # Using RSA, we should end up with a number as big as the key size and
    # provided as big endian bytes.
    # However it is possible the number can be represented with less bytes if
    # enough of its most significant bits are equal to zero.
    # In such case, certain implementations (at least oscrypt on macOS 12) trim
    # the null bytes and hence return a bytes array smaller than the key size.
    #
    # For instance, considering a 16bits RSA key size (state-of-the-art security ^^)
    # and a RSA output `42`, the output can be represented as b"\x42" or b"\x00\x42)
    #
    # Long story short, we want to make sure our RSA output are always of the
    # same size than the key, this simplify splitting messages and protect us
    # if an RSA implementation on another platform is picky about output size.
    if len(output) == key.byte_size:
        return output
    else:
        assert len(output) <= key.byte_size  # Sanity check
        # Add enough null bytes to have output of the same size than the key
        return (b"\x00" * (key.byte_size - len(output))) + output


class _SequesterPublicKeyDer:
    __slots__ = ("_key",)

    def __init__(self, der_key: Union[bytes, oscrypto.asymmetric.PublicKey]):
        if isinstance(der_key, bytes):
            try:
                self._key = oscrypto.asymmetric.load_public_key(der_key)
            except (oscrypto.errors.AsymmetricKeyError, OSError) as exc:
                raise ValueError(str(exc)) from exc
        else:
            assert isinstance(der_key, oscrypto.asymmetric.PublicKey)
            self._key = der_key

        if self._key.algorithm != "rsa":
            raise ValueError("Unsupported key format")

    def __eq__(self, other):
        if not isinstance(other, type(self)):
            return NotImplemented
        return self._key == other._key

    @property
    def algorithm(self) -> SequesterKeyAlgorithm:
        return SequesterKeyAlgorithm.RSA

    def dump(self) -> bytes:
        return oscrypto.asymmetric.dump_public_key(self._key, encoding="der")

    def __repr__(self):
        return f"{type(self).__name__}({self._key})"

    @property
    def secret(self) -> bytes:
        return self._key


class SequesterVerifyKeyDer(_SequesterPublicKeyDer):
    __slots__ = ()

    # Signature format:
    #   <algorithm name>:<signature><data>
    SIGNING_ALGORITHM = b"RSASSA-PSS-SHA256"

    def verify(self, data: bytes) -> bytes:
        """
        Raises:
            CryptoError: if key or signature are invalid.
        """
        try:
            algo, signature_and_content = data.split(b":", 1)
            if algo != self.SIGNING_ALGORITHM:
                raise ValueError
        except ValueError as exc:
            raise CryptoError("Unsupported algorithm") from exc

        # In RSASSA-PSS, signature is as big as the key size
        signature = signature_and_content[: self._key.byte_size]
        content = signature_and_content[self._key.byte_size :]

        try:
            oscrypto.asymmetric.rsa_pss_verify(self._key, signature, content, "sha256")
        except (oscrypto.errors.SignatureError, OSError) as exc:
            raise CryptoError(str(exc)) from exc

        return content


def sequester_authority_sign(signing_key: oscrypto.asymmetric.PrivateKey, data: bytes) -> bytes:
    try:
        signature = oscrypto.asymmetric.rsa_pss_sign(signing_key, data, "sha256")
    except OSError as exc:
        raise CryptoError(str(exc)) from exc

    # It is import `signature` has a constant size given this is how it is
    # retreived during verification
    signature = _enforce_rsa_output_has_key_size(signing_key, signature)

    return SequesterVerifyKeyDer.SIGNING_ALGORITHM + b":" + signature + data


class SequesterEncryptionKeyDer(_SequesterPublicKeyDer):
    __slots__ = ()

    # Encryption format:
    #   <algorithm name>:<encrypted secret key with RSA key><encrypted data with secret key>
    ENCRYPTION_ALGORITHM = b"RSAES-OAEP-XSALSA20-POLY1305"

    def encrypt(self, data: bytes) -> bytes:
        """
        Raises:
            CryptoError: if key is invalid.
        """
        secret_key = SecretKey.generate()
        try:
            # RSAES-OAEP uses 42 bytes for padding, hence even with an unsecure
            # 1024 bits RSA key there is still 86 bytes available for payload
            # which is plenty to store the 32 bytes XSalsa20 key
            secret_key_encrypted = oscrypto.asymmetric.rsa_oaep_encrypt(
                self._key, secret_key.secret
            )
        except OSError as exc:
            raise CryptoError(str(exc)) from exc

        # It is import `secret_key_encrypted` has a constant size given this
        # is how it is retreived during decryption
        secret_key_encrypted = _enforce_rsa_output_has_key_size(self._key, secret_key_encrypted)

        return self.ENCRYPTION_ALGORITHM + b":" + secret_key_encrypted + secret_key.encrypt(data)


def sequester_service_decrypt(decryption_key: oscrypto.asymmetric.PrivateKey, data: bytes) -> bytes:
    try:
        algo, cipherkey_and_ciphertext = data.split(b":", 1)
        if algo != SequesterEncryptionKeyDer.ENCRYPTION_ALGORITHM:
            raise ValueError
    except ValueError as exc:
        raise CryptoError("Unsupported algorithm") from exc

    cipherkey = cipherkey_and_ciphertext[: decryption_key.byte_size]
    ciphertext = cipherkey_and_ciphertext[decryption_key.byte_size :]

    try:
        clearkey = SecretKey(oscrypto.asymmetric.rsa_oaep_decrypt(decryption_key, cipherkey))
    except OSError as exc:
        raise CryptoError(str(exc)) from exc

    cleartext = clearkey.decrypt(ciphertext)
    return cleartext
