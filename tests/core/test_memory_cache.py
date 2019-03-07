# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from parsec.core.memory_cache import MemoryCache
from parsec.core.memory_cache import DEFAULT_BLOCK_SIZE as block_size
from parsec.core.types import ManifestAccess
from tests.common import freeze_time


@pytest.fixture
def memory_db():
    return MemoryCache(max_cache_size=3 * block_size)


def test_block_limit(memory_db):
    assert memory_db.block_limit == 3


@pytest.mark.parametrize("clean", [True, False])
def test_get_nb_blocks(clean, memory_db):
    access_1 = ManifestAccess()
    access_2 = ManifestAccess()
    assert memory_db.get_nb_blocks(clean) == 0
    # Add one block
    memory_db.set_block(clean, access_1, b"data")
    assert memory_db.get_nb_blocks(clean) == 1
    # Add the same block again
    memory_db.set_block(clean, access_1, b"data")
    assert memory_db.get_nb_blocks(clean) == 1
    # Add another block
    memory_db.set_block(clean, access_2, b"data")
    assert memory_db.get_nb_blocks(clean) == 2
    assert memory_db.get_nb_blocks(not clean) == 0
    # Clear block again
    assert memory_db.clear_block(clean, access_1) is True
    assert memory_db.get_nb_blocks(clean) == 1
    assert memory_db.clear_block(clean, access_1) is False
    assert memory_db.get_nb_blocks(clean) == 1


def test_user(memory_db):
    access = ManifestAccess()
    # Not found
    result = memory_db.get_user(access)
    assert result is None
    # Found
    with freeze_time("2000-01-01"):
        memory_db.set_user(access, b"data")
        memory_db.set_user(access, b"data_bis")  # Replace silently if added again
    result = memory_db.get_user(access)
    assert result == [str(access.id), b"data_bis", "2000-01-01T00:00:00+00:00"]


@pytest.mark.parametrize("clean", [True, False])
def test_manifest(clean, memory_db):
    access = ManifestAccess()
    # Found
    memory_db.set_manifest(clean, access, b"data")
    memory_db.set_manifest(clean, access, b"data_bis")  # Replace silently if added again
    result = memory_db.get_manifest(clean, access)
    assert result == [str(access.id), b"data_bis"]
    # Not found
    result = memory_db.get_manifest(not clean, access)
    assert result is None
    # Clear
    memory_db.clear_manifest(clean, access)
    memory_db.clear_manifest(clean, access)  # Skip silently if cleared again
    # Not found
    result = memory_db.get_manifest(clean, access)
    assert result is None


@pytest.mark.parametrize("clean", [True, False])
def test_block(clean, memory_db):
    access_1 = ManifestAccess()
    access_2 = ManifestAccess()
    access_3 = ManifestAccess()
    access_4 = ManifestAccess()
    # Found
    memory_db.set_block(clean, access_1, b"data_1")
    memory_db.set_block(clean, access_1, b"data_1_bis")  # Replace silently if added again
    with freeze_time("2000-01-01"):
        result = memory_db.get_block(clean, access_1)
    assert result == [access_1, "2000-01-01T00:00:00+00:00", b"data_1_bis"]
    # Not found
    result = memory_db.get_block(not clean, access_1)
    assert result is None
    # Clear
    assert memory_db.clear_block(clean, access_1) is True
    # Clear not found
    assert memory_db.clear_block(clean, access_1) is False
    # Not found
    result = memory_db.get_block(clean, access_1)
    assert result is None
    # Garbage collector
    memory_db.set_block(clean, access_1, b"data_1")
    memory_db.set_block(clean, access_2, b"data_2")
    assert memory_db.get_nb_blocks(clean) == 2
    assert memory_db.set_block(clean, access_3, b"data_3") is None
    assert memory_db.set_block(clean, access_4, b"data_4")  # Trigger garbage collector
    assert memory_db.get_nb_blocks(clean) == 3
    with freeze_time("2000-01-01"):
        result = memory_db.get_block(clean, access_4)
    assert result == [access_4, "2000-01-01T00:00:00+00:00", b"data_4"]
    results = [
        memory_db.get_block(clean, access) for access in [access_1, access_2, access_3, access_4]
    ]
    assert len(results) == 4
    assert len([i for i in results if i]) == 3
