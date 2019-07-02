# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import attr
from uuid import UUID
from typing import Optional
from pendulum import Pendulum

from parsec.types import DeviceID, UserID
from parsec.api.protocole.realm import RealmRole, RealmRoleField
from parsec.serde import Serializer, UnknownCheckedSchema, fields
from parsec.crypto_types import SigningKey, VerifyKey, PublicKey
from parsec.crypto.exceptions import CryptoWrappedMsgPackingError, CryptoWrappedMsgValidationError
from parsec.crypto.signed import (
    build_signed_msg,
    unsecure_extract_signed_msg_meta_and_data,
    unsecure_extract_signed_msg_meta,
    verify_signed_msg,
)


# TODO: make certif schemas inherit from WrappedMsgSchema instead ?


class CertifiedDeviceSchema(UnknownCheckedSchema):
    type = fields.CheckedConstant("device", required=True)
    device_id = fields.DeviceID(required=True)
    verify_key = fields.VerifyKey(required=True)


class CertifiedUserSchema(UnknownCheckedSchema):
    type = fields.CheckedConstant("user", required=True)
    user_id = fields.UserID(required=True)
    public_key = fields.PublicKey(required=True)
    is_admin = fields.Boolean(required=True)


class CertifiedDeviceRevokedSchema(UnknownCheckedSchema):
    type = fields.CheckedConstant("device_revoked", required=True)
    device_id = fields.DeviceID(required=True)


class CertifiedRealmRoleSchema(UnknownCheckedSchema):
    type = fields.CheckedConstant("user", required=True)
    realm_id = fields.UUID(required=True)
    user_id = fields.UserID(required=True)
    role = RealmRoleField(allow_none=True, missing=None)


device_certificate_schema = Serializer(
    CertifiedDeviceSchema,
    validation_exc=CryptoWrappedMsgValidationError,
    packing_exc=CryptoWrappedMsgPackingError,
)
user_certificate_schema = Serializer(
    CertifiedUserSchema,
    validation_exc=CryptoWrappedMsgValidationError,
    packing_exc=CryptoWrappedMsgPackingError,
)
revoked_device_certificate_schema = Serializer(
    CertifiedDeviceRevokedSchema,
    validation_exc=CryptoWrappedMsgValidationError,
    packing_exc=CryptoWrappedMsgPackingError,
)
realm_role_certificate_schema = Serializer(
    CertifiedRealmRoleSchema,
    validation_exc=CryptoWrappedMsgValidationError,
    packing_exc=CryptoWrappedMsgPackingError,
)


@attr.s(slots=True, frozen=True, auto_attribs=True)
class CertifiedDeviceData:
    device_id: DeviceID
    verify_key: VerifyKey
    certified_by: DeviceID
    certified_on: Pendulum


@attr.s(slots=True, frozen=True, auto_attribs=True)
class CertifiedRevokedDeviceData:
    device_id: DeviceID
    certified_by: DeviceID
    certified_on: Pendulum


@attr.s(slots=True, frozen=True, auto_attribs=True)
class CertifiedUserData:
    user_id: DeviceID
    public_key: VerifyKey
    is_admin: bool
    certified_by: DeviceID
    certified_on: Pendulum


@attr.s(slots=True, frozen=True, auto_attribs=True)
class CertifiedRealmRoleData:
    realm_id: UUID
    user_id: UserID
    role: Optional[RealmRole]
    certified_by: DeviceID
    certified_on: Pendulum


def verify_device_certificate(
    device_certificate: bytes, expected_author_id: DeviceID, author_verify_key: VerifyKey
) -> CertifiedDeviceData:
    """
    Raises:
        CryptoError: if signature was forged or otherwise corrupt.
        CryptoWrappedMsgValidationError
        CryptoWrappedMsgPackingError
        CryptoSignatureAuthorMismatchError
        CryptoSignatureTimestampMismatchError
    """
    _, timestamp = unsecure_extract_signed_msg_meta(device_certificate)
    content = verify_signed_msg(
        device_certificate, expected_author_id, author_verify_key, timestamp
    )
    data = device_certificate_schema.loads(content)
    return CertifiedDeviceData(data["device_id"], data["verify_key"], expected_author_id, timestamp)


