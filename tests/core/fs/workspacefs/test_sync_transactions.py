# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS
from __future__ import annotations

import pytest

from parsec._parsec import Regex
from parsec.api.protocol import DeviceID
from parsec.core.core_events import CoreEvent
from parsec.core.types import EntryID, EntryName, Chunk, LocalFolderManifest, LocalFileManifest
from parsec.core.fs import FsPath
from parsec.core.fs.workspacefs.sync_transactions import (
    full_name,
    merge_manifests,
    merge_folder_children,
    FSFileConflictError,
)


empty_pattern = Regex.from_regex_str(r"^\b$")


@pytest.mark.parametrize(
    "test_input, expected",
    [
        ("my document", "my document (conflicting with a@a)"),
        ("my document.doc", "my document (conflicting with a@a).doc"),
        ("my document.tar.gz", "my document (conflicting with a@a).tar.gz"),
        ("my document.tar.gz.0", "my document (conflicting with a@a).tar.gz.0"),
        (".my document.tar.gz", ".my document (conflicting with a@a).tar.gz"),
        ("..my document.tar.gz", "..my document (conflicting with a@a).tar.gz"),
        ("...my document.tar.gz", "...my document (conflicting with a@a).tar.gz"),
        ("......", "...... (conflicting with a@a)"),  # Edge case of no non-empty parts
        ("abc" * 82 + ".data", "abc" * 75 + "a (conflicting with a@a).data"),
        ("ঔ" * 82 + ".data", "ঔ" * 72 + " (conflicting with a@a).data"),
        ("This is a test." + "abc" * 80, "This is a test (conflicting with a@a)"),
        ("This is a test." + "abc" * 78 + ".xyz", "This is a test (conflicting with a@a).xyz"),
        ("This is a test.xyz." + "abc" * 78, "This is a test (conflicting with a@a)"),
    ],
)
def test_full_name(test_input, expected):
    test_input = EntryName(test_input)
    expected = EntryName(expected)
    result = full_name(test_input, "conflicting with a@a")
    assert result == expected


@pytest.mark.parametrize(
    "preferred_language, suffix",
    [("en", "name conflict"), ("fr", "Conflit de nom"), ("dummy", "name conflict")],
)
def test_merge_folder_children(preferred_language, suffix):
    m1 = EntryID.new()
    m2 = EntryID.new()
    m3 = EntryID.new()
    a1 = {EntryName("a"): m1}
    a2 = {EntryName("a"): m2}
    b1 = {EntryName("b.txt"): m1}
    b2 = {EntryName("b.txt"): m2}
    c1 = {EntryName("c.tar.gz"): m1}
    c2 = {EntryName("c.tar.gz"): m2}
    # Empty folder
    assert merge_folder_children({}, {}, {}, preferred_language) == {}

    # Adding children
    assert merge_folder_children({}, a1, {}, preferred_language) == a1
    assert merge_folder_children({}, {}, a1, preferred_language) == a1
    assert merge_folder_children({}, a1, a1, preferred_language) == a1

    # Removing children
    assert merge_folder_children(a1, {}, a1, preferred_language) == {}
    assert merge_folder_children(a1, a1, {}, preferred_language) == {}
    assert merge_folder_children(a1, {}, {}, preferred_language) == {}

    # Renaming children
    assert merge_folder_children(a1, a1, b1, preferred_language) == b1
    assert merge_folder_children(a1, b1, a1, preferred_language) == b1
    assert merge_folder_children(a1, b1, b1, preferred_language) == b1

    # Conflicting renaming
    result = merge_folder_children(a1, b1, c1, preferred_language)
    assert result == {EntryName("c.tar.gz"): m1}

    # Conflicting names
    result = merge_folder_children({}, a1, a2, preferred_language)
    assert result == {EntryName("a"): m2, EntryName(f"a (Parsec - {suffix})"): m1}
    result = merge_folder_children({}, b1, b2, preferred_language)
    assert result == {EntryName("b.txt"): m2, EntryName(f"b (Parsec - {suffix}).txt"): m1}
    result = merge_folder_children({}, c1, c2, preferred_language)
    assert result == {EntryName("c.tar.gz"): m2, EntryName(f"c (Parsec - {suffix}).tar.gz"): m1}

    # Conflicting name with special pattern filename
    base = {EntryName(f"a (Parsec - {suffix})"): m3}

    a3 = {**base, **a1}
    b3 = {**base, **a2}

    result = merge_folder_children(base, a3, b3, preferred_language)
    assert result == {
        EntryName("a"): m2,
        EntryName(f"a (Parsec - {suffix})"): m3,
        EntryName(f"a (Parsec - {suffix} (2))"): m1,
    }

    m4 = EntryID.new()
    base = {**base, EntryName(f"a (Parsec - {suffix} (2))"): m4}
    a3 = {**base, **a1}
    b3 = {**base, **a2}

    result = merge_folder_children(base, a3, b3, preferred_language)

    assert result == {
        EntryName("a"): m2,
        EntryName(f"a (Parsec - {suffix})"): m3,
        EntryName(f"a (Parsec - {suffix} (2))"): m4,
        EntryName(f"a (Parsec - {suffix} (3))"): m1,
    }


