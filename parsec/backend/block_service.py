from datetime import datetime
from uuid import uuid4
from marshmallow import fields

from parsec.service import BaseService, cmd, service
from parsec.exceptions import ParsecError
from parsec.tools import BaseCmdSchema


class BlockError(ParsecError):
    pass


class BlockNotFound(BlockError):
    status = 'not_found'


class cmd_CREATE_Schema(BaseCmdSchema):
    content = fields.String(required=True)


class cmd_READ_Schema(BaseCmdSchema):
    id = fields.String(required=True)


class BaseBlockService(BaseService):

    name = 'BlockService'

    vlob_service = service('MockedVlobService')

    @cmd('block_create')
    async def _cmd_CREATE(self, session, msg):
        msg = cmd_CREATE_Schema().load(msg)
        id = await self.create(msg['content'])
        return {'status': 'ok', 'id': id}

    @cmd('block_read')
    async def _cmd_READ(self, session, msg):
        msg = cmd_READ_Schema().load(msg)
        block = await self.read(msg['id'])
        block.update({'status': 'ok'})
        return block

    @cmd('block_stat')
    async def _cmd_STAT(self, session, msg):
        msg = cmd_READ_Schema().load(msg)
        stat = await self.stat(msg['id'])
        stat.update({'status': 'ok'})
        return stat

    async def create(self, content):
        raise NotImplementedError()

    async def read(self, id):
        raise NotImplementedError()

    async def stat(self, id):
        raise NotImplementedError()


class MockedBlockService(BaseBlockService):
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
            raise BlockNotFound('Block not found.')

    async def stat(self, id):
        try:
            return {'access_timestamp': self._blocks[id]['access_timestamp'],
                    'creation_timestamp': self._blocks[id]['creation_timestamp']}
        except KeyError:
            raise BlockNotFound('Block not found.')
