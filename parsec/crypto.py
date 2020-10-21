# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import os

from typing import Tuple
from base64 import b32decode, b32encode
from hashlib import sha256
import time
import struct

# import hmac
# from ctypes import cdll, c_size_t, c_uint8, byref, cast, POINTER, c_int


from nacl.exceptions import CryptoError  # noqa: republishing
from nacl.public import SealedBox, PrivateKey as _PrivateKey, PublicKey as _PublicKey
from nacl.signing import SigningKey as _SigningKey, VerifyKey as _VerifyKey
from nacl.bindings import crypto_sign_BYTES, crypto_scalarmult
from nacl.hash import blake2b, BLAKE2B_BYTES
from nacl.pwhash import argon2i
from nacl.utils import random
from nacl.encoding import RawEncoder

from cryptography.hazmat.primitives.ciphers import Cipher, algorithms, modes
from cryptography.exceptions import InvalidTag


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
    # Helpers
    "export_root_verify_key",
    "import_root_verify_key",
    "derivate_secret_key_from_password",
)


# TODO: SENSITIVE is really slow which is not good for unittests...
# CRYPTO_OPSLIMIT = argon2i.OPSLIMIT_SENSITIVE
# CRYPTO_MEMLIMIT = argon2i.MEMLIMIT_SENSITIVE
CRYPTO_OPSLIMIT = argon2i.OPSLIMIT_INTERACTIVE
CRYPTO_MEMLIMIT = argon2i.MEMLIMIT_INTERACTIVE

# SGX_AESGCM_MAC_SIZE = 16
# SGX_AESGCM_IV_SIZE = 12

# DIGEST_SIZE = 32


# class SecretKey(bytes):
#     __slots__ = ()

#     @classmethod
#     def generate(cls) -> "SecretKey":
#         return cls(os.urandom(16))

#     def __repr__(self):
#         # Avoid leaking the key in logs
#         return f"<{type(self).__module__}.{type(self).__qualname__} object at {hex(id(self))}>"

#     # Using AES with SGX enclave
#     def encrypt(self, data):
#         # Converting to bytes in case of BytesArray
#         data = bytes(data)
#         if len(data) < 1:
#             return b""
#         ecall_encryptText = LibSgx.ecall_encryptText
#         ecall_encryptText.restype = POINTER(c_uint8)
#         encMessageLen = c_size_t()
#         status_code = c_int()
#         encrypted_ptr = ecall_encryptText(
#             byref(status_code), self, data, len(data), byref(encMessageLen)
#         )
#         if status_code.value != SgxStatus.SGX_SUCCESS.value:
#             raise CryptoError
#             return b""
#         else:
#             encrypted_data = cast(encrypted_ptr, POINTER(c_uint8 * encMessageLen.value))
#             encrypted_data = bytes(list(encrypted_data.contents))
#             token = self.hmac(encrypted_data).digest() + encrypted_data
#             return token

#     def decrypt(self, token):
#         if len(token) < DIGEST_SIZE:
#             raise CryptoError(f"Tag must be at least {DIGEST_SIZE} bytes")
#         hmac = token[:DIGEST_SIZE]
#         data = token[DIGEST_SIZE:]
#         if self.hmac(data).digest() != hmac:
#             raise CryptoError
#             return b""
#         # Converting to bytes in case of BytesArray
#         data = bytes(data)
#         data_len = len(data)
#         if data_len < 1:
#             return b""
#         output_len = data_len - SGX_AESGCM_MAC_SIZE - SGX_AESGCM_IV_SIZE
#         ecall_decryptText = LibSgx.ecall_decryptText
#         ecall_decryptText.restype = POINTER(c_uint8 * output_len)
#         status_code = c_int()
#         decrypted_data = bytes(
#             list(ecall_decryptText(byref(status_code), self, data, data_len).contents)
#         )
#         if status_code.value != SgxStatus.SGX_SUCCESS.value:
#             return b""
#         else:
#             return decrypted_data

#     def hmac(self, data: bytes) -> bytes:
#         return hmac.new(self, data, sha256)


