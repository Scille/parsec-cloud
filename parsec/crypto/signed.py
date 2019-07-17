# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from typing import Optional, Tuple

from nacl.bindings import crypto_sign_BYTES
from nacl.public import SealedBox
from nacl.secret import SecretBox
from pendulum import Pendulum

from parsec.crypto.exceptions import (
    CryptoSignatureAuthorMismatchError,
    CryptoSignatureTimestampMismatchError,
    CryptoWrappedMsgPackingError,
    CryptoWrappedMsgValidationError,
)
from parsec.crypto_types import PrivateKey, PublicKey, SigningKey, VerifyKey
from parsec.serde import Serializer, UnknownCheckedSchema, fields
from parsec.types import DeviceID

TIMESTAMP_MAX_DT = 30 * 60


def timestamps_in_the_ballpark(ts1: Pendulum, ts2: Pendulum, max_dt=TIMESTAMP_MAX_DT) -> bool:
    """
    Useful to compare signed message timestamp with the one stored by the
    backend.
    """
    return abs((ts1 - ts2).total_seconds()) < max_dt


# A word about signed data format & naming (in pseudo code):
# msg = <content>
# wrapped_msg = WrappedMsgSchema(<device_id>, <timestamp>, <content>)
# signed_msg = SigningKey(<wrapped_msg>) = <msg_signature><wrapped_msg>


class WrappedMsgSchema(UnknownCheckedSchema):
    # No device_id means it has been signed by the root key
    device_id = fields.DeviceID(missing=None)
    timestamp = fields.DateTime(required=True)
    content = fields.Bytes(required=True)


wrapped_msg_serializer = Serializer(
    WrappedMsgSchema,
    validation_exc=CryptoWrappedMsgValidationError,
    packing_exc=CryptoWrappedMsgPackingError,
)


def build_signed_msg(
    device_id: Optional[DeviceID], device_signkey: SigningKey, content: bytes, timestamp: Pendulum
) -> bytes:
    """
    Raises:
        CryptoError: if the signature operation fails.
        CryptoWrappedMsgPackingError
        CryptoWrappedMsgValidationError
    """
    msg_with_meta = wrapped_msg_serializer.dumps(
        {"device_id": device_id, "timestamp": timestamp, "content": content}
    )
    return device_signkey.sign(msg_with_meta)


def unsecure_extract_signed_msg_meta_and_data(
    signed_msg: bytes
) -> Tuple[Optional[DeviceID], Pendulum, bytes]:
    """
    Raises:
        CryptoWrappedMsgPackingError
        CryptoWrappedMsgValidationError
    """
    wrapped_msg = signed_msg[crypto_sign_BYTES:]
    meta = wrapped_msg_serializer.loads(wrapped_msg)
    return meta["device_id"], meta["timestamp"], meta["content"]


def unsecure_extract_signed_msg_meta(signed_msg: bytes) -> Tuple[Optional[DeviceID], Pendulum]:
    """
    Raises:
        CryptoWrappedMsgPackingError
        CryptoWrappedMsgValidationError
    """
    return unsecure_extract_signed_msg_meta_and_data(signed_msg)[:-1]


def verify_signed_msg(
    signed_msg: bytes,
    expected_author_id: Optional[DeviceID],  # Remember `None` stands for root key here
    author_verifykey: VerifyKey,
    expected_timestamp: Pendulum,
) -> bytes:
    """
    Verify signature and decode message.

    Returns: The plain text message.

    Raises:
        CryptoError: if signature was forged or otherwise corrupt.
        CryptoSignatureAuthorMismatchError
        CryptoSignatureTimestampMismatchError
    """
    wrapped_msg = author_verifykey.verify(signed_msg)
    meta = wrapped_msg_serializer.loads(wrapped_msg)
    if meta["device_id"] != expected_author_id:
        raise CryptoSignatureAuthorMismatchError(
            f"Author mismatch: expected `{expected_author_id}`, got `{meta['device_id']}`"
        )
    if meta["timestamp"] != expected_timestamp:
        raise CryptoSignatureTimestampMismatchError(
            f"Timestamp mismatch: expected `{expected_timestamp}`, got `{meta['timestamp']}`"
        )
    return meta["content"]


