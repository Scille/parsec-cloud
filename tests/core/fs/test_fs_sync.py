# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from pendulum import Pendulum
from unittest.mock import ANY

from parsec.core.types import WorkspaceEntry, WorkspaceRole, ManifestAccess
from parsec.core.backend_connection import BackendNotAvailable
from parsec.core.fs.exceptions import FSBackendOfflineError

from tests.common import freeze_time, create_shared_workspace


async def assert_same_fs(fs1, fs2):
    async def _recursive_assert(fs1, fs2, path):
        stat1 = await fs1.stat(path)
        assert not stat1["need_sync"]
        stat2 = await fs2.stat(path)
        assert stat1 == stat2

        cooked_children = {}
        # TODO: type root should not exist
        if stat1["type"] in ("folder", "root"):
            for child in stat1["children"]:
                cooked_children[child] = await _recursive_assert(fs1, fs2, f"{path}/{child}")
            stat1["children"] = cooked_children

        return stat1

    return await _recursive_assert(fs1, fs2, "/")


@pytest.mark.trio
async def test_new_workspace(running_backend, alice, alice_user_fs, alice2_user_fs):
    with freeze_time("2000-01-02"):
        wid = await alice_user_fs.workspace_create("w")
        workspace = alice_user_fs.get_workspace(wid)

    with alice_user_fs.event_bus.listen() as spy:
        with freeze_time("2000-01-03"):
            await workspace.sync("/")
    spy.assert_events_occured(
        [("fs.entry.synced", {"workspace_id": wid, "id": wid}, Pendulum(2000, 1, 3))]
    )

    workspace2 = alice_user_fs.get_workspace(wid)
    await alice_user_fs.sync()
    await workspace2.sync("/")

    workspace_entry = workspace.get_workspace_entry()
    path_info = await workspace.path_info("/")
    assert path_info == {
        "type": "folder",
        "id": wid,
        "is_placeholder": False,
        "need_sync": False,
        "base_version": 1,
        "children": [],
        "created": Pendulum(2000, 1, 2),
        "updated": Pendulum(2000, 1, 2),
    }
    assert workspace_entry == WorkspaceEntry(
        name="w",
        access=ManifestAccess(id=wid, key=spy.ANY),
        granted_on=Pendulum(2000, 1, 2),
        role=WorkspaceRole.OWNER,
    )
    workspace_entry2 = workspace.get_workspace_entry()
    path_info2 = await workspace.path_info("/")
    assert workspace_entry == workspace_entry2
    assert path_info == path_info2


@pytest.mark.trio
@pytest.mark.parametrize("type", ["file", "folder"])
async def test_new_empty_entry(type, running_backend, alice_user_fs, alice2_user_fs):
    wid = await create_shared_workspace("w", alice_user_fs, alice2_user_fs)
    workspace = alice_user_fs.get_workspace(wid)
    with freeze_time("2000-01-02"):
        if type == "file":
            await workspace.touch("/foo")
        else:
            await workspace.mkdir("/foo")

    info = await workspace.path_info("/foo")
    fid = info["id"]
    with alice_user_fs.event_bus.listen() as spy:
        with freeze_time("2000-01-03"):
            await workspace.sync("/")

    if type == "file":  # TODO: file and folder should generate the same events after the migration
        expected_events = [
            ("fs.entry.synced", {"workspace_id": wid, "id": wid}, Pendulum(2000, 1, 3))
        ]
    else:
        expected_events = [
            ("fs.entry.synced", {"workspace_id": wid, "id": fid}, Pendulum(2000, 1, 3)),
            ("fs.entry.synced", {"workspace_id": wid, "id": wid}, Pendulum(2000, 1, 3)),
        ]
    spy.assert_events_occured(expected_events)

    workspace2 = alice2_user_fs.get_workspace(wid)
    await workspace2.sync("/")

    info = await workspace.path_info("/foo")
    if type == "file":
        assert info == {
            "type": "file",
            "id": ANY,
            "is_placeholder": False,
            "need_sync": False,
            "base_version": 1,
            "created": Pendulum(2000, 1, 2),
            "updated": Pendulum(2000, 1, 2),
            "size": 0,
        }
    else:
        assert info == {
            "type": "folder",
            "id": ANY,
            "is_placeholder": False,
            "need_sync": False,
            "base_version": 1,
            "created": Pendulum(2000, 1, 2),
            "updated": Pendulum(2000, 1, 2),
            "children": [],
        }
    info2 = await workspace2.path_info("/foo")
    assert info == info2


