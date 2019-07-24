# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from pendulum import Pendulum

from parsec.core.types import FsPath, BlockAccess, EntryID
from parsec.core.types import FolderManifest, FileManifest, LocalFolderManifest, LocalFileManifest
from parsec.core.local_storage import LocalStorageMissingError

from parsec.core.fs.workspacefs.sync_transactions import merge_manifests
from parsec.core.fs.workspacefs.sync_transactions import merge_folder_children
from parsec.core.fs.exceptions import FSReshapingRequiredError, FSFileConflictError


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


def test_merge_folder_manifests():
    now = Pendulum.now()
    v1 = FolderManifest(
        author="b@b", parent_id=EntryID(), version=1, created=now, updated=now, children={}
    )

    # Initial base manifest
    m1 = v1.to_local("a@a")
    assert merge_manifests(m1) == m1

    # Local change
    m2 = m1.evolve_children_and_mark_updated({"a": EntryID()})
    assert merge_manifests(m2) == m2

    # Successful upload
    v2 = m2.to_remote().evolve(version=2)
    m3 = merge_manifests(m2, v2)
    assert m3 == v2.to_local("a@a")

    # Two local changes
    m4 = m3.evolve_children_and_mark_updated({"b": EntryID()})
    assert merge_manifests(m4) == m4
    m5 = m4.evolve_children_and_mark_updated({"c": EntryID()})
    assert merge_manifests(m4) == m4

    # M4 has been successfully uploaded
    v3 = m4.to_remote().evolve(version=3)
    m6 = merge_manifests(m5, v3)
    assert m6 == m5.evolve(base_manifest=v3)

    # The remote has changed
    v4 = v3.evolve(version=4, children={"d": EntryID(), **v3.children}, author="b@b")
    m7 = merge_manifests(m6, v4)
    assert m7.base_version == 4
    assert sorted(m7.children) == ["a", "b", "c", "d"]
    assert m7.need_sync

    # Successful upload
    v5 = m7.to_remote().evolve(version=5)
    m8 = merge_manifests(m7, v5)
    assert m8 == v5.to_local("a@a")

    # The remote has changed
    v6 = v5.evolve(version=6, children={"e": EntryID(), **v5.children}, author="b@b")
    m9 = merge_manifests(m8, v6)
    assert m9 == v6.to_local("a@a")


def test_merge_manifests_with_a_placeholder():
    m1 = LocalFolderManifest.make_placeholder("a@a", parent_id=EntryID())
    m2 = merge_manifests(m1)
    assert m2 == m1
    v1 = m1.to_remote().evolve(version=1)

    m2a = merge_manifests(m1, v1)
    assert m2a == v1.to_local(author="a@a")

    m2b = m1.evolve_children_and_mark_updated({"a": EntryID()})
    m3b = merge_manifests(m2b, v1)
    assert m3b == m2b.evolve(base_manifest=v1)

    v2 = v1.evolve(version=2, author="b@b", children={"b": EntryID()})
    m2c = m1.evolve_children_and_mark_updated({"a": EntryID()})
    m3c = merge_manifests(m2c, v2)
    children = {**v2.children, **m2c.children}
    assert m3c == m2c.evolve(base_manifest=v2, children=children, updated=m3c.updated)


def test_merge_file_manifests():
    now = Pendulum.now()
    v1 = FileManifest(
        author="b@b", parent_id=EntryID(), version=1, created=now, updated=now, blocks=[], size=0
    )

    # Initial base manifest
    m1 = v1.to_local("a@a")
    assert merge_manifests(m1) == m1

    # Local change
    m2 = m1.evolve_and_mark_updated(size=1)
    assert merge_manifests(m2) == m2

    # Successful upload
    v2 = m2.to_remote().evolve(version=2)
    m3 = merge_manifests(m2, v2)
    assert m3 == v2.to_local("a@a")

    # Two local changes
    m4 = m3.evolve_and_mark_updated(size=2)
    assert merge_manifests(m4) == m4
    m5 = m4.evolve_and_mark_updated(size=3)
    assert merge_manifests(m4) == m4

    # M4 has been successfully uploaded
    v3 = m4.to_remote().evolve(version=3)
    m6 = merge_manifests(m5, v3)
    assert m6 == m5.evolve(base_manifest=v3)

    # The remote has changed
    v4 = v3.evolve(version=4, size=4, author="b@b")
    with pytest.raises(FSFileConflictError):
        merge_manifests(m6, v4)


