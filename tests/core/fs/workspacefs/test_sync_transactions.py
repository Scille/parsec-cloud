# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from pendulum import Pendulum

from parsec.core.types import FsPath
from parsec.core.types import ManifestAccess, FolderManifest, LocalFolderManifest

from parsec.core.fs.workspacefs.sync_transactions import SynchronizationRequiredError
from parsec.core.fs.workspacefs.sync_transactions import merge_folder_children
from parsec.core.fs.workspacefs.sync_transactions import merge_folder_manifests


def test_merge_folder_children():
    m1 = ManifestAccess()
    m2 = ManifestAccess()
    a1 = {"a": m1}
    a2 = {"a": m2}
    b1 = {"b.txt": m1}
    b2 = {"b.txt": m2}
    c1 = {"c.tar.gz": m1}
    c2 = {"c.tar.gz": m2}

    # Empty folder
    assert merge_folder_children({}, {}, {}, "a@a") == {}

    # Adding children
    assert merge_folder_children({}, a1, {}, "a@a") == a1
    assert merge_folder_children({}, {}, a1, "a@a") == a1
    assert merge_folder_children({}, a1, a1, "a@a") == a1

    # Removing children
    assert merge_folder_children(a1, {}, a1, "a@a") == {}
    assert merge_folder_children(a1, a1, {}, "a@a") == {}
    assert merge_folder_children(a1, {}, {}, "a@a") == {}

    # Renaming children
    assert merge_folder_children(a1, a1, b1, "a@a") == b1
    assert merge_folder_children(a1, b1, a1, "a@a") == b1
    assert merge_folder_children(a1, b1, b1, "a@a") == b1

    # Conflicting renaming
    result = merge_folder_children(a1, b1, c1, "a@a")
    assert result == {"c (renamed by a@a).tar.gz": m1}

    # Conflicting names
    result = merge_folder_children({}, a1, a2, "a@a")
    assert result == {"a": m2, "a (conflicting with a@a)": m1}
    result = merge_folder_children({}, b1, b2, "a@a")
    assert result == {"b.txt": m2, "b (conflicting with a@a).txt": m1}
    result = merge_folder_children({}, c1, c2, "a@a")
    assert result == {"c.tar.gz": m2, "c (conflicting with a@a).tar.gz": m1}


def test_merge_folder_manifests():
    now = Pendulum.now()
    v1 = FolderManifest(author="b@b", version=1, created=now, updated=now, children={})

    # Initial base manifest
    m1 = v1.to_local("a@a")
    assert merge_folder_manifests(m1, v1) == m1

    # Local change
    m2 = m1.evolve_children_and_mark_updated({"a": ManifestAccess()})
    assert merge_folder_manifests(m2, v1) == m2

    # Successful upload
    v2 = m2.to_remote().evolve(version=2)
    m3 = merge_folder_manifests(m2, v1, v2)
    assert m3 == v2.to_local("a@a")

    # Two local changes
    m4 = m3.evolve_children_and_mark_updated({"b": ManifestAccess()})
    assert merge_folder_manifests(m4, v2) == m4
    m5 = m4.evolve_children_and_mark_updated({"c": ManifestAccess()})
    assert merge_folder_manifests(m4, v2) == m4

    # M4 has been successfully uploaded
    v3 = m4.to_remote().evolve(version=3)
    m6 = merge_folder_manifests(m5, v2, v3)
    assert m6 == m5.evolve(base_version=3)

    # The remote has changed
    v4 = v3.evolve(version=4, children={"d": ManifestAccess(), **v3.children}, author="b@b")
    m7 = merge_folder_manifests(m6, v3, v4)
    assert m7.base_version == 4
    assert sorted(m7.children) == ["a", "b", "c", "d"]
    assert m7.need_sync

    # Successful upload
    v5 = m7.to_remote().evolve(version=5)
    m8 = merge_folder_manifests(m7, v4, v5)
    assert m8 == v5.to_local("a@a")

    # The remote has changed
    v6 = v5.evolve(version=6, children={"e": ManifestAccess(), **v5.children}, author="b@b")
    m9 = merge_folder_manifests(m8, v5, v6)
    assert m9 == v6.to_local("a@a")


def test_merge_folder_manifests_with_a_placeholder():
    m1 = LocalFolderManifest("a@a")
    m2 = merge_folder_manifests(m1)
    assert m2 == m1
    v1 = m1.to_remote().evolve(version=1)

    m2a = merge_folder_manifests(m1, None, v1)
    assert m2a == v1.to_local(author="a@a")

    m2b = m1.evolve_children_and_mark_updated({"a": ManifestAccess()})
    m3b = merge_folder_manifests(m2b, None, v1)
    assert m3b == m2b.evolve(base_version=1)

    v2 = v1.evolve(version=2, author="b@b", children={"b": ManifestAccess()})
    m2c = m1.evolve_children_and_mark_updated({"a": ManifestAccess()})
    m3c = merge_folder_manifests(m2c, None, v2)
    children = {**v2.children, **m2c.children}
    assert m3c == m2c.evolve(base_version=2, children=children, updated=m3c.updated)


@pytest.mark.trio
async def test_folder_sync_transaction(sync_transactions, entry_transactions):
    folder_sync = sync_transactions.folder_sync
    access = entry_transactions.get_workspace_entry().access

    # Sync a placeholder
    manifest = await folder_sync(access)

    # Acknowledge a successful synchronization
    assert await folder_sync(access, manifest) is None

    # Local change
    a_id = await entry_transactions.folder_create(FsPath("/a"))

    # Can't sync parent with a placeholder child
    with pytest.raises(SynchronizationRequiredError) as context:
        manifest = await folder_sync(access)
    a_access = context.value.access
    assert a_access.id == a_id

    # Sync child
    a_manifest = await folder_sync(a_access)
    assert await folder_sync(a_access, a_manifest) is None

    # Try again
    manifest = await folder_sync(access)
    assert sorted(manifest.children) == ["a"]
    assert await folder_sync(access, manifest) is None

    # Local change
    b_id = await entry_transactions.folder_create(FsPath("/b"))

    # Remote change
    children = {**manifest.children, "c": ManifestAccess()}
    manifest = manifest.evolve(version=5, children=children, author="b@b")

    # Can't sync parent with a placeholder child
    with pytest.raises(SynchronizationRequiredError) as context:
        manifest = await folder_sync(access, manifest)
    b_access = context.value.access
    assert b_access.id == b_id

    # Sync child
    b_manifest = await folder_sync(b_access)
    assert await folder_sync(b_access, b_manifest) is None

    # Try again
    manifest = await folder_sync(access, manifest)
    assert sorted(manifest.children) == ["a", "b", "c"]
    assert await folder_sync(access, manifest) is None
