# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest
from unittest.mock import ANY
from uuid import uuid4
from pendulum import Pendulum

from parsec.core.types import WorkspaceEntry, WorkspaceRole, ManifestAccess, LocalUserManifest
from parsec.core.fs import (
    FSError,
    FSWorkspaceNotFoundError,
    FSBackendOfflineError,
    FSSharingNotAllowedError,
)

from tests.common import freeze_time


@pytest.mark.trio
async def test_share_unknown(running_backend, alice_user_fs, bob):
    wid = uuid4()
    with pytest.raises(FSWorkspaceNotFoundError):
        await alice_user_fs.workspace_share(wid, bob.user_id, WorkspaceRole.MANAGER)


@pytest.mark.trio
async def test_share_to_oneself(running_backend, alice_user_fs, alice):
    with freeze_time("2000-01-02"):
        wid = await alice_user_fs.workspace_create("w1")

    with pytest.raises(FSError) as exc:
        await alice_user_fs.workspace_share(wid, alice.user_id, WorkspaceRole.MANAGER)
    assert str(exc.value) == "Cannot share to oneself"


@pytest.mark.trio
async def test_share_bad_recipient(running_backend, alice_user_fs, alice, mallory):
    with freeze_time("2000-01-02"):
        wid = await alice_user_fs.workspace_create("w1")

    with pytest.raises(FSError) as exc:
        await alice_user_fs.workspace_share(wid, mallory.user_id, WorkspaceRole.MANAGER)
    assert str(exc.value) == "Cannot retreive recipient: User `mallory` doesn't exist in backend"


@pytest.mark.trio
async def test_share_offline(alice_user_fs, bob):
    with freeze_time("2000-01-02"):
        wid = await alice_user_fs.workspace_create("w1")

    with pytest.raises(FSBackendOfflineError):
        await alice_user_fs.workspace_share(wid, bob.user_id, WorkspaceRole.MANAGER)


@pytest.mark.trio
@pytest.mark.parametrize("presynced", (True, False))
async def test_share_ok(running_backend, alice_user_fs, bob_user_fs, alice, bob, presynced):
    with freeze_time("2000-01-02"):
        wid = await alice_user_fs.workspace_create("w1")

    if presynced:
        await alice_user_fs.sync()

    await alice_user_fs.workspace_share(wid, bob.user_id, WorkspaceRole.MANAGER)

    with bob_user_fs.event_bus.listen() as spy:
        with freeze_time("2000-01-03"):
            await bob_user_fs.process_last_messages()
    spy.assert_event_occured(
        "sharing.granted",
        kwargs={
            "new_entry": WorkspaceEntry(
                name="w1 (shared by alice)",
                access=ManifestAccess(wid, spy.ANY),
                granted_on=Pendulum(2000, 1, 3),
                role=WorkspaceRole.MANAGER,
            )
        },
    )

    aum = alice_user_fs.get_user_manifest()
    bum = bob_user_fs.get_user_manifest()
    assert len(aum.workspaces) == 1
    assert len(bum.workspaces) == 1
    awe = aum.get_workspace_entry(wid)
    bwe = bum.get_workspace_entry(wid)

    assert bwe.name == "w1 (shared by alice)"
    assert bwe.access == awe.access
    assert bwe.role == WorkspaceRole.MANAGER

    aw = alice_user_fs.get_workspace(wid)
    bw = bob_user_fs.get_workspace(wid)
    aw_stat = await aw.path_info("/")
    bw_stat = await bw.path_info("/")
    # TODO: currently workspace minimal sync in userfs cannot
    # update need_sync field
    aw_stat.pop("need_sync")
    bw_stat.pop("need_sync")
    assert aw_stat == bw_stat