@pytest.mark.trio
@pytest.mark.parametrize("type", ["file", "folder"])
async def test_synchronization_step_transaction(
    sync_transactions, entry_transactions, file_transactions, type
):
    synchronization_step = sync_transactions.synchronization_step
    entry_id = entry_transactions.get_workspace_entry().id

    # Sync a placeholder
    manifest = await synchronization_step(entry_id)

    # Acknowledge a successful synchronization
    assert await synchronization_step(entry_id, manifest) is None

    # Local change
    if type == "file":
        a_id, fd = await entry_transactions.file_create(FsPath("/a"))
        await file_transactions.fd_write(fd, b"abc", 0)
        await file_transactions.fd_close(fd)
    else:
        a_id = await entry_transactions.folder_create(FsPath("/a"))

    # Sync parent with a placeholder child
    manifest = await synchronization_step(entry_id)
    children = []
    for child in sync_transactions.get_placeholder_children(manifest):
        children.append(child)
    a_entry_id, = children
    assert a_entry_id == a_id

    # Sync child
    if type == "file":
        with pytest.raises(FSReshapingRequiredError):
            await synchronization_step(a_entry_id)
        await sync_transactions.file_reshape(a_entry_id)
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


def test_reshape_blocks(sync_transactions):
    # No block
    placeholder = LocalFileManifest.make_placeholder("a@a", EntryID())
    manifest = placeholder.evolve()

    blocks, old_blocks, new_blocks, missing = sync_transactions._reshape_blocks(manifest)
    assert blocks == old_blocks == new_blocks == missing == []

    # One block
    access = BlockAccess.from_block(b"abc", 0)
    manifest = placeholder.evolve(blocks=[access], dirty_blocks=[], size=3)

    blocks, old_blocks, new_blocks, missing = sync_transactions._reshape_blocks(manifest)
    assert old_blocks == new_blocks == missing == []
    assert blocks == [access]

    # One dirty block
    access = BlockAccess.from_block(b"abc", 0)
    manifest = placeholder.evolve(blocks=[], dirty_blocks=[access], size=3)

    blocks, old_blocks, new_blocks, missing = sync_transactions._reshape_blocks(manifest)
    assert old_blocks == new_blocks == missing == []
    assert blocks == [access]

    # Several dirty blocks
    size = 0
    dirty = []
    for data in [b"abc", b"def", b"ghi"]:
        access = BlockAccess.from_block(data, size)
        sync_transactions.local_storage.set_dirty_block(access.id, data)
        size += len(data)
        dirty.append(access)
    manifest = placeholder.evolve(blocks=[], dirty_blocks=dirty, size=size)

    blocks, old_blocks, new_blocks, missing = sync_transactions._reshape_blocks(manifest)
    assert missing == []
    assert old_blocks == dirty
    assert len(new_blocks) == 1
    (access, data), = new_blocks
    assert data == b"abcdefghi"
    assert blocks == [access]

    # One block - several dirty blocks
    size = 0
    data = b"abc"
    dirty = []
    clean = [BlockAccess.from_block(data, size)]
    sync_transactions.local_storage.set_clean_block(clean[0].id, data)
    size += len(data)
    for data in [b"def", b"ghi"]:
        access = BlockAccess.from_block(data, size)
        sync_transactions.local_storage.set_dirty_block(access.id, data)
        size += len(data)
        dirty.append(access)
    manifest = placeholder.evolve(blocks=clean, dirty_blocks=dirty, size=size)

    blocks, old_blocks, new_blocks, missing = sync_transactions._reshape_blocks(manifest)
    assert missing == []
    assert old_blocks == clean + dirty
    assert len(new_blocks) == 1
    (access, data), = new_blocks
    assert data == b"abcdefghi"
    assert blocks == [access]

    # One missing block - several dirty blocks
    size = 0
    data = b"abc"
    dirty = []
    clean = [BlockAccess.from_block(data, size)]
    size += len(data)
    for data in [b"def", b"ghi"]:
        access = BlockAccess.from_block(data, size)
        sync_transactions.local_storage.set_dirty_block(access.id, data)
        size += len(data)
        dirty.append(access)
    manifest = placeholder.evolve(blocks=clean, dirty_blocks=dirty, size=size)

    blocks, old_blocks, new_blocks, missing = sync_transactions._reshape_blocks(manifest)
    assert missing == clean
    assert old_blocks == dirty
    assert len(new_blocks) == 1
    (access, data), = new_blocks
    assert data == b"\x00\x00\x00defghi"
    assert blocks == [access]


