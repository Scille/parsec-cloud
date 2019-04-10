# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from parsec.serde import Serializer, UnknownCheckedSchema, OneOfSchema, fields

from parsec.core.fs.exceptions import FSValidationError, FSPackingError


class SharingGrantedMessageContentSchema(UnknownCheckedSchema):
    type = fields.CheckedConstant("sharing.granted", required=True)
    name = fields.String(required=True)
    id = fields.UUID(required=True)
    key = fields.SymetricKey(missing=None)
    # Don't include access rights given the receiver will have anyway to
    # interrogate the backend for workspace's access rights list
    # to make sure the message sender is an admin


class SharingRevokedMessageContentSchema(UnknownCheckedSchema):
    type = fields.CheckedConstant("sharing.revoked", required=True)
    id = fields.UUID(required=True)


class PingMessageContentSchema(UnknownCheckedSchema):
    type = fields.CheckedConstant("ping", required=True)
    ping = fields.String(required=True)


class MessageContentSchema(OneOfSchema):
    type_field = "type"
    type_field_remove = False
    type_schemas = {
        "sharing.granted": SharingGrantedMessageContentSchema,
        "sharing.revoked": SharingRevokedMessageContentSchema,
        "ping": PingMessageContentSchema,
    }

    def get_obj_type(self, obj):
        return obj["type"]


message_content_serializer = Serializer(
    MessageContentSchema, validation_exc=FSValidationError, packing_exc=FSPackingError
)
