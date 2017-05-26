from datetime import datetime
from uuid import uuid4
from marshmallow import fields

from parsec.service import BaseService, cmd
from parsec.exceptions import ParsecError
from parsec.tools import BaseCmdSchema, logger


class BlockError(ParsecError):
    status = 'block_error'


class BlockNotFound(BlockError):
    status = 'not_found'


class cmd_CREATE_Schema(BaseCmdSchema):
    content = fields.String(required=True)
    id = fields.String(missing=None)


class cmd_READ_Schema(BaseCmdSchema):
    id = fields.String(required=True)


class BaseBlockService(BaseService):

    name = 'BlockService'

    @cmd('block_create')
    async def _cmd_CREATE(self, session, msg):
        msg = cmd_CREATE_Schema().load(msg)
        id = await self.create(msg['content'], msg['id'])
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

    async def create(self, content, id):
        raise NotImplementedError()

    async def read(self, id):
        raise NotImplementedError()

    async def stat(self, id):
        raise NotImplementedError()


class MockedBlockService(BaseBlockService):

    def __init__(self):
        super().__init__()
        self._blocks = {}

    async def create(self, content, id=None):
        id = id if id else uuid4().hex  # TODO uuid4 or trust seed?
        timestamp = datetime.utcnow().timestamp()
        self._blocks[id] = {'content': content, 'creation_timestamp': timestamp}
        return id

    async def read(self, id):
        try:
            response = self._blocks[id]
        except KeyError:
            raise BlockNotFound('Block not found.')
        return response

    async def stat(self, id):
        try:
            response = {'creation_timestamp': self._blocks[id]['creation_timestamp']}
        except KeyError:
            raise BlockNotFound('Block not found.')
        return response


class MetaBlockService(BaseBlockService):

    def __init__(self, backends):
        super().__init__()
        # TODO set method preference for results and call others methods in background
        self.block_services = {}
        for backend in backends:
            self.block_services[backend.__class__.__name__] = backend()

    async def _do_operation(self, operation, *args, **kwargs):
        result = None
        for _, block_service in self.block_services.items():
            try:
                result = await getattr(block_service, operation)(*args, **kwargs)
            except BlockError:
                logger.warning('%s backend failed to complete %s operation.' %
                               (block_service.__class__.__name__, operation))
            if result and operation in ['read', 'stat']:
                break
            # TODO continue creation operation in background for others backends?
        if not result:
            raise BlockError('All backends failed to complete %s operation.' % operation)
        return result

    async def create(self, content, id=None):
        id = id if id else uuid4().hex  # TODO uuid4 or trust seed?
        return await self._do_operation('create', content, id)

    async def read(self, id):
        return await self._do_operation('read', id)

    async def stat(self, id):
        return await self._do_operation('stat', id)
