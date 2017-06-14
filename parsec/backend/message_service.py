from collections import defaultdict
from marshmallow import fields

from parsec.service import BaseService, cmd, event
from parsec.tools import BaseCmdSchema, to_jsonb64


class cmd_NEW_Schema(BaseCmdSchema):
    recipient = fields.String(required=True)
    body = fields.Base64Bytes(required=True)


class cmd_GET_Schema(BaseCmdSchema):
    recipient = fields.String(required=True)
    offset = fields.Int(missing=0)


class BaseMessageService(BaseService):

    name = 'MessageService'

    on_arrived = event('on_message_arrived')

    @cmd('message_new')
    async def _cmd_NEW(self, session, msg):
        msg = cmd_NEW_Schema().load(msg)
        await self.new(msg['recipient'], msg['body'])
        return {'status': 'ok'}

    @cmd('message_get')
    async def _cmd_GET(self, session, msg):
        msg = cmd_GET_Schema().load(msg)
        # TODO: use session.identity to retrieve recipient ?
        offset = msg['offset']
        messages = [{'count': i, 'body': to_jsonb64(msg)} for i, msg in enumerate(
            await self.get(msg['recipient'], offset), 1 + offset)]
        return {'status': 'ok', 'messages': messages}

    async def new(self, recipient: str, body: bytes):
        raise NotImplementedError()

    async def get(self, recipient: str, offset: int=0):
        raise NotImplementedError()


class InMemoryMessageService(BaseMessageService):
    def __init__(self):
        super().__init__()
        self._messages = defaultdict(list)

    async def new(self, recipient, body):
        self._messages[recipient].append(body)
        self.on_arrived.send(recipient)

    async def get(self, recipient, offset=0):
        return self._messages[recipient][offset:]
