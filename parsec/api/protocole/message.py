from parsec.schema import UnknownCheckedSchema, fields
from parsec.api.protocole.base import BaseReqSchema, BaseRepSchema, CmdSerializer


__all__ = ("message_send_serializer", "message_get_serializer")


class MessageSendReqSchema(BaseReqSchema):
    recipient = fields.UserID(required=True)
    body = fields.Base64Bytes(required=True)


class MessageSendRepSchema(BaseRepSchema):
    pass


message_send_serializer = CmdSerializer(MessageSendReqSchema, MessageSendRepSchema)


class MessageGetReqSchema(BaseReqSchema):
    offset = fields.Integer(missing=0)


class MessageSchema(UnknownCheckedSchema):
    sender = fields.UserID(required=True)
    offset = fields.Integer(required=True)
    body = fields.Base64Bytes(required=True)


class MessageGetRepSchema(BaseRepSchema):
    messages = fields.List(fields.Nested(MessageSchema), required=True)


message_get_serializer = CmdSerializer(MessageGetReqSchema, MessageGetRepSchema)
