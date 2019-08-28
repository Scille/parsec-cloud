# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from parsec.serde import UnknownCheckedSchema, fields
from parsec.api.protocol.base import BaseReqSchema, BaseRepSchema, CmdSerializer
from parsec.api.protocol.types import UserIDField, DeviceIDField


__all__ = ("message_send_serializer", "message_get_serializer")


class MessageSendReqSchema(BaseReqSchema):
    recipient = UserIDField(required=True)
    timestamp = fields.DateTime(required=True)
    body = fields.Bytes(required=True)


class MessageSendRepSchema(BaseRepSchema):
    pass


message_send_serializer = CmdSerializer(MessageSendReqSchema, MessageSendRepSchema)


class MessageGetReqSchema(BaseReqSchema):
    offset = fields.Integer(required=True, validate=lambda n: n >= 0)


class MessageSchema(UnknownCheckedSchema):
    count = fields.Integer(required=True)
    sender = DeviceIDField(required=True)
    timestamp = fields.DateTime(required=True)
    body = fields.Bytes(required=True)


class MessageGetRepSchema(BaseRepSchema):
    messages = fields.List(fields.Nested(MessageSchema), required=True)


message_get_serializer = CmdSerializer(MessageGetReqSchema, MessageGetRepSchema)
