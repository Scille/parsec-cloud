import pytest

from parsec.core.fs import *
from parsec.core.manifests_manager import ManifestsManager
from parsec.core.blocks_manager import BlocksManager

from tests.common import AsyncMock


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
