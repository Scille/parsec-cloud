# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import pytest
from pendulum import datetime

from parsec.api.protocol import RealmRole
from parsec.api.data import UserManifest, WorkspaceEntry
from parsec.core.types import LocalUserManifest
from parsec.core.fs.userfs.merging import merge_local_user_manifests


@pytest.fixture
def gen_date():
    curr = datetime(2000, 1, 1)

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

    w1 = WorkspaceEntry.new(name="w1", now=d2)
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
            encrypted_on=d5,
            encryption_revision=1,
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
    w1 = WorkspaceEntry.new(name="w1")
    d1, d2, d3, d4 = [gen_date() for _ in range(4)]

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
        w2 = WorkspaceEntry.new(name="w1")
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
    w1 = WorkspaceEntry.new(name="w1")
    w2 = WorkspaceEntry.new(name="w2")
    w3 = WorkspaceEntry.new(name="w3")

    diverged = LocalUserManifest.new_placeholder(
        alice.device_id, id=alice.user_manifest_id, now=d4, speculative=speculative_placeholder
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
