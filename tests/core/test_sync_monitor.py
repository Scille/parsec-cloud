# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
import trio


@pytest.mark.trio
async def test_autosync_on_modification(mock_clock, running_backend, alice_core, alice2_user_fs):
    mock_clock.rate = 0  # Avoid potential concurrency with monitors
    await alice_core.event_bus.spy.wait_for_backend_connection_ready()
    wid = await alice_core.user_fs.workspace_create("w")
    workspace = alice_core.user_fs.get_workspace(wid)
    await alice_core.user_fs.sync()

    mock_clock.rate = 1
    mock_clock.autojump_threshold = 0.1
    with alice_core.event_bus.listen() as spy:
        await workspace.mkdir("/foo")
        foo_id = await workspace.path_id("/foo")
        with trio.fail_after(60):  # autojump, so not *really* 60s
            await spy.wait("fs.entry.synced", kwargs={"workspace_id": wid, "id": foo_id})

    await alice2_user_fs.sync()
    workspace2 = alice2_user_fs.get_workspace(wid)
    path_info = await workspace.path_info("/foo")
    path_info2 = await workspace2.path_info("/foo")
    assert path_info == path_info2
