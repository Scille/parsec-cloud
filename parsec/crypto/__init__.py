# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from parsec.crypto.certif import (
    CertifiedDeviceData,
    CertifiedRealmRoleData,
    CertifiedRevokedDeviceData,
    CertifiedUserData,
    build_device_certificate,
    build_realm_role_certificate,
    build_realm_self_role_certificate,
    build_revoked_device_certificate,
    build_user_certificate,
    unsecure_read_device_certificate,
    unsecure_read_realm_role_certificate,
    unsecure_read_revoked_device_certificate,
    unsecure_read_user_certificate,
    verify_device_certificate,
    verify_realm_role_certificate,
    verify_revoked_device_certificate,
    verify_user_certificate,
)
from parsec.crypto.exceptions import (
    BadSignatureError,
    CryptoError,
    CryptoSignatureAuthorMismatchError,
    CryptoSignatureTimestampMismatchError,
    CryptoWrappedMsgPackingError,
    CryptoWrappedMsgValidationError,
)
from parsec.crypto.raw import (
    decrypt_raw_for,
    decrypt_raw_with_secret_key,
    derivate_secret_key_from_password,
    encrypt_raw_for,
    encrypt_raw_with_secret_key,
)
from parsec.crypto.signed import (
    build_signed_msg,
    decrypt_and_verify_signed_msg_for,
    decrypt_and_verify_signed_msg_with_secret_key,
    decrypt_signed_msg_for,
    decrypt_signed_msg_with_secret_key,
    encrypt_signed_msg_for,
    encrypt_signed_msg_with_secret_key,
    timestamps_in_the_ballpark,
    unsecure_extract_signed_msg_meta,
    verify_signed_msg,
)

# Republish `parsec.crypto_type` given it not part of
# `parsec.crypto` only to avoid recursive imports
from parsec.crypto_types import (
    HashDigest,
    PrivateKey,
    PublicKey,
    SecretKey,
    SigningKey,
    VerifyKey,
    export_root_verify_key,
    import_root_verify_key,
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
    "CertifiedRealmRoleData",
    "verify_device_certificate",
    "verify_revoked_device_certificate",
    "verify_user_certificate",
    "verify_realm_role_certificate",
    "unsecure_read_device_certificate",
    "unsecure_read_revoked_device_certificate",
    "unsecure_read_user_certificate",
    "unsecure_read_realm_role_certificate",
    "build_device_certificate",
    "build_revoked_device_certificate",
    "build_user_certificate",
    "build_realm_role_certificate",
    "build_realm_self_role_certificate",
)
