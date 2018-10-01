import pytest


@pytest.mark.trio
async def test_autosync_on_modification(mock_clock, running_backend, alice_core, alice2_fs):
    mock_clock.autojump_threshold = 0.1

    await alice_core.event_bus.spy.wait_for_backend_connection_ready()

    with alice_core.event_bus.listen() as spy:
        await alice_core.fs.folder_create("/foo")
        await spy.wait("fs.entry.synced", kwargs={"path": "/foo", "id": spy.ANY})

    await alice2_fs.sync("/")

    stat = await alice_core.fs.stat("/foo")
    stat2 = await alice2_fs.stat("/foo")
    assert stat == stat2
