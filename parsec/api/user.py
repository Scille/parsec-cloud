from parsec.schema import UnknownCheckedSchema, fields
from parsec.api.base import (
    BaseReqSchema,
    BaseRepSchema,
    DeviceIDField,
    UserIDField,
    DeviceNameField,
)


class UserGetReqSchema(BaseReqSchema):
    user_id = UserIDField(required=True)


class DeviceSchema(UnknownCheckedSchema):
    device_id = DeviceIDField(required=True)
    created_on = fields.DateTime(required=True)
    revocated_on = fields.DateTime(allow_none=True)
    certified_verify_key = fields.Base64Bytes(required=True)


class UserSchema(UnknownCheckedSchema):
    user_id = fields.String(required=True)
    created_on = fields.DateTime(required=True)
    revocated_on = fields.DateTime(allow_none=True)
    certified_public_key = fields.Base64Bytes(required=True)
    devices = fields.Map(DeviceNameField(), fields.Nested(DeviceSchema), required=True)


class UserGetRepSchema(BaseRepSchema, UserSchema):
    pass


get_user_req_schema = UserGetReqSchema()
get_user_rep_schema = UserGetRepSchema()


class FindUserReqSchema(BaseReqSchema):
    query = fields.String(missing=None, allow_none=True)
    page = fields.Int(missing=1, validate=lambda n: n > 0)
    per_page = fields.Integer(missing=100, validate=lambda n: 0 < n <= 100)


class FindUserRepSchema(BaseRepSchema):
    results = fields.List(UserIDField())
    page = fields.Int(validate=lambda n: n > 0)
    per_page = fields.Integer(validate=lambda n: 0 < n <= 100)
    total = fields.Int(validate=lambda n: n >= 0)


find_user_rep_schema = FindUserRepSchema()
find_user_req_schema = FindUserReqSchema()


class UserInviteReqSchema(BaseReqSchema):
    user_id = UserIDField(required=True)


class UserInviteRepSchema(BaseRepSchema):
    user_id = UserIDField(required=True)
    invalid_token = fields.String(required=True)


user_invite_req_schema = UserInviteReqSchema()
user_invite_rep_schema = UserInviteRepSchema()
