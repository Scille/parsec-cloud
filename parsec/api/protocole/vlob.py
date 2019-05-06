# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from enum import Enum

from parsec.serde import UnknownCheckedSchema, fields, validate
from parsec.serde.fields import ValidationError
from parsec.api.protocole.base import BaseReqSchema, BaseRepSchema, CmdSerializer


__all__ = (
    "VlobGroupRole",
    "vlob_group_check_serializer",
    "vlob_create_serializer",
    "vlob_read_serializer",
    "vlob_update_serializer",
    "vlob_group_update_roles_serializer",
    "vlob_group_get_roles_serializer",
    "vlob_group_poll_serializer",
)


class VlobGroupRole(Enum):
    OWNER = "OWNER"
    MANAGER = "MANAGER"
    CONTRIBUTOR = "CONTRIBUTOR"
    READER = "READER"


class VlobGroupRoleField(fields.Field):
    def _serialize(self, value, attr, obj):
        if value is None:
            return None

        if not isinstance(value, VlobGroupRole):
            raise ValidationError("Not a VlobGroupRole")

        return value.value

    def _deserialize(self, value, attr, data):
        if value is None:
            return None

        if not isinstance(value, str):
            raise ValidationError("Not string")

        for choice in VlobGroupRole:
            if choice.value == value:
                return choice
        else:
            raise ValidationError(f"Invalid role `{value}`")


_validate_trust_seed = validate.Length(max=32)
_validate_version = validate.Range(min=1)


class CheckEntrySchema(UnknownCheckedSchema):
    id = fields.UUID(required=True)
    version = fields.Integer(required=True, validate=validate.Range(min=0))


class ChangedEntrySchema(UnknownCheckedSchema):
    id = fields.UUID(required=True)
    version = fields.Integer(required=True)


# TODO: still useful ? (replaced by vlob_group_poll)


class VlobGroupCheckReqSchema(BaseReqSchema):
    to_check = fields.List(fields.Nested(CheckEntrySchema), required=True)


class VlobGroupCheckRepSchema(BaseRepSchema):
    changed = fields.List(fields.Nested(ChangedEntrySchema), required=True)


vlob_group_check_serializer = CmdSerializer(VlobGroupCheckReqSchema, VlobGroupCheckRepSchema)


class VlobCreateReqSchema(BaseReqSchema):
    group = fields.UUID(required=True)
    id = fields.UUID(required=True)
    # If blob contains a signed message, it timestamp cannot be directly enforced
    # by the backend (given the message is probably also encrypted).
    # Hence the timestamp is passed in clear so backend can reject the message
    # if it considers the timestamp invalid. On top of that each client asking
    # for the message will receive the declared timestamp to check against
    # the actual timestamp within the message.
    timestamp = fields.DateTime(required=True)
    blob = fields.Bytes(required=True)


class VlobCreateRepSchema(BaseRepSchema):
    pass


vlob_create_serializer = CmdSerializer(VlobCreateReqSchema, VlobCreateRepSchema)


class VlobReadReqSchema(BaseReqSchema):
    id = fields.UUID(required=True)
    version = fields.Integer(validate=lambda n: n is None or _validate_version(n), missing=None)


class VlobReadRepSchema(BaseRepSchema):
    version = fields.Integer(required=True, validate=_validate_version)
    blob = fields.Bytes(required=True)
    author = fields.DeviceID(required=True)
    timestamp = fields.DateTime(required=True)


vlob_read_serializer = CmdSerializer(VlobReadReqSchema, VlobReadRepSchema)


class VlobUpdateReqSchema(BaseReqSchema):
    id = fields.UUID(required=True)
    timestamp = fields.DateTime(required=True)
    version = fields.Integer(required=True, validate=_validate_version)
    blob = fields.Bytes(required=True)


class VlobUpdateRepSchema(BaseRepSchema):
    pass


vlob_update_serializer = CmdSerializer(VlobUpdateReqSchema, VlobUpdateRepSchema)


class VlobGroupUpdateRolesReqSchema(BaseReqSchema):
    id = fields.UUID(required=True)
    user = fields.UserID(required=True)
    role = VlobGroupRoleField(allow_none=True, missing=None)


class VlobGroupUpdateRolesRepSchema(BaseRepSchema):
    pass


vlob_group_update_roles_serializer = CmdSerializer(
    VlobGroupUpdateRolesReqSchema, VlobGroupUpdateRolesRepSchema
)


class VlobGroupGetRolesReqSchema(BaseReqSchema):
    id = fields.UUID(required=True)


class VlobGroupGetRolesRepSchema(BaseRepSchema):
    users = fields.Map(fields.UserID(), VlobGroupRoleField(required=True))


vlob_group_get_roles_serializer = CmdSerializer(
    VlobGroupGetRolesReqSchema, VlobGroupGetRolesRepSchema
)


class VlobGroupPollReqSchema(BaseReqSchema):
    id = fields.UUID(required=True)
    last_checkpoint = fields.Integer(required=True)


class VlobGroupPollRepSchema(BaseRepSchema):
    changes = fields.Map(fields.UUID(), fields.Integer(), required=True)
    current_checkpoint = fields.Integer(required=True)


vlob_group_poll_serializer = CmdSerializer(VlobGroupPollReqSchema, VlobGroupPollRepSchema)
