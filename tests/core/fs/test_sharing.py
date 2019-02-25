# Parsec Cloud (https://parsec.cloud) Copyright (c) AGPLv3 2019 Scille SAS

import pytest

from parsec.core.backend_connection import BackendNotAvailable
from parsec.core.fs.sharing import (
    SharingRecipientError,
    SharingBackendMessageError,
    SharingNotAWorkspace,
)


@pytest.mark.trio
async def test_share_workspace(running_backend, alice, bob, alice_fs, bob_fs):
    # First, create a populated workspace and sync it on backend
    await alice_fs.workspace_create("/w")
    await alice_fs.folder_create("/w/spam")
    await alice_fs.folder_create("/w/spam/zob")
    await alice_fs.file_create("/w/spam/bar.txt")
    await alice_fs.file_write("/w/spam/bar.txt", b"Hello from Alice !")
    await alice_fs.sync("/w")

    # Now we can share this workspace with Bob
    await alice_fs.share("/w", recipient=bob.user_id)

    # Bob should get a notification
    with bob_fs.event_bus.listen() as spy:
        await bob_fs.process_last_messages()
    spy.assert_event_occured("sharing.new", kwargs={"path": "/w", "access": spy.ANY})

    # Now Bob can access the file just like Alice would do
    bob_root_stat = await bob_fs.stat("/")
    assert bob_root_stat["children"] == ["w"]

    for path in ("", "/spam", "/spam/zob", "/spam/bar.txt"):
        path = f"/w{path}"
        alice_file_stat = await alice_fs.stat(path)
        bob_file_stat = await bob_fs.stat(path)
        assert bob_file_stat == alice_file_stat

        if path == "/w":
            # Also make sure Bob is marked among the workspace's participants
            assert alice_file_stat["participants"] == [alice.user_id, bob.user_id]

    bob_file_data = await bob_fs.file_read("/w/spam/bar.txt")
    assert bob_file_data == b"Hello from Alice !"


@pytest.mark.trio
async def test_share_workspace_multiple_times(running_backend, alice, bob, alice_fs, bob_fs):
    # Create a workspace with Alice
    await alice_fs.workspace_create("/foo")
    await alice_fs.folder_create("/foo/spam")
    await alice_fs.file_create("/foo/spam/bar.txt")
    await alice_fs.file_write("/foo/spam/bar.txt", b"Alice workspace")
    await alice_fs.sync("/foo")

    # Create a workspace with Bob
    await bob_fs.workspace_create("/foo")
    await bob_fs.folder_create("/foo/spam")
    await bob_fs.file_create("/foo/spam/bar.txt")
    await bob_fs.file_write("/foo/spam/bar.txt", b"Bob workspace")
    await bob_fs.sync("/foo")

    # Now we can share this workspace with Bob
    await alice_fs.share("/foo", recipient=bob.user_id)

    # Bob should get a notification
    bob_foo_name = "foo 2"
    with bob_fs.event_bus.listen() as spy:
        await bob_fs.process_last_messages()
    spy.assert_event_occured("sharing.new", kwargs={"path": f"/{bob_foo_name}", "access": spy.ANY})

    # Bob shares his workspace with Alice
    await bob_fs.share("/foo", recipient=alice.user_id)

    # Bob should get a notification
    alice_foo_name = "foo 2"
    with alice_fs.event_bus.listen() as spy:
        await alice_fs.process_last_messages()
    spy.assert_event_occured(
        "sharing.new", kwargs={"path": f"/{alice_foo_name}", "access": spy.ANY}
    )

    # Read the data in Bob workspace with Bob
    bob_file_data = await bob_fs.file_read("/foo/spam/bar.txt")
    assert bob_file_data == b"Bob workspace"

    # Read the data in Alice workspace with Bob
    alice_file_data = await bob_fs.file_read("/foo 2/spam/bar.txt")
    assert alice_file_data == b"Alice workspace"

    # Read the data in Alice workspace with Alice
    alice_file_data = await alice_fs.file_read("/foo/spam/bar.txt")
    assert alice_file_data == b"Alice workspace"

    # Read the data in Bob workspace with Alice
    bob_file_data = await alice_fs.file_read("/foo 2/spam/bar.txt")
    assert bob_file_data == b"Bob workspace"


