# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
import trio

from parsec.core.types import FsPath


@pytest.mark.trio
async def test_autosync_on_modification(mock_clock, running_backend, alice_core, alice2_fs):
    mock_clock.autojump_threshold = 0.1

    await alice_core.event_bus.spy.wait_for_backend_connection_ready()
    wid = await alice_core.user_fs.workspace_create("w")
    workspace = alice_core.user_fs.get_workspace(wid)

    with alice_core.event_bus.listen() as spy:
        await workspace.folder_create(FsPath("/foo"))
        with trio.fail_after(60):  # autojump, so not *really* 60s
            await spy.wait("fs.entry.synced", kwargs={"path": "/w/foo", "id": spy.ANY})

    await alice2_fs.sync("/")
    workspace2 = alice2_fs.user_fs.get_workspace(wid)
    stat = await workspace.entry_info(FsPath("/foo"))
    stat2 = await workspace2.entry_info(FsPath("/foo"))
    assert stat == stat2