def encrypt_signed_msg_for(
    author_id: DeviceID,
    author_signkey: SigningKey,
    recipient_pubkey: PublicKey,
    data: bytes,
    timestamp: Pendulum,
) -> bytes:
    """
    Sign and encrypt a message.

    Raises:
        CryptoError: if encryption or signature fails.
    """
    signed_msg = build_signed_msg(author_id, author_signkey, data, timestamp)

    box = SealedBox(recipient_pubkey)
    return box.encrypt(signed_msg)


def decrypt_signed_msg_for(
    recipient_privkey: PrivateKey, ciphered: bytes
) -> Tuple[DeviceID, Pendulum, bytes]:
    """
    Decrypt a message and return it signed data and author metadata.

    Raises:
        CryptoError: if decryption or signature verifying fails.
        CryptoWrappedMsgPackingError: if signed message is invalid
        CryptoWrappedMsgValidationError: if signed message is invalid

    Returns: a tuple of (<device_id>, <timestamp>, <signed_msg>)

    Note: Once decrypted, the message should be passed to
    :func:`verify_signed_msg` to be finally converted to plain text.
    """
    box = SealedBox(recipient_privkey)
    signed_msg = box.decrypt(ciphered)
    return (*unsecure_extract_signed_msg_meta(signed_msg), signed_msg)


def decrypt_and_verify_signed_msg_for(
    recipient_privkey: PrivateKey,
    ciphered: bytes,
    expected_author_id: Optional[DeviceID],  # Remember `None` stands for root key here
    author_verifykey: VerifyKey,
    expected_timestamp: Pendulum,
) -> bytes:
    """
    Decrypt and verify signature of the given message.

    Raises:
        CryptoError: if the decryption operation fails.
        CryptoWrappedMsgPackingError: if signed message is invalid
        CryptoWrappedMsgValidationError: if signed message is invalid
    """
    box = SealedBox(recipient_privkey)
    signed_msg = box.decrypt(ciphered)
    return verify_signed_msg(signed_msg, expected_author_id, author_verifykey, expected_timestamp)


def encrypt_signed_msg_with_secret_key(
    author_id: DeviceID, author_signkey: SigningKey, key: bytes, data: bytes, timestamp: Pendulum
) -> bytes:
    """
    Sign and encrypt a message with a symetric key.

    Raises:
        CryptoError: if the encryption or signature operation fails.
    """
    signed_msg = build_signed_msg(author_id, author_signkey, data, timestamp)
    box = SecretBox(key)
    return box.encrypt(signed_msg)


def decrypt_signed_msg_with_secret_key(
    key: bytes, ciphered: bytes
) -> Tuple[DeviceID, Pendulum, bytes]:
    """
    Decrypt a signed message with a symetric key.

    Raises:
        CryptoError: if the decryption operation fails.
        CryptoWrappedMsgPackingError: if signed message is invalid
        CryptoWrappedMsgValidationError: if signed message is invalid

    Returns: a tuple of (<device_id>, <timestamp>, <signed_msg>)

    Note: Once decrypted, the message should be passed to
    :func:`verify_signed_msg` to be finally converted to plain text.
    """
    box = SecretBox(key)
    signed_msg = box.decrypt(ciphered)
    return (*unsecure_extract_signed_msg_meta(signed_msg), signed_msg)


def decrypt_and_verify_signed_msg_with_secret_key(
    key: bytes,
    ciphered: bytes,
    expected_author_id: Optional[DeviceID],  # Remember `None` stands for root key here
    author_verifykey: VerifyKey,
    expected_timestamp: Pendulum,
) -> bytes:
    """
    Decrypt and verify signature of the given message.

    Raises:
        CryptoError: if the decryption operation fails.
        CryptoWrappedMsgPackingError: if signed message is invalid
        CryptoWrappedMsgValidationError: if signed message is invalid
    """
    box = SecretBox(key)
    signed_msg = box.decrypt(ciphered)
    return verify_signed_msg(signed_msg, expected_author_id, author_verifykey, expected_timestamp)
