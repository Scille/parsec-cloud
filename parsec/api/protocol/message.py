# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from parsec.serde import BaseSchema, fields
from parsec.api.protocol.base import BaseReqSchema, BaseRepSchema, CmdSerializer
from parsec.api.protocol.types import DeviceIDField


__all__ = ("message_get_serializer",)


class MessageGetReqSchema(BaseReqSchema):
    offset = fields.Integer(required=True, validate=lambda n: n >= 0)


class MessageSchema(BaseSchema):
    count = fields.Integer(required=True)
    sender = DeviceIDField(required=True)
    timestamp = fields.DateTime(required=True)
    body = fields.Bytes(required=True)


class MessageGetRepSchema(BaseRepSchema):
    messages = fields.List(fields.Nested(MessageSchema), required=True)


message_get_serializer = CmdSerializer(MessageGetReqSchema, MessageGetRepSchema)
