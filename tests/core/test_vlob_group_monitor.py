# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import trio
import pytest


from tests.common import create_shared_workspace


@pytest.mark.trio
@pytest.mark.parametrize("type", ("folder", "file"))
async def test_vlob_group_notif_on_new_entry_sync(type, running_backend, alice_core, alice2_fs):
    await create_shared_workspace("/w", alice_core, alice2_fs)
    await alice_core.event_bus.spy.wait_for_backend_connection_ready()

    if type == "folder":
        await alice2_fs.folder_create("/w/foo")
    elif type == "file":
        await alice2_fs.file_create("/w/foo")

    dump_fs = alice2_fs._local_folder_fs.dump()
    vlob_group_id = dump_fs["children"]["w"]["access"]["id"]
    entry_id = dump_fs["children"]["w"]["children"]["foo"]["access"]["id"]

    with alice_core.event_bus.listen() as spy:
        await alice2_fs.sync("/")
        with trio.fail_after(1):

            await spy.wait_multiple(
                [
                    (
                        "backend.vlob_group.updated",
                        {
                            "id": vlob_group_id,
                            "checkpoint": 2,
                            "src_id": entry_id,
                            "src_version": 1,
                        },
                    ),
                    ("fs.entry.updated", {"id": entry_id}),
                    (
                        "backend.vlob_group.updated",
                        {
                            "id": vlob_group_id,
                            "checkpoint": 3,
                            "src_id": vlob_group_id,
                            "src_version": 2,
                        },
                    ),
                    ("fs.entry.updated", {"id": vlob_group_id}),
                ]
            )


@pytest.mark.trio
async def test_vlob_group_notif_on_new_workspace_sync(running_backend, alice_core, alice2_fs):
    await alice_core.event_bus.spy.wait_for_backend_connection_ready()

    await alice2_fs.workspace_create("/foo")

    dump_fs = alice2_fs._local_folder_fs.dump()
    root_id = dump_fs["access"]["id"]

    with alice_core.event_bus.listen() as spy:
        await alice2_fs.sync("/")
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


# TODO: lazy loading of workspaces make this pretty cumbersome to test...
@pytest.mark.trio
@pytest.mark.xfail
@pytest.mark.parametrize("type", ("folder", "file"))
async def test_vlob_group_notif_on_new_nested_entry_sync(
    type, running_backend, alice_core, alice2_fs
):
    await alice_core.event_bus.spy.wait_for_backend_connection_ready()

    # A workspace already exists and is synced between parties
    await create_shared_workspace("/foo", alice_core, alice2_fs)

    # Create the new item
    if type == "folder":
        await alice2_fs.folder_create("/foo/bar")
    elif type == "file":
        await alice2_fs.file_create("/foo/bar")

    dump_fs = alice2_fs._local_folder_fs.dump()
    vlob_group_id = dump_fs["children"]["foo"]["vlob_group_id"]
    workspace_id = dump_fs["children"]["foo"]["access"]["id"]
    entry_id = dump_fs["children"]["foo"]["children"]["bar"]["access"]["id"]

    with alice_core.event_bus.listen() as spy:
        await alice2_fs.sync("/foo/bar")

        with trio.fail_after(1):
            await spy.wait_multiple(
                [
                    (
                        "backend.vlob_group.updated",
                        {
                            "id": vlob_group_id,
                            "checkpoint": 1,
                            "src_id": entry_id,
                            "src_version": 1,
                        },
                    ),
                    ("fs.entry.updated", {"id": entry_id}),
                    (
                        "backend.vlob_group.updated",
                        {
                            "id": vlob_group_id,
                            "checkpoint": 2,
                            "src_id": workspace_id,
                            "src_version": 2,
                        },
                    ),
                    ("fs.entry.updated", {"id": workspace_id}),
                ]
            )
