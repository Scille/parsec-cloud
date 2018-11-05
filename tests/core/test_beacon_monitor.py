import trio
import pytest


@pytest.mark.trio
@pytest.mark.parametrize("type", ("workspace", "folder", "file"))
async def test_beacon_notif_on_new_entry_sync(type, running_backend, alice_core, alice2_fs):
    await alice_core.event_bus.spy.wait_for_backend_connection_ready()

    if type == "workspace":
        await alice2_fs.workspace_create("/foo")
    elif type == "folder":
        await alice2_fs.folder_create("/foo")
    elif type == "file":
        await alice2_fs.file_create("/foo")

    dump_fs = alice2_fs._local_folder_fs.dump()
    root_id = dump_fs["access"]["id"]
    entry_id = dump_fs["children"]["foo"]["access"]["id"]

    with alice_core.event_bus.listen() as spy:
        await alice2_fs.sync("/")
        with trio.fail_after(1):

            await spy.wait_multiple(
                [
                    (
                        "backend.beacon.updated",
                        {"beacon_id": root_id, "index": 1, "src_id": entry_id, "src_version": 1},
                    ),
                    ("fs.entry.updated", {"id": entry_id}),
                    (
                        "backend.beacon.updated",
                        {"beacon_id": root_id, "index": 2, "src_id": root_id, "src_version": 2},
                    ),
                    ("fs.entry.updated", {"id": root_id}),
                ]
            )


# TODO: lazy loading of workspaces make this pretty cumbersome to test...
@pytest.mark.trio
@pytest.mark.xfail
@pytest.mark.parametrize("type", ("folder", "file"))
async def test_beacon_notif_on_new_nested_entry_sync(type, running_backend, alice_core, alice2_fs):
    await alice_core.event_bus.spy.wait_for_backend_connection_ready()

    # A workspace already exists and is synced between parties
    await alice2_fs.workspace_create("/foo")
    with alice_core.event_bus.listen() as spy:
        await alice2_fs.sync("/")
        # Restart due to new workspace's beacon listening
        with trio.fail_after(1):
            await spy.wait("backend.listener.restarted")

    # Create the new item
    if type == "folder":
        await alice2_fs.folder_create("/foo/bar")
    elif type == "file":
        await alice2_fs.file_create("/foo/bar")

    dump_fs = alice2_fs._local_folder_fs.dump()
    beacon_id = dump_fs["children"]["foo"]["beacon_id"]
    workspace_id = dump_fs["children"]["foo"]["access"]["id"]
    entry_id = dump_fs["children"]["foo"]["children"]["bar"]["access"]["id"]

    with alice_core.event_bus.listen() as spy:
        await alice2_fs.sync("/foo/bar")

        with trio.fail_after(1):
            await spy.wait_multiple(
                [
                    (
                        "backend.beacon.updated",
                        {"beacon_id": beacon_id, "index": 1, "src_id": entry_id, "src_version": 1},
                    ),
                    ("fs.entry.updated", {"id": entry_id}),
                    (
                        "backend.beacon.updated",
                        {
                            "beacon_id": beacon_id,
                            "index": 2,
                            "src_id": workspace_id,
                            "src_version": 2,
                        },
                    ),
                    ("fs.entry.updated", {"id": workspace_id}),
                ]
            )
