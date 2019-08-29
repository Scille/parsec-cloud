# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from typing import Optional
from uuid import UUID

from parsec.crypto_types import VerifyKey, PublicKey
from parsec.serde import fields, post_load
from parsec.api.protocol import (
    DeviceID,
    UserID,
    RealmRole,
    DeviceIDField,
    UserIDField,
    RealmRoleField,
)
from parsec.api.data.base import DataValidationError, BaseSignedData, BaseSignedDataSchema


class UserCertificateContent(BaseSignedData):
    class SCHEMA_CLS(BaseSignedDataSchema):
        type = fields.CheckedConstant("user_certificate", required=True)
        user_id = UserIDField(required=True)
        public_key = fields.PublicKey(required=True)
        is_admin = fields.Boolean(required=True)

        @post_load
        def make_obj(self, data):
            data.pop("type")
            return UserCertificateContent(**data)

    @classmethod
    def verify_and_load(
        cls, *args, expected_user: Optional[UserID] = None, **kwargs
    ) -> BaseSignedData:
        data = super().verify_and_load(*args, **kwargs)
        if expected_user is not None and data.user_id != expected_user:
            raise DataValidationError(
                f"Invalid user ID: expected `{expected_user}`, got `{data.user_id}`"
            )
        return data

    user_id: UserID
    public_key: PublicKey
    is_admin: bool


class DeviceCertificateContent(BaseSignedData):
    class SCHEMA_CLS(BaseSignedDataSchema):
        type = fields.CheckedConstant("device_certificate", required=True)
        device_id = DeviceIDField(required=True)
        verify_key = fields.VerifyKey(required=True)

        @post_load
        def make_obj(self, data):
            data.pop("type")
            return DeviceCertificateContent(**data)

    @classmethod
    def verify_and_load(
        cls, *args, expected_device: Optional[DeviceID] = None, **kwargs
    ) -> BaseSignedData:
        data = super().verify_and_load(*args, **kwargs)
        if expected_device is not None and data.device_id != expected_device:
            raise DataValidationError(
                f"Invalid device ID: expected `{expected_device}`, got `{data.device_id}`"
            )
        return data

    device_id: DeviceID
    verify_key: VerifyKey


class RevokedDeviceCertificateContent(BaseSignedData):
    class SCHEMA_CLS(BaseSignedDataSchema):
        type = fields.CheckedConstant("revoked_device_certificate", required=True)
        device_id = DeviceIDField(required=True)

        @post_load
        def make_obj(self, data):
            data.pop("type")
            return RevokedDeviceCertificateContent(**data)

    device_id: DeviceID


class RealmRoleCertificateContent(BaseSignedData):
    class SCHEMA_CLS(BaseSignedDataSchema):
        type = fields.CheckedConstant("realm_role_certificate", required=True)
        realm_id = fields.UUID(required=True)
        user_id = UserIDField(required=True)
        role = RealmRoleField(required=True, allow_none=True)

        @post_load
        def make_obj(self, data):
            data.pop("type")
            return RealmRoleCertificateContent(**data)

    realm_id: UUID
    user_id: UserID
    role: Optional[RealmRole]  # Set to None if role removed

    @classmethod
    def build_realm_root_certif(cls, author, timestamp, realm_id):
        return cls(
            author=author,
            timestamp=timestamp,
            realm_id=realm_id,
            user_id=author.user_id,
            role=RealmRole.OWNER,
        )
