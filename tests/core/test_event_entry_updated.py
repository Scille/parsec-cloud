# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

import pytest

from unittest.mock import ANY

from parsec.api.data import EntryName
from parsec.core.core_events import CoreEvent


@pytest.mark.trio
async def test_event_entry_updated(alice_user_fs, running_backend):
    event_bus = alice_user_fs.event_bus

    # Testing event when creating a workspace
    with event_bus.listen() as spy:
        wid = await alice_user_fs.workspace_create(EntryName("tempw"))
    spy.assert_events_exactly_occured(
        [
            (CoreEvent.FS_ENTRY_UPDATED, {"id": ANY}),
            (CoreEvent.FS_WORKSPACE_CREATED, {"new_entry": ANY}),
        ]
    )

    # Testing event when renaming a workspace
    with event_bus.listen() as spy:
        await alice_user_fs.workspace_rename(wid, EntryName("w"))
    spy.assert_events_exactly_occured([(CoreEvent.FS_ENTRY_UPDATED, {"id": ANY})])
    workspace = alice_user_fs.get_workspace(wid)

    # Testing event when creating a folder
    with event_bus.listen() as spy:
        await workspace.mkdir("/foo")
    spy.assert_events_exactly_occured(
        [
            (CoreEvent.FS_ENTRY_UPDATED, {"workspace_id": wid, "id": ANY}),
            (CoreEvent.FS_ENTRY_UPDATED, {"workspace_id": wid, "id": ANY}),
        ]
    )
    # Testing event when creating a sub folder
    with event_bus.listen() as spy:
        await workspace.mkdir("/foo/bar")
    spy.assert_events_exactly_occured(
        [
            (CoreEvent.FS_ENTRY_UPDATED, {"workspace_id": wid, "id": ANY}),
            (CoreEvent.FS_ENTRY_UPDATED, {"workspace_id": wid, "id": ANY}),
        ]
    )
    # Testing event when renaming a folder
    with event_bus.listen() as spy:
        await workspace.rename("/foo/bar", "/foo/beer")
    spy.assert_events_exactly_occured(
        [(CoreEvent.FS_ENTRY_UPDATED, {"workspace_id": wid, "id": ANY})]
    )
    # Testing event when deleting a folder
    with event_bus.listen() as spy:
        await workspace.rmdir("/foo/beer")
    spy.assert_events_exactly_occured(
        [(CoreEvent.FS_ENTRY_UPDATED, {"workspace_id": wid, "id": ANY})]
    )
    # Testing event when creating a file
    with event_bus.listen() as spy:
        await workspace.touch("/bar")
    spy.assert_events_exactly_occured(
        [
            (CoreEvent.FS_ENTRY_UPDATED, {"workspace_id": wid, "id": ANY}),
            (CoreEvent.FS_ENTRY_UPDATED, {"workspace_id": wid, "id": ANY}),
        ]
    )
    # Testing event when editing a file
    with event_bus.listen() as spy:
        await workspace.write_bytes("/bar", b"abcde")
    spy.assert_events_exactly_occured(
        [(CoreEvent.FS_ENTRY_UPDATED, {"workspace_id": wid, "id": ANY})]
    )
    # Testing event when resizing a file
    with event_bus.listen() as spy:
        await workspace.truncate("/bar", 3)
    spy.assert_events_exactly_occured(
        [(CoreEvent.FS_ENTRY_UPDATED, {"workspace_id": wid, "id": ANY})]
    )
    # Testing event when deleting a file
    with event_bus.listen() as spy:
        await workspace.unlink("/bar")
    spy.assert_events_exactly_occured(
        [(CoreEvent.FS_ENTRY_UPDATED, {"workspace_id": wid, "id": ANY})]
    )

    # Same file test with fd
    assert not await workspace.exists("/bar")
    # Creating file
    with event_bus.listen() as spy:
        workspace_file = await workspace.open_file("/bar", "wb+")
    assert await workspace.exists("/bar")
    spy.assert_events_exactly_occured(
        [
            (CoreEvent.FS_ENTRY_UPDATED, {"workspace_id": wid, "id": ANY}),
            (CoreEvent.FS_ENTRY_UPDATED, {"workspace_id": wid, "id": ANY}),
        ]
    )
    # Test write
    with event_bus.listen() as spy:
        await workspace_file.write(b"abcde")
    spy.assert_events_exactly_occured(
        [(CoreEvent.FS_ENTRY_UPDATED, {"workspace_id": wid, "id": ANY})]
    )

    # Test truncate
    with event_bus.listen() as spy:
        await workspace_file.truncate(3)
    spy.assert_events_exactly_occured(
        [(CoreEvent.FS_ENTRY_UPDATED, {"workspace_id": wid, "id": ANY})]
    )
