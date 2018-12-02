from parsec.schema import UnknownCheckedSchema, fields
from parsec.api.base import BaseReqSchema, BaseRepSchema


class UserGetReqSchema(BaseReqSchema):
    user_id = fields.UserID(required=True)


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


class UserInviteReqSchema(BaseReqSchema):
    certified_invitation = fields.Base64Bytes(required=True)


class UserInviteRepSchema(BaseRepSchema):
    user_id = fields.UserID(required=True)


user_invite_req_schema = UserInviteReqSchema()
user_invite_rep_schema = UserInviteRepSchema()


class UserGetInvitationCreatorReqSchema(BaseReqSchema):
    invited_user_id = fields.UserID(required=True)


class UserGetInvitationCreatorRepSchema(BaseRepSchema, DeviceSchema):
    pass


user_get_invitation_creator_req_schema = UserGetInvitationCreatorReqSchema()
user_get_invitation_creator_rep_schema = UserGetInvitationCreatorRepSchema()


class UserClaimInvitationReqSchema(BaseReqSchema):
    invited_user_id = fields.UserID(required=True)
    encrypted_claim = fields.Base64Bytes(required=True)


class UserClaimInvitationRepSchema(BaseRepSchema):
    pass


user_claim_invitation_req_schema = UserClaimInvitationReqSchema()
user_claim_invitation_rep_schema = UserClaimInvitationRepSchema()


class UserResolveInvitationClaimReqSchema(BaseReqSchema):
    user_id = fields.UserID(required=True)
    outcome = fields.UserID(required=True)  # TODO: use oneof field ?
    certified_user = fields.Base64Bytes()
    certified_device = fields.Base64Bytes()


class UserResolveInvitationClaimRepSchema(BaseRepSchema):
    pass


user_resolve_invitation_claim_req_schema = UserResolveInvitationClaimReqSchema()
user_resolve_invitation_claim_rep_schema = UserResolveInvitationClaimRepSchema()
