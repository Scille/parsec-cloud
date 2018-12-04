from parsec.schema import UnknownCheckedSchema, fields
from parsec.api.base import BaseReqSchema, BaseRepSchema


class DeviceSchema(UnknownCheckedSchema):
    device_id = fields.DeviceID(required=True)
    created_on = fields.DateTime(required=True)

    revocated_on = fields.DateTime(allow_none=True)
    certified_revocation = fields.Base64Bytes(allow_none=True)
    revocation_certifier = fields.Nested("DeviceSchema", allow_none=True)

    certified_device = fields.Base64Bytes(required=True)
    device_certifier = fields.Nested("DeviceSchema", allow_none=True)


class UserSchema(UnknownCheckedSchema):
    user_id = fields.String(required=True)
    created_on = fields.DateTime(required=True)

    revocated_on = fields.DateTime(allow_none=True)
    certified_revocation = fields.Base64Bytes(allow_none=True)
    revocation_certifier = fields.Nested(DeviceSchema, allow_none=True)

    certified_user = fields.Base64Bytes(required=True)
    user_certifier = fields.Nested(DeviceSchema, allow_none=True)

    devices = fields.Map(fields.DeviceName(), fields.Nested(DeviceSchema), required=True)


#### Access user API ####


class UserGetReqSchema(BaseReqSchema):
    user_id = fields.UserID(required=True)


class UserGetRepSchema(BaseRepSchema, UserSchema):
    pass


user_get_req_schema = UserGetReqSchema()
user_get_rep_schema = UserGetRepSchema()


class FindUserReqSchema(BaseReqSchema):
    query = fields.String(missing=None, allow_none=True)
    page = fields.Int(missing=1, validate=lambda n: n > 0)
    per_page = fields.Integer(missing=100, validate=lambda n: 0 < n <= 100)


class FindUserRepSchema(BaseRepSchema):
    results = fields.List(fields.UserID())
    page = fields.Int(validate=lambda n: n > 0)
    per_page = fields.Integer(validate=lambda n: 0 < n <= 100)
    total = fields.Int(validate=lambda n: n >= 0)


user_find_rep_schema = FindUserRepSchema()
user_find_req_schema = FindUserReqSchema()


#### User creation API ####


class UserInviteReqSchema(BaseReqSchema):
    user_id = fields.UserID(required=True)


class UserInviteRepSchema(BaseRepSchema):
    encrypted_claim = fields.Base64Bytes(required=True)


user_invite_req_schema = UserInviteReqSchema()
user_invite_rep_schema = UserInviteRepSchema()


class UserGetInvitationCreatorReqSchema(BaseReqSchema):
    invited_user_id = fields.UserID(required=True)


class UserGetInvitationCreatorRepSchema(BaseRepSchema, DeviceSchema):
    pass


user_get_invitation_creator_req_schema = UserGetInvitationCreatorReqSchema()
user_get_invitation_creator_rep_schema = UserGetInvitationCreatorRepSchema()


class UserClaimReqSchema(BaseReqSchema):
    invited_user_id = fields.UserID(required=True)
    encrypted_claim = fields.Base64Bytes(required=True)


class UserClaimRepSchema(BaseRepSchema):
    pass


user_claim_req_schema = UserClaimReqSchema()
user_claim_rep_schema = UserClaimRepSchema()


class UserCancelInvitationReqSchema(BaseReqSchema):
    user_id = fields.UserID(required=True)


class UserCancelInvitationRepSchema(BaseRepSchema):
    pass


user_cancel_invitation_req_schema = UserCancelInvitationReqSchema()
user_cancel_invitation_rep_schema = UserCancelInvitationRepSchema()


class UserCreateReqSchema(BaseReqSchema):
    certified_user = fields.Base64Bytes(required=True)
    certified_device = fields.Base64Bytes(required=True)


class UserCreateRepSchema(BaseRepSchema):
    pass


user_create_req_schema = UserCreateReqSchema()
user_create_rep_schema = UserCreateRepSchema()


class UserRevokeReqSchema(BaseReqSchema):
    certified_revocation = fields.Base64Bytes(required=True)


class UserRevokeRepSchema(BaseRepSchema):
    pass


user_revoke_req_schema = UserRevokeReqSchema()
user_revoke_rep_schema = UserRevokeRepSchema()


#### Device creation API ####


class DeviceInviteReqSchema(BaseReqSchema):
    device_id = fields.DeviceID(required=True)


class DeviceInviteRepSchema(BaseRepSchema):
    encrypted_claim = fields.Base64Bytes(required=True)


device_invite_req_schema = DeviceInviteReqSchema()
device_invite_rep_schema = DeviceInviteRepSchema()


class DeviceGetInvitationCreatorReqSchema(BaseReqSchema):
    invited_device_id = fields.DeviceID(required=True)


class DeviceGetInvitationCreatorRepSchema(BaseRepSchema, DeviceSchema):
    pass


device_get_invitation_creator_req_schema = DeviceGetInvitationCreatorReqSchema()
device_get_invitation_creator_rep_schema = DeviceGetInvitationCreatorRepSchema()


class DeviceClaimReqSchema(BaseReqSchema):
    invited_device_id = fields.DeviceID(required=True)
    encrypted_claim = fields.Base64Bytes(required=True)


class DeviceClaimRepSchema(BaseRepSchema):
    encrypted_answer = fields.Base64Bytes(required=True)


device_claim_req_schema = DeviceClaimReqSchema()
device_claim_rep_schema = DeviceClaimRepSchema()


class DeviceCancelInvitationReqSchema(BaseReqSchema):
    device_id = fields.DeviceID(required=True)


class DeviceCancelInvitationRepSchema(BaseRepSchema):
    pass


device_cancel_invitation_req_schema = DeviceCancelInvitationReqSchema()
device_cancel_invitation_rep_schema = DeviceCancelInvitationRepSchema()


class DeviceCreateReqSchema(BaseReqSchema):
    certified_device = fields.Base64Bytes(required=True)
    encrypted_answer = fields.Base64Bytes(required=True)


class DeviceCreateRepSchema(BaseRepSchema):
    pass


device_create_req_schema = DeviceCreateReqSchema()
device_create_rep_schema = DeviceCreateRepSchema()


class DeviceRevokeReqSchema(BaseReqSchema):
    certified_revocation = fields.Base64Bytes(required=True)


class DeviceRevokeRepSchema(BaseRepSchema):
    pass


device_revoke_req_schema = DeviceRevokeReqSchema()
device_revoke_rep_schema = DeviceRevokeRepSchema()
