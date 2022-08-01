# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from typing import Tuple, TYPE_CHECKING
from base64 import b32decode, b32encode
from hashlib import sha256

from nacl.exceptions import CryptoError  # noqa: republishing
from nacl.public import SealedBox, PrivateKey as _PrivateKey, PublicKey as _PublicKey
from nacl.signing import SigningKey as _SigningKey, VerifyKey as _VerifyKey
from nacl.secret import SecretBox
from nacl.bindings import crypto_sign_BYTES, crypto_scalarmult
from nacl.hash import blake2b, BLAKE2B_BYTES
from nacl.pwhash import argon2i
from nacl.utils import random
from nacl.encoding import RawEncoder


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


# Types


class SecretKey:
    __slots__ = ("_secret",)

    def __init__(self, secret: bytes):
        self._secret = secret

    def __new__(cls, rawkey: bytes) -> "SecretKey":
        if len(rawkey) != SecretBox.KEY_SIZE:
            raise ValueError("Invalid key size")
        return super().__new__(cls)

    def __eq__(self, other):
        if not isinstance(other, type(self)):
            return NotImplemented
        return self.secret == other.secret

    @classmethod
    def generate(cls) -> "SecretKey":
        return cls(random(SecretBox.KEY_SIZE))

    def __repr__(self):
        # Avoid leaking the key in logs
        return "SecretKey(<redacted>)"

    def encrypt(self, data: bytes) -> bytes:
        """
        Raises:
            CryptoError: if key is invalid.
        """
        box = SecretBox(self.secret)
        return box.encrypt(data)

    def decrypt(self, ciphered: bytes) -> bytes:
        """
        Raises:
            CryptoError: if key is invalid.
        """
        box = SecretBox(self.secret)
        return box.decrypt(ciphered)

    def hmac(self, data: bytes, digest_size=BLAKE2B_BYTES) -> bytes:
        return blake2b(data, digest_size=digest_size, key=self.secret, encoder=RawEncoder)

    @property
    def secret(self) -> bytes:
        return self._secret


_PySecretKey = SecretKey
if not TYPE_CHECKING:
    try:
        from libparsec.hazmat import SecretKey as _RsSecretKey
    except ImportError:
        pass
    else:
        SecretKey = _RsSecretKey


class HashDigest:
    __slots__ = ("_digest",)

    def __init__(self, digest: bytes):
        self._digest = digest

    def __repr__(self) -> str:
        return f"HashDigest({self.hexdigest()})"

    def __eq__(self, other):
        if not isinstance(other, type(self)):
            return NotImplemented
        return self.digest == other.digest

    @classmethod
    def from_data(cls, data: bytes) -> "HashDigest":
        # nacl's sha256 doesn't accept bytearray, so stick to `hashlib.sha256`
        return cls(sha256(data).digest())

    @property
    def digest(self) -> bytes:
        return self._digest

    def hexdigest(self) -> str:
        return self._digest.hex()


_PyHashDigest = HashDigest
if not TYPE_CHECKING:
    try:
        from libparsec.hazmat import HashDigest as _RsHashDigest
    except ImportError:
        pass
    else:
        HashDigest = _RsHashDigest


# Basically just add comparison support to nacl keys


class SigningKey(_SigningKey):
    __slots__ = ()

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.verify_key.__class__ = _PyVerifyKey

    @classmethod
    def generate(cls, *args, **kwargs) -> "SigningKey":
        obj = super().generate(*args, **kwargs)
        obj.__class__ = cls
        return obj

    def __eq__(self, other):
        return isinstance(other, _SigningKey) and self._signing_key == other._signing_key

    def __repr__(self):
        # Avoid leaking the key in logs
        return "SigningKey(<redacted>)"


_PySigningKey = SigningKey
if not TYPE_CHECKING:
    try:
        from libparsec.hazmat import SigningKey as _RsSigningKey
    except ImportError:
        pass
    else:
        SigningKey = _RsSigningKey


class VerifyKey(_VerifyKey):
    __slots__ = ()

    def __eq__(self, other):
        return isinstance(other, _VerifyKey) and self._key == other._key

    @classmethod
    def unsecure_unwrap(cls, signed: bytes) -> bytes:
        return signed[crypto_sign_BYTES:]


_PyVerifyKey = VerifyKey
if not TYPE_CHECKING:
    try:
        from libparsec.hazmat import VerifyKey as _RsVerifyKey
    except ImportError:
        pass
    else:
        VerifyKey = _RsVerifyKey


class PrivateKey(_PrivateKey):
    __slots__ = ()

    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        self.public_key.__class__ = _PyPublicKey

    @classmethod
    def generate(cls, *args, **kwargs) -> "PrivateKey":
        obj = super().generate(*args, **kwargs)
        obj.__class__ = cls
        return obj

    def __eq__(self, other):
        return isinstance(other, _PrivateKey) and self._private_key == other._private_key

    def decrypt_from_self(self, ciphered: bytes) -> bytes:
        """
        raises:
            CryptoError
        """
        return SealedBox(self).decrypt(ciphered)

    def __repr__(self):
        # Avoid leaking the key in logs
        return "PrivateKey(<redacted>)"


