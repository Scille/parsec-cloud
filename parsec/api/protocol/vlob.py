# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

from typing import TYPE_CHECKING

from parsec.types import UUID4
from parsec.serde import BaseSchema, fields, validate
from parsec.api.protocol.base import BaseReqSchema, BaseRepSchema, CmdSerializer
from parsec.api.protocol.types import DeviceIDField
from parsec.api.protocol.realm import RealmIDField


__all__ = (
    "VlobID",
    "VlobIDField",
    "vlob_create_serializer",
    "vlob_read_serializer",
    "vlob_update_serializer",
    "vlob_poll_changes_serializer",
    "vlob_list_versions_serializer",
    "vlob_maintenance_get_reencryption_batch_serializer",
    "vlob_maintenance_save_reencryption_batch_serializer",
)


class VlobID(UUID4):
    __slots__ = ()


_PyVlobID = VlobID
if not TYPE_CHECKING:
    try:
        from libparsec.types import VlobID as _RsVlobID
    except:
        pass
    else:
        VlobID = _RsVlobID


VlobIDField = fields.uuid_based_field_factory(VlobID)


_validate_version = validate.Range(min=1)


class VlobCreateReqSchema(BaseReqSchema):
    realm_id = RealmIDField(required=True)
    encryption_revision = fields.Integer(required=True)
    vlob_id = VlobIDField(required=True)
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
    encryption_revision = fields.Integer(required=True)
    vlob_id = VlobIDField(required=True)
    version = fields.Integer(validate=lambda n: n is None or _validate_version(n), missing=None)
    timestamp = fields.DateTime(allow_none=True, missing=None)


class VlobReadRepSchema(BaseRepSchema):
    version = fields.Integer(required=True, validate=_validate_version)
    blob = fields.Bytes(required=True)
    author = DeviceIDField(required=True)
    timestamp = fields.DateTime(required=True)
    # This field is used by the client to figure out if its role certificate cache is up-to-date enough
    # to be able to perform the proper integrity checks on the manifest timestamp.
    # The `missing=None` argument is used to provide compatibilty of new clients with old backends.
    # New in API version 2.3
    author_last_role_granted_on = fields.DateTime(allow_none=True, missing=None)


vlob_read_serializer = CmdSerializer(VlobReadReqSchema, VlobReadRepSchema)


class VlobUpdateReqSchema(BaseReqSchema):
    encryption_revision = fields.Integer(required=True)
    vlob_id = VlobIDField(required=True)
    timestamp = fields.DateTime(required=True)
    version = fields.Integer(required=True, validate=_validate_version)
    blob = fields.Bytes(required=True)


class VlobUpdateRepSchema(BaseRepSchema):
    pass


vlob_update_serializer = CmdSerializer(VlobUpdateReqSchema, VlobUpdateRepSchema)


class VlobPollChangesReqSchema(BaseReqSchema):
    realm_id = RealmIDField(required=True)
    last_checkpoint = fields.Integer(required=True)


class VlobPollChangesRepSchema(BaseRepSchema):
    changes = fields.Map(VlobIDField(), fields.Integer(required=True), required=True)
    current_checkpoint = fields.Integer(required=True)


vlob_poll_changes_serializer = CmdSerializer(VlobPollChangesReqSchema, VlobPollChangesRepSchema)


# List available vlobs
class VlobListVersionsReqSchema(BaseReqSchema):
    vlob_id = VlobIDField(required=True)


class VlobListVersionsRepSchema(BaseRepSchema):
    versions = fields.Map(
        fields.Integer(required=True),
        fields.Tuple(fields.DateTime(required=True), DeviceIDField(required=True)),
        required=True,
    )


vlob_list_versions_serializer = CmdSerializer(VlobListVersionsReqSchema, VlobListVersionsRepSchema)


# Maintenance stuff


class VlobMaintenanceGetReencryptionBatchReqSchema(BaseReqSchema):
    realm_id = RealmIDField(required=True)
    encryption_revision = fields.Integer(required=True)
    size = fields.Integer(required=True, validate=validate.Range(min=0, max=1000))


class ReencryptionBatchEntrySchema(BaseSchema):
    vlob_id = VlobIDField(required=True)
    version = fields.Integer(required=True, validate=validate.Range(min=0))
    blob = fields.Bytes(required=True)


class VlobMaintenanceGetReencryptionBatchRepSchema(BaseRepSchema):
    batch = fields.List(fields.Nested(ReencryptionBatchEntrySchema), required=True)


vlob_maintenance_get_reencryption_batch_serializer = CmdSerializer(
    VlobMaintenanceGetReencryptionBatchReqSchema, VlobMaintenanceGetReencryptionBatchRepSchema
)


class VlobMaintenanceSaveReencryptionBatchReqSchema(BaseReqSchema):
    realm_id = RealmIDField(required=True)
    encryption_revision = fields.Integer(required=True)
    batch = fields.List(fields.Nested(ReencryptionBatchEntrySchema), required=True)


class VlobMaintenanceSaveReencryptionBatchRepSchema(BaseRepSchema):
    total = fields.Integer(required=True)
    done = fields.Integer(required=True)


vlob_maintenance_save_reencryption_batch_serializer = CmdSerializer(
    VlobMaintenanceSaveReencryptionBatchReqSchema, VlobMaintenanceSaveReencryptionBatchRepSchema
)