@pytest.mark.trio
async def test_simple_sync(running_backend, alice_fs, alice2_fs):
    wid = await create_shared_workspace("w", alice_fs, alice2_fs)

    # 0) Make sure workspace is loaded for alice2
    # (otherwise won't get synced event during step 2)
    await alice2_fs.stat("/w")

    # 1) Create&sync file

    with freeze_time("2000-01-02"):
        await alice_fs.touch("/w/foo.txt")

    with freeze_time("2000-01-03"):
        await alice_fs.file_write("/w/foo.txt", b"hello world !")

    with alice_fs.event_bus.listen() as spy:
        with freeze_time("2000-01-04"):
            await alice_fs.sync("/w")

    spy.assert_events_occured(
        [("fs.entry.synced", {"workspace_id": wid, "id": wid}, Pendulum(2000, 1, 4))]
    )

    # 2) Fetch back file from another fs

    with alice2_fs.event_bus.listen() as spy:
        with freeze_time("2000-01-05"):
            await alice2_fs.sync("/w")
    spy.assert_events_occured(
        [("fs.entry.downsynced", {"workspace_id": wid, "id": wid}, Pendulum(2000, 1, 5))]
    )

    # 3) Finally make sure both fs have the same data
    final_fs = await assert_same_fs(alice_fs, alice2_fs)
    assert final_fs["children"]["w"]["children"].keys() == {"foo.txt"}

    data = await alice_fs.file_read("/w/foo.txt")
    data2 = await alice2_fs.file_read("/w/foo.txt")
    assert data == data2


@pytest.mark.trio
async def test_fs_recursive_sync(running_backend, alice_fs):
    wid = await create_shared_workspace("w", alice_fs)

    # 1) Create data

    with freeze_time("2000-01-02"):
        await alice_fs.touch("/w/foo.txt")
        await alice_fs.folder_create("/w/bar")
        await alice_fs.touch("/w/bar/wizz.txt")
        await alice_fs.folder_create("/w/bar/spam")

    # 2) Sync it

    with alice_fs.event_bus.listen() as spy:
        with freeze_time("2000-01-03"):
            await alice_fs.sync("/w")
    sync_date = Pendulum(2000, 1, 3)
    spy.assert_events_occured(
        [
            ("fs.entry.synced", {"workspace_id": wid, "id": spy.ANY}, sync_date),
            ("fs.entry.synced", {"workspace_id": wid, "id": spy.ANY}, sync_date),
            ("fs.entry.synced", {"workspace_id": wid, "id": spy.ANY}, sync_date),
        ]
    )

    # 2) Now additional sync should not trigger any event

    with alice_fs.event_bus.listen() as spy:
        with freeze_time("2000-01-04"):
            await alice_fs.sync("/w")
    assert not spy.events

    # 3) Make sure everything is considered synced
    for path in ["/", "/w", "/w/foo.txt", "/w/bar", "/w/bar/wizz.txt", "/w/bar/spam"]:
        stat = await alice_fs.stat(path)
        assert not stat["need_sync"]


# TODO: a complex but interesting test would be to do concurrent changes
# during sync


@pytest.mark.trio
async def test_cross_sync(running_backend, alice_fs, alice2_fs):
    wid = await create_shared_workspace("w", alice_fs, alice2_fs)

    # 1) Both fs have things to sync

    with freeze_time("2000-01-02"):
        await alice_fs.touch("/w/foo.txt")

    with freeze_time("2000-01-03"):
        await alice2_fs.folder_create("/w/bar")
        await alice2_fs.folder_create("/w/bar/spam")

    # 2) Do the cross sync

    with alice_fs.event_bus.listen() as spy:
        with freeze_time("2000-01-04"):
            await alice_fs.sync("/w")

    spy.assert_events_occured(
        [("fs.entry.synced", {"workspace_id": wid, "id": wid}, Pendulum(2000, 1, 4))]
    )

    with alice2_fs.event_bus.listen() as spy:
        with freeze_time("2000-01-05"):
            await alice2_fs.sync("/w")

    spy.assert_events_occured(
        [
            ("fs.entry.downsynced", {"workspace_id": wid, "id": wid}, Pendulum(2000, 1, 5)),
            ("fs.entry.synced", {"workspace_id": wid, "id": wid}, Pendulum(2000, 1, 5)),
            ("fs.entry.synced", {"workspace_id": wid, "id": spy.ANY}, Pendulum(2000, 1, 5)),
            ("fs.entry.synced", {"workspace_id": wid, "id": spy.ANY}, Pendulum(2000, 1, 5)),
        ]
    )

    with alice_fs.event_bus.listen() as spy:
        with freeze_time("2000-01-06"):
            await alice_fs.sync("/w")

    spy.assert_events_occured(
        [("fs.entry.downsynced", {"workspace_id": wid, "id": wid}, Pendulum(2000, 1, 6))]
    )

    # 3) Finally make sure both fs have the same data

    final_fs = await assert_same_fs(alice_fs, alice2_fs)
    final_wkps = final_fs["children"]["w"]
    assert final_wkps["children"].keys() == {"foo.txt", "bar"}
    assert final_wkps["children"]["bar"]["children"].keys() == {"spam"}

    assert final_wkps["base_version"] == 3
    assert final_wkps["children"]["bar"]["base_version"] == 2
    assert final_wkps["children"]["foo.txt"]["base_version"] == 1

    data = await alice_fs.file_read("/w/foo.txt")
    data2 = await alice2_fs.file_read("/w/foo.txt")
    assert data == data2 == b""


