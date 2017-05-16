import pytest

from parsec.core import MockedCacheService
from parsec.core.cache_service import CacheNotFound


@pytest.fixture
def cache_svc():
    return MockedCacheService()


class TestCacheService:

    @pytest.mark.asyncio
    async def test_get_and_set(self, cache_svc):
        id = 'i1234'
        content = 'r1234'
        # Not in cache
        with pytest.raises(CacheNotFound):
            await cache_svc.get(id)
        # Set cache
        await cache_svc.set(id, content)
        response = await cache_svc.get(id)
        assert content == response
        # Update cache
        content = 'r5678'
        await cache_svc.set(id, content)
        response = await cache_svc.get(id)
        assert content == response
