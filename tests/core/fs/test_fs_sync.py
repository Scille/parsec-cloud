# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPL-3.0 2016-present Scille SAS

from parsec.core.backend_connection.exceptions import BackendOutOfBallparkError
from parsec.core.core_events import CoreEvent
import pytest
from pendulum import datetime
from unittest.mock import ANY
from parsec.core.fs.remote_loader import MANIFEST_STAMP_AHEAD_US
from parsec.crypto import SecretKey
from parsec.api.data import EntryName
from parsec.core.types import WorkspaceEntry, WorkspaceRole
from parsec.core.backend_connection import BackendNotAvailable
from parsec.core.fs.exceptions import FSBackendOfflineError, FSRemoteOperationError
from parsec.utils import BALLPARK_CLIENT_EARLY_OFFSET, BALLPARK_CLIENT_LATE_OFFSET

from tests.common import freeze_time, create_shared_workspace


async def assert_same_workspace(workspace, workspace2):
    async def _recursive_assert(workspace, workspace2, path):
        path_info_1 = await workspace.path_info(path)
        assert not path_info_1["need_sync"]
        path_info_2 = await workspace2.path_info(path)
        assert path_info_1 == path_info_2

        cooked_children = {}
        # TODO: type root should not exist
        if path_info_1["type"] in ("folder", "root"):
            for child in path_info_1["children"]:
                cooked_children[child] = await _recursive_assert(
                    workspace, workspace2, f"{path}/{child}"
                )
            path_info_1["children"] = cooked_children

        return path_info_1

    return await _recursive_assert(workspace, workspace2, "/")


@pytest.mark.trio
async def test_new_workspace(running_backend, alice_user_fs):
    with freeze_time("2000-01-02"):
        wid = await alice_user_fs.workspace_create(EntryName("w"))
        workspace = alice_user_fs.get_workspace(wid)

    with alice_user_fs.event_bus.listen() as spy:
        with freeze_time("2000-01-03"):
            await workspace.sync()
    spy.assert_events_occured(
        [(CoreEvent.FS_ENTRY_SYNCED, {"workspace_id": wid, "id": wid}, datetime(2000, 1, 3))]
    )

    workspace2 = alice_user_fs.get_workspace(wid)
    await alice_user_fs.sync()
    await workspace2.sync()

    workspace_entry = workspace.get_workspace_entry()
    path_info = await workspace.path_info("/")
    assert path_info == {
        "type": "folder",
        "id": wid,
        "is_placeholder": False,
        "need_sync": False,
        "base_version": 1,
        "children": [],
        "created": datetime(2000, 1, 2),
        "updated": datetime(2000, 1, 2),
        "confinement_point": None,
    }
    KEY = SecretKey.generate()
    workspace_entry = workspace_entry.evolve(key=KEY)
    assert workspace_entry == WorkspaceEntry(
        name=EntryName("w"),
        id=wid,
        key=KEY,
        encryption_revision=1,
        encrypted_on=datetime(2000, 1, 2),
        role_cached_on=datetime(2000, 1, 2),
        role=WorkspaceRole.OWNER,
    )
    workspace_entry2 = workspace.get_workspace_entry()
    workspace_entry2 = workspace_entry2.evolve(key=KEY)
    path_info2 = await workspace.path_info("/")
    assert workspace_entry == workspace_entry2
    assert path_info == path_info2


