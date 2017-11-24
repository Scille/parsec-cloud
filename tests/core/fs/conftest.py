import pytest
from unittest.mock import Mock
from inspect import iscoroutinefunction

from parsec.core.fs import *
from parsec.core.manifests_manager import ManifestsManager
from parsec.core.blocks_manager import BlocksManager


class AsyncMock(Mock):
    def __init__(self, *args, **kwargs):
        super().__init__(*args, **kwargs)
        spec = kwargs.get('spec')
        if spec:
            for field in dir(spec):
                if iscoroutinefunction(getattr(spec, field)):
                    getattr(self, field).is_async = True

    async def __async_call(self, *args, **kwargs):
        return super().__call__(*args, **kwargs)

    def __call__(self, *args, **kwargs):
        if getattr(self, 'is_async', False) is True:
            if iscoroutinefunction(self.side_effect):
                return self.side_effect(*args, **kwargs)
            else:
                return self.__async_call(*args, **kwargs)
        else:
            return super().__call__(*args, **kwargs)


@pytest.fixture
def mocked_manifests_manager():
    return AsyncMock(spec=ManifestsManager)


@pytest.fixture
def mocked_blocks_manager():
    return AsyncMock(spec=BlocksManager)


@pytest.fixture
def fs(mocked_manifests_manager, mocked_blocks_manager):
    fs = FS(mocked_manifests_manager, mocked_blocks_manager)
    return fs
