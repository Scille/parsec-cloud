from datetime import datetime
from uuid import uuid4
from marshmallow import fields

from parsec.core.cache import cached_block
from parsec.service import BaseService
from parsec.exceptions import BlockError, BlockNotFound
from parsec.tools import logger


class BaseBlockService(BaseService):

    name = 'BlockService'

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

    @cached_block
    async def create(self, content, id=None):
        id = id if id else uuid4().hex  # TODO uuid4 or trust seed?
        date = datetime.utcnow().isoformat()
        self._blocks[id] = {'content': content, 'creation_date': date}
        return id

    @cached_block
    async def read(self, id):
        try:
            response = self._blocks[id]
        except KeyError:
            raise BlockNotFound('Block not found.')
        return response

    @cached_block
    async def stat(self, id):
        try:
            response = {'creation_date': self._blocks[id]['creation_date']}
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

    @cached_block
    async def create(self, content, id=None):
        id = id if id else uuid4().hex  # TODO uuid4 or trust seed?
        return await self._do_operation('create', content, id)

    @cached_block
    async def read(self, id):
        return await self._do_operation('read', id)

    @cached_block
    async def stat(self, id):
        return await self._do_operation('stat', id)
