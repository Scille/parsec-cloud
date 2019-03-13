# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from parsec.core.local_storage import LocalStorage
from parsec.core.local_storage import DEFAULT_BLOCK_SIZE as block_size
from parsec.core.persistent_storage import LocalStorageMissingEntry
from parsec.core.types import LocalUserManifest, ManifestAccess
from tests.common import freeze_time
from unittest import mock


@pytest.fixture
def local_storage(tmpdir):
    with LocalStorage(tmpdir, max_memory_cache_size=3 * block_size) as db:
        yield db


def test_block_limit(local_storage):
    assert local_storage.block_limit == 3


@pytest.mark.parametrize("clean", [True, False])
def test_get_nb_blocks(clean, local_storage):
    set_block = local_storage.set_clean_block if clean else local_storage.set_dirty_block
    clear_block = local_storage.clear_clean_block if clean else local_storage.clear_dirty_block
    access_1 = ManifestAccess()
    access_2 = ManifestAccess()
    assert local_storage.get_nb_blocks(clean) == 0
    # Add one block
    set_block(access_1, b"data")
    assert local_storage.get_nb_blocks(clean) == 1
    # Add the same block again
    set_block(access_1, b"data")
    assert local_storage.get_nb_blocks(clean) == 1
    # Add another block
    set_block(access_2, b"data")
    assert local_storage.get_nb_blocks(clean) == 2
    assert local_storage.get_nb_blocks(not clean) == 0
    # Clear block again
    clear_block(access_1)
    assert local_storage.get_nb_blocks(clean) == 1
    clear_block(access_1)
    assert local_storage.get_nb_blocks(clean) == 1


# def test_user(local_storage):
#     access = ManifestAccess()
#     # Not found
#     result = local_storage.get_user(access)
#     assert result is None
#     # Found
#     with freeze_time("2000-01-01"):
#         local_storage.set_user(access, b"data")
#         local_storage.set_user(access, b"data_bis")  # Replace silently if added again
#     result = local_storage.get_user(access)
#     assert result == [str(access.id), b"data_bis", "2000-01-01T00:00:00+00:00"]


# @pytest.mark.parametrize("clean", [True, False])
# def test_manifest(clean, local_storage):
#     access = ManifestAccess()
#     # Found
#     local_storage.set_manifest(clean, access, b"data")
#     local_storage.set_manifest(clean, access, b"data_bis")  # Replace silently if added again
#     result = local_storage.get_manifest(clean, access)
#     assert result == [str(access.id), b"data_bis"]
#     # Not found
#     result = local_storage.get_manifest(not clean, access)
#     assert result is None
#     # Clear
#     local_storage.clear_manifest(clean, access)
#     local_storage.clear_manifest(clean, access)  # Skip silently if cleared again
#     # Not found
#     result = local_storage.get_manifest(clean, access)
#     assert result is None


@mock.patch("parsec.core.persistent_storage.PersistentStorage.delete_invalid_blocks")
def test_local_storage_enter(delete_invalid_blocks_mock, tmpdir):
    delete_invalid_blocks_mock

    with LocalStorage(tmpdir, max_memory_cache_size=3 * block_size):
        delete_invalid_blocks_mock.assert_called_once()


@mock.patch("parsec.core.persistent_storage.PersistentStorage.set_clean_block")
@mock.patch("parsec.core.persistent_storage.PersistentStorage.update_block_accessed_on")
def test_local_storage_exit(update_block_accessed_on_mock, set_clean_block_mock, tmpdir):
    access = ManifestAccess()

    def _run_local_storage(persisted):
        with LocalStorage(tmpdir, max_memory_cache_size=3 * block_size) as local_storage:
            local_storage.clean_block_cache = {
                access.id: {
                    "access": access,
                    "accessed_on": "2019",
                    "block": b"data",
                    "persisted": persisted,
                }
            }

    _run_local_storage(False)

    set_clean_block_mock.assert_called_once_with(access, b"data")
    update_block_accessed_on_mock.assert_called_once_with(True, access, "2019")
    set_clean_block_mock.reset_mock()
    update_block_accessed_on_mock.reset_mock()

    _run_local_storage(True)
    set_clean_block_mock.assert_not_called()
    update_block_accessed_on_mock.assert_called_once_with(True, access, "2019")


@pytest.mark.xfail
def test_get_user(alice, local_storage):
    pass


@pytest.mark.xfail
def test_set_user(local_storage):
    pass


def test_get_manifest(local_storage):
    access = ManifestAccess()
    with pytest.raises(LocalStorageMissingEntry):
        local_storage.get_manifest(access)
    local_storage.manifest_cache[access.id] = "manifest"
    assert local_storage.get_manifest(access) == "manifest"


