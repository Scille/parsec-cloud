# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from typing import Optional
from uuid import UUID

from parsec.types import DeviceID, UserID
from parsec.crypto_types import VerifyKey, PublicKey
from parsec.serde import fields, post_load
from parsec.api.protocol.realm import RealmRole, RealmRoleField
from parsec.api.data.base import BaseSignedData, BaseSignedDataSchema


class UserCertificateContent(BaseSignedData):
    class SCHEMA_CLS(BaseSignedDataSchema):
        type = fields.CheckedConstant("user_certificate", required=True)
        user_id = fields.UserID(required=True)
        public_key = fields.PublicKey(required=True)
        is_admin = fields.Boolean(required=True)

        @post_load
        def make_obj(self, data):
            data.pop("type")
            return UserCertificateContent(**data)

    user_id: UserID
    public_key: PublicKey
    is_admin: bool


class DeviceCertificateContent(BaseSignedData):
    class SCHEMA_CLS(BaseSignedDataSchema):
        type = fields.CheckedConstant("device_certificate", required=True)
        device_id = fields.DeviceID(required=True)
        verify_key = fields.VerifyKey(required=True)

        @post_load
        def make_obj(self, data):
            data.pop("type")
            return DeviceCertificateContent(**data)

    device_id: DeviceID
    verify_key: VerifyKey


class RevokedDeviceCertificateContent(BaseSignedData):
    class SCHEMA_CLS(BaseSignedDataSchema):
        type = fields.CheckedConstant("revoked_device_certificate", required=True)
        device_id = fields.DeviceID(required=True)

        @post_load
        def make_obj(self, data):
            data.pop("type")
            return RevokedDeviceCertificateContent(**data)

    device_id: DeviceID


class RealmRoleCertificateContent(BaseSignedData):
    class SCHEMA_CLS(BaseSignedDataSchema):
        type = fields.CheckedConstant("realm_role_certificate", required=True)
        realm_id = fields.UUID(required=True)
        user_id = fields.UserID(required=True)
        role = RealmRoleField(allow_none=True, missing=None)

        @post_load
        def make_obj(self, data):
            data.pop("type")
            return RealmRoleCertificateContent(**data)

    realm_id: UUID
    user_id: UserID
    role: Optional[RealmRole]  # Set to None if role removed
