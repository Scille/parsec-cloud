from parsec.utils import to_jsonb64
from parsec.schema import BaseCmdSchema, fields



class cmd_NEW_Schema(BaseCmdSchema):
    recipient = fields.String(required=True)
    body = fields.Base64Bytes(required=True)


class cmd_GET_Schema(BaseCmdSchema):
    offset = fields.Int(missing=0)


class BaseMessageComponent:

    def __init__(self, signal_ns):
        self._signal_message_arrived = signal_ns.signal('message_arrived')

    async def api_message_new(self, client_ctx, msg):
        msg = cmd_NEW_Schema().load_or_abort(msg)
        await self.new(**msg)
        return {'status': 'ok'}

    async def api_message_get(self, client_ctx, msg):
        msg = cmd_GET_Schema().load_or_abort(msg)
        messages = await self.get(client_ctx.id, **msg)
        offset = msg['offset']
        return {
            'status': 'ok',
            'messages': [{'count': i, 'body': to_jsonb64(msg)}
                         for i, msg in enumerate(messages, offset + 1)]
        }