@pytest.mark.trio
@pytest.mark.parametrize("type", ["file", "folder"])
async def test_new_empty_entry(type, running_backend, alice_user_fs, alice2_user_fs):
    with freeze_time("2000-01-01"):
        wid = await create_shared_workspace(EntryName("w"), alice_user_fs, alice2_user_fs)
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
            await workspace.sync()

    if type == "file":  # TODO: file and folder should generate the same events after the migration
        expected_events = [
            (CoreEvent.FS_ENTRY_SYNCED, {"workspace_id": wid, "id": wid}, datetime(2000, 1, 3))
        ]
    else:
        expected_events = [
            (CoreEvent.FS_ENTRY_SYNCED, {"workspace_id": wid, "id": fid}, datetime(2000, 1, 3)),
            (CoreEvent.FS_ENTRY_SYNCED, {"workspace_id": wid, "id": wid}, datetime(2000, 1, 3)),
        ]
    spy.assert_events_occured(expected_events)

    workspace2 = alice2_user_fs.get_workspace(wid)
    await workspace2.sync()

    info = await workspace.path_info("/foo")
    if type == "file":
        assert {
            "type": "file",
            "id": ANY,
            "is_placeholder": False,
            "need_sync": False,
            "base_version": 1,
            "created": datetime(2000, 1, 2),
            "updated": datetime(2000, 1, 2),
            "size": 0,
            "confinement_point": None,
        } == info
    else:
        assert {
            "type": "folder",
            "id": ANY,
            "is_placeholder": False,
            "need_sync": False,
            "base_version": 1,
            "created": datetime(2000, 1, 2),
            "updated": datetime(2000, 1, 2),
            "children": [],
            "confinement_point": None,
        } == info
    info2 = await workspace2.path_info("/foo")
    assert info == info2


@pytest.mark.trio
async def test_simple_sync(running_backend, alice_user_fs, alice2_user_fs):
    with freeze_time("2000-01-01"):
        wid = await create_shared_workspace(EntryName("w"), alice_user_fs, alice2_user_fs)
    workspace = alice_user_fs.get_workspace(wid)
    workspace2 = alice2_user_fs.get_workspace(wid)

    # 0) Make sure workspace is loaded for alice2
    # (otherwise won't get synced event during step 2)
    await workspace2.path_info("/")

    # 1) Create&sync file

    with freeze_time("2000-01-02"):
        await workspace.touch("/foo.txt")

    with freeze_time("2000-01-03"):
        await workspace.write_bytes("/foo.txt", b"hello world !")

    with workspace.event_bus.listen() as spy:
        with freeze_time("2000-01-04"):
            await workspace.sync()
    spy.assert_events_occured(
        [(CoreEvent.FS_ENTRY_SYNCED, {"workspace_id": wid, "id": wid}, datetime(2000, 1, 4))]
    )

    # 2) Fetch back file from another fs

    with alice2_user_fs.event_bus.listen() as spy:
        with freeze_time("2000-01-05"):
            # TODO: `sync` on not loaded entry should load it
            await workspace2.sync()
    spy.assert_events_occured(
        [(CoreEvent.FS_ENTRY_DOWNSYNCED, {"workspace_id": wid, "id": wid}, datetime(2000, 1, 5))]
    )

    # 3) Finally make sure both fs have the same data
    final_fs = await assert_same_workspace(workspace, workspace2)
    assert final_fs["children"].keys() == {EntryName("foo.txt")}

    data = await workspace.read_bytes("/foo.txt")
    data2 = await workspace2.read_bytes("/foo.txt")
    assert data == data2


@pytest.mark.trio
async def test_fs_recursive_sync(running_backend, alice_user_fs):
    with freeze_time("2000-01-01"):
        wid = await create_shared_workspace(EntryName("w"), alice_user_fs)
    workspace = alice_user_fs.get_workspace(wid)

    # 1) Create data

    with freeze_time("2000-01-02"):
        await workspace.touch("/foo.txt")
        await workspace.mkdir("/bar")
        await workspace.touch("/bar/wizz.txt")
        await workspace.mkdir("/bar/spam")

    # 2) Sync it

    with alice_user_fs.event_bus.listen() as spy:
        with freeze_time("2000-01-03"):
            await workspace.sync()
    sync_date = datetime(2000, 1, 3)
    spy.assert_events_occured(
        [
            (CoreEvent.FS_ENTRY_SYNCED, {"workspace_id": wid, "id": spy.ANY}, sync_date),
            (CoreEvent.FS_ENTRY_SYNCED, {"workspace_id": wid, "id": spy.ANY}, sync_date),
            (CoreEvent.FS_ENTRY_SYNCED, {"workspace_id": wid, "id": spy.ANY}, sync_date),
        ]
    )

    # 2) Now additional sync should not trigger any event

    with alice_user_fs.event_bus.listen() as spy:
        with freeze_time("2000-01-04"):
            await workspace.sync()
    assert not spy.events

    # 3) Make sure everything is considered synced
    for path in ["/", "/foo.txt", "/bar", "/bar/wizz.txt", "/bar/spam"]:
        path_info = await workspace.path_info(path)
        assert not path_info["need_sync"]