_PyPrivateKey = PrivateKey
if not TYPE_CHECKING:
    try:
        from libparsec.hazmat import PrivateKey as _RsPrivateKey
    except ImportError:
        pass
    else:
        PrivateKey = _RsPrivateKey


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


_PyPublicKey = PublicKey
if not TYPE_CHECKING:
    try:
        from libparsec.hazmat import PublicKey as _RsPublicKey
    except ImportError:
        pass
    else:
        PublicKey = _RsPublicKey


# Helpers


# Binary encoder/decoder for url use.
# Notes:
# - We replace padding char `=` by a simple `s` (which is not part of
#   the base32 table so no risk of collision) to avoid copy/paste errors
#   and silly escaping issues when carrying the key around.
# - We could be using base64url (see RFC 4648) which would be more efficient,
#   but backward compatibility prevent us from doing it :'(


def binary_urlsafe_encode(data: bytes) -> str:
    """
    Raises:
        ValueError
    """
    return b32encode(data).decode("utf8").replace("=", "s")


def binary_urlsafe_decode(data: str) -> bytes:
    """
    Raises:
        ValueError
    """
    return b32decode(data.replace("s", "=").encode("utf8"))


def export_root_verify_key(key: VerifyKey) -> str:
    """
    Raises:
        ValueError
    """
    return binary_urlsafe_encode(key.encode())


def import_root_verify_key(raw: str) -> VerifyKey:
    """
    Raises:
        ValueError
    """
    raw_binary = binary_urlsafe_decode(raw)
    try:
        return VerifyKey(raw_binary)
    except CryptoError as exc:
        raise ValueError("Invalid verify key") from exc


def derivate_secret_key_from_password(password: str, salt: bytes = None) -> Tuple[SecretKey, bytes]:
    salt = salt or random(argon2i.SALTBYTES)
    rawkey = argon2i.kdf(
        SecretBox.KEY_SIZE,
        password.encode("utf8"),
        salt,
        opslimit=CRYPTO_OPSLIMIT,
        memlimit=CRYPTO_MEMLIMIT,
    )
    return SecretKey(rawkey), salt


def generate_shared_secret_key(
    our_private_key: PrivateKey, peer_public_key: PublicKey
) -> SecretKey:
    return SecretKey(crypto_scalarmult(our_private_key.encode(), peer_public_key.encode()))


def generate_nonce(size=64) -> bytes:
    return random(size=size)


# We want to provide the user with an easier to type format for the recovery password.
# We use base32 so all characters are in one case, and the alphabet contains less
# colliding letters (ie 0 O) then form 4 letters groups separated by dashes.
# When decoding, we remove any characters not included in the alphabet (spaces, new lines, ...)
# and decode to get our password back.


def generate_recovery_passphrase() -> Tuple[str, SecretKey]:
    """
    passphrase is typically something like:
    D5VR-53YO-QYJW-VJ4A-4DQR-4LVC-W425-3CXN-F3AQ-J6X2-YVPZ-XBAO-NU4Q

    Yes, it looks like a good old CD key *insert keygen music*
    """
    key = SecretKey.generate()
    b32 = b32encode(key.secret).decode().rstrip("=")
    passphrase = "-".join(b32[i : i + 4] for i in range(0, len(b32), 4))
    return passphrase, key


# 34 symbols to 32 values due to 0/O and 1/I
RECOVERY_PASSPHRASE_SYMBOLS = set("ABCDEFGHIJKLMNOPQRSTUVWXYZ01234567")


def derivate_secret_key_from_recovery_passphrase(passphrase: str) -> SecretKey:
    """
    Raise: ValueError
    """
    # Lowercases are not allowed in theory, but it's too tempting to fix this here ;-)
    passphrase = passphrase.upper()
    # Filter out any unknown characters, this is typically useful to remove
    # the `-` and whitespaces.
    # Note we also discard possible typos from the user (for instance if he types
    # a `8` or a `9`), but this is no big deal given 1) it should not happen
    # because GUI should use `RECOVERY_PASSPHRASE_SYMBOLS` to prevent user
    # from being able to provide invalid characters and 2) it will most likely
    # lead to a bad password anyway
    passphrase = "".join(c for c in passphrase if c in RECOVERY_PASSPHRASE_SYMBOLS)
    b32 = passphrase + "=" * (-len(passphrase) % 8)  # Add padding
    # ` map01 option to convert `zero => O` and `one => I`
    rawkey = b32decode(b32, map01="I")
    return SecretKey(rawkey)
