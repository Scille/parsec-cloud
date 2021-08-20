# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2016-2021 Scille SAS

import pytest

from parsec.core.core_events import CoreEvent
from parsec.core.types import FsPath, EntryID

from parsec.core.fs.exceptions import FSFileConflictError


@pytest.mark.trio
@pytest.mark.parametrize("type", ["file", "folder"])
async def test_synchronization_step_transaction(alice_sync_transactions, type):
    sync_transactions = alice_sync_transactions
    synchronization_step = sync_transactions.synchronization_step
    entry_id = sync_transactions.get_workspace_entry().id

    # Sync a placeholder
    manifest = await synchronization_step(entry_id)

    # Acknowledge a successful synchronization
    assert await synchronization_step(entry_id, manifest) is None

    # Local change
    if type == "file":
        a_id, fd = await sync_transactions.file_create(FsPath("/a"))
        await sync_transactions.fd_write(fd, b"abc", 0)
        await sync_transactions.fd_close(fd)
    else:
        a_id = await sync_transactions.folder_create(FsPath("/a"))

    # Sync parent with a placeholder child
    manifest = await synchronization_step(entry_id)
    children = []
    async for child in sync_transactions.get_placeholder_children(manifest):
        children.append(child)
    a_entry_id, = children
    assert a_entry_id == a_id

    # Sync child
    if type == "file":
        await synchronization_step(a_entry_id)
    a_manifest = await synchronization_step(a_entry_id)
    assert await synchronization_step(a_entry_id, a_manifest) is None

    # Acknowledge the manifest
    assert sorted(manifest.children) == ["a"]
    assert await synchronization_step(entry_id, manifest) is None

    # Local change
    b_id = await sync_transactions.folder_create(FsPath("/b"))

    # Remote change
    children = {**manifest.children, "c": EntryID.new()}
    manifest = manifest.evolve(version=5, children=children, author="b@b")

    # Sync parent with a placeholder child
    manifest = await synchronization_step(entry_id, manifest)
    children = []
    async for child in sync_transactions.get_placeholder_children(manifest):
        children.append(child)
    b_entry_id, = children
    assert b_entry_id == b_id

    # Sync child
    b_manifest = await synchronization_step(b_entry_id)
    assert await synchronization_step(b_entry_id, b_manifest) is None

    # Acknowledge the manifest
    assert sorted(manifest.children) == ["a", "b", "c"]
    assert await synchronization_step(entry_id, manifest) is None


@pytest.mark.trio
async def test_get_minimal_remote_manifest(alice, alice_sync_transactions):
    sync_transactions = alice_sync_transactions

    # Prepare
    w_id = sync_transactions.workspace_id
    a_id, fd = await sync_transactions.file_create(FsPath("/a"))
    await sync_transactions.fd_write(fd, b"abc", 0)
    await sync_transactions.fd_close(fd)
    b_id = await sync_transactions.folder_create(FsPath("/b"))
    c_id = await sync_transactions.folder_create(FsPath("/b/c"))

    # Workspace manifest
    minimal = await sync_transactions.get_minimal_remote_manifest(w_id)
    local = await sync_transactions.local_storage.get_manifest(w_id)
    expected = local.to_remote(author=alice.device_id, timestamp=minimal.timestamp).evolve(
        children={}, updated=local.created
    )
    assert minimal == expected

    await sync_transactions.synchronization_step(w_id, minimal)
    assert await sync_transactions.get_minimal_remote_manifest(w_id) is None

    # File manifest
    minimal = await sync_transactions.get_minimal_remote_manifest(a_id)
    local = await sync_transactions.local_storage.get_manifest(a_id)
    expected = local.evolve(blocks=(), updated=local.created, size=0).to_remote(
        author=alice.device_id, timestamp=minimal.timestamp
    )
    assert minimal == expected
    await sync_transactions.file_reshape(a_id)
    await sync_transactions.synchronization_step(a_id, minimal)
    assert await sync_transactions.get_minimal_remote_manifest(a_id) is None

    # Folder manifest
    minimal = await sync_transactions.get_minimal_remote_manifest(b_id)
    local = await sync_transactions.local_storage.get_manifest(b_id)
    expected = local.to_remote(author=alice.device_id, timestamp=minimal.timestamp).evolve(
        children={}, updated=local.created
    )
    assert minimal == expected
    await sync_transactions.synchronization_step(b_id, minimal)
    assert await sync_transactions.get_minimal_remote_manifest(b_id) is None

    # Empty folder manifest
    minimal = await sync_transactions.get_minimal_remote_manifest(c_id)
    local = await sync_transactions.local_storage.get_manifest(c_id)
    expected = local.to_remote(author=alice.device_id, timestamp=minimal.timestamp)
    assert minimal == expected
    await sync_transactions.synchronization_step(c_id, minimal)
    assert await sync_transactions.get_minimal_remote_manifest(c_id) is None