@pytest.mark.trio
async def test_share_workspace_then_rename_it(
    running_backend, alice_user_fs, bob_user_fs, alice, bob
):
    # Share a workspace between Alice and Bob
    with freeze_time("2000-01-02"):
        wid = await alice_user_fs.workspace_create("w")
    await alice_user_fs.workspace_share(wid, bob.user_id, WorkspaceRole.MANAGER)
    with freeze_time("2000-01-03"):
        await bob_user_fs.process_last_messages()

    # Now Bob and alice both rename the workpsace for there own taste
    await bob_user_fs.workspace_rename(wid, "from_alice")
    await alice_user_fs.workspace_rename(wid, "to_bob")

    await bob_user_fs.sync()
    await alice_user_fs.sync()

    # This should have not changed the workspace in any way
    bw = bob_user_fs.get_workspace(wid)
    aw = alice_user_fs.get_workspace(wid)
    await bw.touch("/ping_bob.txt")
    await aw.mkdir("/ping_alice")

    await bw.sync("/")
    await aw.sync("/")
    await bw.sync("/")

    aw_stat = await aw.path_info("/")
    bw_stat = await bw.path_info("/")
    assert aw_stat == bw_stat
    assert aw_stat["id"] == wid


@pytest.mark.trio
async def test_unshare_ok(running_backend, alice_user_fs, bob_user_fs, alice, bob):
    # Share a workspace...
    with freeze_time("2000-01-02"):
        wid = await alice_user_fs.workspace_create("w1")
    await alice_user_fs.workspace_share(wid, bob.user_id, WorkspaceRole.OWNER)
    await bob_user_fs.process_last_messages()

    # ...and unshare it
    await bob_user_fs.workspace_share(wid, alice.user_id, None)
    with alice_user_fs.event_bus.listen() as spy:
        with freeze_time("2000-01-03"):
            await alice_user_fs.process_last_messages()
    spy.assert_event_occured(
        "sharing.revoked",
        kwargs={
            "new_entry": WorkspaceEntry(
                name="w1",
                access=ManifestAccess(wid, spy.ANY),
                granted_on=Pendulum(2000, 1, 3),
                role=None,
            ),
            "previous_entry": WorkspaceEntry(
                name="w1",
                access=ManifestAccess(wid, spy.ANY),
                granted_on=Pendulum(2000, 1, 2),
                role=WorkspaceRole.OWNER,
            ),
        },
    )

    aum = alice_user_fs.get_user_manifest()
    aw = aum.workspaces[0]
    assert not aw.role

    # TODO: check workspace access is no longer possible


@pytest.mark.trio
async def test_unshare_not_shared(running_backend, alice_user_fs, bob_user_fs, alice, bob):
    with freeze_time("2000-01-02"):
        wid = await alice_user_fs.workspace_create("w1")
    await alice_user_fs.workspace_share(wid, bob.user_id, None)
    with alice_user_fs.event_bus.listen() as spy:
        await bob_user_fs.process_last_messages()
    assert not spy.events

    # Workspace unsharing should have been ignored
    bum = bob_user_fs.get_user_manifest()
    assert not bum.workspaces


@pytest.mark.trio
async def test_share_to_another_after_beeing_unshared(
    running_backend, alice_user_fs, bob_user_fs, alice, bob
):
    # Share a workspace...
    with freeze_time("2000-01-02"):
        wid = await alice_user_fs.workspace_create("w1")
    await alice_user_fs.workspace_share(wid, bob.user_id, WorkspaceRole.MANAGER)
    await bob_user_fs.process_last_messages()

    # ...and unshare it
    await alice_user_fs.workspace_share(wid, bob.user_id, None)
    await bob_user_fs.process_last_messages()

    # Shouldn't be able to share the workspace anymore
    with pytest.raises(FSSharingNotAllowedError):
        await bob_user_fs.workspace_share(wid, alice.user_id, None)


