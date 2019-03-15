# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from collections import OrderedDict
import pytest
from unittest import mock

from parsec.core.local_storage import LocalStorage
from parsec.core.persistent_storage import LocalStorageMissingEntry
from parsec.core.types import LocalUserManifest, ManifestAccess


@pytest.fixture
def local_storage(tmpdir):
    with LocalStorage(tmpdir) as db:
        yield db


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
    local_storage.manifest_cache = OrderedDict()
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
    local_storage.manifest_cache == OrderedDict({access.id: manifest})
    local_storage.clear_manifest(access)
    clear_clean_manifest_mock.assert_called_once_with(access)
    clear_dirty_manifest_mock.assert_not_called()
    clear_clean_manifest_mock.reset_mock()
    assert local_storage.manifest_cache == OrderedDict()

    local_storage.manifest_cache == OrderedDict({access.id: manifest})
    clear_clean_manifest_mock.side_effect = LocalStorageMissingEntry(access)
    local_storage.clear_manifest(access)
    clear_clean_manifest_mock.assert_called_once_with(access)
    clear_dirty_manifest_mock.assert_called_once_with(access)
    assert local_storage.manifest_cache == OrderedDict()

    clear_clean_manifest_mock.side_effect = LocalStorageMissingEntry(access)
    clear_dirty_manifest_mock.side_effect = LocalStorageMissingEntry(access)
    local_storage.clear_manifest(access)
