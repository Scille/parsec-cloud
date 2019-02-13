# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from parsec.serde import UnknownCheckedSchema, fields
from parsec.api.protocole.base import BaseReqSchema, BaseRepSchema, CmdSerializer


__all__ = (
    "user_get_serializer",
    "user_find_serializer",
    "user_invite_serializer",
    "user_get_invitation_creator_serializer",
    "user_claim_serializer",
    "user_cancel_invitation_serializer",
    "user_create_serializer",
    "device_invite_serializer",
    "device_get_invitation_creator_serializer",
    "device_claim_serializer",
    "device_cancel_invitation_serializer",
    "device_create_serializer",
    "device_revoke_serializer",
)


class DeviceSchema(UnknownCheckedSchema):
    device_id = fields.DeviceID(required=True)
    created_on = fields.DateTime(required=True)

    revocated_on = fields.DateTime(allow_none=True)
    certified_revocation = fields.Bytes(allow_none=True)
    revocation_certifier = fields.DeviceID(allow_none=True)

    certified_device = fields.Bytes(required=True)
    device_certifier = fields.DeviceID(allow_none=True)


class UserSchema(UnknownCheckedSchema):
    user_id = fields.String(required=True)
    is_admin = fields.Boolean(required=True)
    created_on = fields.DateTime(required=True)

    certified_user = fields.Bytes(required=True)
    user_certifier = fields.DeviceID(allow_none=True)

    devices = fields.Map(fields.DeviceName(), fields.Nested(DeviceSchema), required=True)


#### Access user API ####


class UserGetReqSchema(BaseReqSchema):
    user_id = fields.UserID(required=True)


class UserGetRepSchema(BaseRepSchema, UserSchema):
    trustchain = fields.Map(fields.DeviceID(), fields.Nested(DeviceSchema), required=True)


user_get_serializer = CmdSerializer(UserGetReqSchema, UserGetRepSchema)


class FindUserReqSchema(BaseReqSchema):
    query = fields.String(missing=None, allow_none=True)
    page = fields.Int(missing=1, validate=lambda n: n > 0)
    per_page = fields.Integer(missing=100, validate=lambda n: 0 < n <= 100)


class FindUserRepSchema(BaseRepSchema):
    results = fields.List(fields.UserID())
    page = fields.Int(validate=lambda n: n > 0)
    per_page = fields.Integer(validate=lambda n: 0 < n <= 100)
    total = fields.Int(validate=lambda n: n >= 0)


user_find_serializer = CmdSerializer(FindUserReqSchema, FindUserRepSchema)


#### User creation API ####


class UserInviteReqSchema(BaseReqSchema):
    user_id = fields.UserID(required=True)


class UserInviteRepSchema(BaseRepSchema):
    encrypted_claim = fields.Bytes(required=True)


user_invite_serializer = CmdSerializer(UserInviteReqSchema, UserInviteRepSchema)


class UserGetInvitationCreatorReqSchema(BaseReqSchema):
    invited_user_id = fields.UserID(required=True)


class UserGetInvitationCreatorRepSchema(BaseRepSchema):
    user_id = fields.String(required=True)
    created_on = fields.DateTime(required=True)

    certified_user = fields.Bytes(required=True)
    user_certifier = fields.DeviceID(allow_none=True)


user_get_invitation_creator_serializer = CmdSerializer(
    UserGetInvitationCreatorReqSchema, UserGetInvitationCreatorRepSchema
)


class UserClaimReqSchema(BaseReqSchema):
    invited_user_id = fields.UserID(required=True)
    encrypted_claim = fields.Bytes(required=True)


class UserClaimRepSchema(BaseRepSchema):
    pass


user_claim_serializer = CmdSerializer(UserClaimReqSchema, UserClaimRepSchema)


class UserCancelInvitationReqSchema(BaseReqSchema):
    user_id = fields.UserID(required=True)


class UserCancelInvitationRepSchema(BaseRepSchema):
    pass


user_cancel_invitation_serializer = CmdSerializer(
    UserCancelInvitationReqSchema, UserCancelInvitationRepSchema
)


class UserCreateReqSchema(BaseReqSchema):
    certified_user = fields.Bytes(required=True)
    certified_device = fields.Bytes(required=True)
    is_admin = fields.Boolean(missing=False)


class UserCreateRepSchema(BaseRepSchema):
    pass


user_create_serializer = CmdSerializer(UserCreateReqSchema, UserCreateRepSchema)


#### Device creation API ####


class DeviceInviteReqSchema(BaseReqSchema):
    invited_device_name = fields.DeviceName(required=True)


class DeviceInviteRepSchema(BaseRepSchema):
    encrypted_claim = fields.Bytes(required=True)


device_invite_serializer = CmdSerializer(DeviceInviteReqSchema, DeviceInviteRepSchema)


class DeviceGetInvitationCreatorReqSchema(BaseReqSchema):
    invited_device_id = fields.DeviceID(required=True)


class DeviceGetInvitationCreatorRepSchema(BaseRepSchema):
    user_id = fields.String(required=True)
    created_on = fields.DateTime(required=True)

    certified_user = fields.Bytes(required=True)
    user_certifier = fields.DeviceID(allow_none=True)


device_get_invitation_creator_serializer = CmdSerializer(
    DeviceGetInvitationCreatorReqSchema, DeviceGetInvitationCreatorRepSchema
)


class DeviceClaimReqSchema(BaseReqSchema):
    invited_device_id = fields.DeviceID(required=True)
    encrypted_claim = fields.Bytes(required=True)


class DeviceClaimRepSchema(BaseRepSchema):
    encrypted_answer = fields.Bytes(required=True)


device_claim_serializer = CmdSerializer(DeviceClaimReqSchema, DeviceClaimRepSchema)


class DeviceCancelInvitationReqSchema(BaseReqSchema):
    invited_device_name = fields.DeviceName(required=True)


class DeviceCancelInvitationRepSchema(BaseRepSchema):
    pass


device_cancel_invitation_serializer = CmdSerializer(
    DeviceCancelInvitationReqSchema, DeviceCancelInvitationRepSchema
)


class DeviceCreateReqSchema(BaseReqSchema):
    certified_device = fields.Bytes(required=True)
    encrypted_answer = fields.Bytes(required=True)


class DeviceCreateRepSchema(BaseRepSchema):
    pass


device_create_serializer = CmdSerializer(DeviceCreateReqSchema, DeviceCreateRepSchema)


class DeviceRevokeReqSchema(BaseReqSchema):
    certified_revocation = fields.Bytes(required=True)


class DeviceRevokeRepSchema(BaseRepSchema):
    pass


device_revoke_serializer = CmdSerializer(DeviceRevokeReqSchema, DeviceRevokeRepSchema)