@pytest.mark.trio
async def test_file_reshape(sync_transactions):
    # Prepare the backend
    workspace_id = sync_transactions.remote_loader.workspace_id
    device_id = sync_transactions.local_storage.device_id
    manifest = await sync_transactions.get_minimal_remote_manifest(workspace_id)
    await sync_transactions.remote_loader.create_realm(workspace_id)
    await sync_transactions.remote_loader.upload_manifest(workspace_id, manifest)

    # No block
    entry_id = EntryID()
    placeholder = LocalFileManifest.make_placeholder(device_id, entry_id)
    manifest = placeholder.evolve()
    sync_transactions.local_storage.set_manifest(entry_id, manifest)

    await sync_transactions.file_reshape(entry_id)

    # One block
    entry_id = EntryID()
    access = BlockAccess.from_block(b"abc", 0)
    manifest = placeholder.evolve(blocks=[access], dirty_blocks=[], size=3)
    sync_transactions.local_storage.set_manifest(entry_id, manifest)

    await sync_transactions.file_reshape(entry_id)

    # One dirty block
    entry_id = EntryID()
    access = BlockAccess.from_block(b"abc", 0)
    manifest = placeholder.evolve(blocks=[], dirty_blocks=[access], size=3)
    sync_transactions.local_storage.set_manifest(entry_id, manifest)

    await sync_transactions.file_reshape(entry_id)
    new_manifest = sync_transactions.local_storage.get_manifest(entry_id)
    assert new_manifest.blocks == (access,)
    assert new_manifest.dirty_blocks == ()

    # Several dirty blocks
    entry_id = EntryID()
    size = 0
    dirty = []
    for data in [b"abc", b"def", b"ghi"]:
        access = BlockAccess.from_block(data, size)
        sync_transactions.local_storage.set_dirty_block(access.id, data)
        size += len(data)
        dirty.append(access)
    manifest = placeholder.evolve(blocks=[], dirty_blocks=dirty, size=size)
    sync_transactions.local_storage.set_manifest(entry_id, manifest)

    await sync_transactions.file_reshape(entry_id)
    new_manifest = sync_transactions.local_storage.get_manifest(entry_id)
    assert new_manifest.dirty_blocks == ()
    assert len(new_manifest.blocks) == 1
    access, = new_manifest.blocks
    assert sync_transactions.local_storage.get_block(access.id) == b"abcdefghi"
    for access in dirty:
        with pytest.raises(LocalStorageMissingError):
            sync_transactions.local_storage.get_block(access.id)

    # One missing block - several dirty blocks
    entry_id = EntryID()
    size = 0
    data = b"abc"
    dirty = []
    clean = [BlockAccess.from_block(data, size)]
    await sync_transactions.remote_loader.upload_block(clean[0], data)
    sync_transactions.local_storage.clear_block(clean[0].id)
    size += len(data)
    for data in [b"def", b"ghi"]:
        access = BlockAccess.from_block(data, size)
        sync_transactions.local_storage.set_dirty_block(access.id, data)
        size += len(data)
        dirty.append(access)
    manifest = placeholder.evolve(blocks=clean, dirty_blocks=dirty, size=size)
    sync_transactions.local_storage.set_manifest(entry_id, manifest)

    await sync_transactions.file_reshape(entry_id)
    new_manifest = sync_transactions.local_storage.get_manifest(entry_id)
    assert new_manifest.dirty_blocks == ()
    assert len(new_manifest.blocks) == 1
    access, = new_manifest.blocks
    assert sync_transactions.local_storage.get_block(access.id) == b"abcdefghi"
    for access in dirty:
        with pytest.raises(LocalStorageMissingError):
            sync_transactions.local_storage.get_block(access.id)


