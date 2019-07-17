# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from typing import Tuple

from nacl.public import SealedBox
from nacl.pwhash import argon2i
from nacl.secret import SecretBox
from nacl.utils import random

from parsec.crypto_types import PrivateKey, PublicKey, SecretKey

# TODO: SENSITIVE is really slow which is not good for unittests...
# CRYPTO_OPSLIMIT = argon2i.OPSLIMIT_SENSITIVE
# CRYPTO_MEMLIMIT = argon2i.MEMLIMIT_SENSITIVE
CRYPTO_OPSLIMIT = argon2i.OPSLIMIT_INTERACTIVE
CRYPTO_MEMLIMIT = argon2i.MEMLIMIT_INTERACTIVE


def derivate_secret_key_from_password(password: str, salt: bytes = None) -> Tuple[SecretKey, bytes]:
    salt = salt or random(argon2i.SALTBYTES)
    key = argon2i.kdf(
        SecretBox.KEY_SIZE,
        password.encode("utf8"),
        salt,
        opslimit=CRYPTO_OPSLIMIT,
        memlimit=CRYPTO_MEMLIMIT,
    )
    return key, salt


def encrypt_raw_for(recipient_pubkey: PublicKey, data: bytes) -> bytes:
    """
    Raises:
        CryptoError: if key is invalid.
    """
    return SealedBox(recipient_pubkey).encrypt(data)


def decrypt_raw_for(recipient_privkey: PrivateKey, ciphered: bytes):
    """
    Raises:
        CryptoError: if key is invalid.
    """
    return SealedBox(recipient_privkey).decrypt(ciphered)


def encrypt_raw_with_secret_key(key: SecretKey, data: bytes) -> bytes:
    """
    Raises:
        CryptoError: if key is invalid.
    """
    box = SecretBox(key)
    return box.encrypt(data)


def decrypt_raw_with_secret_key(key: SecretKey, ciphered: bytes) -> bytes:
    """
    Raises:
        CryptoError: if key is invalid.
    """
    box = SecretBox(key)
    return box.decrypt(ciphered)