@pytest.mark.trio
async def test_sync_growth_by_truncate_file(running_backend, alice_fs, alice2_fs):
    await create_shared_workspace("w", alice_fs, alice2_fs)

    # Growth by truncate is special because no blocks are created to hold
    # the newly created null bytes

    with freeze_time("2000-01-02"):
        await alice_fs.touch("/w/foo.txt")

    with freeze_time("2000-01-03"):
        await alice_fs.file_truncate("/w/foo.txt", length=24)

    await alice_fs.sync("/w")
    await alice2_fs.sync("/w")

    stat = await alice2_fs.stat("/w/foo.txt")
    assert stat["size"] == 24
    data = await alice2_fs.file_read("/w/foo.txt")
    assert data == b"\x00" * 24


@pytest.mark.trio
async def test_concurrent_update(running_backend, alice_user_fs, alice2_user_fs):
    wid = await create_shared_workspace("w", alice_user_fs, alice2_user_fs)
    workspace = alice_user_fs.get_workspace(wid)
    workspace2 = alice2_user_fs.get_workspace(wid)

    # 1) Create existing items in both fs

    with freeze_time("2000-01-02"):
        await workspace.touch("/foo.txt")
        await workspace.write_bytes("/foo.txt", b"v1")
        await workspace.mkdir("/bar")
        barid = await workspace.path_id("/bar")

    await workspace.sync("/")
    await workspace2.sync("/")

    # 2) Make both fs diverged

    with freeze_time("2000-01-03"):
        await workspace.write_bytes("/foo.txt", b"alice's v2")
        await workspace.mkdir("/bar/from_alice")
        await workspace.mkdir("/bar/spam")
        await workspace.touch("/bar/buzz.txt")

    with freeze_time("2000-01-04"):
        await workspace2.write_bytes("/foo.txt", b"alice2's v2")
        await workspace2.mkdir("/bar/from_alice2")
        await workspace2.mkdir("/bar/spam")
        await workspace2.touch("/bar/buzz.txt")

    # 3) Sync Alice first, should go fine

    with alice_user_fs.event_bus.listen() as spy:
        with freeze_time("2000-01-05"):
            await workspace.sync("/")
    date_sync = Pendulum(2000, 1, 5)
    spy.assert_events_occured(
        [
            ("fs.entry.synced", {"workspace_id": wid, "id": barid}, date_sync),
            # TODO: add more events
        ]
    )

    # 4) Sync Alice2, with conflicts on `/z`, `/w/bar/buzz.txt` and `/w/bar/spam`

    with alice2_user_fs.event_bus.listen() as spy:
        with freeze_time("2000-01-06"):
            await workspace2.sync("/")
    date_sync = Pendulum(2000, 1, 6)
    spy.assert_events_occured(
        [
            ("fs.entry.synced", {"workspace_id": wid, "id": barid}, date_sync),
            # TODO: add more events
        ]
    )
    expected = [
        "/bar/buzz (conflicting with alice@dev1).txt",
        "/bar/buzz.txt",
        "/bar/from_alice",
        "/bar/from_alice2",
        "/bar/spam",
        "/bar/spam (conflicting with alice@dev1)",
    ]
    children = sorted(str(x) for x in await workspace2.listdir("/bar"))
    assert children == expected

    # 5) Sync Alice again to take into account changes from the second fs's sync

    with alice_user_fs.event_bus.listen() as spy:
        with freeze_time("2000-01-08"):
            await workspace.sync("/")
    date_sync = Pendulum(2000, 1, 8)
    spy.assert_events_occured(
        [
            ("fs.entry.downsynced", {"workspace_id": wid, "id": barid}, date_sync),
            # TODO: add more events
        ]
    )
    children = sorted(str(x) for x in await workspace2.listdir("/bar"))
    assert children == expected

    # TODO: add tests for file conflicts