# TODO: a complex but interesting test would be to do concurrent changes
# during sync


@pytest.mark.trio
async def test_cross_sync(running_backend, alice_user_fs, alice2_user_fs):
    with freeze_time("2000-01-01"):
        wid = await create_shared_workspace(EntryName("w"), alice_user_fs, alice2_user_fs)
    workspace = alice_user_fs.get_workspace(wid)
    workspace2 = alice2_user_fs.get_workspace(wid)

    # 1) Both fs have things to sync

    with freeze_time("2000-01-02"):
        await workspace.touch("/foo.txt")

    with freeze_time("2000-01-03"):
        await workspace2.mkdir("/bar")
        await workspace2.mkdir("/bar/spam")

    # 2) Do the cross sync

    with workspace.event_bus.listen() as spy:
        with freeze_time("2000-01-04"):
            await workspace.sync()

    spy.assert_events_occured(
        [(CoreEvent.FS_ENTRY_SYNCED, {"workspace_id": wid, "id": wid}, datetime(2000, 1, 4))]
    )

    with alice2_user_fs.event_bus.listen() as spy:
        with freeze_time("2000-01-05"):
            await workspace2.sync()

    spy.assert_events_occured(
        [
            (CoreEvent.FS_ENTRY_DOWNSYNCED, {"workspace_id": wid, "id": wid}, datetime(2000, 1, 5)),
            (CoreEvent.FS_ENTRY_SYNCED, {"workspace_id": wid, "id": wid}, datetime(2000, 1, 5)),
            (CoreEvent.FS_ENTRY_SYNCED, {"workspace_id": wid, "id": spy.ANY}, datetime(2000, 1, 5)),
            (CoreEvent.FS_ENTRY_SYNCED, {"workspace_id": wid, "id": spy.ANY}, datetime(2000, 1, 5)),
        ]
    )

    with workspace.event_bus.listen() as spy:
        with freeze_time("2000-01-06"):
            await workspace.sync()

    spy.assert_events_occured(
        [(CoreEvent.FS_ENTRY_DOWNSYNCED, {"workspace_id": wid, "id": wid}, datetime(2000, 1, 6))]
    )

    # 3) Finally make sure both fs have the same data

    final_wkps = await assert_same_workspace(workspace, workspace2)
    assert final_wkps["children"].keys() == {EntryName("foo.txt"), EntryName("bar")}
    assert final_wkps["children"][EntryName("bar")]["children"].keys() == {EntryName("spam")}

    assert final_wkps["base_version"] == 3
    assert final_wkps["children"][EntryName("bar")]["base_version"] == 2
    assert final_wkps["children"][EntryName("foo.txt")]["base_version"] == 1

    data = await workspace.read_bytes("/foo.txt")
    data2 = await workspace2.read_bytes("/foo.txt")
    assert data == data2 == b""


@pytest.mark.trio
async def test_sync_growth_by_truncate_file(running_backend, alice_user_fs, alice2_user_fs):
    with freeze_time("2000-01-01"):
        wid = await create_shared_workspace(EntryName("w"), alice_user_fs, alice2_user_fs)
    workspace = alice_user_fs.get_workspace(wid)
    workspace2 = alice2_user_fs.get_workspace(wid)

    # Growth by truncate is special because no blocks are created to hold
    # the newly created null bytes

    with freeze_time("2000-01-02"):
        await workspace.touch("/foo.txt")

    with freeze_time("2000-01-03"):
        await workspace.truncate("/foo.txt", length=24)

    await workspace.sync()
    await workspace2.sync()

    path_info = await workspace2.path_info("/foo.txt")
    assert path_info["size"] == 24
    data = await workspace2.read_bytes("/foo.txt")
    assert data == b"\x00" * 24