@pytest.mark.trio
async def test_share_workspace_placeholder(running_backend, alice, bob, alice_fs, bob_fs):
    # First, create the workspace
    await alice_fs.workspace_create("/w")

    # Now we can share this workspace with Bob, this should trigger sync
    with alice_fs.event_bus.listen() as spy:
        await alice_fs.share("/w", recipient=bob.user_id)
    spy.assert_event_occured("fs.entry.synced", kwargs={"path": "/w", "id": spy.ANY})

    # Bob should get a notification
    with bob_fs.event_bus.listen() as spy:
        await bob_fs.process_last_messages()
    spy.assert_event_occured("sharing.new", kwargs={"path": "/w", "access": spy.ANY})

    # Now Bob can access the file just like Alice would do
    bob_root_stat = await bob_fs.stat("/")
    assert bob_root_stat["children"] == ["w"]

    alice_file_stat = await alice_fs.stat("/w")
    bob_file_stat = await bob_fs.stat("/w")
    assert bob_file_stat == alice_file_stat
    # Also make sure Bob is marked among the workspace's participants
    assert alice_file_stat["participants"] == [alice.user_id, bob.user_id]


@pytest.mark.trio
async def test_share_workspace_then_rename_it(running_backend, bob, alice_fs, bob_fs):
    # First, create the workspace
    await alice_fs.workspace_create("/w")

    # Now we can share this workspace with Bob, this should trigger sync
    with alice_fs.event_bus.listen() as spy:
        await alice_fs.share("/w", recipient=bob.user_id)
    spy.assert_event_occured("fs.entry.synced", kwargs={"path": "/w", "id": spy.ANY})

    # Bob should get a notification
    with bob_fs.event_bus.listen() as spy:
        await bob_fs.process_last_messages()
    spy.assert_event_occured("sharing.new", kwargs={"path": "/w", "access": spy.ANY})

    # Now Bob and alice both rename the workpsace for there own taste
    await bob_fs.workspace_rename("/w", "/from_alice")
    await alice_fs.workspace_rename("/w", "/to_bob")

    await bob_fs.sync("/")
    await alice_fs.sync("/")

    # This should have not changed the workspace in any way
    await bob_fs.touch("/from_alice/ping_bob.txt")
    await alice_fs.mkdir("/to_bob/ping_alice")

    await bob_fs.sync("/from_alice")
    await alice_fs.sync("/to_bob")
    await bob_fs.sync("/from_alice")

    alice_workspace_stat = await alice_fs.stat("/to_bob")
    bob_workspace_stat = await bob_fs.stat("/from_alice")
    assert alice_workspace_stat == bob_workspace_stat


@pytest.mark.trio
async def test_share_backend_offline(alice_fs, bob):
    await alice_fs.workspace_create("/w")

    with pytest.raises(BackendNotAvailable):
        await alice_fs.share("/w", recipient=bob.user_id)


@pytest.mark.trio
async def test_share_bad_entry(running_backend, alice_fs, bob):
    with pytest.raises(FileNotFoundError) as exc:
        await alice_fs.share("/dummy", recipient=bob.user_id)
    assert exc.value.args == (2, "No such file or directory")


@pytest.mark.trio
async def test_share_not_a_valid_path(running_backend, alice_fs, bob):
    with pytest.raises(ValueError) as exc:
        await alice_fs.share("dummy", recipient=bob.user_id)
    assert exc.value.args == ("Path must be absolute",)


@pytest.mark.trio
async def test_share_bad_recipient(running_backend, mallory, alice_fs):
    await alice_fs.workspace_create("/w")

    with pytest.raises(SharingBackendMessageError) as exc:
        await alice_fs.share("/w", recipient=mallory.user_id)
    assert exc.value.args == (
        "Error while trying to set beacon rights in backend: {'reason': 'Unknown user', 'status': 'error'}",
    )


@pytest.mark.trio
async def test_share_not_a_workspace(running_backend, bob, alice_fs):
    # First, create the workspace
    await alice_fs.workspace_create("/w")

    await alice_fs.file_create("/w/foo.txt")
    await alice_fs.folder_create("/w/spam")

    for path in ["/w/foo.txt", "/w/spam"]:
        with pytest.raises(SharingNotAWorkspace) as exc:
            await alice_fs.share(path, recipient=bob.user_id)
        assert exc.value.args == (f"`{path}` is not a workspace, hence cannot be shared",)


@pytest.mark.trio
async def test_share_invalid_recipient(running_backend, alice_fs, alice):
    await alice_fs.workspace_create("/w")

    with pytest.raises(SharingRecipientError) as exc:
        await alice_fs.share("/w", recipient=alice.user_id)
    assert exc.value.args == ("Cannot share to oneself.",)


# @pytest.mark.trio
# async def test_share_with_receiver_concurrency(alice_core_sock, running_backend):
#     # Bob is connected on multiple cores, which will fight to update the
#     # main manifest.
#     # TODO
#     pass


# @pytest.mark.trio
# async def test_share_with_sharing_name_already_taken(alice_core_sock, running_backend):
#     # TODO
#     pass