@pytest.mark.trio
async def test_create_already_existing_folder_vlob(running_backend, alice_user_fs, alice2_user_fs):

    # First create data locally
    with freeze_time("2000-01-02"):
        wid = await create_shared_workspace("w", alice_user_fs, alice2_user_fs)
        workspace = alice_user_fs.get_workspace(wid)
        await workspace.mkdir("/x")

    remote_loader = workspace.remote_loader
    original_vlob_create = remote_loader._vlob_create

    async def mocked_vlob_create(*args, **kwargs):
        await original_vlob_create(*args, **kwargs)
        raise FSBackendOfflineError

    remote_loader._vlob_create = mocked_vlob_create

    with pytest.raises(FSBackendOfflineError):
        await workspace.sync("/")

    remote_loader._vlob_create = original_vlob_create
    await workspace.sync("/")

    info = await workspace.path_info("/")
    assert info == {
        "type": "folder",
        "id": wid,
        "base_version": 2,
        "is_placeholder": False,
        "need_sync": False,
        "created": Pendulum(2000, 1, 2),
        "updated": Pendulum(2000, 1, 2),
        "children": ["x"],
    }

    workspace2 = alice2_user_fs.get_workspace(wid)
    await workspace2.sync("/")
    info2 = await workspace2.path_info("/")
    assert info == info2


@pytest.mark.trio
@pytest.mark.skip  # TODO: rewrite this test
async def test_create_already_existing_file_vlob(running_backend, alice_fs, alice2_fs):
    await create_shared_workspace("w", alice_fs, alice2_fs)

    # First create data locally
    with freeze_time("2000-01-02"):
        fd = await alice_fs.file_create("/w/foo.txt")
        await alice_fs.file_fd_close(fd)

    vanilla_backend_vlob_create = alice_fs._syncer._backend_vlob_create

    async def mocked_backend_vlob_create(*args, **kwargs):
        await vanilla_backend_vlob_create(*args, **kwargs)
        raise BackendNotAvailable()

    alice_fs._syncer._backend_vlob_create = mocked_backend_vlob_create

    with pytest.raises(BackendNotAvailable):
        await alice_fs.sync("/w/foo.txt")

    alice_fs._syncer._backend_vlob_create = vanilla_backend_vlob_create
    await alice_fs.sync("/w/foo.txt")

    stat = await alice_fs.stat("/w/foo.txt")
    assert stat == {
        "type": "file",
        "id": ANY,
        "is_folder": False,
        "is_placeholder": False,
        "need_sync": False,
        "created": Pendulum(2000, 1, 2),
        "updated": Pendulum(2000, 1, 2),
        "base_version": 1,
        "size": 0,
    }

    await alice2_fs.sync("/w")
    stat2 = await alice2_fs.stat("/w/foo.txt")
    assert stat == stat2


@pytest.mark.trio
@pytest.mark.skip  # TODO: rewrite this test
async def test_create_already_existing_block(running_backend, alice_fs, alice2_fs):
    # First create&sync an empty file

    with freeze_time("2000-01-02"):
        await alice_fs.workspace_create("/w")
        fd = await alice_fs.file_create("/w/foo.txt")
        await alice_fs.file_fd_close(fd)
        await alice_fs.sync("/w/foo.txt")

    stat = await alice_fs.stat("/w/foo.txt")
    assert stat["base_version"] == 1

    # Now hack a bit the fs to simulate poor connection with backend

    vanilla_backend_block_create = alice_fs._syncer._backend_block_create

    async def mocked_backend_block_create(*args, **kwargs):
        await vanilla_backend_block_create(*args, **kwargs)
        raise BackendNotAvailable()

    alice_fs._syncer._backend_block_create = mocked_backend_block_create

    # Write into the file locally and try to sync this.
    # We should end up with a block synced in the backend but still considered
    # as a dirty block in the fs.

    with freeze_time("2000-01-03"):
        await alice_fs.file_write("/w/foo.txt", b"data")
    with pytest.raises(BackendNotAvailable):
        await alice_fs.sync("/w/foo.txt")

    # Now retry the sync with a good connection, we should be able to reach
    # eventual consistency.

    alice_fs._syncer._backend_block_create = vanilla_backend_block_create
    await alice_fs.sync("/w/foo.txt")

    # Finally test this so-called consistency ;-)

    data = await alice_fs.file_read("/w/foo.txt")
    assert data == b"data"
    stat = await alice_fs.stat("/w/foo.txt")
    assert stat == {
        "type": "file",
        "id": ANY,
        "is_folder": False,
        "is_placeholder": False,
        "need_sync": False,
        "created": Pendulum(2000, 1, 2),
        "updated": Pendulum(2000, 1, 3),
        "base_version": 2,
        "size": 4,
    }

    await alice2_fs.sync("/")
    data2 = await alice2_fs.file_read("/w/foo.txt")
    assert data2 == b"data"


# TODO: test data/manifest updated between failed and new syncs
