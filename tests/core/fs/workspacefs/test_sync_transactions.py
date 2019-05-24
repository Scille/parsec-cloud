# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from pendulum import Pendulum

from parsec.core.types import FsPath
from parsec.core.types import EntryID, FolderManifest, LocalFolderManifest

from parsec.core.fs.workspacefs.sync_transactions import merge_manifests
from parsec.core.fs.workspacefs.sync_transactions import merge_folder_children


def test_merge_folder_children():
    m1 = EntryID()
    m2 = EntryID()
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


def test_merge_manifests():
    now = Pendulum.now()
    v1 = FolderManifest(author="b@b", version=1, created=now, updated=now, children={})

    # Initial base manifest
    m1 = v1.to_local("a@a")
    assert merge_manifests(m1, v1) == m1

    # Local change
    m2 = m1.evolve_children_and_mark_updated({"a": EntryID()})
    assert merge_manifests(m2, v1) == m2

    # Successful upload
    v2 = m2.to_remote().evolve(version=2)
    m3 = merge_manifests(m2, v1, v2)
    assert m3 == v2.to_local("a@a")

    # Two local changes
    m4 = m3.evolve_children_and_mark_updated({"b": EntryID()})
    assert merge_manifests(m4, v2) == m4
    m5 = m4.evolve_children_and_mark_updated({"c": EntryID()})
    assert merge_manifests(m4, v2) == m4

    # M4 has been successfully uploaded
    v3 = m4.to_remote().evolve(version=3)
    m6 = merge_manifests(m5, v2, v3)
    assert m6 == m5.evolve(base_version=3)

    # The remote has changed
    v4 = v3.evolve(version=4, children={"d": EntryID(), **v3.children}, author="b@b")
    m7 = merge_manifests(m6, v3, v4)
    assert m7.base_version == 4
    assert sorted(m7.children) == ["a", "b", "c", "d"]
    assert m7.need_sync

    # Successful upload
    v5 = m7.to_remote().evolve(version=5)
    m8 = merge_manifests(m7, v4, v5)
    assert m8 == v5.to_local("a@a")

    # The remote has changed
    v6 = v5.evolve(version=6, children={"e": EntryID(), **v5.children}, author="b@b")
    m9 = merge_manifests(m8, v5, v6)
    assert m9 == v6.to_local("a@a")


def test_merge_manifests_with_a_placeholder():
    m1 = LocalFolderManifest("a@a")
    m2 = merge_manifests(m1)
    assert m2 == m1
    v1 = m1.to_remote().evolve(version=1)

    m2a = merge_manifests(m1, None, v1)
    assert m2a == v1.to_local(author="a@a")

    m2b = m1.evolve_children_and_mark_updated({"a": EntryID()})
    m3b = merge_manifests(m2b, None, v1)
    assert m3b == m2b.evolve(base_version=1, is_placeholder=False)

    v2 = v1.evolve(version=2, author="b@b", children={"b": EntryID()})
    m2c = m1.evolve_children_and_mark_updated({"a": EntryID()})
    m3c = merge_manifests(m2c, None, v2)
    children = {**v2.children, **m2c.children}
    assert m3c == m2c.evolve(
        is_placeholder=False, base_version=2, children=children, updated=m3c.updated
    )


@pytest.mark.trio
async def test_synchronization_step_transaction(sync_transactions, entry_transactions):
    synchronization_step = sync_transactions.synchronization_step
    entry_id = entry_transactions.get_workspace_entry().id

    # Sync a placeholder
    manifest = await synchronization_step(entry_id)

    # Acknowledge a successful synchronization
    assert await synchronization_step(entry_id, manifest) is None

    # Local change
    a_id = await entry_transactions.folder_create(FsPath("/a"))

    # Sync parent with a placeholder child
    manifest = await synchronization_step(entry_id)
    children = []
    for child in sync_transactions.get_placeholder_children(manifest):
        children.append(child)
    a_entry_id, = children
    assert a_entry_id == a_id

    # Sync child
    a_manifest = await synchronization_step(a_entry_id)
    assert await synchronization_step(a_entry_id, a_manifest) is None

    # Acknowledge the manifest
    assert sorted(manifest.children) == ["a"]
    assert await synchronization_step(entry_id, manifest) is None

    # Local change
    b_id = await entry_transactions.folder_create(FsPath("/b"))

    # Remote change
    children = {**manifest.children, "c": EntryID()}
    manifest = manifest.evolve(version=5, children=children, author="b@b")

    # Sync parent with a placeholder child
    manifest = await synchronization_step(entry_id, manifest)
    children = []
    for child in sync_transactions.get_placeholder_children(manifest):
        children.append(child)
    b_entry_id, = children
    assert b_entry_id == b_id

    # Sync child
    b_manifest = await synchronization_step(b_entry_id)
    assert await synchronization_step(b_entry_id, b_manifest) is None

    # Acknowledge the manifest
    assert sorted(manifest.children) == ["a", "b", "c"]
    assert await synchronization_step(entry_id, manifest) is None