def verify_revoked_device_certificate(
    revoked_device_certificate: bytes, expected_author_id: DeviceID, author_verify_key: VerifyKey
) -> CertifiedRevokedDeviceData:
    """
    Raises:
        CryptoError: if signature was forged or otherwise corrupt.
        CryptoWrappedMsgValidationError
        CryptoWrappedMsgPackingError
        CryptoSignatureAuthorMismatchError
        CryptoSignatureTimestampMismatchError
    """
    _, timestamp = unsecure_extract_signed_msg_meta(revoked_device_certificate)
    content = verify_signed_msg(
        revoked_device_certificate, expected_author_id, author_verify_key, timestamp
    )
    data = revoked_device_certificate_schema.loads(content)
    return CertifiedRevokedDeviceData(data["device_id"], expected_author_id, timestamp)


def verify_user_certificate(
    user_certificate: bytes, expected_author_id: DeviceID, author_verify_key: VerifyKey
) -> CertifiedUserData:
    """
    Raises:
        CryptoError: if signature was forged or otherwise corrupt.
        CryptoWrappedMsgValidationError
        CryptoWrappedMsgPackingError
        CryptoSignatureAuthorMismatchError
        CryptoSignatureTimestampMismatchError
    """
    _, timestamp = unsecure_extract_signed_msg_meta(user_certificate)
    content = verify_signed_msg(user_certificate, expected_author_id, author_verify_key, timestamp)
    data = user_certificate_schema.loads(content)
    return CertifiedUserData(
        data["user_id"], data["public_key"], data["is_admin"], expected_author_id, timestamp
    )


def verify_realm_role_certificate(
    realm_role_certificate: bytes, expected_author_id: DeviceID, author_verify_key: VerifyKey
) -> CertifiedRealmRoleData:
    """
    Raises:
        CryptoError: if signature was forged or otherwise corrupt.
        CryptoWrappedMsgValidationError
        CryptoWrappedMsgPackingError
        CryptoSignatureAuthorMismatchError
        CryptoSignatureTimestampMismatchError
    """
    _, timestamp = unsecure_extract_signed_msg_meta(realm_role_certificate)
    content = verify_signed_msg(
        realm_role_certificate, expected_author_id, author_verify_key, timestamp
    )
    data = realm_role_certificate_schema.loads(content)
    return CertifiedRealmRoleData(
        data["realm_id"], data["user_id"], data["role"], expected_author_id, timestamp
    )


def unsecure_read_device_certificate(device_certificate: bytes) -> CertifiedDeviceData:
    """
    Raises:
        CryptoWrappedMsgValidationError
        CryptoWrappedMsgPackingError
    """
    certified_by, certified_on, content = unsecure_extract_signed_msg_meta_and_data(
        device_certificate
    )
    data = device_certificate_schema.loads(content)
    return CertifiedDeviceData(
        device_id=data["device_id"],
        verify_key=data["verify_key"],
        certified_by=certified_by,
        certified_on=certified_on,
    )


def unsecure_read_revoked_device_certificate(
    device_certificate: bytes
) -> CertifiedRevokedDeviceData:
    """
    Raises:
        CryptoWrappedMsgValidationError
        CryptoWrappedMsgPackingError
    """
    certified_by, certified_on, content = unsecure_extract_signed_msg_meta_and_data(
        device_certificate
    )
    data = revoked_device_certificate_schema.loads(content)
    return CertifiedRevokedDeviceData(
        device_id=data["device_id"], certified_by=certified_by, certified_on=certified_on
    )


