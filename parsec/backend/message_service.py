from blinker import signal
from collections import defaultdict

from parsec.service import BaseService, cmd, event


class BaseMessageService(BaseService):

    on_msg_arrived = event('arrived')

    @cmd('new')
    async def _cmd_NEW(self, msg):
        await self.new(msg['recipient'], msg['body'])
        return {'status': 'ok'}

    @cmd('get')
    async def _cmd_GET(self, msg):
        offset = msg.get('offset', 0)
        messages = [{'count': i, 'body': msg} for i, msg in enumerate(
            await self.get(msg['recipient'], offset), 1 + offset)]
        return {'status': 'ok', 'messages': messages}


class MessageService(BaseMessageService):
    def __init__(self):
        self._messages = defaultdict(list)

    async def new(self, recipient, body):
        self._messages[recipient].append(body)
        signal('message:arrived').send(recipient)

    async def get(self, recipient, offset=0):
        return self._messages[recipient][offset:]
