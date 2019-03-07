# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from pendulum import Pendulum

from parsec.core.types import FsPath
from parsec.core.backend_connection import BackendNotAvailable

from tests.common import freeze_time, create_shared_workspace


async def assert_same_fs(fs1, fs2):
    async def _recursive_assert(fs1, fs2, path):
        stat1 = await fs1.stat(path)
        assert not stat1["need_sync"]
        stat2 = await fs2.stat(path)
        assert stat1 == stat2

        cooked_children = {}
        if stat1["is_folder"]:
            for child in stat1["children"]:
                cooked_children[child] = await _recursive_assert(fs1, fs2, f"{path}/{child}")
            stat1["children"] = cooked_children

        return stat1

    return await _recursive_assert(fs1, fs2, "/")


@pytest.mark.trio
async def test_new_workspace(running_backend, alice, alice_fs, alice2_fs):
    with freeze_time("2000-01-02"):
        w_id = await alice_fs.workspace_create("/w")

    with alice_fs.event_bus.listen() as spy:
        with freeze_time("2000-01-03"):
            await alice_fs.sync("/w")
    spy.assert_events_occured(
        [
            ("fs.entry.minimal_synced", {"path": "/w", "id": spy.ANY}, Pendulum(2000, 1, 3)),
            ("fs.entry.synced", {"path": "/", "id": spy.ANY}, Pendulum(2000, 1, 3)),
            ("fs.entry.synced", {"path": "/w", "id": spy.ANY}, Pendulum(2000, 1, 3)),
        ]
    )

    await alice2_fs.sync("/")

    stat = await alice_fs.stat("/w")
    assert stat == {
        "type": "workspace",
        "id": w_id,
        "admin_right": True,
        "read_right": True,
        "write_right": True,
        "is_folder": True,
        "is_placeholder": False,
        "need_sync": False,
        "base_version": 1,
        "created": Pendulum(2000, 1, 2),
        "updated": Pendulum(2000, 1, 2),
        "participants": [alice.user_id],
        "creator": alice.user_id,
        "children": [],
    }
    stat2 = await alice2_fs.stat("/w")
    assert stat == stat2


@pytest.mark.trio
@pytest.mark.parametrize("type", ["file", "folder"])
async def test_new_empty_entry(type, running_backend, alice_fs, alice2_fs):
    await create_shared_workspace("w", alice_fs, alice2_fs)
    with freeze_time("2000-01-02"):
        if type == "file":
            obj_id = await alice_fs.file_create("/w/foo")
        else:
            obj_id = await alice_fs.folder_create("/w/foo")

    with alice_fs.event_bus.listen() as spy:
        with freeze_time("2000-01-03"):
            await alice_fs.sync("/w")
    spy.assert_events_occured(
        [
            ("fs.entry.minimal_synced", {"path": "/w/foo", "id": spy.ANY}, Pendulum(2000, 1, 3)),
            ("fs.entry.synced", {"path": "/w", "id": spy.ANY}, Pendulum(2000, 1, 3)),
            ("fs.entry.synced", {"path": "/w/foo", "id": spy.ANY}, Pendulum(2000, 1, 3)),
        ]
    )

    await alice2_fs.sync("/w")

    stat = await alice_fs.stat("/w/foo")
    if type == "file":
        assert stat == {
            "type": "file",
            "id": obj_id,
            "is_folder": False,
            "is_placeholder": False,
            "need_sync": False,
            "base_version": 1,
            "created": Pendulum(2000, 1, 2),
            "updated": Pendulum(2000, 1, 2),
            "size": 0,
        }
    else:
        assert stat == {
            "type": "folder",
            "id": obj_id,
            "is_folder": True,
            "is_placeholder": False,
            "need_sync": False,
            "base_version": 1,
            "created": Pendulum(2000, 1, 2),
            "updated": Pendulum(2000, 1, 2),
            "children": [],
        }
    stat2 = await alice2_fs.stat("/w/foo")
    assert stat == stat2


