# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from parsec.serde import BaseSchema, fields
from parsec.api.protocol.base import BaseReqSchema, BaseRepSchema, CmdSerializer
from parsec.api.protocol.types import UserIDField, HumanHandleField


__all__ = (
    "user_get_serializer",
    "user_create_serializer",
    "user_revoke_serializer",
    "device_create_serializer",
    "human_find_serializer",
)


#### Access user API ####


class TrustchainSchema(BaseSchema):
    devices = fields.List(fields.Bytes(required=True))  # type: ignore[arg-type]
    users = fields.List(fields.Bytes(required=True))  # type: ignore[arg-type]
    revoked_users = fields.List(fields.Bytes(required=True))  # type: ignore[arg-type]


class UserGetReqSchema(BaseReqSchema):
    user_id = UserIDField(required=True)  # type: ignore[arg-type]


class UserGetRepSchema(BaseRepSchema):
    user_certificate = fields.Bytes(required=True)
    revoked_user_certificate = fields.Bytes(required=True, allow_none=True)  # type: ignore[arg-type]
    device_certificates = fields.List(fields.Bytes(required=True), required=True)  # type: ignore[arg-type]

    trustchain = fields.Nested(TrustchainSchema, required=True)


user_get_serializer = CmdSerializer(UserGetReqSchema, UserGetRepSchema)


#### User creation API ####


class UserCreateReqSchema(BaseReqSchema):
    user_certificate = fields.Bytes(required=True)
    device_certificate = fields.Bytes(required=True)
    # Same certificates than above, but expurged of human_handle/device_label
    redacted_user_certificate = fields.Bytes(required=True)
    redacted_device_certificate = fields.Bytes(required=True)


class UserCreateRepSchema(BaseRepSchema):
    pass


user_create_serializer = CmdSerializer(UserCreateReqSchema, UserCreateRepSchema)


class UserRevokeReqSchema(BaseReqSchema):
    revoked_user_certificate = fields.Bytes(required=True)


class UserRevokeRepSchema(BaseRepSchema):
    pass


user_revoke_serializer = CmdSerializer(UserRevokeReqSchema, UserRevokeRepSchema)


#### Device creation API ####


class DeviceCreateReqSchema(BaseReqSchema):
    device_certificate = fields.Bytes(required=True)
    # Same certificate than above, but expurged of device_label
    redacted_device_certificate = fields.Bytes(required=True)


class DeviceCreateRepSchema(BaseRepSchema):
    pass


device_create_serializer = CmdSerializer(DeviceCreateReqSchema, DeviceCreateRepSchema)


# Human search API


class HumanFindReqSchema(BaseReqSchema):
    query = fields.String(required=True, allow_none=True)
    omit_revoked = fields.Boolean(required=True)
    omit_non_human = fields.Boolean(required=True)
    # First page is 1
    page = fields.Int(required=True, validate=lambda n: n > 0)
    per_page = fields.Integer(required=True, validate=lambda n: 0 < n <= 100)


class HumanFindResultItemSchema(BaseSchema):
    user_id = UserIDField(required=True)  # type: ignore[arg-type]
    human_handle = HumanHandleField(required=True, allow_none=True)
    revoked = fields.Boolean(required=True)


class HumanFindRepSchema(BaseRepSchema):
    results = fields.List(fields.Nested(HumanFindResultItemSchema, required=True))
    page = fields.Int(required=True, validate=lambda n: n > 0)
    per_page = fields.Integer(required=True, validate=lambda n: 0 < n <= 100)
    total = fields.Int(required=True, validate=lambda n: n >= 0)


human_find_serializer = CmdSerializer(HumanFindReqSchema, HumanFindRepSchema)