@mock.patch("parsec.core.persistent_storage.PersistentStorage.clear_dirty_manifest")
def test_set_clean_manifest(clear_dirty_manifest_mock, local_storage):
    access = ManifestAccess()
    manifest = LocalUserManifest(author=123, base_version=1, is_placeholder=False, need_sync=False)
    local_storage.set_clean_manifest(access, manifest, False)
    clear_dirty_manifest_mock.assert_not_called()
    assert local_storage.manifest_cache[access.id] == manifest
    local_storage.manifest_cache = {}
    # Force clear
    local_storage.set_clean_manifest(access, manifest, True)
    clear_dirty_manifest_mock.assert_called_once_with(access)
    assert local_storage.manifest_cache[access.id] == manifest


def test_set_dirty_manifest(local_storage):
    access = ManifestAccess()
    manifest = LocalUserManifest(author=123, base_version=1, is_placeholder=False, need_sync=False)
    local_storage.set_dirty_manifest(access, manifest)
    assert local_storage.manifest_cache[access.id] == manifest


@mock.patch("parsec.core.persistent_storage.PersistentStorage.clear_clean_manifest")
@mock.patch("parsec.core.persistent_storage.PersistentStorage.clear_dirty_manifest")
def test_clear_manifest(clear_dirty_manifest_mock, clear_clean_manifest_mock, local_storage):
    access = ManifestAccess()
    manifest = LocalUserManifest(author=123, base_version=1, is_placeholder=False, need_sync=False)
    local_storage.manifest_cache == {access.id: manifest}
    local_storage.clear_manifest(access)
    clear_clean_manifest_mock.assert_called_once_with(access)
    clear_dirty_manifest_mock.assert_not_called()
    clear_clean_manifest_mock.reset_mock()
    assert local_storage.manifest_cache == {}

    local_storage.manifest_cache == {access.id: manifest}
    clear_clean_manifest_mock.side_effect = LocalStorageMissingEntry(access)
    local_storage.clear_manifest(access)
    clear_clean_manifest_mock.assert_called_once_with(access)
    clear_dirty_manifest_mock.assert_called_once_with(access)
    assert local_storage.manifest_cache == {}

    clear_clean_manifest_mock.side_effect = LocalStorageMissingEntry(access)
    clear_dirty_manifest_mock.side_effect = LocalStorageMissingEntry(access)
    local_storage.clear_manifest(access)


@mock.patch("random.choice")
def test_add_block_in_dirty_cache(choice_mock, local_storage):
    def mock_choice(key):
        return key

    access_1 = ManifestAccess()
    access_2 = ManifestAccess()
    access_3 = ManifestAccess()
    access_4 = ManifestAccess()

    choice_mock.side_effect = lambda _: mock_choice(access_1.id)

    local_storage.add_block_in_dirty_cache(access_1, b"data_1")
    local_storage.add_block_in_dirty_cache(access_2, b"data_2")
    local_storage.add_block_in_dirty_cache(access_3, b"data_3")
    assert local_storage.dirty_block_cache == {
        access_1.id: b"data_1",
        access_2.id: b"data_2",
        access_3.id: b"data_3",
    }

    # Garbage collector
    assert len(local_storage.dirty_block_cache) == 3
    local_storage.add_block_in_dirty_cache(access_4, b"data_4")
    assert len(local_storage.dirty_block_cache) == 3
    assert access_1.id not in local_storage.dirty_block_cache
    assert local_storage.dirty_block_cache == {
        access_2.id: b"data_2",
        access_3.id: b"data_3",
        access_4.id: b"data_4",
    }


@mock.patch("parsec.core.persistent_storage.PersistentStorage.get_clean_block")
@mock.patch("parsec.core.persistent_storage.PersistentStorage.get_dirty_block")
@mock.patch("parsec.core.persistent_storage.PersistentStorage.update_block_accessed_on")
def test_get_block(
    update_block_accessed_on_mock, get_dirty_block_mock, get_clean_block_mock, tmpdir
):
    local_storage = LocalStorage(tmpdir, max_memory_cache_size=3 * block_size)
    local_storage.__enter__()

    # Block in dirty cache
    access = ManifestAccess()
    local_storage.clean_block_cache = {}
    local_storage.dirty_block_cache = {
        access.id: {"access": access, "accessed_on": None, "block": b"data_1", "persisted": False}
    }
    with freeze_time("2000-01-01"):
        local_storage.get_block(access)
    update_block_accessed_on_mock.assert_called_with(False, access, "2000-01-01T00:00:00+00:00")
    update_block_accessed_on_mock.reset_mock()

    # Block in clean cache
    local_storage.clean_block_cache = {
        access.id: {
            "access": access,
            "accessed_on": "2000-01-01T00:00:00+00:00",
            "block": b"data_1",
            "persisted": False,
        }
    }
    local_storage.dirty_block_cache = {}
    with freeze_time("2000-01-02"):
        local_storage.get_block(access)
    update_block_accessed_on_mock.assert_not_called()
    assert local_storage.clean_block_cache[access.id]["accessed_on"] == "2000-01-02T00:00:00+00:00"

    # Block in dirty persisted storage
    local_storage.clean_block_cache = {}
    local_storage.dirty_block_cache = {}
    local_storage.get_block(access)
    update_block_accessed_on_mock.assert_not_called()
    get_clean_block_mock.assert_not_called()
    get_dirty_block_mock.assert_called_once_with(access)
    get_dirty_block_mock.reset_mock()

    # Block in clean persisted storage
    get_dirty_block_mock.side_effect = LocalStorageMissingEntry(access)
    local_storage.clean_block_cache = {}
    local_storage.dirty_block_cache = {}
    local_storage.get_block(access)
    update_block_accessed_on_mock.assert_called_once_with(True, access, None)
    get_clean_block_mock.assert_called_once_with(access)
    get_dirty_block_mock.assert_called_once_with(access)

    local_storage.__exit__()


