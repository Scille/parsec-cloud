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
    certified_invited_user_id = fields.Base64Bytes(required=True)
    user_id = fields.UserID(required=True)


class UserInviteRepSchema(BaseRepSchema):
    pass


user_invite_req_schema = UserInviteReqSchema()
user_invite_rep_schema = UserInviteRepSchema()
