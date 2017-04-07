from collections import defaultdict
from marshmallow import fields

from parsec.service import BaseService, cmd, event
from parsec.tools import BaseCmdSchema


class cmd_NEW_Schema(BaseCmdSchema):
    recipient = fields.String(required=True)
    body = fields.String(required=True)


class cmd_GET_Schema(BaseCmdSchema):
    recipient = fields.String(required=True)
    offset = fields.Int(missing=0)


class BaseMessageService(BaseService):

    on_msg_arrived = event('arrived')

    @cmd('new')
    async def _cmd_NEW(self, session, msg):
        msg = cmd_NEW_Schema().load(msg)
        await self.new(msg['recipient'], msg['body'])
        return {'status': 'ok'}

    @cmd('get')
    async def _cmd_GET(self, session, msg):
        msg = cmd_GET_Schema().load(msg)
        # TODO: use session.identity to get retrieve recipient ?
        offset = msg['offset']
        messages = [{'count': i, 'body': msg} for i, msg in enumerate(
            await self.get(msg['recipient'], offset), 1 + offset)]
        return {'status': 'ok', 'messages': messages}


class InMemoryMessageService(BaseMessageService):
    def __init__(self):
        super().__init__()
        self._messages = defaultdict(list)

    async def new(self, recipient, body):
        self._messages[recipient].append(body)
        self.on_msg_arrived.send(recipient)

    async def get(self, recipient, offset=0):
        return self._messages[recipient][offset:]