@mock.patch("parsec.core.persistent_storage.PersistentStorage.set_dirty_block")
def test_set_dirty_block(set_dirty_block_mock, local_storage):
    access = ManifestAccess()
    local_storage.set_dirty_block(access, b"data")
    assert local_storage.clean_block_cache == {}
    assert local_storage.dirty_block_cache == {access.id: b"data"}
    set_dirty_block_mock.assert_called_once_with(access, b"data")


@mock.patch("random.choice")
@mock.patch("parsec.core.persistent_storage.PersistentStorage.set_clean_block")
@mock.patch("parsec.core.persistent_storage.PersistentStorage.update_block_accessed_on")
def test_set_clean_block(
    update_block_accessed_on_mock, set_clean_block_mock, choice_mock, local_storage
):
    def mock_choice(key):
        return key

    access_1 = ManifestAccess()
    access_2 = ManifestAccess()
    access_3 = ManifestAccess()
    access_4 = ManifestAccess()

    choice_mock.side_effect = lambda _: mock_choice(access_1.id)

    local_storage.set_clean_block(access_1, b"data_1")
    local_storage.set_clean_block(access_2, b"data_2")
    local_storage.set_clean_block(access_3, b"data_3")
    assert local_storage.clean_block_cache == {
        access_1.id: {
            "access": access_1,
            "accessed_on": None,
            "block": b"data_1",
            "persisted": False,
        },
        access_2.id: {
            "access": access_2,
            "accessed_on": None,
            "block": b"data_2",
            "persisted": False,
        },
        access_3.id: {
            "access": access_3,
            "accessed_on": None,
            "block": b"data_3",
            "persisted": False,
        },
    }
    assert local_storage.dirty_block_cache == {}
    set_clean_block_mock.assert_not_called()

    # Garbage collector (remove not persisted block)
    assert len(local_storage.clean_block_cache) == 3
    local_storage.set_clean_block(access_4, b"data_4")
    assert len(local_storage.clean_block_cache) == 3
    assert access_1.id not in local_storage.clean_block_cache
    assert local_storage.clean_block_cache[access_4.id] == {
        "access": access_4,
        "accessed_on": None,
        "block": b"data_4",
        "persisted": False,
    }
    assert local_storage.dirty_block_cache == {}
    set_clean_block_mock.assert_called_once_with(access_1, b"data_1")
    set_clean_block_mock.reset_mock()
    update_block_accessed_on_mock.assert_not_called()

    # Garbage collector (remove persisted block)
    choice_mock.side_effect = lambda _: mock_choice(access_2.id)

    local_storage.clean_block_cache[access_2.id]["persisted"] = True
    local_storage.clean_block_cache[access_2.id]["accessed_on"] = "2019"
    assert len(local_storage.clean_block_cache) == 3
    local_storage.set_clean_block(access_1, b"data_1")
    assert len(local_storage.clean_block_cache) == 3
    assert access_2.id not in local_storage.clean_block_cache
    assert local_storage.clean_block_cache[access_1.id] == {
        "access": access_1,
        "accessed_on": None,
        "block": b"data_1",
        "persisted": False,
    }
    assert local_storage.dirty_block_cache == {}
    set_clean_block_mock.assert_not_called()
    update_block_accessed_on_mock.assert_called_once_with(True, access_2, "2019")


@mock.patch("parsec.core.persistent_storage.PersistentStorage.clear_dirty_block")
def test_clear_dirty_block(clear_dirty_block_mock, local_storage):
    access = ManifestAccess()
    local_storage.clear_dirty_block(access)
    local_storage.clear_dirty_block(access)


@mock.patch("parsec.core.persistent_storage.PersistentStorage.clear_clean_block")
def test_clear_clean_block(clear_clean_block_mock, local_storage):
    access = ManifestAccess()
    local_storage.clear_clean_block(access)
    local_storage.clear_clean_block(access)
