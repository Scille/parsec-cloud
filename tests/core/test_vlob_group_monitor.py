# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio
import pytest

from tests.common import create_shared_workspace


@pytest.mark.trio
@pytest.mark.parametrize("type", ("folder", "file"))
async def test_vlob_group_notif_on_new_entry_sync(
    mock_clock, running_backend, alice_core, alice2_fs, type
):
    mock_clock.rate = 1
    await create_shared_workspace("/w", alice_core, alice2_fs)
    # Suspend time to freeze core background tasks
    mock_clock.rate = 0

    if type == "folder":
        await alice2_fs.folder_create("/w/foo")
    elif type == "file":
        await alice2_fs.file_create("/w/foo")

    dump_fs = alice2_fs._local_folder_fs.dump()
    vlob_group_id = dump_fs["children"]["w"]["access"]["id"]
    entry_id = dump_fs["children"]["w"]["children"]["foo"]["access"]["id"]

    with alice_core.event_bus.listen() as spy:
        await alice2_fs.sync("/")
        mock_clock.rate = 1
        with trio.fail_after(1):

            await spy.wait_multiple(
                [
                    (
                        "backend.vlob_group.updated",
                        {
                            "id": vlob_group_id,
                            "checkpoint": 3,
                            "src_id": entry_id,
                            "src_version": 1,
                        },
                    ),
                    ("fs.entry.updated", {"id": entry_id}),
                    (
                        "backend.vlob_group.updated",
                        {
                            "id": vlob_group_id,
                            "checkpoint": 4,
                            "src_id": vlob_group_id,
                            "src_version": 3,
                        },
                    ),
                    ("fs.entry.updated", {"id": vlob_group_id}),
                ]
            )


@pytest.mark.trio
async def test_vlob_group_notif_on_new_workspace_sync(
    mock_clock, running_backend, alice_core, alice2_fs
):
    # Suspend time to freeze core background tasks
    mock_clock.rate = 0
    await alice2_fs.workspace_create("/foo")

    dump_fs = alice2_fs._local_folder_fs.dump()
    root_id = dump_fs["access"]["id"]

    with alice_core.event_bus.listen() as spy:
        await alice2_fs.sync("/")
        mock_clock.rate = 1
        with trio.fail_after(1):

            await spy.wait_multiple(
                [
                    (
                        "backend.vlob_group.updated",
                        {"id": root_id, "checkpoint": 2, "src_id": root_id, "src_version": 2},
                    ),
                    ("fs.entry.updated", {"id": root_id}),
                ]
            )


@pytest.mark.trio
@pytest.mark.parametrize("type", ("folder", "file"))
async def test_vlob_group_notif_on_new_nested_entry_sync(
    mock_clock, running_backend, alice_core, alice2_fs, type
):
    mock_clock.rate = 1
    # A workspace already exists and is synced between parties
    await create_shared_workspace("/w", alice_core, alice2_fs)
    # Suspend time to freeze core background tasks
    mock_clock.rate = 0

    # Create the new item
    if type == "folder":
        await alice2_fs.folder_create("/w/foo")
    elif type == "file":
        await alice2_fs.file_create("/w/foo")

    dump_fs = alice2_fs._local_folder_fs.dump()
    workspace_id = vlob_group_id = dump_fs["children"]["w"]["access"]["id"]
    entry_id = dump_fs["children"]["w"]["children"]["foo"]["access"]["id"]

    with alice_core.event_bus.listen() as spy:
        await alice2_fs.sync("/w/foo")

        mock_clock.rate = 1
        with trio.fail_after(1):
            await spy.wait_multiple(
                [
                    (
                        "backend.vlob_group.updated",
                        {
                            "id": vlob_group_id,
                            "checkpoint": 3,
                            "src_id": entry_id,
                            "src_version": 1,
                        },
                    ),
                    ("fs.entry.updated", {"id": entry_id}),
                    (
                        "backend.vlob_group.updated",
                        {
                            "id": vlob_group_id,
                            "checkpoint": 4,
                            "src_id": workspace_id,
                            "src_version": 3,
                        },
                    ),
                    ("fs.entry.updated", {"id": workspace_id}),
                ]
            )
