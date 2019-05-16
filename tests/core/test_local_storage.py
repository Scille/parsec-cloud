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
    local_storage.local_manifest_cache[access.id] = "manifest"
    assert local_storage.get_manifest(access) == "manifest"


def test_base_manifest(local_storage):
    access = ManifestAccess()
    manifest = LocalUserManifest(
        author="a@b", base_version=1, is_placeholder=False, need_sync=False
    )
    remote_manifest = manifest.to_remote()
    local_storage.set_base_manifest(access, remote_manifest)
    assert local_storage.get_base_manifest(access) == remote_manifest
    assert local_storage.get_manifest(access) == manifest

    assert local_storage.base_manifest_cache[access.id] == remote_manifest
    local_storage.base_manifest_cache.clear()

    assert local_storage.get_base_manifest(access) == remote_manifest
    assert local_storage.get_manifest(access) == manifest


def test_set_manifest(local_storage):
    access = ManifestAccess()
    manifest = LocalUserManifest(author=123, base_version=1, is_placeholder=False, need_sync=False)
    local_storage.set_manifest(access, manifest)
    assert local_storage.local_manifest_cache[access.id] == manifest


def test_clear_manifest(local_storage):
    access = ManifestAccess()
    manifest = LocalUserManifest(author=123, base_version=1, is_placeholder=False, need_sync=False)
    local_storage.set_base_manifest(access, manifest.to_remote())
    local_storage.set_manifest(access, manifest)

    assert local_storage.get_base_manifest(access)
    assert local_storage.get_manifest(access)

    local_storage.clear_manifest(access)
    assert access not in local_storage.base_manifest_cache
    assert access not in local_storage.local_manifest_cache
    with pytest.raises(LocalStorageMissingEntry):
        local_storage.get_base_manifest(access)
    with pytest.raises(LocalStorageMissingEntry):
        local_storage.get_manifest(access)
