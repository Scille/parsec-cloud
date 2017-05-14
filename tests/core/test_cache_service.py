import pytest

from parsec.core import MockedCacheService
from parsec.core.cache_service import CacheNotFound


@pytest.fixture
def cache_svc():
    return MockedCacheService()


class TestCacheService:

    @pytest.mark.asyncio
    @pytest.mark.parametrize('params', [('i1234', 'r1234')])
    async def test_get_and_set(self, cache_svc, params):
        with pytest.raises(CacheNotFound):
            await cache_svc.get(params[0])
        await cache_svc.set(params[0], params[1])
        response = await cache_svc.get(params[0])
        assert params[1] == response
