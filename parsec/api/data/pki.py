from typing import Optional

import attr

from parsec.api.data.base import BaseAPIData
from parsec.crypto import VerifyKey, PublicKey
from parsec.api.protocol import (
    DeviceID,
    DeviceIDField,
    HumanHandle,
    HumanHandleField,
    UserProfile,
    UserProfileField,
)
from parsec.serde import fields, BaseSchema


@attr.s(slots=True, frozen=True, auto_attribs=True, kw_only=True, eq=False)
class PkiRequest(BaseAPIData):
    class SCHEMA_CLS(BaseSchema):
        der_x509_certificate = fields.Bytes(require=True)
        signed_verify_key = fields.VerifyKey(required=True)
        signed_public_key = fields.PublicKey(required=True)
        requested_human_handle = HumanHandleField(required=True)
        requested_device_name = fields.String(required=True)

    der_x509_certificate: bytes
    signed_verify_key: VerifyKey
    signed_public_key: PublicKey
    requested_human_handle: Optional[HumanHandle]
    requested_device_name: Optional[str]


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


PkiRequestField = fields.bytes_based_field_factory(PkiRequest)
