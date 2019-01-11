from parsec.serde import UnknownCheckedSchema, fields, validate
from parsec.api.protocole.base import BaseReqSchema, BaseRepSchema, CmdSerializer


__all__ = (
    "vlob_group_check_serializer",
    "vlob_create_serializer",
    "vlob_read_serializer",
    "vlob_update_serializer",
)


_validate_trust_seed = validate.Length(max=32)
_validate_version = validate.Range(min=1)


class CheckEntrySchema(UnknownCheckedSchema):
    id = fields.UUID(required=True)
    rts = fields.String(required=True, validate=_validate_trust_seed)
    version = fields.Integer(required=True, validate=validate.Range(min=0))


class ChangedEntrySchema(UnknownCheckedSchema):
    id = fields.UUID(required=True)
    version = fields.Integer(required=True)


class VlobGroupCheckReqSchema(BaseReqSchema):
    to_check = fields.List(fields.Nested(CheckEntrySchema), required=True)


class VlobGroupCheckRepSchema(BaseRepSchema):
    changed = fields.List(fields.Nested(ChangedEntrySchema), required=True)


vlob_group_check_serializer = CmdSerializer(VlobGroupCheckReqSchema, VlobGroupCheckRepSchema)


class VlobCreateReqSchema(BaseReqSchema):
    id = fields.UUID(required=True)
    rts = fields.String(required=True, validate=_validate_trust_seed)
    wts = fields.String(required=True, validate=_validate_trust_seed)
    blob = fields.Bytes(required=True)
    notify_beacon = fields.UUID(missing=None)


class VlobCreateRepSchema(BaseRepSchema):
    pass


vlob_create_serializer = CmdSerializer(VlobCreateReqSchema, VlobCreateRepSchema)


class VlobReadReqSchema(BaseReqSchema):
    id = fields.UUID(required=True)
    rts = fields.String(required=True, validate=_validate_trust_seed)
    version = fields.Integer(validate=lambda n: n is None or _validate_version(n), missing=None)


class VlobReadRepSchema(BaseRepSchema):
    version = fields.Integer(required=True, validate=_validate_version)
    blob = fields.Bytes(required=True)


vlob_read_serializer = CmdSerializer(VlobReadReqSchema, VlobReadRepSchema)


class VlobUpdateReqSchema(BaseReqSchema):
    id = fields.UUID(required=True)
    version = fields.Integer(required=True, validate=_validate_version)
    wts = fields.String(required=True, validate=_validate_trust_seed)
    blob = fields.Bytes(required=True)
    notify_beacon = fields.UUID(missing=None)


class VlobUpdateRepSchema(BaseRepSchema):
    pass


vlob_update_serializer = CmdSerializer(VlobUpdateReqSchema, VlobUpdateRepSchema)