@pytest.mark.trio
async def test_concurrent_update(running_backend, alice_user_fs, alice2_user_fs):
    with freeze_time("2000-01-01"):
        wid = await create_shared_workspace(EntryName("w"), alice_user_fs, alice2_user_fs)
    workspace = alice_user_fs.get_workspace(wid)
    workspace2 = alice2_user_fs.get_workspace(wid)

    # 1) Create existing items in both fs

    with freeze_time("2000-01-02"):
        await workspace.touch("/foo.txt")
        await workspace.write_bytes("/foo.txt", b"v1")
        await workspace.mkdir("/bar")
        barid = await workspace.path_id("/bar")

        await workspace.sync()
        await workspace2.sync()

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
            await workspace.sync()
    date_sync = datetime(2000, 1, 5)
    spy.assert_events_occured(
        [
            (CoreEvent.FS_ENTRY_SYNCED, {"workspace_id": wid, "id": barid}, date_sync),
            # TODO: add more events
        ]
    )

    # 4) Sync Alice2, with conflicts on `/foo.txt`, `/bar/buzz.txt` and `/bar/spam`

    with alice2_user_fs.event_bus.listen() as spy:
        with freeze_time("2000-01-06"):
            await workspace2.sync()
    date_sync = datetime(2000, 1, 6)
    spy.assert_events_occured(
        [
            (CoreEvent.FS_ENTRY_SYNCED, {"workspace_id": wid, "id": barid}, date_sync),
            # TODO: add more events
        ]
    )
    root_expected = ["/bar", "/foo (Parsec - content conflict).txt", "/foo.txt"]
    bar_expected = [
        "/bar/buzz (Parsec - name conflict).txt",
        "/bar/buzz.txt",
        "/bar/from_alice",
        "/bar/from_alice2",
        "/bar/spam",
        "/bar/spam (Parsec - name conflict)",
    ]
    root_children = sorted(str(x) for x in await workspace2.listdir("/"))
    bar_children = sorted(str(x) for x in await workspace2.listdir("/bar"))
    assert root_children == root_expected
    assert bar_children == bar_expected

    # 5) Sync Alice again to take into account changes from the second fs's sync

    with alice_user_fs.event_bus.listen() as spy:
        with freeze_time("2000-01-08"):
            await workspace.sync()
    date_sync = datetime(2000, 1, 8)
    spy.assert_events_occured(
        [
            (CoreEvent.FS_ENTRY_DOWNSYNCED, {"workspace_id": wid, "id": barid}, date_sync),
            # TODO: add more events
        ]
    )
    root_children = sorted(str(x) for x in await workspace.listdir("/"))
    bar_children = sorted(str(x) for x in await workspace.listdir("/bar"))
    assert root_children == root_expected
    assert bar_children == bar_expected

    # TODO: add more tests for file conflicts


@pytest.mark.trio
async def test_update_invalid_timestamp(running_backend, alice_user_fs, alice2_user_fs):
    with freeze_time("2000-01-01"):
        wid = await create_shared_workspace(EntryName("w"), alice_user_fs, alice2_user_fs)
    workspace = alice_user_fs.get_workspace(wid)
    await workspace.touch("/foo.txt")
    with freeze_time("2000-01-02") as t2:
        await workspace.sync()
    await workspace.write_bytes("/foo.txt", b"ok")
    with freeze_time("2000-01-03") as t3:
        await workspace.sync()
    await workspace.write_bytes("/foo.txt", b"ko")
    with freeze_time(t2):
        with pytest.raises(FSRemoteOperationError) as context:
            await workspace.sync()
        cause = context.value.__cause__
        assert isinstance(cause, BackendOutOfBallparkError)
        rep, = cause.args
        assert rep == {
            "status": "bad_timestamp",
            "client_timestamp": t3.add(microseconds=MANIFEST_STAMP_AHEAD_US),
            "backend_timestamp": t2,
            "ballpark_client_early_offset": BALLPARK_CLIENT_EARLY_OFFSET,
            "ballpark_client_late_offset": BALLPARK_CLIENT_LATE_OFFSET,
        }