class SecretKey(bytes):
    __slots__ = ()

    @classmethod
    def generate(cls) -> "SecretKey":
        return cls(os.urandom(16))

    def __repr__(self):
        # Avoid leaking the key in logs
        return f"<{type(self).__module__}.{type(self).__qualname__} object at {hex(id(self))}>"

    # Using AES WITHOUT ENCLAVE
    def encrypt(self, data):
        time_stamp = struct.pack(">Q", int(time.time()))
        iv = os.urandom(16)
        cipher = Cipher(algorithms.AES(self), modes.GCM(iv))
        encryptor = cipher.encryptor()
        ciphered = encryptor.update(data) + encryptor.finalize()
        tag = encryptor.tag
        version = b"\x80"
        token = version + time_stamp + iv + ciphered + tag
        return token

    def decrypt(self, token):
        version = token[0:1]
        time_stamp = token[1:9]
        iv = token[9:25]
        tag = token[-16:]
        ciphered = token[25:-16]
        assert version == b"\x80"
        if time_stamp > struct.pack(">Q", int(time.time())):
            raise CryptoError("Invalid token")
        try:
            cipher = Cipher(algorithms.AES(self), modes.GCM(iv, tag))
            decryptor = cipher.decryptor()
            decrypted_message = decryptor.update(ciphered) + decryptor.finalize()
            return decrypted_message
        except (InvalidTag, ValueError) as exc:
            raise CryptoError(str(exc)) from exc

    def hmac(self, data: bytes, digest_size=BLAKE2B_BYTES) -> bytes:
        return blake2b(data, digest_size=digest_size, key=self, encoder=RawEncoder)


class HashDigest(bytes):
    __slots__ = ()

    @classmethod
    def from_data(self, data: bytes) -> "HashDigest":
        # nacl's sha256 doesn't accept bytearray, so stick to `hashlib.sha256`
        return HashDigest(sha256(data).digest())


# Basically just add comparison support to nacl keys


class SigningKey(_SigningKey):
    __slots__ = ()

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.verify_key.__class__ = VerifyKey

    @classmethod
    def generate(cls, *args, **kwargs) -> "SigningKey":
        obj = super().generate(*args, **kwargs)
        obj.__class__ = SigningKey
        return obj

    def __eq__(self, other):
        return isinstance(other, _SigningKey) and self._signing_key == other._signing_key


class VerifyKey(_VerifyKey):
    __slots__ = ()

    def __eq__(self, other):
        return isinstance(other, _VerifyKey) and self._key == other._key

    @classmethod
    def unsecure_unwrap(self, signed: bytes) -> bytes:
        return signed[crypto_sign_BYTES:]


class PrivateKey(_PrivateKey):
    __slots__ = ()

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.public_key.__class__ = PublicKey

    @classmethod
    def generate(cls, *args, **kwargs) -> "PrivateKey":
        obj = super().generate(*args, **kwargs)
        obj.__class__ = PrivateKey
        return obj

    def __eq__(self, other):
        return isinstance(other, _PrivateKey) and self._private_key == other._private_key

    def decrypt_from_self(self, ciphered: bytes) -> bytes:
        """
        raises:
            CryptoError
        """
        return SealedBox(self).decrypt(ciphered)


class PublicKey(_PublicKey):
    __slots__ = ()

    def __eq__(self, other):
        return isinstance(other, _PublicKey) and self._public_key == other._public_key

    def encrypt_for_self(self, data: bytes) -> bytes:
        """
        raises:
            CryptoError
        """
        return SealedBox(self).encrypt(data)


# Helpers


def export_root_verify_key(key: VerifyKey) -> str:
    """
    Raises:
        ValueError
    """
    # Note we replace padding char `=` by a simple `s` (which is not part of
    # the base32 table so no risk of collision) to avoid copy/paste errors
    # and silly escaping issues when carrying the key around.
    return b32encode(key.encode()).decode("utf8").replace("=", "s")


def import_root_verify_key(raw: str) -> VerifyKey:
    """
    Raises:
        ValueError
    """
    if isinstance(raw, VerifyKey):
        # Useful during tests
        return raw
    try:
        return VerifyKey(b32decode(raw.replace("s", "=").encode("utf8")))
    except CryptoError as exc:
        raise ValueError("Invalid verify key") from exc


def derivate_secret_key_from_password(password: str, salt: bytes = None) -> Tuple[SecretKey, bytes]:
    salt = salt or random(argon2i.SALTBYTES)
    rawkey = argon2i.kdf(
        16, password.encode("utf8"), salt, opslimit=CRYPTO_OPSLIMIT, memlimit=CRYPTO_MEMLIMIT
    )
    return SecretKey(rawkey), salt


def generate_shared_secret_key(
    our_private_key: PrivateKey, peer_public_key: PublicKey
) -> SecretKey:
    return SecretKey(crypto_scalarmult(our_private_key.encode(), peer_public_key.encode()))


def generate_nonce(size=64) -> bytes:
    return random(size=size)
