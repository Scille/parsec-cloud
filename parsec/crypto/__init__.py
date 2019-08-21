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


#  Compatibility helpers
# TODO: remove this


def build_device_certificate(certifier_id, certifier_key, device_id, verify_key, timestamp):
    from parsec.api.data import DeviceCertificateContent, DataError

    try:
        return DeviceCertificateContent(
            author=certifier_id, timestamp=timestamp, device_id=device_id, verify_key=verify_key
        ).dump_and_sign(certifier_key)
    except DataError as exc:
        raise CryptoError(str(exc)) from exc


def unsecure_read_device_certificate(device_certificate):
    from parsec.api.data import DeviceCertificateContent, DataError

    try:
        return DeviceCertificateContent.unsecure_load(device_certificate)
    except DataError as exc:
        raise CryptoError(str(exc)) from exc


def verify_device_certificate(device_certificate, expected_author_id, author_verify_key):
    from parsec.api.data import DeviceCertificateContent, DataError

    try:
        return DeviceCertificateContent.verify_and_load(
            device_certificate,
            author_verify_key=author_verify_key,
            expected_author=expected_author_id,
        )
    except DataError as exc:
        raise CryptoError(str(exc)) from exc


def verify_revoked_device_certificate(
    revoked_device_certificate, expected_author_id, author_verify_key
):
    from parsec.api.data import RevokedDeviceCertificateContent, DataError

    try:
        return RevokedDeviceCertificateContent.verify_and_load(
            revoked_device_certificate,
            author_verify_key=author_verify_key,
            expected_author=expected_author_id,
        )
    except DataError as exc:
        raise CryptoError(str(exc)) from exc


def verify_user_certificate(user_certificate, expected_author_id, author_verify_key):
    from parsec.api.data import UserCertificateContent, DataError

    try:
        return UserCertificateContent.verify_and_load(
            user_certificate,
            author_verify_key=author_verify_key,
            expected_author=expected_author_id,
        )
    except DataError as exc:
        raise CryptoError(str(exc)) from exc


def unsecure_read_revoked_device_certificate(revoked_device_certificate):
    from parsec.api.data import RevokedDeviceCertificateContent, DataError

    try:
        return RevokedDeviceCertificateContent.unsecure_load(revoked_device_certificate)
    except DataError as exc:
        raise CryptoError(str(exc)) from exc


def unsecure_read_user_certificate(user_certificate):
    from parsec.api.data import UserCertificateContent, DataError

    try:
        return UserCertificateContent.unsecure_load(user_certificate)
    except DataError as exc:
        raise CryptoError(str(exc)) from exc


def build_revoked_device_certificate(certifier_id, certifier_key, revoked_device_id, timestamp):
    from parsec.api.data import RevokedDeviceCertificateContent, DataError

    try:
        return RevokedDeviceCertificateContent(
            author=certifier_id, timestamp=timestamp, device_id=revoked_device_id
        ).dump_and_sign(certifier_key)
    except DataError as exc:
        raise CryptoError(str(exc)) from exc


def build_user_certificate(certifier_id, certifier_key, user_id, public_key, is_admin, timestamp):
    from parsec.api.data import UserCertificateContent, DataError

    try:
        return UserCertificateContent(
            author=certifier_id,
            timestamp=timestamp,
            user_id=user_id,
            public_key=public_key,
            is_admin=is_admin,
        ).dump_and_sign(certifier_key)
    except DataError as exc:
        raise CryptoError(str(exc)) from exc


def verify_realm_role_certificate(realm_role_certificate, expected_author_id, author_verify_key):
    from parsec.api.data import RealmRoleCertificateContent, DataError

    try:
        return RealmRoleCertificateContent.verify_and_load(
            realm_role_certificate,
            author_verify_key=author_verify_key,
            expected_author=expected_author_id,
        )
    except DataError as exc:
        raise CryptoError(str(exc)) from exc


def unsecure_read_realm_role_certificate(realm_role_certificate):
    from parsec.api.data import RealmRoleCertificateContent, DataError

    try:
        return RealmRoleCertificateContent.unsecure_load(realm_role_certificate)
    except DataError as exc:
        raise CryptoError(str(exc)) from exc


def build_realm_role_certificate(certifier_id, certifier_key, realm_id, user_id, role, timestamp):
    from parsec.api.data import RealmRoleCertificateContent, DataError

    try:
        return RealmRoleCertificateContent(
            author=certifier_id, timestamp=timestamp, realm_id=realm_id, user_id=user_id, role=role
        ).dump_and_sign(certifier_key)
    except DataError as exc:
        raise CryptoError(str(exc)) from exc


def build_realm_self_role_certificate(self_id, self_key, realm_id, timestamp):
    from parsec.api.data import RealmRoleCertificateContent, DataError
    from parsec.api.protocol.realm import RealmRole

    try:
        return RealmRoleCertificateContent(
            author=self_id,
            timestamp=timestamp,
            realm_id=realm_id,
            user_id=self_id.user_id,
            role=RealmRole.OWNER,
        ).dump_and_sign(self_key)
    except DataError as exc:
        raise CryptoError(str(exc)) from exc


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
