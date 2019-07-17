# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from parsec.core.fs.exceptions import FSPackingError, FSValidationError
from parsec.core.types.base import EntryIDField
from parsec.serde import OneOfSchema, Serializer, UnknownCheckedSchema, fields


class SharingGrantedMessageContentSchema(UnknownCheckedSchema):
    type = fields.CheckedConstant("sharing.granted", required=True)
    name = fields.String(required=True)
    id = EntryIDField(required=True)
    encryption_revision = fields.Integer(required=True)
    encrypted_on = fields.DateTime(required=True)
    key = fields.SecretKey(missing=None)
    # Don't include access rights given the receiver will have anyway to
    # interrogate the backend for workspace's access rights list
    # to make sure the message sender is an admin


class SharingReencryptedMessageContentSchema(SharingGrantedMessageContentSchema):
    type = fields.CheckedConstant("sharing.reencrypted", required=True)


class SharingRevokedMessageContentSchema(UnknownCheckedSchema):
    type = fields.CheckedConstant("sharing.revoked", required=True)
    id = EntryIDField(required=True)


class PingMessageContentSchema(UnknownCheckedSchema):
    type = fields.CheckedConstant("ping", required=True)
    ping = fields.String(required=True)


class MessageContentSchema(OneOfSchema):
    type_field = "type"
    type_field_remove = False
    type_schemas = {
        "sharing.granted": SharingGrantedMessageContentSchema,
        "sharing.reencrypted": SharingReencryptedMessageContentSchema,
        "sharing.revoked": SharingRevokedMessageContentSchema,
        "ping": PingMessageContentSchema,
    }

    def get_obj_type(self, obj):
        return obj["type"]


message_content_serializer = Serializer(
    MessageContentSchema, validation_exc=FSValidationError, packing_exc=FSPackingError
)