def test_merge_folder_manifests(alice, bob):
    timestamp = alice.timestamp()
    my_device = alice.device_id
    other_device = bob.device_id
    parent = EntryID.new()
    v1 = LocalFolderManifest.new_placeholder(
        my_device, parent=parent, timestamp=timestamp
    ).to_remote(author=other_device, timestamp=timestamp)

    # Initial base manifest
    m1 = LocalFolderManifest.from_remote(v1, empty_pattern)
    assert merge_manifests(my_device, timestamp, empty_pattern, m1) == m1

    # Local change
    m2 = m1.evolve_children_and_mark_updated(
        {EntryName("a"): EntryID.new()}, empty_pattern, timestamp=timestamp
    )
    assert merge_manifests(my_device, timestamp, empty_pattern, m2) == m2

    # Successful upload
    v2 = m2.to_remote(author=my_device, timestamp=timestamp)
    m3 = merge_manifests(my_device, timestamp, empty_pattern, m2, v2)
    assert m3 == LocalFolderManifest.from_remote(v2, empty_pattern)

    # Two local changes
    m4 = m3.evolve_children_and_mark_updated(
        {EntryName("b"): EntryID.new()}, empty_pattern, timestamp=timestamp
    )
    assert merge_manifests(my_device, timestamp, empty_pattern, m4) == m4
    m5 = m4.evolve_children_and_mark_updated(
        {EntryName("c"): EntryID.new()}, empty_pattern, timestamp=timestamp
    )
    assert merge_manifests(my_device, timestamp, empty_pattern, m4) == m4

    # M4 has been successfully uploaded
    v3 = m4.to_remote(author=my_device, timestamp=timestamp)
    m6 = merge_manifests(my_device, timestamp, empty_pattern, m5, v3)
    assert m6 == m5.evolve(base=v3)

    # The remote has changed
    v4 = v3.evolve(
        version=4, children={EntryName("d"): EntryID.new(), **v3.children}, author=other_device
    )
    m7 = merge_manifests(my_device, timestamp, empty_pattern, m6, v4)
    assert m7.base_version == 4
    assert sorted(m7.children) == [EntryName("a"), EntryName("b"), EntryName("c"), EntryName("d")]
    assert m7.need_sync

    # Successful upload
    v5 = m7.to_remote(author=my_device, timestamp=timestamp)
    m8 = merge_manifests(my_device, timestamp, empty_pattern, m7, v5)
    assert m8 == LocalFolderManifest.from_remote(v5, empty_pattern)

    # The remote has changed
    v6 = v5.evolve(
        version=6, children={EntryName("e"): EntryID.new(), **v5.children}, author=other_device
    )
    m9 = merge_manifests(my_device, timestamp, empty_pattern, m8, v6)
    assert m9 == LocalFolderManifest.from_remote(v6, empty_pattern)


