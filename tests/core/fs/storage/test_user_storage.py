# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import pytest

from parsec.core.fs.storage import UserStorage
from parsec.core.types import LocalUserManifest, EntryID

from tests.common import customize_fixtures


@pytest.fixture
def user_manifest(alice):
    timestamp = alice.timestamp()
    return LocalUserManifest.new_placeholder(
        alice.device_id, id=alice.user_manifest_id, timestamp=timestamp
    )


@pytest.fixture
async def alice_user_storage(data_base_dir, alice):
    async with UserStorage.run(data_base_dir, alice) as aus:
        yield aus


@pytest.mark.trio
@customize_fixtures(real_data_storage=True)
async def test_initialization(alice, data_base_dir, alice_user_storage, user_manifest):
    aus = alice_user_storage

    # Brand new user storage contains user manifest placeholder
    user_manifest_v0 = aus.get_user_manifest()
    assert user_manifest_v0.id == alice.user_manifest_id
    assert user_manifest_v0.need_sync
    assert user_manifest_v0.base.author == alice.device_id
    assert user_manifest_v0.base.id == alice.user_manifest_id
    assert user_manifest_v0.base.version == 0

    await aus.set_user_manifest(user_manifest)
    assert aus.get_user_manifest() == user_manifest
    async with UserStorage.run(data_base_dir, aus.device) as aus2:
        assert aus2.get_user_manifest() == user_manifest

    new_user_manifest = user_manifest.evolve(need_sync=False)
    await aus.set_user_manifest(new_user_manifest)
    assert aus.get_user_manifest() == new_user_manifest
    async with UserStorage.run(data_base_dir, aus.device) as aus2:
        assert aus2.get_user_manifest() == new_user_manifest


@pytest.mark.trio
@customize_fixtures(real_data_storage=True)
async def test_realm_checkpoint(alice, alice_user_storage, initial_user_manifest_state):
    aws = alice_user_storage
    user_manifest_id = alice.user_manifest_id

    # Brand new user storage contains user manifest placeholder
    assert await aws.get_realm_checkpoint() == 0
    assert await aws.get_need_sync_entries() == ({user_manifest_id}, set())

    # Modified entries not in storage should be ignored
    await aws.update_realm_checkpoint(1, {EntryID.new(): 2})
    assert await aws.get_realm_checkpoint() == 1
    assert await aws.get_need_sync_entries() == ({user_manifest_id}, set())

    # Another device updated the user manifest remotly
    await aws.update_realm_checkpoint(2, {user_manifest_id: 1})
    assert await aws.get_realm_checkpoint() == 2
    assert await aws.get_need_sync_entries() == ({user_manifest_id}, {user_manifest_id})

    # Load the up to date version of the user manifest
    manifest_v1 = initial_user_manifest_state.get_user_manifest_v1_for_device(alice)
    await aws.set_user_manifest(manifest_v1)
    assert await aws.get_realm_checkpoint() == 2
    assert await aws.get_need_sync_entries() == (set(), set())

    # Provide new remote changes
    await aws.update_realm_checkpoint(3, {user_manifest_id: 2})
    assert await aws.get_realm_checkpoint() == 3
    assert await aws.get_need_sync_entries() == (set(), {user_manifest_id})

    # Provide new local changes too
    manifest_v1_modified = manifest_v1.evolve(need_sync=True)
    await aws.set_user_manifest(manifest_v1_modified)
    assert await aws.get_realm_checkpoint() == 3
    assert await aws.get_need_sync_entries() == ({user_manifest_id}, {user_manifest_id})

    # Checkpoint's remote version should be ignored if manifest base version is greater
    manifest_v2 = manifest_v1_modified.evolve(base=manifest_v1.base.evolve(version=4))
    await aws.set_user_manifest(manifest_v2)
    assert await aws.get_realm_checkpoint() == 3
    assert await aws.get_need_sync_entries() == ({user_manifest_id}, set())


@pytest.mark.trio
@customize_fixtures(real_data_storage=True)
async def test_vacuum(alice_user_storage):
    # Should be no-op
    await alice_user_storage.run_vacuum()


@pytest.mark.trio
@customize_fixtures(real_data_storage=True)
async def test_storage_file_tree(tmp_path, alice):
    manifest_sqlite_db = tmp_path / alice.slug / "user_data-v1.sqlite"

    async with UserStorage.run(tmp_path, alice) as aus:
        assert aus.manifest_storage.path == manifest_sqlite_db

    assert manifest_sqlite_db.is_file()