@pytest.mark.trio
async def test_reshare_workspace(running_backend, alice_user_fs, bob_user_fs, alice, bob):
    # Share a workspace...
    with freeze_time("2000-01-02"):
        wid = await alice_user_fs.workspace_create("w1")
    await alice_user_fs.workspace_share(wid, bob.user_id, WorkspaceRole.MANAGER)
    await bob_user_fs.process_last_messages()

    # ...and unshare it...
    await alice_user_fs.workspace_share(wid, bob.user_id, None)
    await bob_user_fs.process_last_messages()

    # ...and re-share it !
    await alice_user_fs.workspace_share(wid, bob.user_id, WorkspaceRole.MANAGER)
    await bob_user_fs.process_last_messages()

    # Check access
    aum = alice_user_fs.get_user_manifest()
    bum = bob_user_fs.get_user_manifest()
    assert len(aum.workspaces) == 1
    assert len(bum.workspaces) == 1
    aw = aum.workspaces[0]
    bw = bum.workspaces[0]

    assert bw.name == "w1 (shared by alice)"
    assert bw.access == aw.access
    assert bw.role == WorkspaceRole.MANAGER


@pytest.mark.trio
async def test_share_with_different_role(running_backend, alice_user_fs, bob_user_fs, alice, bob):
    with freeze_time("2000-01-02"):
        wid = await alice_user_fs.workspace_create("w1")
    aum = alice_user_fs.get_user_manifest()
    aw = aum.workspaces[0]

    for role in WorkspaceRole:
        # (re)share with rights
        await alice_user_fs.workspace_share(wid, bob.user_id, role)
        await bob_user_fs.process_last_messages()

        # Check access
        bum = bob_user_fs.get_user_manifest()
        assert len(bum.workspaces) == 1
        bw = bum.workspaces[0]

        assert bw.name == "w1 (shared by alice)"
        assert bw.access == aw.access
        assert bw.role == role


@pytest.mark.trio
async def test_share_no_manager_right(running_backend, alice_user_fs, alice, bob):
    with freeze_time("2000-01-02"):
        wid = await alice_user_fs.workspace_create("w1")
    await alice_user_fs.sync()

    # Drop manager right (and give to Bob the ownership)
    await running_backend.backend.vlob.update_group_roles(
        alice.organization_id, alice.user_id, wid, bob.user_id, role=WorkspaceRole.OWNER
    )
    await running_backend.backend.vlob.update_group_roles(
        alice.organization_id, bob.user_id, wid, alice.user_id, role=WorkspaceRole.CONTRIBUTOR
    )

    with pytest.raises(FSSharingNotAllowedError) as exc:
        await alice_user_fs.workspace_share(wid, bob.user_id, WorkspaceRole.MANAGER)
    assert exc.value.args == ("Must be Owner or Manager on the workspace is mandatory to share it",)


@pytest.mark.trio
async def test_share_with_sharing_name_already_taken(
    running_backend, alice_user_fs, bob_user_fs, alice, bob
):
    # Bob and Alice both has a workspace with similar name
    with freeze_time("2000-01-01"):
        awid = await alice_user_fs.workspace_create("w")
        bwid = await bob_user_fs.workspace_create("w")
        # bw2id = await bob_user_fs.workspace_create("w (shared by alice)")
        await bob_user_fs.workspace_create("w (shared by alice)")

    # Sharing them shouldn't be a trouble
    await bob_user_fs.sync()
    await alice_user_fs.workspace_share(awid, bob.user_id, WorkspaceRole.MANAGER)

    # Bob should get a notification
    with bob_user_fs.event_bus.listen() as spy:
        with freeze_time("2000-01-02"):
            await bob_user_fs.process_last_messages()
    spy.assert_event_occured(
        "sharing.granted",
        kwargs={
            "new_entry": WorkspaceEntry(
                name="w (shared by alice)",
                access=ManifestAccess(awid, spy.ANY),
                granted_on=Pendulum(2000, 1, 2),
                role=WorkspaceRole.MANAGER,
            )
        },
    )

    assert len(bob_user_fs.get_user_manifest().workspaces) == 3

    b_aw_stat = await bob_user_fs.get_workspace(awid).path_info("/")
    a_aw_stat = await alice_user_fs.get_workspace(awid).path_info("/")
    b_aw_stat.pop("need_sync")
    a_aw_stat.pop("need_sync")
    assert b_aw_stat == a_aw_stat

    b_bw_stat = await bob_user_fs.get_workspace(bwid).path_info("/")
    assert b_bw_stat["id"] == bwid
    # TODO: currently workspaces with same name shadow each other
    # should be solve once legacy FS class is dropped
    # b_bw2_stat = await bob_user_fs.get_workspace(bw2id).stat("/")
    # assert b_bw2_stat["id"] == bw2id


