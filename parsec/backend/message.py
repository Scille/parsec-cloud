from collections import defaultdict
from marshmallow import fields

from parsec.utils import UnknownCheckedSchema, to_jsonb64



class cmd_NEW_Schema(UnknownCheckedSchema):
    recipient = fields.String(required=True)
    body = fields.Base64Bytes(required=True)


class cmd_GET_Schema(UnknownCheckedSchema):
    offset = fields.Int(missing=0)


class BaseMessageComponent:

    def __init__(self, signal_ns):
        self._signal_message_arrived = signal_ns.signal('message_arrived')

    async def api_message_new(self, client_ctx, msg):
        msg = cmd_NEW_Schema().load(msg)
        await self.new(**msg)
        return {'status': 'ok'}


    async def api_message_get(self, client_ctx, msg):
        msg = cmd_GET_Schema().load(msg)
        messages = await self.get(client_ctx.id, **msg)
        offset = msg['offset']
        return {
            'status': 'ok',
            'messages': [{'count': i, 'body': to_jsonb64(msg)}
                         for i, msg in enumerate(messages, offset + 1)]
        }


class InMemoryMessageComponent(BaseMessageComponent):
    def __init__(self, *args):
        super().__init__(*args)
        self._messages = defaultdict(list)

    async def perform_message_new(self, recipient, body):
        self._messages[recipient].append(body)
        self._signal_message_arrived.send(recipient)

    async def perform_message_get(self, id, offset):
        return self._messages[id][offset:]