@pytest.mark.parametrize("local_change", ("rename", "prevent_sync_rename"))
@pytest.mark.parametrize("remote_change", ("same_entry_moved", "new_entry_added"))
def test_merge_folder_manifests_with_concurrent_remote_change(
    local_change, remote_change, alice, bob
):
    timestamp = alice.timestamp()
    my_device = alice.device_id
    other_device = bob.device_id
    parent = EntryID.new()
    foo_txt = EntryID.new()
    remote_manifest_v1 = (
        LocalFolderManifest.new_placeholder(my_device, parent=parent, timestamp=timestamp)
        .evolve(children={EntryName("foo.txt"): foo_txt})
        .to_remote(author=my_device, timestamp=timestamp)
    )

    prevent_sync_pattern = Regex.from_regex_str(r".*\.tmp\z")

    # Load the manifest in local
    local_manifest = LocalFolderManifest.from_remote(
        remote_manifest_v1, prevent_sync_pattern=prevent_sync_pattern
    )

    # In local, `foo.txt` is renamed
    if local_change == "rename":
        foo_txt_new_name = EntryName("foo2.txt")
    else:
        assert local_change == "prevent_sync_rename"
        foo_txt_new_name = EntryName("foo.txt.tmp")
    local_manifest = local_manifest.evolve_children_and_mark_updated(
        data={EntryName("foo.txt"): None, foo_txt_new_name: foo_txt},
        prevent_sync_pattern=prevent_sync_pattern,
        timestamp=timestamp,
    )

    # In remote, a change also occurs
    if remote_change == "same_entry_moved":
        remote_manifest_v2_children = {
            EntryName("bar.txt"): remote_manifest_v1.children[EntryName("foo.txt")]
        }
    else:
        assert remote_change == "new_entry_added"
        remote_manifest_v2_children = {
            **remote_manifest_v1.children,
            EntryName("bar.txt"): EntryID.new(),
        }

    remote_manifest_v2 = remote_manifest_v1.evolve(
        author=other_device,
        version=remote_manifest_v1.version + 1,
        children=remote_manifest_v2_children,
    )

    # Now merging should detect the duplication
    merged_manifest = merge_manifests(
        local_author=my_device,
        timestamp=timestamp,
        prevent_sync_pattern=prevent_sync_pattern,
        local_manifest=local_manifest,
        remote_manifest=remote_manifest_v2,
        force_apply_pattern=False,
    )

    if remote_change == "same_entry_moved":
        assert list(merged_manifest.children) == [EntryName("bar.txt")]
    else:
        assert remote_change == "new_entry_added"
        if local_change == "rename":
            assert sorted(merged_manifest.children) == [EntryName("bar.txt"), EntryName("foo2.txt")]
        else:
            assert local_change == "prevent_sync_rename"
            assert sorted(merged_manifest.children) == [
                EntryName("bar.txt"),
                EntryName("foo.txt.tmp"),
            ]


def test_merge_manifests_with_a_placeholder(alice, bob):
    timestamp = alice.timestamp()
    my_device = alice.device_id
    other_device = bob.device_id
    parent = EntryID.new()

    m1 = LocalFolderManifest.new_placeholder(my_device, parent=parent, timestamp=timestamp)
    m2 = merge_manifests(my_device, timestamp, empty_pattern, m1)
    assert m2 == m1
    v1 = m1.to_remote(author=my_device, timestamp=timestamp)

    m2a = merge_manifests(my_device, timestamp, empty_pattern, m1, v1)
    assert m2a == LocalFolderManifest.from_remote(v1, empty_pattern)

    m2b = m1.evolve_children_and_mark_updated(
        {EntryName("a"): EntryID.new()}, empty_pattern, timestamp=timestamp
    )
    m3b = merge_manifests(my_device, timestamp, empty_pattern, m2b, v1)
    assert m3b == m2b.evolve(base=v1)

    v2 = v1.evolve(version=2, author=other_device, children={EntryName("b"): EntryID.new()})
    m2c = m1.evolve_children_and_mark_updated(
        {EntryName("a"): EntryID.new()}, empty_pattern, timestamp=timestamp
    )
    m3c = merge_manifests(my_device, timestamp, empty_pattern, m2c, v2)
    children = {**v2.children, **m2c.children}
    assert m3c == m2c.evolve(base=v2, children=children, updated=m3c.updated)