def unsecure_read_user_certificate(user_certificate: bytes) -> CertifiedUserData:
    """
    Raises:
        CryptoWrappedMsgValidationError
        CryptoWrappedMsgPackingError
    """
    certified_by, certified_on, content = unsecure_extract_signed_msg_meta_and_data(
        user_certificate
    )
    data = user_certificate_schema.loads(content)
    return CertifiedUserData(
        user_id=data["user_id"],
        public_key=data["public_key"],
        is_admin=data["is_admin"],
        certified_by=certified_by,
        certified_on=certified_on,
    )


def unsecure_read_realm_role_certificate(realm_role_certificate: bytes) -> CertifiedRealmRoleData:
    """
    Raises:
        CryptoWrappedMsgValidationError
        CryptoWrappedMsgPackingError
    """
    certified_by, certified_on, content = unsecure_extract_signed_msg_meta_and_data(
        realm_role_certificate
    )
    data = realm_role_certificate_schema.loads(content)
    return CertifiedRealmRoleData(
        realm_id=data["realm_id"],
        user_id=data["user_id"],
        role=data["role"],
        certified_by=certified_by,
        certified_on=certified_on,
    )


def build_device_certificate(
    certifier_id: Optional[DeviceID],
    certifier_key: SigningKey,
    device_id: DeviceID,
    verify_key: VerifyKey,
    timestamp: Pendulum,
) -> bytes:
    """
    Raises:
        CryptoError: if the signature operation fails.
        CryptoWrappedMsgValidationError
        CryptoWrappedMsgPackingError
    """
    content = device_certificate_schema.dumps(
        {"type": "device", "device_id": device_id, "verify_key": verify_key}
    )
    return build_signed_msg(certifier_id, certifier_key, content, timestamp)


def build_revoked_device_certificate(
    certifier_id: DeviceID,
    certifier_key: SigningKey,
    revoked_device_id: DeviceID,
    timestamp: Pendulum,
) -> bytes:
    """
    Raises:
        CryptoError: if the signature operation fails.
        CryptoWrappedMsgValidationError
        CryptoWrappedMsgPackingError
    """
    content = revoked_device_certificate_schema.dumps(
        {"type": "device_revoked", "device_id": revoked_device_id}
    )
    return build_signed_msg(certifier_id, certifier_key, content, timestamp)


def build_user_certificate(
    certifier_id: Optional[DeviceID],
    certifier_key: SigningKey,
    user_id: UserID,
    public_key: PublicKey,
    is_admin: bool,
    timestamp: Pendulum,
) -> bytes:
    """
    Raises:
        CryptoError: if the signature operation fails.
        CryptoWrappedMsgValidationError
        CryptoWrappedMsgPackingError
    """
    content = user_certificate_schema.dumps(
        {"type": "user", "user_id": user_id, "public_key": public_key, "is_admin": is_admin}
    )
    return build_signed_msg(certifier_id, certifier_key, content, timestamp)


def build_realm_role_certificate(
    certifier_id: DeviceID,
    certifier_key: SigningKey,
    realm_id: UUID,
    user_id: UserID,
    role: RealmRole,
    timestamp: Pendulum,
) -> bytes:
    """
    Raises:
        CryptoError: if the signature operation fails.
        CryptoWrappedMsgValidationError
        CryptoWrappedMsgPackingError
    """
    content = realm_role_certificate_schema.dumps(
        {"type": "user", "realm_id": realm_id, "user_id": user_id, "role": role}
    )
    return build_signed_msg(certifier_id, certifier_key, content, timestamp)


def build_realm_self_role_certificate(
    self_id: DeviceID, self_key: SigningKey, realm_id: UUID, timestamp: Pendulum
) -> bytes:
    """
    Raises:
        CryptoError: if the signature operation fails.
        CryptoWrappedMsgValidationError
        CryptoWrappedMsgPackingError
    """
    return build_realm_role_certificate(
        certifier_id=self_id,
        certifier_key=self_key,
        realm_id=realm_id,
        user_id=self_id.user_id,
        role=RealmRole.OWNER,
        timestamp=timestamp,
    )
