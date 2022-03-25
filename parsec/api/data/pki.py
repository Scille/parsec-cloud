# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import attr

from parsec.api.data.base import BaseAPIData
from parsec.api.protocol import (
    DeviceID,
    DeviceIDField,
    HumanHandle,
    HumanHandleField,
    UserProfile,
    UserProfileField,
)
from parsec.crypto import PublicKey, VerifyKey
from parsec.serde import BaseSchema, fields


@attr.s(slots=True, frozen=True, auto_attribs=True, kw_only=True, eq=False)
class PkiRequest(BaseAPIData):
    class SCHEMA_CLS(BaseSchema):
        der_x509_certificate = fields.Bytes(require=True)
        verify_key = fields.VerifyKey(required=True)
        public_key = fields.PublicKey(required=True)
        signature = fields.Bytes(required=True)
        requested_human_handle = HumanHandleField(required=True)
        requested_device_name = fields.String(required=True)

    der_x509_certificate: bytes
    verify_key: VerifyKey
    public_key: PublicKey
    signature: bytes
    requested_human_handle: HumanHandle
    requested_device_name: str


@attr.s(slots=True, frozen=True, auto_attribs=True, kw_only=True, eq=False)
class PkiReply(BaseAPIData):
    class SCHEMA_CLS(BaseSchema):
        der_x509_admin_certificate = fields.Bytes(required=True)
        device_id = DeviceIDField(required=True)
        root_verify_key = fields.VerifyKey(required=True)
        device_label = fields.String(required=True)
        human_handle = HumanHandleField(required=True)
        profile = UserProfileField(required=True)

    der_x509_admin_certificate: bytes
    device_id: DeviceID
    root_verify_key: VerifyKey
    device_label: str
    human_handle: HumanHandle
    profile: UserProfile
