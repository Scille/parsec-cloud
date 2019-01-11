from parsec.serde import UnknownCheckedSchema, fields
from parsec.api.protocole.base import BaseReqSchema, BaseRepSchema, CmdSerializer


__all__ = "beacon_read_serializer"


class BeaconReadReqSchema(BaseReqSchema):
    id = fields.UUID(required=True)
    offset = fields.Integer(required=True)


class BeaconItemSchema(UnknownCheckedSchema):
    src_id = fields.UUID(required=True)
    src_version = fields.Integer(required=True)


class BeaconReadRepSchema(BaseRepSchema):
    items = fields.List(fields.Nested(BeaconItemSchema), required=True)


beacon_read_serializer = CmdSerializer(BeaconReadReqSchema, BeaconReadRepSchema)