@pytest.mark.trio
async def test_simple_sync(running_backend, alice_fs, alice2_fs):
    await create_shared_workspace("/w", alice_fs, alice2_fs)

    # 0) Make sure workspace is loaded for alice2
    # (otherwise won't get synced event during step 2)
    await alice2_fs.stat("/w")

    # 1) Create&sync file

    with freeze_time("2000-01-02"):
        await alice_fs.file_create("/w/foo.txt")

    with freeze_time("2000-01-03"):
        await alice_fs.file_write("/w/foo.txt", b"hello world !")

    with alice_fs.event_bus.listen() as spy:
        with freeze_time("2000-01-04"):
            await alice_fs.sync("/w")
    spy.assert_events_occured(
        [
            (
                "fs.entry.minimal_synced",
                {"path": "/w/foo.txt", "id": spy.ANY},
                Pendulum(2000, 1, 4),
            ),
            ("fs.entry.synced", {"path": "/w", "id": spy.ANY}, Pendulum(2000, 1, 4)),
            ("fs.entry.synced", {"path": "/w/foo.txt", "id": spy.ANY}, Pendulum(2000, 1, 4)),
        ]
    )

    # 2) Fetch back file from another fs

    with alice2_fs.event_bus.listen() as spy:
        with freeze_time("2000-01-05"):
            # TODO: `sync` on not loaded entry should load it
            await alice2_fs.sync("/w")
    spy.assert_events_occured(
        [("fs.entry.remote_changed", {"path": "/w", "id": spy.ANY}, Pendulum(2000, 1, 5))]
    )

    # 3) Finally make sure both fs have the same data
    final_fs = await assert_same_fs(alice_fs, alice2_fs)
    assert final_fs["children"]["w"]["children"].keys() == {"foo.txt"}

    data = await alice_fs.file_read("/w/foo.txt")
    data2 = await alice2_fs.file_read("/w/foo.txt")
    assert data == data2


