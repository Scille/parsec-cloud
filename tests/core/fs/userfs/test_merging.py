# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from pendulum import Pendulum

from parsec.core.fs.userfs.merging import merge_local_user_manifests, merge_remote_user_manifests
from parsec.core.types import LocalUserManifest, UserManifest, WorkspaceEntry


@pytest.fixture
def gen_date():
    curr = Pendulum(2000, 1, 1)

    def _gen():
        nonlocal curr
        while True:
            yield curr
            curr = curr.add(days=1)

    g = _gen()
    return lambda: next(g)


@pytest.mark.parametrize("with_ignored_changes", (False, True))
def test_merge_local_user_manifest_no_changes_in_diverged_placeholder(
    gen_date, alice, bob, with_ignored_changes
):
    w1 = WorkspaceEntry(name="w1")
    d1, d2, d3, d4 = [gen_date() for _ in range(4)]

    base = LocalUserManifest(alice.user_id, created=d1, updated=d2, workspaces=(w1,))
    if with_ignored_changes:
        diverged = base.evolve(updated=d4, author=bob.user_id, need_sync=True)
    else:
        diverged = base
    target = base.evolve(base_version=2, updated=d3, need_sync=False, is_placeholder=False)
    expected_merged = target

    merged = merge_local_user_manifests(None, diverged, target)
    assert merged == expected_merged


@pytest.mark.parametrize("with_ignored_changes", (False, True))
def test_merge_local_user_manifest_no_changes_in_diverged(
    gen_date, alice, bob, with_ignored_changes
):
    w1 = WorkspaceEntry(name="w1")
    d1, d2, d3, d4 = [gen_date() for _ in range(4)]

    base = LocalUserManifest(
        alice.user_id,
        created=d1,
        updated=d2,
        workspaces=(w1,),
        base_version=1,
        need_sync=False,
        is_placeholder=False,
    )
    if with_ignored_changes:
        diverged = base.evolve(updated=d4, author=bob.user_id, need_sync=True)
    else:
        diverged = base
    target = base.evolve(base_version=2, updated=d3, need_sync=False)
    expected_merged = target

    merged = merge_local_user_manifests(base, diverged, target)
    assert merged == expected_merged


@pytest.mark.parametrize("with_ignored_changes", (False, True))
def test_merge_remote_user_manifest_no_changes_in_diverged_placeholder(
    gen_date, alice, bob, with_ignored_changes
):
    d1, d2, d3, d4 = [gen_date() for _ in range(4)]
    w1 = WorkspaceEntry(name="w1")

    base = UserManifest(
        alice.user_id,
        version=1,
        created=d1,
        updated=d2,
        last_processed_message=10,
        workspaces=(w1,),
    )
    if with_ignored_changes:
        diverged = base.evolve(updated=d4, author=bob.user_id, version=1)
    else:
        diverged = base.evolve(version=1)
    target = base.evolve(version=3, updated=d3)
    expected_merged = target.evolve(version=target.version + 1)

    merged, need_sync = merge_remote_user_manifests(None, diverged, target)
    assert not need_sync
    assert merged == expected_merged


@pytest.mark.parametrize("with_ignored_changes", (False, True))
def test_merge_remote_user_manifest_no_changes_in_diverged(
    gen_date, alice, bob, with_ignored_changes
):
    d1, d2, d3, d4 = [gen_date() for _ in range(4)]
    w1 = WorkspaceEntry(name="w1")

    base = UserManifest(
        alice.user_id,
        version=1,
        created=d1,
        updated=d2,
        last_processed_message=10,
        workspaces=(w1,),
    )
    if with_ignored_changes:
        diverged = base.evolve(updated=d4, author=bob.user_id, version=2)
    else:
        diverged = base.evolve(version=2)
    target = base.evolve(version=3, updated=d3)
    expected_merged = target.evolve(version=target.version + 1)

    merged, need_sync = merge_remote_user_manifests(base, diverged, target)
    assert not need_sync
    assert merged == expected_merged


def test_merge_local_user_manifest_changes_placeholder(gen_date, alice):
    d1, d2, d3, d4 = [gen_date() for _ in range(4)]
    w1 = WorkspaceEntry(name="w1")
    w2 = WorkspaceEntry(name="w2")
    w3 = WorkspaceEntry(name="w3")

    base = LocalUserManifest(
        alice.user_id, base_version=0, created=d1, updated=d2, workspaces=(w1,)
    )
    diverged = base.evolve(updated=d4, workspaces=(w1, w3), last_processed_message=30)
    target = base.evolve(
        updated=d3,
        base_version=3,
        need_sync=False,
        is_placeholder=False,
        workspaces=(w1, w2),
        last_processed_message=20,
    )
    expected_merged = target.evolve(
        updated=d4,
        base_version=3,
        need_sync=True,
        workspaces=(w1, w2, w3),
        last_processed_message=30,
    )

    merged = merge_local_user_manifests(base, diverged, target)
    assert merged == expected_merged


def test_merge_remote_user_manifest_changes(gen_date, alice):
    d1, d2, d3, d4 = [gen_date() for _ in range(4)]
    w1 = WorkspaceEntry(name="w1")
    w2 = WorkspaceEntry(name="w2")
    w3 = WorkspaceEntry(name="w3")

    base = UserManifest(
        alice.user_id,
        version=1,
        created=d1,
        updated=d2,
        workspaces=(w1,),
        last_processed_message=10,
    )
    diverged = base.evolve(updated=d4, version=2, workspaces=(w1, w3), last_processed_message=20)
    target = base.evolve(updated=d3, version=3, workspaces=(w1, w2), last_processed_message=30)
    expected_merged = target.evolve(
        updated=d4, version=4, workspaces=(w1, w2, w3), last_processed_message=30
    )

    merged, need_sync = merge_remote_user_manifests(base, diverged, target)
    assert need_sync
    assert merged == expected_merged