@pytest.mark.trio
async def test_file_conflict(alice_sync_transactions):
    sync_transactions = alice_sync_transactions

    # Prepare
    a_id, fd = await sync_transactions.file_create(FsPath("/a"))
    await sync_transactions.fd_write(fd, b"abc", offset=0)
    await sync_transactions.file_reshape(a_id)
    remote = await sync_transactions.synchronization_step(a_id)
    assert await sync_transactions.synchronization_step(a_id, remote) is None
    await sync_transactions.fd_write(fd, b"def", offset=3)
    await sync_transactions.file_reshape(a_id)
    changed_remote = remote.evolve(version=2, blocks=[], size=0, author="b@b")

    # Try a synchronization
    with pytest.raises(FSFileConflictError) as ctx:
        await sync_transactions.synchronization_step(a_id, changed_remote)
    local, remote = ctx.value.args

    # Write some more
    await sync_transactions.fd_write(fd, b"ghi", offset=6)

    # Also create a fake previous conflict file
    await sync_transactions.file_create(FsPath("/a (conflicting with b@b)"), open=False)

    # Solve conflict
    with sync_transactions.event_bus.listen() as spy:
        await sync_transactions.file_conflict(a_id, local, remote)
    assert await sync_transactions.fd_read(fd, size=-1, offset=0) == b""
    a2_id, fd2 = await sync_transactions.file_open(
        FsPath("/a (conflicting with b@b - 2)"), write_mode=False
    )
    assert await sync_transactions.fd_read(fd2, size=-1, offset=0) == b"abcdefghi"
    spy.assert_events_exactly_occured(
        [
            (
                CoreEvent.FS_ENTRY_UPDATED,
                {"workspace_id": sync_transactions.workspace_id, "id": a2_id},
            ),
            (
                CoreEvent.FS_ENTRY_UPDATED,
                {
                    "workspace_id": sync_transactions.workspace_id,
                    "id": sync_transactions.workspace_id,
                },
            ),
            (
                CoreEvent.FS_ENTRY_FILE_CONFLICT_RESOLVED,
                {"workspace_id": sync_transactions.workspace_id, "id": a_id, "backup_id": a2_id},
            ),
        ]
    )

    # Finish synchronization
    await sync_transactions.file_reshape(a2_id)
    assert await sync_transactions.synchronization_step(a_id, changed_remote) is None
    remote2 = await sync_transactions.synchronization_step(a2_id)
    assert await sync_transactions.synchronization_step(a2_id, remote2) is None

    # Create a new conflict then remove a
    await sync_transactions.fd_write(fd, b"abc", 0)
    await sync_transactions.file_reshape(a_id)
    changed_remote = changed_remote.evolve(version=3)
    await sync_transactions.file_delete(FsPath("/a"))

    # Conflict solving should still succeed
    with pytest.raises(FSFileConflictError) as ctx:
        await sync_transactions.synchronization_step(a_id, changed_remote)
    local, remote = ctx.value.args
    with sync_transactions.event_bus.listen() as spy:
        await sync_transactions.file_conflict(a_id, local, remote)
    spy.assert_events_exactly_occured([])

    # Close fds
    await sync_transactions.fd_close(fd)
    await sync_transactions.fd_close(fd2)