def test_merge_file_manifests(alice, bob):
    timestamp = alice.timestamp()
    my_device = alice.device_id
    other_device = bob.device_id
    parent = EntryID.new()
    v1 = LocalFileManifest.new_placeholder(my_device, parent=parent, timestamp=timestamp).to_remote(
        author=other_device, timestamp=timestamp
    )

    def evolve(m, n):
        chunk = Chunk.new(0, n).evolve_as_block(b"a" * n)
        blocks = ((chunk,),)
        return m1.evolve_and_mark_updated(size=n, blocks=blocks, timestamp=timestamp)

    # Initial base manifest
    m1 = LocalFileManifest.from_remote(v1)
    assert merge_manifests(my_device, timestamp, empty_pattern, m1) == m1

    # Local change
    m2 = evolve(m1, 1)
    assert merge_manifests(my_device, timestamp, empty_pattern, m2) == m2

    # Successful upload
    v2 = m2.to_remote(author=my_device, timestamp=timestamp)
    m3 = merge_manifests(my_device, timestamp, empty_pattern, m2, v2)
    assert m3 == LocalFileManifest.from_remote(v2)

    # Two local changes
    m4 = evolve(m3, 2)
    assert merge_manifests(my_device, timestamp, empty_pattern, m4) == m4
    m5 = evolve(m4, 3)
    assert merge_manifests(my_device, timestamp, empty_pattern, m4) == m4

    # M4 has been successfully uploaded
    v3 = m4.to_remote(author=my_device, timestamp=timestamp)
    m6 = merge_manifests(my_device, timestamp, empty_pattern, m5, v3)
    assert m6 == m5.evolve(base=v3)

    # The remote has changed
    v4 = v3.evolve(version=4, size=0, author=other_device)
    with pytest.raises(FSFileConflictError):
        merge_manifests(my_device, timestamp, empty_pattern, m6, v4)


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
    (a_entry_id,) = children
    assert a_entry_id == a_id

    # Sync child
    if type == "file":
        await synchronization_step(a_entry_id)
    a_manifest = await synchronization_step(a_entry_id)
    assert await synchronization_step(a_entry_id, a_manifest) is None

    # Acknowledge the manifest
    assert sorted(manifest.children) == [EntryName("a")]
    assert await synchronization_step(entry_id, manifest) is None

    # Local change
    b_id = await sync_transactions.folder_create(FsPath("/b"))

    # Remote change
    children = {**manifest.children, EntryName("c"): EntryID.new()}
    manifest = manifest.evolve(version=5, children=children, author=DeviceID("b@b"))

    # Sync parent with a placeholder child
    manifest = await synchronization_step(entry_id, manifest)
    children = []
    async for child in sync_transactions.get_placeholder_children(manifest):
        children.append(child)
    (b_entry_id,) = children
    assert b_entry_id == b_id

    # Sync child
    b_manifest = await synchronization_step(b_entry_id)
    assert await synchronization_step(b_entry_id, b_manifest) is None

    # Acknowledge the manifest
    assert sorted(manifest.children) == [EntryName("a"), EntryName("b"), EntryName("c")]
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
@pytest.mark.parametrize(
    "preferred_language, suffix",
    [("en", "content conflict"), ("fr", "Conflit de contenu"), ("dummy", "content conflict")],
)
async def test_file_conflict(alice_sync_transactions, preferred_language, suffix):
    sync_transactions = alice_sync_transactions
    sync_transactions.preferred_language = preferred_language
    # Prepare
    a_id, fd = await sync_transactions.file_create(FsPath("/a"))
    await sync_transactions.fd_write(fd, b"abc", offset=0)
    await sync_transactions.file_reshape(a_id)
    remote = await sync_transactions.synchronization_step(a_id)
    assert await sync_transactions.synchronization_step(a_id, remote) is None
    await sync_transactions.fd_write(fd, b"def", offset=3)
    await sync_transactions.file_reshape(a_id)
    changed_remote = remote.evolve(version=2, blocks=(), size=0, author=DeviceID("b@b"))

    # Try a synchronization
    with pytest.raises(FSFileConflictError) as ctx:
        await sync_transactions.synchronization_step(a_id, changed_remote)
    local, remote = ctx.value.args

    # Write some more
    await sync_transactions.fd_write(fd, b"ghi", offset=6)

    # Also create a fake previous conflict file
    await sync_transactions.file_create(FsPath(f"/a (Parsec - {suffix})"), open=False)

    # Solve conflict
    with sync_transactions.event_bus.listen() as spy:
        await sync_transactions.file_conflict(a_id, local, remote)
    assert await sync_transactions.fd_read(fd, size=-1, offset=0) == b""
    a2_id, fd2 = await sync_transactions.file_open(
        FsPath(f"/a (Parsec - {suffix} (2))"), write_mode=False
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
