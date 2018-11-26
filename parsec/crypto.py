from typing import Tuple
import pendulum
from nacl.public import PrivateKey, PublicKey, SealedBox
from nacl.signing import SigningKey, VerifyKey
from nacl.secret import SecretBox
from nacl.utils import random
from nacl.pwhash import argon2i
from nacl.exceptions import CryptoError

from parsec.api.base import DeviceIDField
from parsec.schema import UnknownCheckedSchema, fields, ValidationError
from parsec.types import DeviceID


__all__ = ("CryptoError", "PrivateKey", "PublicKey", "SigningKey", "VerifyKey")


# TODO: SENSITIVE is really slow which is not good for unittests...
# CRYPTO_OPSLIMIT = argon2i.OPSLIMIT_SENSITIVE
# CRYPTO_MEMLIMIT = argon2i.MEMLIMIT_SENSITIVE
CRYPTO_OPSLIMIT = argon2i.OPSLIMIT_INTERACTIVE
CRYPTO_MEMLIMIT = argon2i.MEMLIMIT_INTERACTIVE


class SignedMetadataSchema(UnknownCheckedSchema):
    device_id = DeviceIDField(required=True)
    timestamp = fields.DateTime(required=True)
    content = fields.Base64Bytes(required=True)


signed_metadata_schema = SignedMetadataSchema(strict=True)


class CryptoMetadataError(CryptoError):
    pass


def generate_secret_key():
    return random(SecretBox.KEY_SIZE)


def derivate_secret_key_from_password(password: str, salt: bytes = None) -> Tuple[bytes, bytes]:
    salt = salt or random(argon2i.SALTBYTES)
    key = argon2i.kdf(
        SecretBox.KEY_SIZE,
        password.encode("utf8"),
        salt,
        opslimit=CRYPTO_OPSLIMIT,
        memlimit=CRYPTO_MEMLIMIT,
    )
    return key, salt


def encrypt_raw_with_secret_key(key: bytes, data: bytes) -> bytes:
    """
    Raises:
        CryptoError: if key is invalid.
    """
    box = SecretBox(key)
    return box.encrypt(data)


def decrypt_raw_with_secret_key(key: bytes, ciphered: bytes) -> bytes:
    """
    Raises:
        CryptoError: if key is invalid.
    """
    box = SecretBox(key)
    return box.decrypt(ciphered)


def sign_and_add_meta(device_id: DeviceID, device_signkey: SigningKey, data: bytes) -> bytes:
    """
    Raises:
        CryptoError: if the signature operation fails.
    """
    return signed_metadata_schema.dumps(
        {"device_id": device_id, "timestamp": pendulum.now(), "content": device_signkey.sign(data)}
    )[0].encode("utf8")


def extract_meta_from_signature(signed_with_meta: bytes) -> Tuple[DeviceID, bytes]:
    """
    Raises:
        CryptoMetadataError: if the metadata cannot be extracted
    """
    try:
        meta = signed_metadata_schema.loads(signed_with_meta.decode("utf8"))[0]
        return DeviceID(meta["device_id"]), meta["content"]

    except (ValidationError, UnicodeDecodeError) as exc:
        raise CryptoMetadataError(
            "Message doesn't contain author metadata along with signed message"
        ) from exc


def encrypt_for_self(
    device_id: DeviceID, device_signkey: SigningKey, device_pubkey: PublicKey, data: bytes
) -> bytes:
    return encrypt_for(device_id, device_signkey, device_pubkey, data)


def encrypt_for(
    author_id: DeviceID, author_signkey: SigningKey, recipient_pubkey: PublicKey, data: bytes
) -> bytes:
    """
    Sign and encrypt a message.

    Raises:
        CryptoError: if encryption or signature fails.
    """
    signed_with_meta = sign_and_add_meta(author_id, author_signkey, data)

    box = SealedBox(recipient_pubkey)
    return box.encrypt(signed_with_meta)


def decrypt_for(recipient_privkey: PrivateKey, ciphered: bytes) -> Tuple[DeviceID, bytes]:
    """
    Decrypt a message and return it signed data and author metadata.

    Raises:
        CryptoMetadataError: if the author metadata cannot be extracted.
        CryptoError: if decryption or signature verifying fails.

    Returns: a tuple of (<user_id>, <device_name>, <signed_message>)

    Note: Once decrypted, the message should be passed to
    :func:`verify_signature_from` to be finally converted to plain text.
    """
    box = SealedBox(recipient_privkey)
    signed_with_meta = box.decrypt(ciphered)
    return extract_meta_from_signature(signed_with_meta)


def verify_signature_from(author_verifykey: VerifyKey, signed_text: bytes) -> bytes:
    """
    Verify signature and decode message.

    Returns: The plain text message.

    Raises:
         CryptoError: if signature was forged or otherwise corrupt.
    """
    return author_verifykey.verify(signed_text)


def encrypt_with_secret_key(
    author_id: DeviceID, author_signkey: SigningKey, key: bytes, data: bytes
) -> bytes:
    """
    Sign and encrypt a message with a symetric key.

    Raises:
        CryptoError: if the encryption or signature operation fails.
    """
    signed_with_meta = sign_and_add_meta(author_id, author_signkey, data)
    box = SecretBox(key)
    return box.encrypt(signed_with_meta)


def decrypt_with_secret_key(key: bytes, ciphered: bytes) -> Tuple[DeviceID, bytes]:
    """
    Decrypt a signed message with a symetric key.

    Raises:
        CryptoMetadataError: if the author metadata cannot be extracted.
        CryptoError: if decryption or signature verifying fails.

    Returns: a tuple of (<user_id>, <device_name>, <signed_message>)

    Note: Once decrypted, the message should be passed to
    :func:`verify_signature_from` to be finally converted to plain text.
    """
    box = SecretBox(key)
    signed_with_meta = box.decrypt(ciphered)
    return extract_meta_from_signature(signed_with_meta)