@pytest.mark.trio
@pytest.mark.parametrize("first_to_sync", ("alice", "alice2"))
async def test_share_workspace_then_conflict_on_rights(
    running_backend, alice_user_fs, alice2_user_fs, bob_user_fs, alice, alice2, bob, first_to_sync
):
    # Bob shares a workspace with Alice...
    wid = await bob_user_fs.workspace_create("w")
    await bob_user_fs.workspace_share(wid, alice.user_id, WorkspaceRole.MANAGER)

    # ...but only Alice's first device get the information
    with freeze_time("2000-01-02"):
        await alice_user_fs.process_last_messages()

    # Now Bob change the sharing rights...
    await bob_user_fs.workspace_share(wid, alice.user_id, WorkspaceRole.CONTRIBUTOR)

    # ...this time it's Alice's second device which get the info
    with freeze_time("2000-01-03"):
        await alice2_user_fs.process_last_messages()

    if first_to_sync == "alice2":
        first = alice_user_fs
        second = alice2_user_fs
        synced_version = 3
    else:
        first = alice2_user_fs
        second = alice_user_fs
        synced_version = 2

    # Finally Alice devices try to reconciliate
    with freeze_time("2000-01-04"):
        await first.sync()
    with freeze_time("2000-01-05"):
        await second.sync()
    # Resync first device to get changes from the 2nd
    with freeze_time("2000-01-06"):
        await first.sync()

    am = alice_user_fs.get_user_manifest()
    a2m = alice2_user_fs.get_user_manifest()
    expected = LocalUserManifest(
        author=alice.device_id,
        created=Pendulum(2000, 1, 1),
        updated=Pendulum(2000, 1, 3),
        base_version=synced_version,
        need_sync=False,
        is_placeholder=False,
        last_processed_message=2,
        workspaces=(
            WorkspaceEntry(
                name="w (shared by bob)",
                access=ManifestAccess(wid, ANY),
                granted_on=Pendulum(2000, 1, 3),
                role=WorkspaceRole.CONTRIBUTOR,
            ),
        ),
    )
    assert am == expected
    assert a2m == expected.evolve(author=alice2.device_id)

    a_w = alice_user_fs.get_workspace(wid)
    a2_w = alice2_user_fs.get_workspace(wid)

    a_w_stat = await a_w.path_info("/")
    a2_w_stat = await a2_w.path_info("/")

    a_w_entry = a_w.get_workspace_entry()
    a2_w_entry = a2_w.get_workspace_entry()

    assert a_w_stat == {
        "type": "folder",
        "is_placeholder": False,
        "id": wid,
        "created": ANY,
        "updated": ANY,
        "base_version": 1,
        "need_sync": False,
        "children": [],
    }
    assert a_w_stat == a2_w_stat

    assert a_w_entry == WorkspaceEntry(
        name=f"w (shared by {bob.user_id})",
        access=ManifestAccess(id=wid, key=ANY),
        granted_on=Pendulum(2000, 1, 3),
        role=WorkspaceRole.CONTRIBUTOR,
    )
    assert a2_w_entry == a_w_entry
