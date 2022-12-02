# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import pytest

from parsec._parsec import DateTime
from parsec.api.data import EntryName, UserManifest, WorkspaceEntry
from parsec.api.protocol import RealmRole
from parsec.core.fs.userfs.merging import merge_local_user_manifests
from parsec.core.types import LocalUserManifest


@pytest.fixture
def gen_date():
    curr = DateTime(2000, 1, 1)

    def _gen():
        nonlocal curr
        while True:
            yield curr
            curr = curr.add(days=1)

    g = _gen()
    return lambda: next(g)


@pytest.mark.parametrize("with_ignored_changes", (False, True))
def test_merge_local_user_manifest_no_changes_in_diverged_placeholder(
    gen_date, alice, alice2, with_ignored_changes
):
    d1, d2, d3, d4, d5, d6, d7 = [gen_date() for _ in range(7)]

    w1 = WorkspaceEntry.new(name=EntryName("w1"), timestamp=d2)
    base = UserManifest(
        author=alice.device_id,
        timestamp=d4,
        id=alice.user_manifest_id,
        version=1,
        created=d1,
        updated=d3,
        last_processed_message=0,
        workspaces=(w1,),
    )

    diverged = LocalUserManifest.from_remote(base)
    if with_ignored_changes:
        w1_bis = WorkspaceEntry(
            name=w1.name,
            id=w1.id,
            key=w1.key,
            # Same encryption revision than remote (so encryption date should be ignored)
            encryption_revision=1,
            encrypted_on=d5,
            # Cache older than remote
            role_cached_on=d1,
            role=RealmRole.MANAGER,
        )
        diverged = diverged.evolve(updated=d4, need_sync=True, workspaces=(w1_bis,))

    target = UserManifest(
        author=alice2.device_id,
        timestamp=d7,
        id=alice2.user_manifest_id,
        version=2,
        created=d1,
        updated=d6,
        last_processed_message=0,
        workspaces=(w1,),
    )

    expected_merged = LocalUserManifest.from_remote(target)
    merged = merge_local_user_manifests(diverged, target)
    assert merged == expected_merged


@pytest.mark.parametrize("with_local_changes", (False, True))
def test_created_field_modified_by_remote(gen_date, alice, with_local_changes):
    d1, d2, d3, d4 = [gen_date() for _ in range(4)]

    w1 = WorkspaceEntry.new(name=EntryName("w1"), timestamp=d2)
    base = UserManifest(
        author=alice.device_id,
        timestamp=d2,
        id=alice.user_manifest_id,
        version=1,
        created=d1,
        updated=d2,
        last_processed_message=0,
        workspaces=(w1,),
    )

    local = LocalUserManifest.from_remote(base)
    if with_local_changes:
        w2 = WorkspaceEntry.new(name=EntryName("w1"), timestamp=d3)
        local = local.evolve(
            need_sync=True, updated=d3, last_processed_message=1, workspaces=(w1, w2)
        )

    target = base.evolve(created=d4, version=2)

    expected_merged = local.evolve(base=target)
    merged = merge_local_user_manifests(local, target)
    # Remote always control the value of the create field
    assert merged == expected_merged


@pytest.mark.parametrize("speculative_placeholder", (False, True))
def test_merge_local_user_manifest_changes_placeholder(gen_date, alice, speculative_placeholder):
    d1, d2, d3, d4 = [gen_date() for _ in range(4)]

    w1 = WorkspaceEntry.new(name=EntryName("w1"), timestamp=d2)
    w2 = WorkspaceEntry.new(name=EntryName("w2"), timestamp=d2)
    w3 = WorkspaceEntry.new(name=EntryName("w3"), timestamp=d2)

    diverged = LocalUserManifest.new_placeholder(
        alice.device_id,
        id=alice.user_manifest_id,
        timestamp=d4,
        speculative=speculative_placeholder,
    ).evolve(last_processed_message=30, workspaces=(w1, w3))
    target = UserManifest(
        author=alice.device_id,
        timestamp=d2,
        id=alice.user_manifest_id,
        version=3,
        created=d1,
        updated=d3,
        last_processed_message=20,
        workspaces=(w1, w2),
    )
    expected_merged = LocalUserManifest(
        base=target,
        updated=d4,
        last_processed_message=30,
        workspaces=(w1, w2, w3),
        need_sync=True,
        speculative=False,
    )

    merged = merge_local_user_manifests(diverged, target)
    assert merged == expected_merged


# Guessing by it name, this test is directed by M. Night Shyamalan ;-)
@pytest.mark.parametrize("local_changes", (False, True))
def test_merge_speculative_with_it_unsuspected_former_self(alice, local_changes):
    d1 = DateTime(2000, 1, 1)
    d2 = DateTime(2000, 1, 2)
    d3 = DateTime(2000, 1, 3)

    # 1) User manifest is originally created by our device
    local = LocalUserManifest.new_placeholder(
        author=alice.device_id, id=alice.user_manifest_id, timestamp=d1, speculative=False
    )
    w1 = WorkspaceEntry.new(EntryName("foo"), timestamp=d1)
    local = local.evolve(workspaces=(w1,), last_processed_message=1)

    # 2) We sync the user manifest
    v1 = local.to_remote(author=alice.device_id, timestamp=d2)

    # 3) Now let's pretend we lost local storage, hence creating a new speculative manifest
    new_local = LocalUserManifest.new_placeholder(
        author=alice.device_id, id=alice.user_manifest_id, timestamp=d3, speculative=True
    )
    if local_changes:
        w2 = WorkspaceEntry.new(EntryName("bar"), timestamp=d3)
        new_local = new_local.evolve(workspaces=(w2,), last_processed_message=2)

    # 4) When syncing the manifest, we shouldn't remove any data from the remote
    merged = merge_local_user_manifests(new_local, v1)

    if local_changes:
        assert merged == LocalUserManifest(
            base=v1,
            updated=d3,
            last_processed_message=2,
            workspaces=(w2, w1),
            need_sync=True,
            speculative=False,
        )
    else:
        assert merged == LocalUserManifest(
            base=v1,
            updated=v1.updated,
            last_processed_message=1,
            workspaces=(w1,),
            need_sync=False,
            speculative=False,
        )
