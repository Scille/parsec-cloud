from parsec.utils import to_jsonb64
from parsec.schema import BaseCmdSchema, fields


class cmd_NEW_Schema(BaseCmdSchema):
    recipient = fields.String(required=True)
    body = fields.Base64Bytes(required=True)


class cmd_GET_Schema(BaseCmdSchema):
    offset = fields.Integer(missing=0)


class BaseMessageComponent:

    def __init__(self, signal_ns):
        self._signal_message_arrived = signal_ns.signal("message_arrived")

    async def perform_message_new(self, sender_device_id, recipient_user_id, body):
        raise NotImplementedError()

    async def perform_message_get(self, recipient_user_id, offset):
        raise NotImplementedError()

    async def api_message_new(self, client_ctx, msg):
        msg = cmd_NEW_Schema().load_or_abort(msg)
        await self.perform_message_new(
            sender_device_id=client_ctx.id, recipient_user_id=msg["recipient"], body=msg["body"]
        )
        return {"status": "ok"}

    async def api_message_get(self, client_ctx, msg):
        msg = cmd_GET_Schema().load_or_abort(msg)
        offset = msg["offset"]
        messages = await self.perform_message_get(client_ctx.user_id, offset)
        return {
            "status": "ok",
            "messages": [
                {"count": i, "body": to_jsonb64(data[1]), "sender_id": data[0]}
                for i, data in enumerate(messages, offset + 1)
            ],
        }