@pytest.mark.trio
async def test_get_minimal_remote_manifest(
    sync_transactions, entry_transactions, file_transactions
):
    # Prepare
    w_id = sync_transactions.workspace_id
    a_id, fd = await entry_transactions.file_create(FsPath("/a"))
    await file_transactions.fd_write(fd, b"abc", 0)
    await file_transactions.fd_close(fd)
    b_id = await entry_transactions.folder_create(FsPath("/b"))
    c_id = await entry_transactions.folder_create(FsPath("/b/c"))

    # Workspace manifest
    minimal = await sync_transactions.get_minimal_remote_manifest(w_id)
    local = sync_transactions.local_storage.get_manifest(w_id)
    expected = local.to_remote().evolve(version=1, children={}, updated=local.created)
    assert minimal == expected

    await sync_transactions.synchronization_step(w_id, minimal)
    assert await sync_transactions.get_minimal_remote_manifest(w_id) is None

    # File manifest
    minimal = await sync_transactions.get_minimal_remote_manifest(a_id)
    local = sync_transactions.local_storage.get_manifest(a_id)
    assert minimal == local.to_remote().evolve(version=1, blocks=[], updated=local.created, size=0)
    await sync_transactions.file_reshape(a_id)
    await sync_transactions.synchronization_step(a_id, minimal)
    assert await sync_transactions.get_minimal_remote_manifest(a_id) is None

    # Folder manifest
    minimal = await sync_transactions.get_minimal_remote_manifest(b_id)
    local = sync_transactions.local_storage.get_manifest(b_id)
    assert minimal == local.to_remote().evolve(version=1, children={}, updated=local.created)
    await sync_transactions.synchronization_step(b_id, minimal)
    assert await sync_transactions.get_minimal_remote_manifest(b_id) is None

    # Empty folder manifest
    minimal = await sync_transactions.get_minimal_remote_manifest(c_id)
    local = sync_transactions.local_storage.get_manifest(c_id)
    assert minimal == local.to_remote().evolve(version=1)
    await sync_transactions.synchronization_step(c_id, minimal)
    assert await sync_transactions.get_minimal_remote_manifest(c_id) is None


@pytest.mark.trio
async def test_file_conflict(sync_transactions, entry_transactions, file_transactions):
    # Prepare
    a_id, fd = await entry_transactions.file_create(FsPath("/a"))
    await file_transactions.fd_write(fd, b"abc", offset=0)
    await sync_transactions.file_reshape(a_id)
    remote = await sync_transactions.synchronization_step(a_id)
    assert await sync_transactions.synchronization_step(a_id, remote) is None
    await file_transactions.fd_write(fd, b"def", offset=3)
    await sync_transactions.file_reshape(a_id)
    changed_remote = remote.evolve(version=2, blocks=[], size=0, author="b@b")

    # Try a synchronization
    with pytest.raises(FSFileConflictError) as ctx:
        await sync_transactions.synchronization_step(a_id, changed_remote)
    local, remote = ctx.value.args

    # Write some more
    await file_transactions.fd_write(fd, b"ghi", offset=6)

    # Also create a fake previous conflict file
    await entry_transactions.file_create(FsPath("/a (conflicting with b@b)"), open=False)

    # Solve conflict
    await sync_transactions.file_conflict(a_id, local, remote)
    assert await file_transactions.fd_read(fd, size=-1, offset=0) == b""
    a2_id, fd2 = await entry_transactions.file_open(FsPath("/a (conflicting with b@b - 2)"))
    assert await file_transactions.fd_read(fd2, size=-1, offset=0) == b"abcdefghi"

    # Finish synchronization
    await sync_transactions.file_reshape(a2_id)
    assert await sync_transactions.synchronization_step(a_id, changed_remote) is None
    remote2 = await sync_transactions.synchronization_step(a2_id)
    assert await sync_transactions.synchronization_step(a2_id, remote2) is None

    # Create a new conflict then remove a
    await file_transactions.fd_write(fd, b"abc", 0)
    await sync_transactions.file_reshape(a_id)
    changed_remote = changed_remote.evolve(version=3)
    await entry_transactions.file_delete(FsPath("/a"))

    # Conflict solving should still succeed
    with pytest.raises(FSFileConflictError) as ctx:
        await sync_transactions.synchronization_step(a_id, changed_remote)
    local, remote = ctx.value.args
    await sync_transactions.file_conflict(a_id, local, remote)

    # Close fds
    await file_transactions.fd_close(fd)
    await file_transactions.fd_close(fd2)