@pytest.mark.trio
async def test_fs_recursive_sync(running_backend, alice_fs):
    await create_shared_workspace("/w", alice_fs)

    # 1) Create data

    with freeze_time("2000-01-02"):
        await alice_fs.file_create("/w/foo.txt")
        await alice_fs.folder_create("/w/bar")
        await alice_fs.file_create("/w/bar/wizz.txt")
        await alice_fs.folder_create("/w/bar/spam")

    # 2) Sync it

    with alice_fs.event_bus.listen() as spy:
        with freeze_time("2000-01-03"):
            await alice_fs.sync("/w")
    sync_date = Pendulum(2000, 1, 3)
    spy.assert_events_occured(
        [
            # Minimal sync on /w/bar and sync it access into it parent
            ("fs.entry.minimal_synced", {"path": "/w/bar", "id": spy.ANY}, sync_date),
            ("fs.entry.synced", {"path": "/w", "id": spy.ANY}, sync_date),
            # Recursive sync on /w/bar children
            ("fs.entry.minimal_synced", {"path": "/w/bar/spam", "id": spy.ANY}, sync_date),
            ("fs.entry.synced", {"path": "/w/bar", "id": spy.ANY}, sync_date),
            ("fs.entry.synced", {"path": "/w/bar/spam", "id": spy.ANY}, sync_date),
            ("fs.entry.minimal_synced", {"path": "/w/bar/wizz.txt", "id": spy.ANY}, sync_date),
            ("fs.entry.synced", {"path": "/w/bar", "id": spy.ANY}, sync_date),
            ("fs.entry.synced", {"path": "/w/bar/wizz.txt", "id": spy.ANY}, sync_date),
            ("fs.entry.minimal_synced", {"path": "/w/foo.txt", "id": spy.ANY}, sync_date),
            ("fs.entry.synced", {"path": "/w", "id": spy.ANY}, sync_date),
            ("fs.entry.synced", {"path": "/w/foo.txt", "id": spy.ANY}, sync_date),
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
    await create_shared_workspace("/w", alice_fs, alice2_fs)

    # 1) Both fs have things to sync

    with freeze_time("2000-01-02"):
        await alice_fs.file_create("/w/foo.txt")

    with freeze_time("2000-01-03"):
        await alice2_fs.folder_create("/w/bar")
        await alice2_fs.folder_create("/w/bar/spam")

    # 2) Do the cross sync

    with alice_fs.event_bus.listen() as spy:
        with freeze_time("2000-01-04"):
            await alice_fs.sync("/w")

    spy.assert_events_occured(
        [
            (
                "fs.entry.minimal_synced",
                {"path": "/w/foo.txt", "id": spy.ANY},
                Pendulum(2000, 1, 4),
            ),
            ("fs.entry.synced", {"path": "/w", "id": spy.ANY}, Pendulum(2000, 1, 4)),
            ("fs.entry.synced", {"path": "/w/foo.txt", "id": spy.ANY}, Pendulum(2000, 1, 4)),
        ]
    )

    with alice2_fs.event_bus.listen() as spy:
        with freeze_time("2000-01-05"):
            await alice2_fs.sync("/w")

    spy.assert_events_occured(
        [
            ("fs.entry.minimal_synced", {"path": "/w/bar", "id": spy.ANY}, Pendulum(2000, 1, 5)),
            ("fs.entry.synced", {"path": "/w", "id": spy.ANY}, Pendulum(2000, 1, 5)),
            (
                "fs.entry.minimal_synced",
                {"path": "/w/bar/spam", "id": spy.ANY},
                Pendulum(2000, 1, 5),
            ),
            ("fs.entry.synced", {"path": "/w/bar", "id": spy.ANY}, Pendulum(2000, 1, 5)),
            ("fs.entry.synced", {"path": "/w/bar/spam", "id": spy.ANY}, Pendulum(2000, 1, 5)),
        ]
    )

    with alice_fs.event_bus.listen() as spy:
        with freeze_time("2000-01-06"):
            await alice_fs.sync("/w")

    spy.assert_events_occured(
        [("fs.entry.remote_changed", {"path": "/w", "id": spy.ANY}, Pendulum(2000, 1, 6))]
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
    await create_shared_workspace("/w", alice_fs, alice2_fs)

    # Growth by truncate is special because no blocks are created to hold
    # the newly created null bytes

    with freeze_time("2000-01-02"):
        await alice_fs.file_create("/w/foo.txt")

    with freeze_time("2000-01-03"):
        await alice_fs.file_truncate("/w/foo.txt", length=24)

    await alice_fs.sync("/w")
    await alice2_fs.sync("/w")

    stat = await alice2_fs.stat("/w/foo.txt")
    assert stat["size"] == 24
    data = await alice2_fs.file_read("/w/foo.txt")
    assert data == b"\x00" * 24


@pytest.mark.trio
async def test_concurrent_update(running_backend, alice_fs, alice2_fs):
    # TODO: break this test down to reduce complexity
    await create_shared_workspace("/w", alice_fs, alice2_fs)

    # 1) Create existing items in both fs

    with freeze_time("2000-01-02"):
        await alice_fs.file_create("/w/foo.txt")
        await alice_fs.file_write("/w/foo.txt", b"v1")
        await alice_fs.folder_create("/w/bar")

    await alice_fs.sync("/")
    await alice2_fs.sync("/")

    # 2) Make both fs diverged

    with freeze_time("2000-01-03"):
        z_by_alice_id = await alice_fs.workspace_create("/z")
        z_by_alice = alice_fs._local_folder_fs.get_access(FsPath("/z"))
        await alice_fs.file_write("/w/foo.txt", b"alice's v2")
        await alice_fs.folder_create("/w/bar/from_alice")
        await alice_fs.folder_create("/w/bar/spam")
        await alice_fs.file_create("/w/bar/buzz.txt")

    with freeze_time("2000-01-04"):
        z_by_alice2_id = await alice2_fs.workspace_create("/z")
        z_by_alice2 = alice2_fs._local_folder_fs.get_access(FsPath("/z"))
        await alice2_fs.file_write("/w/foo.txt", b"alice2's v2")
        await alice2_fs.folder_create("/w/bar/from_alice2")
        await alice2_fs.folder_create("/w/bar/spam")
        await alice2_fs.file_create("/w/bar/buzz.txt")

    # 3) Sync Alice first, should go fine

    with alice_fs.event_bus.listen() as spy:
        with freeze_time("2000-01-05"):
            await alice_fs.sync("/")
    date_sync = Pendulum(2000, 1, 5)
    spy.assert_events_exactly_occured(
        [
            ("fs.entry.minimal_synced", {"path": "/w/bar/buzz.txt", "id": spy.ANY}, date_sync),
            ("fs.entry.synced", {"path": "/w/bar", "id": spy.ANY}, date_sync),
            ("fs.entry.synced", {"path": "/w/bar/buzz.txt", "id": spy.ANY}, date_sync),
            ("fs.entry.minimal_synced", {"path": "/w/bar/from_alice", "id": spy.ANY}, date_sync),
            ("fs.entry.synced", {"path": "/w/bar", "id": spy.ANY}, date_sync),
            ("fs.entry.synced", {"path": "/w/bar/from_alice", "id": spy.ANY}, date_sync),
            ("fs.entry.minimal_synced", {"path": "/w/bar/spam", "id": spy.ANY}, date_sync),
            ("fs.entry.synced", {"path": "/w/bar", "id": spy.ANY}, date_sync),
            ("fs.entry.synced", {"path": "/w/bar/spam", "id": spy.ANY}, date_sync),
            ("fs.entry.synced", {"path": "/w/foo.txt", "id": spy.ANY}, date_sync),
            ("fs.entry.minimal_synced", {"path": "/z", "id": spy.ANY}, date_sync),
            ("fs.entry.synced", {"path": "/", "id": spy.ANY}, date_sync),
            ("fs.entry.synced", {"path": "/z", "id": spy.ANY}, date_sync),
        ]
    )
    stat = await alice_fs.stat("/z")
    assert not stat["need_sync"]
    assert stat["id"] == z_by_alice_id

    # 4) Sync Alice2, with conflicts on `/z`, `/w/bar/buzz.txt` and `/w/bar/spam`

    with alice2_fs.event_bus.listen() as spy:
        with freeze_time("2000-01-06"):
            await alice2_fs.sync("/")
    date_sync = Pendulum(2000, 1, 6)
    spy.assert_events_exactly_occured(
        [
            ("fs.entry.minimal_synced", {"path": "/w/bar/buzz.txt", "id": spy.ANY}, date_sync),
            # Try to sync `/w/bar` with new entry `/w/bar/buzz.txt`, get alice's changes
            (
                "fs.entry.name_conflicted",
                {
                    "path": "/w/bar/buzz.txt",
                    "diverged_path": "/w/bar/buzz (conflict 2000-01-06 00:00:00).txt",
                    "original_id": spy.ANY,
                    "diverged_id": spy.ANY,
                },
                date_sync,
            ),
            (
                "fs.entry.name_conflicted",
                {
                    "path": "/w/bar/spam",
                    "diverged_path": "/w/bar/spam (conflict 2000-01-06 00:00:00)",
                    "original_id": spy.ANY,
                    "diverged_id": spy.ANY,
                },
                date_sync,
            ),
            ("fs.entry.synced", {"path": "/w/bar", "id": spy.ANY}, date_sync),
            ("fs.entry.synced", {"path": "/w/bar/buzz.txt", "id": spy.ANY}, date_sync),
            ("fs.entry.minimal_synced", {"path": "/w/bar/from_alice2", "id": spy.ANY}, date_sync),
            ("fs.entry.synced", {"path": "/w/bar", "id": spy.ANY}, date_sync),
            ("fs.entry.synced", {"path": "/w/bar/from_alice2", "id": spy.ANY}, date_sync),
            ("fs.entry.synced", {"path": "/w/bar", "id": spy.ANY}, date_sync),
            (
                "fs.entry.file_update_conflicted",
                {
                    "path": "/w/foo.txt",
                    "diverged_path": "/w/foo (conflict 2000-01-06 00:00:00).txt",
                    "original_id": spy.ANY,
                    "diverged_id": spy.ANY,
                },
                date_sync,
            ),
            # Event to notify sync monitor of `/w/foo (conflict ...).txt` creation
            ("fs.entry.updated", {"id": spy.ANY}, date_sync),
            ("fs.entry.synced", {"path": "/w/foo.txt", "id": spy.ANY}, date_sync),
            ("fs.entry.synced", {"path": "/w", "id": spy.ANY}, date_sync),
            ("fs.entry.minimal_synced", {"path": "/z", "id": spy.ANY}, date_sync),
            (
                "fs.entry.name_conflicted",
                {
                    "path": "/z",
                    "diverged_path": "/z (conflict 2000-01-06 00:00:00)",
                    "original_id": spy.ANY,
                    "diverged_id": spy.ANY,
                },
                date_sync,
            ),
            ("fs.entry.synced", {"path": "/", "id": spy.ANY}, date_sync),
            ("fs.entry.synced", {"path": "/z", "id": spy.ANY}, date_sync),
        ]
    )
    stat = await alice2_fs.stat("/z")
    assert not stat["need_sync"]
    assert stat["id"] == z_by_alice_id

    stat = await alice2_fs.stat("/z (conflict 2000-01-06 00:00:00)")
    assert not stat["need_sync"]
    assert stat["id"] == z_by_alice2_id

    # 4) Sync another time Alice2, needed for diverged files created from conflicts
    # Note name conflicts has already been synchronized given they are not
    # made of a placeholder (hence a simple sync on the parent contain them).

    with alice2_fs.event_bus.listen() as spy:
        with freeze_time("2000-01-07"):
            await alice2_fs.sync("/")
    date_sync = Pendulum(2000, 1, 7)
    spy.assert_events_exactly_occured(
        [
            (
                "fs.entry.minimal_synced",
                {"path": "/w/bar/spam (conflict 2000-01-06 00:00:00)", "id": spy.ANY},
                date_sync,
            ),
            ("fs.entry.synced", {"path": "/w/bar", "id": spy.ANY}, date_sync),
            (
                "fs.entry.synced",
                {"path": "/w/bar/spam (conflict 2000-01-06 00:00:00)", "id": spy.ANY},
                date_sync,
            ),
            (
                "fs.entry.minimal_synced",
                {"path": "/w/foo (conflict 2000-01-06 00:00:00).txt", "id": spy.ANY},
                date_sync,
            ),
            ("fs.entry.synced", {"path": "/w", "id": spy.ANY}, date_sync),
            (
                "fs.entry.synced",
                {"path": "/w/foo (conflict 2000-01-06 00:00:00).txt", "id": spy.ANY},
                date_sync,
            ),
        ]
    )

    # 5) Sync Alice again to take into account changes from the second fs's sync
    with alice_fs.event_bus.listen() as spy:
        with freeze_time("2000-01-08"):
            await alice_fs.sync("/")
            await alice_fs.sync("/")
    date_sync = Pendulum(2000, 1, 8)
    spy.assert_events_occured(
        [
            ("fs.entry.remote_changed", {"path": "/w/bar", "id": spy.ANY}, date_sync),
            ("fs.entry.remote_changed", {"path": "/w", "id": spy.ANY}, date_sync),
            ("fs.entry.remote_changed", {"path": "/", "id": spy.ANY}, date_sync),
        ]
    )

    # 6) Finally compare the resulting fs

    final_fs = await assert_same_fs(alice_fs, alice2_fs)
    assert final_fs["children"].keys() == {"w", "z", "z (conflict 2000-01-06 00:00:00)"}
    # Make sure z conflict hasn't changed workspace access
    current_z = alice_fs._local_folder_fs.get_access(FsPath("/z"))
    assert current_z == z_by_alice
    diverged_z = alice_fs._local_folder_fs.get_access(FsPath("/z (conflict 2000-01-06 00:00:00)"))
    assert diverged_z == z_by_alice2

    final_wkps = final_fs["children"]["w"]
    assert final_wkps["children"].keys() == {
        "bar",
        "foo (conflict 2000-01-06 00:00:00).txt",
        "foo.txt",
    }
    assert final_wkps["children"]["bar"]["children"].keys() == {
        "buzz (conflict 2000-01-06 00:00:00).txt",
        "buzz.txt",
        "from_alice",
        "from_alice2",
        "spam",
        "spam (conflict 2000-01-06 00:00:00)",
    }

    data = await alice_fs.file_read("/w/foo.txt")
    data2 = await alice2_fs.file_read("/w/foo.txt")
    assert data == data2 == b"alice's v2"

    data = await alice_fs.file_read("/w/foo (conflict 2000-01-06 00:00:00).txt")
    data2 = await alice2_fs.file_read("/w/foo (conflict 2000-01-06 00:00:00).txt")
    assert data == data2 == b"alice2's v2"


@pytest.mark.trio
async def test_create_already_existing_folder_vlob(running_backend, alice, alice_fs, alice2_fs):
    # First create data locally
    with freeze_time("2000-01-02"):
        # File and folder are handled basically the same
        w_id = await alice_fs.workspace_create("/w")

    vanilla_backend_vlob_create = alice_fs._syncer._backend_vlob_create

    async def mocked_backend_vlob_create(*args, **kwargs):
        await vanilla_backend_vlob_create(*args, **kwargs)
        raise BackendNotAvailable()

    alice_fs._syncer._backend_vlob_create = mocked_backend_vlob_create

    with pytest.raises(BackendNotAvailable):
        await alice_fs.sync("/w")

    alice_fs._syncer._backend_vlob_create = vanilla_backend_vlob_create
    await alice_fs.sync("/w")

    stat = await alice_fs.stat("/w")
    assert stat == {
        "type": "workspace",
        "id": w_id,
        "admin_right": True,
        "read_right": True,
        "write_right": True,
        "base_version": 1,
        "is_folder": True,
        "is_placeholder": False,
        "need_sync": False,
        "created": Pendulum(2000, 1, 2),
        "updated": Pendulum(2000, 1, 2),
        "creator": alice.user_id,
        "participants": [alice.user_id],
        "children": [],
    }

    await alice2_fs.sync("/")
    stat2 = await alice2_fs.stat("/w")
    assert stat == stat2


@pytest.mark.trio
async def test_create_already_existing_file_vlob(running_backend, alice_fs, alice2_fs):
    await create_shared_workspace("/w", alice_fs, alice2_fs)

    # First create data locally
    with freeze_time("2000-01-02"):
        obj_id = await alice_fs.file_create("/w/foo.txt")

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
        "id": obj_id,
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
async def test_create_already_existing_block(running_backend, alice_fs, alice2_fs):
    # First create&sync an empty file

    with freeze_time("2000-01-02"):
        await alice_fs.workspace_create("/w")
        obj_id = await alice_fs.file_create("/w/foo.txt")
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
        "id": obj_id,
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
