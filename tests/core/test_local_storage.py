# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest

from parsec.types import DeviceID
from parsec.crypto import SecretKey
from parsec.core.local_storage import LocalStorage
from parsec.core.persistent_storage import LocalStorageMissingError
from parsec.core.types import LocalUserManifest, EntryID


DEVICE = DeviceID("a@b")


@pytest.fixture
def local_storage(tmpdir):
    key = SecretKey.generate()
    with LocalStorage(DEVICE, key, tmpdir) as db:
        yield db


def test_get_manifest(local_storage):
    entry_id = EntryID()
    with pytest.raises(LocalStorageMissingError):
        local_storage.get_manifest(entry_id)
    local_storage.local_manifest_cache[entry_id] = "manifest"
    assert local_storage.get_manifest(entry_id) == "manifest"


def test_base_manifest(local_storage):
    entry_id = EntryID()
    manifest = LocalUserManifest(
        author=DEVICE, base_version=1, is_placeholder=False, need_sync=False
    )
    remote_manifest = manifest.to_remote()
    local_storage.set_base_manifest(entry_id, remote_manifest)
    assert local_storage.get_base_manifest(entry_id) == remote_manifest
    assert local_storage.get_manifest(entry_id) == manifest

    assert local_storage.base_manifest_cache[entry_id] == remote_manifest
    local_storage.base_manifest_cache.clear()

    assert local_storage.get_base_manifest(entry_id) == remote_manifest
    assert local_storage.get_manifest(entry_id) == manifest


def test_set_manifest(local_storage):
    entry_id = EntryID()
    manifest = LocalUserManifest(
        author=DEVICE, base_version=1, is_placeholder=False, need_sync=False
    )
    local_storage.set_manifest(entry_id, manifest)
    assert local_storage.local_manifest_cache[entry_id] == manifest


def test_clear_manifest(local_storage):
    entry_id = EntryID()
    manifest = LocalUserManifest(
        author=DEVICE, base_version=1, is_placeholder=False, need_sync=False
    )
    local_storage.set_base_manifest(entry_id, manifest.to_remote())
    local_storage.set_manifest(entry_id, manifest)

    assert local_storage.get_base_manifest(entry_id)
    assert local_storage.get_manifest(entry_id)

    local_storage.clear_manifest(entry_id)
    assert entry_id not in local_storage.base_manifest_cache
    assert entry_id not in local_storage.local_manifest_cache
    with pytest.raises(LocalStorageMissingError):
        local_storage.get_base_manifest(entry_id)
    with pytest.raises(LocalStorageMissingError):
        local_storage.get_manifest(entry_id)
