# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

from pathlib import Path

import pytest

from parsec.core.fs.storage import UserStorage
from parsec.core.types import LocalUserManifest, EntryID


@pytest.fixture
def user_manifest(alice):
    return LocalUserManifest.new_placeholder(id=alice.user_manifest_id)


@pytest.fixture
async def alice_user_storage(tmpdir, alice):
    async with UserStorage.run(alice, tmpdir) as aus:
        yield aus


@pytest.mark.trio
async def test_initialization(alice_user_storage, user_manifest):
    aus = alice_user_storage
    assert aus.get_user_manifest() != user_manifest

    await aus.set_user_manifest(user_manifest)
    assert aus.get_user_manifest() == user_manifest
    async with UserStorage.run(aus.device, aus.path) as aus2:
        assert aus2.get_user_manifest() == user_manifest

    new_user_manifest = user_manifest.evolve(need_sync=False)
    await aus.set_user_manifest(new_user_manifest)
    assert aus.get_user_manifest() == new_user_manifest
    async with UserStorage.run(aus.device, aus.path) as aus2:
        assert aus2.get_user_manifest() == new_user_manifest


@pytest.mark.trio
async def test_realm_checkpoint(alice_user_storage, user_manifest):
    aws = alice_user_storage
    manifest = user_manifest

    assert await aws.get_realm_checkpoint() == 0
    assert await aws.get_need_sync_entries() == (set(), set())

    await aws.update_realm_checkpoint(11, {manifest.id: 22, EntryID(): 33})

    assert await aws.get_realm_checkpoint() == 11
    assert await aws.get_need_sync_entries() == (set(), set())

    await aws.set_user_manifest(manifest)

    assert await aws.get_realm_checkpoint() == 11
    assert await aws.get_need_sync_entries() == (set([manifest.id]), set())

    await aws.set_user_manifest(manifest.evolve(need_sync=False))

    assert await aws.get_realm_checkpoint() == 11
    assert await aws.get_need_sync_entries() == (set(), set())

    await aws.update_realm_checkpoint(44, {manifest.id: 55, EntryID(): 66})

    assert await aws.get_realm_checkpoint() == 44
    assert await aws.get_need_sync_entries() == (set(), set([manifest.id]))


@pytest.mark.trio
async def test_vacuum(alice_user_storage):
    # Should be no-op
    await alice_user_storage.run_vacuum()


@pytest.mark.trio
async def test_storage_file_tree(tmpdir, alice):
    path = Path(tmpdir)
    manifest_sqlite_db = path / "user_data-v1.sqlite"

    async with UserStorage.run(alice, tmpdir) as aus:
        assert aus.manifest_storage.path == manifest_sqlite_db

    assert set(path.iterdir()) == {manifest_sqlite_db}
