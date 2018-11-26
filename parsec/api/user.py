from parsec.schema import UnknownCheckedSchema, BaseCmdSchema, fields
from parsec.api.base import UserIDField


class FindUserReqSchema(BaseCmdSchema):
    query = fields.String(missing=None, allow_none=True)
    page = fields.Int(missing=1, validate=lambda n: n > 0)
    per_page = fields.Integer(missing=100, validate=lambda n: 0 < n <= 100)


class FindUserRepSchema(UnknownCheckedSchema):
    status = fields.String(required=True)
    results = fields.List(UserIDField())
    page = fields.Int(validate=lambda n: n > 0)
    per_page = fields.Integer(validate=lambda n: 0 < n <= 100)
    total = fields.Int(validate=lambda n: n >= 0)


find_user_rep_schema = FindUserRepSchema()
find_user_req_schema = FindUserReqSchema()
