from parsec.backend import MockedVlobService, MockedNamedVlobService, MockedBlockService
from parsec.service import BaseService


class BaseBackendAPIService(BaseService):

    name = 'BackendAPIService'


class BackendAPIService(BaseBackendAPIService):
    def __init__(self, backend_host, backend_port):
        # TODO
        pass


class MockedBackendAPIService(BaseBackendAPIService):

    def __init__(self):
        super().__init__()
        self._named_vlob_service = MockedNamedVlobService()
        self._vlob_service = MockedVlobService()
        self._block_service = MockedBlockService()

    async def block_create(self, *args, **kwargs):
        return await self._block_service.create(*args, **kwargs)

    async def block_read(self, *args, **kwargs):
        return await self._block_service.read(*args, **kwargs)

    async def block_stat(self, *args, **kwargs):
        return await self._block_service.stat(*args, **kwargs)

    async def message_new(self, *args, **kwargs):
        return await self._message_service.new(*args, **kwargs)

    async def message_get(self, *args, **kwargs):
        return await self._message_service.get(*args, **kwargs)

    async def named_vlob_create(self, *args, **kwargs):
        return await self._named_vlob_service.create(*args, **kwargs)

    async def named_vlob_read(self, *args, **kwargs):
        return await self._named_vlob_service.read(*args, **kwargs)

    async def named_vlob_update(self, *args, **kwargs):
        return await self._named_vlob_service.update(*args, **kwargs)

    async def vlob_create(self, *args, **kwargs):
        return await self._vlob_service.create(*args, **kwargs)

    async def vlob_read(self, *args, **kwargs):
        return await self._vlob_service.read(*args, **kwargs)

    async def vlob_update(self, *args, **kwargs):
        return await self._vlob_service.update(*args, **kwargs)
