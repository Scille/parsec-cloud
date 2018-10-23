import trio
import pytest


@pytest.mark.trio
async def test_autosync_on_modification(mock_clock, running_backend, alice_core, alice2_fs):
    mock_clock.autojump_threshold = 0.1

    await alice_core.event_bus.spy.wait_for_backend_connection_ready()
    await alice_core.fs.workspace_create("/w")

    with alice_core.event_bus.listen() as spy:
        await alice_core.fs.folder_create("/w/foo")
        await spy.wait("fs.entry.synced", kwargs={"path": "/w/foo", "id": spy.ANY})

    await trio.sleep(0.1)
    await alice2_fs.sync("/")

    stat = await alice_core.fs.stat("/w/foo")
    stat2 = await alice2_fs.stat("/w/foo")
    assert stat == stat2
