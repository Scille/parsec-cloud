from datetime import datetime
from uuid import uuid4

from parsec.service import BaseService, cmd, event, service
from parsec.exceptions import ParsecError


class BlockError(ParsecError):
    pass


class BlockNotFound(BlockError):
    status = 'not_found'


class BaseBlockService(BaseService):

    vlob_service = service('VlobService')
    on_vlob_updated = event('updated')

    @cmd('create')
    async def _cmd_CREATE(self, msg):
        id = await self.create(msg['content'])
        return {
            'status': 'ok',
            'id': id
        }

    @cmd('read')
    async def _cmd_READ(self, msg):
        block = await self.read(msg['id'])
        block.update({'status': 'ok'})
        return block

    @cmd('stat')
    async def _cmd_STAT(self, msg):
        stat = await self.stat(msg['id'])
        stat.update({'status': 'ok'})
        return stat


class BlockService(BaseBlockService):
    def __init__(self):
        super().__init__()
        self._blocks = {}

    async def create(self, content):
        id = uuid4().hex  # TODO uuid4 or trust seed?
        timestamp = datetime.utcnow().timestamp()
        self._blocks[id] = {'content': content,
                            'access_timestamp': timestamp,
                            'creation_timestamp': timestamp}
        return id

    async def read(self, id):
        try:
            self._blocks[id]['access_timestamp'] = datetime.utcnow().timestamp()
            return self._blocks[id]
        except KeyError:
            raise BlockNotFound()

    async def stat(self, id):
        try:
            return {'access_timestamp': self._blocks[id]['access_timestamp'],
                    'creation_timestamp': self._blocks[id]['creation_timestamp']}
        except KeyError:
            raise BlockNotFound()
