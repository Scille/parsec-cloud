from parsec.service import BaseService
from parsec.exceptions import ParsecError


class CacheError(ParsecError):
    pass


class CacheNotFound(CacheError):
    status = 'not_found'


class BaseCacheService(BaseService):

    name = 'CacheService'


# TODO subscribe to events in order to invalidate named_vlob_manifests
class MockedCacheService(BaseCacheService):

    def __init__(self):
        super().__init__()
        self.cache = {}

    async def get(self, id):
        try:
            return self.cache[id]
        except KeyError:
            raise CacheNotFound('Requested item not found.')

    async def set(self, id, content):
        self.cache[id] = content
