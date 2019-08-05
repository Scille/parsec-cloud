# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest

from parsec.types import DeviceID
from parsec.crypto import SecretKey
from parsec.core.fs.local_storage import LocalStorage
from parsec.core.fs import FSLocalMissError
from parsec.core.types import (
    LocalUserManifest,
    LocalWorkspaceManifest,
    LocalFolderManifest,
    LocalFileManifest,
    EntryID,
)


DEVICE = DeviceID("a@b")


def create_entry(type=LocalUserManifest):
    entry_id = EntryID()
    if type is LocalUserManifest:
        manifest = LocalUserManifest(
            author=DEVICE, base_version=0, is_placeholder=True, need_sync=True
        )
    elif type is LocalWorkspaceManifest:
        manifest = type.make_placeholder(entry_id=entry_id, author=DEVICE)
    else:
        manifest = type.make_placeholder(entry_id=entry_id, author=DEVICE, parent_id=EntryID())
    return entry_id, manifest


@pytest.fixture
def local_storage(tmpdir):
    key = SecretKey.generate()
    with LocalStorage(DEVICE, key, tmpdir) as db:
        yield db


def test_get_manifest(local_storage):
    entry_id, manifest = create_entry()
    with pytest.raises(FSLocalMissError):
        local_storage.get_manifest(entry_id)
    local_storage.local_manifest_cache[entry_id] = manifest
    assert local_storage.get_manifest(entry_id) == manifest


@pytest.mark.trio
async def test_set_manifest(local_storage):
    entry_id, manifest = create_entry()
    async with local_storage.lock_entry_id(entry_id):
        local_storage.set_manifest(entry_id, manifest)
    assert local_storage.local_manifest_cache[entry_id] == manifest


@pytest.mark.trio
async def test_clear_manifest(local_storage):
    entry_id, manifest = create_entry()
    async with local_storage.lock_entry_id(entry_id):
        local_storage.set_manifest(entry_id, manifest)
    assert local_storage.get_manifest(entry_id) == manifest

    local_storage.clear_manifest(entry_id)
    assert entry_id not in local_storage.local_manifest_cache
    with pytest.raises(FSLocalMissError):
        local_storage.get_manifest(entry_id)


@pytest.mark.parametrize(
    "type", [LocalUserManifest, LocalWorkspaceManifest, LocalFolderManifest, LocalFileManifest]
)
@pytest.mark.trio
async def test_get_manifest_with_cache_miss(local_storage, type):
    entry_id, manifest = create_entry(type)
    async with local_storage.lock_entry_id(entry_id):
        local_storage.set_manifest(entry_id, manifest)
    assert local_storage.get_manifest(entry_id) == manifest
    local_storage.clear_memory_cache()
    assert local_storage.get_manifest(entry_id) == manifest
