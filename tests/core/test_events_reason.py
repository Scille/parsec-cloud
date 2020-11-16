# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS
import pytest

from unittest.mock import ANY

from parsec.core.core_events import CoreEvent, FSEntryUpdatedReason


@pytest.mark.trio
async def test_fs_update_event_reason(alice_user_fs, running_backend):
    event_bus = alice_user_fs.event_bus

    # Testing event when creating a workspace
    with event_bus.listen() as spy:
        wid = await alice_user_fs.workspace_create("tempw")
    spy.assert_events_exactly_occured(
        [
            (
                CoreEvent.FS_ENTRY_UPDATED,
                {
                    "workspace_name": "tempw",
                    "entry_id": wid,
                    "id": ANY,
                    "reason": FSEntryUpdatedReason.WORKSPACE_CREATE,
                },
            ),
            (CoreEvent.FS_WORKSPACE_CREATED, {"new_entry": ANY}),
        ]
    )

    # Testing event when renaming a workspace
    with event_bus.listen() as spy:
        await alice_user_fs.workspace_rename(wid, "w")
    spy.assert_events_exactly_occured(
        [
            (
                CoreEvent.FS_ENTRY_UPDATED,
                {
                    "id": ANY,
                    "reason": FSEntryUpdatedReason.WORKSPACE_RENAME,
                    "entry_id": wid,
                    "workspace_old_name": "tempw",
                    "workspace_new_name": "w",
                },
            )
        ]
    )
    workspace = alice_user_fs.get_workspace(wid)

    # Testing event when creating a folder
    with event_bus.listen() as spy:
        await workspace.mkdir("/foo")
    spy.assert_events_exactly_occured(
        [
            (
                CoreEvent.FS_ENTRY_UPDATED,
                {
                    "workspace_id": wid,
                    "id": ANY,
                    "reason": FSEntryUpdatedReason.FOLDER_CREATE,
                    "entry_id": ANY,
                    "entry_name": "foo",
                },
            ),
            (
                CoreEvent.FS_ENTRY_UPDATED,
                {
                    "workspace_id": wid,
                    "id": ANY,
                    "reason": FSEntryUpdatedReason.FOLDER_CREATE_ENTRY_CREATION,
                },
            ),
        ]
    )
    # Testing event when creating a sub folder
    with event_bus.listen() as spy:
        await workspace.mkdir("/foo/bar")
    spy.assert_events_exactly_occured(
        [
            (
                CoreEvent.FS_ENTRY_UPDATED,
                {
                    "workspace_id": wid,
                    "id": ANY,
                    "reason": FSEntryUpdatedReason.FOLDER_CREATE,
                    "entry_id": ANY,
                    "entry_name": "bar",
                },
            ),
            (
                CoreEvent.FS_ENTRY_UPDATED,
                {
                    "workspace_id": wid,
                    "id": ANY,
                    "reason": FSEntryUpdatedReason.FOLDER_CREATE_ENTRY_CREATION,
                },
            ),
        ]
    )
    # Testing event when renaming a folder
    with event_bus.listen() as spy:
        await workspace.rename("/foo/bar", "/foo/beer")
    spy.assert_events_exactly_occured(
        [
            (
                CoreEvent.FS_ENTRY_UPDATED,
                {
                    "workspace_id": wid,
                    "id": ANY,
                    "reason": FSEntryUpdatedReason.ENTRY_RENAME,
                    "entry_source_name": "bar",
                    "entry_id": ANY,
                    "entry_destination_name": "beer",
                },
            )
        ]
    )
    # Testing event when deleting a folder
    with event_bus.listen() as spy:
        await workspace.rmdir("/foo/beer")
    spy.assert_events_exactly_occured(
        [
            (
                CoreEvent.FS_ENTRY_UPDATED,
                {
                    "workspace_id": wid,
                    "id": ANY,
                    "reason": FSEntryUpdatedReason.FOLDER_DELETE,
                    "entry_name": "beer",
                    "entry_id": ANY,
                },
            )
        ]
    )
    # Testing event when creating a file
    with event_bus.listen() as spy:
        await workspace.touch("/bar")
    spy.assert_events_exactly_occured(
        [
            (
                CoreEvent.FS_ENTRY_UPDATED,
                {
                    "workspace_id": wid,
                    "id": ANY,
                    "reason": FSEntryUpdatedReason.FILE_CREATE,
                    "entry_name": "bar",
                    "entry_id": ANY,
                    "entry_creation_date": ANY,
                    "entry_updated": ANY,
                    "entry_size": 0,
                },
            ),
            (
                CoreEvent.FS_ENTRY_UPDATED,
                {
                    "workspace_id": wid,
                    "id": ANY,
                    "reason": FSEntryUpdatedReason.FILE_CREATE_ENTRY_CREATION,
                },
            ),
        ]
    )
    # Testing event when editing a file
    with event_bus.listen() as spy:
        await workspace.write_bytes("/bar", b"abcde")
    spy.assert_events_exactly_occured(
        [
            (
                CoreEvent.FS_ENTRY_UPDATED,
                {"workspace_id": wid, "id": ANY, "reason": FSEntryUpdatedReason.FILE_WRITE},
            )
        ]
    )
    # Testing event when resizing a file
    with event_bus.listen() as spy:
        await workspace.truncate("/bar", 3)
    spy.assert_events_exactly_occured(
        [
            (
                CoreEvent.FS_ENTRY_UPDATED,
                {"workspace_id": wid, "id": ANY, "reason": FSEntryUpdatedReason.FILE_RESIZE},
            )
        ]
    )
    # Testing event when deleting a file
    with event_bus.listen() as spy:
        await workspace.unlink("/bar")
    spy.assert_events_exactly_occured(
        [
            (
                CoreEvent.FS_ENTRY_UPDATED,
                {
                    "workspace_id": wid,
                    "id": ANY,
                    "reason": FSEntryUpdatedReason.FILE_DELETE,
                    "entry_name": "bar",
                    "entry_id": ANY,
                },
            )
        ]
    )

    # Same file test with fd
    assert not await workspace.exists("/bar")
    # Creating file
    with event_bus.listen() as spy:
        workspace_file = await workspace.open_file("/bar", "wb+")
    assert await workspace.exists("/bar")
    spy.assert_events_exactly_occured(
        [
            (
                CoreEvent.FS_ENTRY_UPDATED,
                {
                    "workspace_id": wid,
                    "id": ANY,
                    "reason": FSEntryUpdatedReason.FILE_CREATE,
                    "entry_name": "bar",
                    "entry_id": ANY,
                    "entry_creation_date": ANY,
                    "entry_updated": ANY,
                    "entry_size": 0,
                },
            ),
            (
                CoreEvent.FS_ENTRY_UPDATED,
                {
                    "workspace_id": wid,
                    "id": ANY,
                    "reason": FSEntryUpdatedReason.FILE_CREATE_ENTRY_CREATION,
                },
            ),
        ]
    )
    # Test write
    with event_bus.listen() as spy:
        await workspace_file.write(b"abcde")
    spy.assert_events_exactly_occured(
        [
            (
                CoreEvent.FS_ENTRY_UPDATED,
                {"workspace_id": wid, "id": ANY, "reason": FSEntryUpdatedReason.FILE_WRITE},
            )
        ]
    )

    # Test truncate
    with event_bus.listen() as spy:
        await workspace_file.truncate(3)
    spy.assert_events_exactly_occured(
        [
            (
                CoreEvent.FS_ENTRY_UPDATED,
                {"workspace_id": wid, "id": ANY, "reason": FSEntryUpdatedReason.FILE_RESIZE},
            )
        ]
    )
