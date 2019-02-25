# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from parsec.serde import UnknownCheckedSchema, fields
from parsec.api.protocole.base import BaseReqSchema, BaseRepSchema, CmdSerializer


__all__ = ("message_send_serializer", "message_get_serializer")


class MessageSendReqSchema(BaseReqSchema):
    recipient = fields.UserID(required=True)
    body = fields.Bytes(required=True)


class MessageSendRepSchema(BaseRepSchema):
    pass


message_send_serializer = CmdSerializer(MessageSendReqSchema, MessageSendRepSchema)


class MessageGetReqSchema(BaseReqSchema):
    offset = fields.Integer(required=True, validate=lambda n: n >= 0)


class MessageSchema(UnknownCheckedSchema):
    sender = fields.DeviceID(required=True)
    count = fields.Integer(required=True)
    body = fields.Bytes(required=True)


class MessageGetRepSchema(BaseRepSchema):
    messages = fields.List(fields.Nested(MessageSchema), required=True)


message_get_serializer = CmdSerializer(MessageGetReqSchema, MessageGetRepSchema)
