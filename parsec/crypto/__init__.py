# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

# Republish `parsec.crypto_type` given it not part of
# `parsec.crypto` only to avoid recursive imports
from parsec.crypto_types import (
    SecretKey,
    HashDigest,
    PrivateKey,
    PublicKey,
    SigningKey,
    VerifyKey,
    export_root_verify_key,
    import_root_verify_key,
)
from parsec.crypto.exceptions import (
    CryptoError,
    BadSignatureError,
    CryptoWrappedMsgValidationError,
    CryptoWrappedMsgPackingError,
    CryptoSignatureAuthorMismatchError,
    CryptoSignatureTimestampMismatchError,
)
from parsec.crypto.raw import (
    derivate_secret_key_from_password,
    encrypt_raw_with_secret_key,
    decrypt_raw_with_secret_key,
    encrypt_raw_for,
    decrypt_raw_for,
)
from parsec.crypto.signed import (
    timestamps_in_the_ballpark,
    build_signed_msg,
    unsecure_extract_signed_msg_meta,
    verify_signed_msg,
    encrypt_signed_msg_for,
    decrypt_signed_msg_for,
    decrypt_and_verify_signed_msg_for,
    encrypt_signed_msg_with_secret_key,
    decrypt_signed_msg_with_secret_key,
    decrypt_and_verify_signed_msg_with_secret_key,
)
from parsec.crypto.certif import (
    CertifiedDeviceData,
    CertifiedRevokedDeviceData,
    CertifiedUserData,
    verify_device_certificate,
    verify_revoked_device_certificate,
    verify_user_certificate,
    unsecure_read_device_certificate,
    unsecure_read_revoked_device_certificate,
    unsecure_read_user_certificate,
    build_device_certificate,
    build_revoked_device_certificate,
    build_user_certificate,
)


__all__ = (
    # types
    "SecretKey",
    "HashDigest",
    "PrivateKey",
    "PublicKey",
    "SigningKey",
    "VerifyKey",
    "export_root_verify_key",
    "import_root_verify_key",
    # exceptions
    "CryptoError",
    "BadSignatureError",
    "CryptoWrappedMsgValidationError",
    "CryptoWrappedMsgPackingError",
    "CryptoSignatureAuthorMismatchError",
    "CryptoSignatureTimestampMismatchError",
    # raw
    "generate_secret_key",
    "derivate_secret_key_from_password",
    "encrypt_raw_with_secret_key",
    "decrypt_raw_with_secret_key",
    "encrypt_raw_for",
    "decrypt_raw_for",
    # signed
    "timestamps_in_the_ballpark",
    "build_signed_msg",
    "unsecure_extract_signed_msg_meta",
    "verify_signed_msg",
    "encrypt_signed_msg_for",
    "decrypt_signed_msg_for",
    "decrypt_and_verify_signed_msg_for",
    "encrypt_signed_msg_with_secret_key",
    "decrypt_signed_msg_with_secret_key",
    "decrypt_and_verify_signed_msg_with_secret_key",
    # certif
    "CertifiedDeviceData",
    "CertifiedRevokedDeviceData",
    "CertifiedUserData",
    "verify_device_certificate",
    "verify_revoked_device_certificate",
    "verify_user_certificate",
    "unsecure_read_device_certificate",
    "unsecure_read_revoked_device_certificate",
    "unsecure_read_user_certificate",
    "build_device_certificate",
    "build_revoked_device_certificate",
    "build_user_certificate",
)
