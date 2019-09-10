# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from parsec.serde import BaseSchema, fields
from parsec.api.protocol.base import BaseReqSchema, BaseRepSchema, CmdSerializer
from parsec.api.protocol.types import UserIDField, DeviceNameField, DeviceIDField


__all__ = (
    "user_get_serializer",
    "user_find_serializer",
    "user_invite_serializer",
    "user_get_invitation_creator_serializer",
    "user_claim_serializer",
    "user_cancel_invitation_serializer",
    "user_create_serializer",
    "user_revoke_serializer",
    "device_invite_serializer",
    "device_get_invitation_creator_serializer",
    "device_claim_serializer",
    "device_cancel_invitation_serializer",
    "device_create_serializer",
)


#### Access user API ####


class TrustchainSchema(BaseSchema):
    devices = fields.List(fields.Bytes(required=True))
    users = fields.List(fields.Bytes(required=True))
    revoked_users = fields.List(fields.Bytes(required=True))


class UserGetReqSchema(BaseReqSchema):
    user_id = UserIDField(required=True)


class UserGetRepSchema(BaseRepSchema):
    user_certificate = fields.Bytes(required=True)
    revoked_user_certificate = fields.Bytes(required=True, allow_none=True)
    device_certificates = fields.List(fields.Bytes(required=True), required=True)

    trustchain = fields.Nested(TrustchainSchema, required=True)


user_get_serializer = CmdSerializer(UserGetReqSchema, UserGetRepSchema)


class FindUserReqSchema(BaseReqSchema):
    query = fields.String(missing=None)
    omit_revoked = fields.Boolean(missing=False)
    page = fields.Int(missing=1, validate=lambda n: n > 0)
    per_page = fields.Integer(missing=100, validate=lambda n: 0 < n <= 100)


class FindUserRepSchema(BaseRepSchema):
    results = fields.List(UserIDField())
    page = fields.Int(validate=lambda n: n > 0)
    per_page = fields.Integer(validate=lambda n: 0 < n <= 100)
    total = fields.Int(validate=lambda n: n >= 0)


user_find_serializer = CmdSerializer(FindUserReqSchema, FindUserRepSchema)


#### User creation API ####


class UserInviteReqSchema(BaseReqSchema):
    user_id = UserIDField(required=True)


class UserInviteRepSchema(BaseRepSchema):
    encrypted_claim = fields.Bytes(required=True)


user_invite_serializer = CmdSerializer(UserInviteReqSchema, UserInviteRepSchema)


class UserGetInvitationCreatorReqSchema(BaseReqSchema):
    invited_user_id = UserIDField(required=True)


class UserGetInvitationCreatorRepSchema(BaseRepSchema):
    device_certificate = fields.Bytes(required=True)
    user_certificate = fields.Bytes(required=True)
    trustchain = fields.Nested(TrustchainSchema, required=True)


user_get_invitation_creator_serializer = CmdSerializer(
    UserGetInvitationCreatorReqSchema, UserGetInvitationCreatorRepSchema
)


class UserClaimReqSchema(BaseReqSchema):
    invited_user_id = UserIDField(required=True)
    encrypted_claim = fields.Bytes(required=True)


class UserClaimRepSchema(BaseRepSchema):
    user_certificate = fields.Bytes(required=True)
    device_certificate = fields.Bytes(required=True)


user_claim_serializer = CmdSerializer(UserClaimReqSchema, UserClaimRepSchema)


class UserCancelInvitationReqSchema(BaseReqSchema):
    user_id = UserIDField(required=True)


class UserCancelInvitationRepSchema(BaseRepSchema):
    pass


user_cancel_invitation_serializer = CmdSerializer(
    UserCancelInvitationReqSchema, UserCancelInvitationRepSchema
)


class UserCreateReqSchema(BaseReqSchema):
    user_certificate = fields.Bytes(required=True)
    device_certificate = fields.Bytes(required=True)


class UserCreateRepSchema(BaseRepSchema):
    pass


user_create_serializer = CmdSerializer(UserCreateReqSchema, UserCreateRepSchema)


class UserRevokeReqSchema(BaseReqSchema):
    revoked_user_certificate = fields.Bytes(required=True)


class UserRevokeRepSchema(BaseRepSchema):
    pass


user_revoke_serializer = CmdSerializer(UserRevokeReqSchema, UserRevokeRepSchema)


#### Device creation API ####


class DeviceInviteReqSchema(BaseReqSchema):
    invited_device_name = DeviceNameField(required=True)


class DeviceInviteRepSchema(BaseRepSchema):
    encrypted_claim = fields.Bytes(required=True)


device_invite_serializer = CmdSerializer(DeviceInviteReqSchema, DeviceInviteRepSchema)


class DeviceGetInvitationCreatorReqSchema(BaseReqSchema):
    invited_device_id = DeviceIDField(required=True)


class DeviceGetInvitationCreatorRepSchema(BaseRepSchema):
    device_certificate = fields.Bytes(required=True)
    user_certificate = fields.Bytes(required=True)
    trustchain = fields.Nested(TrustchainSchema, required=True)


device_get_invitation_creator_serializer = CmdSerializer(
    DeviceGetInvitationCreatorReqSchema, DeviceGetInvitationCreatorRepSchema
)


class DeviceClaimReqSchema(BaseReqSchema):
    invited_device_id = DeviceIDField(required=True)
    encrypted_claim = fields.Bytes(required=True)


class DeviceClaimRepSchema(BaseRepSchema):
    device_certificate = fields.Bytes(required=True)
    encrypted_answer = fields.Bytes(required=True)


device_claim_serializer = CmdSerializer(DeviceClaimReqSchema, DeviceClaimRepSchema)


class DeviceCancelInvitationReqSchema(BaseReqSchema):
    invited_device_name = DeviceNameField(required=True)


class DeviceCancelInvitationRepSchema(BaseRepSchema):
    pass


device_cancel_invitation_serializer = CmdSerializer(
    DeviceCancelInvitationReqSchema, DeviceCancelInvitationRepSchema
)


class DeviceCreateReqSchema(BaseReqSchema):
    device_certificate = fields.Bytes(required=True)
    encrypted_answer = fields.Bytes(required=True)


class DeviceCreateRepSchema(BaseRepSchema):
    pass


device_create_serializer = CmdSerializer(DeviceCreateReqSchema, DeviceCreateRepSchema)
