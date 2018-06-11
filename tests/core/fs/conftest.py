import pytest
from unittest.mock import Mock

from parsec.core.fs import FS
from parsec.core.manifests_manager import ManifestsManager
from parsec.core.blocks_manager import BlocksManager

from tests.common import AsyncMock


@pytest.fixture
def mocked_manifests_manager(alice):
    mocked_manifests_manager = AsyncMock(spec=ManifestsManager)
    mocked_manifests_manager.device = alice
    mocked_manifests_manager.fetch_user_manifest_from_local.return_value = None
    return mocked_manifests_manager


@pytest.fixture
def mocked_blocks_manager():
    return AsyncMock(spec=BlocksManager)


@pytest.fixture
def fs(alice, mocked_manifests_manager, mocked_blocks_manager):
    fs = FS(alice, mocked_manifests_manager, mocked_blocks_manager)
    return fs
