from uuid import uuid4

from parsec.service import BaseService, cmd


class BaseBlockService(BaseService):

    @cmd('create')
    async def _cmd_CREATE(self, msg):
        id = await self.create(msg['content'])
        return {
            'status': 'ok',
            'id': id
        }

    @cmd('read')
    async def _cmd_READ(self, msg):
        content = await self.read(msg['id'])
        return {'status': 'ok', 'content': content}


class BlockService(BaseBlockService):
    def __init__(self):
        super().__init__()
        self._blocks = {}

    async def create(self, content):
        id = uuid4().hex
        self._blocks[id] = content
        return id

    async def read(self, id):
        return self._blocks[id]