@pytest.mark.trio
async def test_create_already_existing_folder_vlob(running_backend, alice_user_fs, alice2_user_fs):

    # First create data locally
    with freeze_time("2000-01-02"):
        wid = await create_shared_workspace(EntryName("w"), alice_user_fs, alice2_user_fs)
        workspace = alice_user_fs.get_workspace(wid)
        await workspace.mkdir("/x")

    remote_loader = workspace.remote_loader
    original_vlob_create = remote_loader._vlob_create

    async def mocked_vlob_create(*args, **kwargs):
        await original_vlob_create(*args, **kwargs)
        raise FSBackendOfflineError

    remote_loader._vlob_create = mocked_vlob_create

    with pytest.raises(FSBackendOfflineError):
        await workspace.sync()

    remote_loader._vlob_create = original_vlob_create
    await workspace.sync()

    info = await workspace.path_info("/")
    assert info == {
        "type": "folder",
        "id": wid,
        "base_version": 2,
        "is_placeholder": False,
        "need_sync": False,
        "created": datetime(2000, 1, 2),
        "updated": datetime(2000, 1, 2),
        "children": [EntryName("x")],
        "confinement_point": None,
    }

    workspace2 = alice2_user_fs.get_workspace(wid)
    await workspace2.sync()
    info2 = await workspace2.path_info("/")
    assert info == info2


@pytest.mark.trio
@pytest.mark.skip  # TODO: rewrite this test
async def test_create_already_existing_file_vlob(running_backend, alice_user_fs, alice2_user_fs):
    with freeze_time("2000-01-01"):
        wid = await create_shared_workspace(EntryName("w"), alice_user_fs, alice2_user_fs)
    workspace = alice_user_fs.get_workspace(wid)
    workspace2 = alice2_user_fs.get_workspace(wid)

    # First create data locally
    with freeze_time("2000-01-02"):
        await workspace.touch("/foo.txt")
        foo_id = await workspace.path_id("/foo.txt")

    vanilla_backend_vlob_create = alice_user_fs._syncer._backend_vlob_create

    async def mocked_backend_vlob_create(*args, **kwargs):
        await vanilla_backend_vlob_create(*args, **kwargs)
        raise BackendNotAvailable()

    alice_user_fs._syncer._backend_vlob_create = mocked_backend_vlob_create

    with pytest.raises(BackendNotAvailable):
        await workspace.sync_by_id(foo_id)

    alice_user_fs._syncer._backend_vlob_create = vanilla_backend_vlob_create
    await workspace.sync_by_id(foo_id)

    path_info = await workspace.path_info("/foo.txt")
    assert path_info == {
        "type": "file",
        "id": ANY,
        "is_placeholder": False,
        "need_sync": False,
        "created": datetime(2000, 1, 2),
        "updated": datetime(2000, 1, 2),
        "base_version": 1,
        "size": 0,
        "confinement_point": None,
    }

    await workspace2.sync()
    path_info_2 = await workspace2.path_info("/foo.txt")
    assert path_info == path_info_2


@pytest.mark.trio
@pytest.mark.skip  # TODO: rewrite this test
async def test_create_already_existing_block(running_backend, alice_user_fs, alice2_user_fs):
    # First create&sync an empty file

    with freeze_time("2000-01-02"):
        wid = await create_shared_workspace(EntryName("w"), alice_user_fs, alice2_user_fs)
        workspace = alice_user_fs.get_workspace(wid)
        workspace2 = alice2_user_fs.get_workspace(wid)
        await workspace.touch("/foo.txt")
        foo_id = await workspace.path_id("/foo.txt")
        await workspace.sync_by_id(foo_id)

    path_info = await workspace.path_info("/foo.txt")
    assert path_info["base_version"] == 1

    # Now hack a bit the fs to simulate poor connection with backend

    vanilla_backend_block_create = alice_user_fs._syncer._backend_block_create

    async def mocked_backend_block_create(*args, **kwargs):
        await vanilla_backend_block_create(*args, **kwargs)
        raise BackendNotAvailable()

    alice_user_fs._syncer._backend_block_create = mocked_backend_block_create

    # Write into the file locally and try to sync this.
    # We should end up with a block synced in the backend but still considered
    # as a dirty block in the fs.

    with freeze_time("2000-01-03"):
        await workspace.write_bytes("/foo.txt", b"data")
    with pytest.raises(BackendNotAvailable):
        await workspace.sync_by_id(foo_id)

    # Now retry the sync with a good connection, we should be able to reach
    # eventual consistency.

    alice_user_fs._syncer._backend_block_create = vanilla_backend_block_create
    await workspace.sync_by_id(foo_id)

    # Finally test this so-called consistency ;-)

    data = await workspace.read_bytes("/foo.txt")
    assert data == b"data"
    path_info = await workspace.path_info("/foo.txt")
    assert path_info == {
        "type": "file",
        "id": ANY,
        "is_placeholder": False,
        "need_sync": False,
        "created": datetime(2000, 1, 2),
        "updated": datetime(2000, 1, 3),
        "base_version": 2,
        "size": 4,
        "confinement_point": None,
    }

    await workspace2.sync()
    data2 = await workspace2.read_bytes("/foo.txt")
    assert data2 == b"data"


@pytest.mark.trio
async def test_sync_data_before_workspace(running_backend, alice_user_fs):
    with freeze_time("2000-01-02"):
        wid = await alice_user_fs.workspace_create(EntryName("w"))
    w = alice_user_fs.get_workspace(wid)
    with freeze_time("2000-01-03"):
        await w.mkdir("/bar")
    with freeze_time("2000-01-04"):
        await w.touch("/bar/foo.txt")
        foo_id = await w.path_id("/bar/foo.txt")
    with freeze_time("2000-01-05"):
        await w.write_bytes("/bar/foo.txt", b"v2")

    # Syncing
    with freeze_time("2000-01-06"):
        await w.sync_by_id(foo_id)

    # Just to be sure, do
    await alice_user_fs.sync()

    foo_info = await w.path_info("/bar/foo.txt")
    assert {
        "id": ANY,
        "type": "file",
        "base_version": 1,
        "created": datetime(2000, 1, 4),
        "updated": datetime(2000, 1, 5),
        "is_placeholder": False,
        "need_sync": False,
        "size": 2,
        "confinement_point": None,
    } == foo_info
    root_info = await w.path_info("/")
    assert {
        "id": wid,
        "type": "folder",
        "base_version": 1,
        "created": datetime(2000, 1, 2),
        "updated": datetime(2000, 1, 3),
        "is_placeholder": False,
        "need_sync": True,
        "children": [EntryName("bar")],
        "confinement_point": None,
    } == root_info


@pytest.mark.trio
async def test_merge_resulting_in_no_need_for_sync(running_backend, user_fs_factory, alice, alice2):
    async with user_fs_factory(alice) as alice_user_fs:
        async with user_fs_factory(alice2) as alice2_user_fs:
            # Create a workspace with an entry
            with freeze_time("2000-01-02"):
                wksp_id = await alice_user_fs.workspace_create(EntryName("wksp"))
                alice_wksp = alice_user_fs.get_workspace(wksp_id)
                await alice_wksp.mkdir("/foo")

            # Sync for Alice and Alice2
            with freeze_time("2000-01-03"):
                await alice_user_fs.sync()
                await alice_wksp.sync()
                await alice2_user_fs.sync()
                alice2_wksp = alice2_user_fs.get_workspace(wksp_id)
                await alice2_wksp.sync()

            # Alice remove the entry and sync this
            with freeze_time("2000-01-04"):
                await alice_wksp.rmdir("/foo")
                await alice_wksp.sync()

            # Now Alice2 does the same, this should not create any remote changes
            with freeze_time("2000-01-05"):
                await alice2_wksp.rmdir("/foo")
                await alice2_wksp.sync()

            # Now, both user fs should have the same view on workspace
            expected_alice_wksp_stat = {
                "id": wksp_id,
                "base_version": 3,
                "created": datetime(2000, 1, 2),
                "updated": datetime(2000, 1, 4),
                "is_placeholder": False,
                "need_sync": False,
                "type": "folder",
                "children": [],
                "confinement_point": None,
            }
            alice_wksp_stat = await alice_wksp.path_info("/")
            alice2_wksp_stat = await alice2_wksp.path_info("/")
            assert alice_wksp_stat == expected_alice_wksp_stat
            assert alice2_wksp_stat == expected_alice_wksp_stat


# TODO: test data/manifest updated between failed and new syncs
