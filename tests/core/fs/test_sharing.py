import pytest

from parsec.core.backend_connection import BackendNotAvailable
from parsec.core.fs.sharing import SharingRecipientError, SharingNotAWorkspace


@pytest.mark.trio
async def test_share_workspace(running_backend, alice_fs, bob_fs):
    # First, create a populated workspace and sync it on backend
    await alice_fs.workspace_create("/foo")
    await alice_fs.folder_create("/foo/spam")
    await alice_fs.folder_create("/foo/spam/zob")
    await alice_fs.file_create("/foo/spam/bar.txt")
    await alice_fs.file_write("/foo/spam/bar.txt", b"Hello from Alice !")
    await alice_fs.sync("/foo")

    # Now we can share this workspace with Bob
    await alice_fs.share("/foo", recipient="bob")

    # Bob should get a notification
    bob_foo_name = "foo (shared by alice)"
    with bob_fs.event_bus.listen() as spy:
        await bob_fs.process_last_messages()
    spy.assert_event_occured("sharing.new", kwargs={"path": f"/{bob_foo_name}", "access": spy.ANY})

    # Now Bob can access the file just like Alice would do
    bob_root_stat = await bob_fs.stat("/")
    assert bob_root_stat["children"] == [bob_foo_name]

    for path in ("", "/spam", "/spam/zob", "/spam/bar.txt"):
        alice_path = f"/foo{path}"
        bob_path = f"/{bob_foo_name}{path}"
        alice_file_stat = await alice_fs.stat(alice_path)
        bob_file_stat = await bob_fs.stat(bob_path)
        assert bob_file_stat == alice_file_stat

        if alice_path == "/foo":
            # Also make sure Bob is marked among the workspace's participants
            assert alice_file_stat["participants"] == ["alice", "bob"]

    bob_file_data = await bob_fs.file_read(f"/{bob_foo_name}/spam/bar.txt")
    assert bob_file_data == b"Hello from Alice !"


@pytest.mark.trio
@pytest.mark.parametrize("already_synced", [True, False])
async def test_share_workspace_placeholder(already_synced, running_backend, alice_fs, bob_fs):
    # First, create the workspace
    await alice_fs.workspace_create("/foo")

    # Now we can share this workspace with Bob, this should trigger sync
    with alice_fs.event_bus.listen() as spy:
        await alice_fs.share("/foo", recipient="bob")
    spy.assert_event_occured("fs.entry.synced", kwargs={"path": f"/foo", "id": spy.ANY})

    # Bob should get a notification
    bob_foo_name = "foo (shared by alice)"
    with bob_fs.event_bus.listen() as spy:
        await bob_fs.process_last_messages()
    spy.assert_event_occured("sharing.new", kwargs={"path": f"/{bob_foo_name}", "access": spy.ANY})

    # Now Bob can access the file just like Alice would do
    bob_root_stat = await bob_fs.stat("/")
    assert bob_root_stat["children"] == [bob_foo_name]

    alice_file_stat = await alice_fs.stat("/foo")
    bob_file_stat = await bob_fs.stat(f"/{bob_foo_name}")
    assert bob_file_stat == alice_file_stat
    # Also make sure Bob is marked among the workspace's participants
    assert alice_file_stat["participants"] == ["alice", "bob"]


@pytest.mark.trio
async def test_share_backend_offline(alice_fs, bob):
    await alice_fs.workspace_create("/foo")

    with pytest.raises(BackendNotAvailable):
        await alice_fs.share("/foo", recipient=bob.user_id)


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
async def test_share_bad_recipient(running_backend, alice_fs):
    await alice_fs.workspace_create("/foo")

    with pytest.raises(SharingRecipientError) as exc:
        await alice_fs.share("/foo", recipient="dummy")
    assert exc.value.args == ("Cannot create message for `dummy`",)


@pytest.mark.trio
async def test_share_not_a_workspace(running_backend, alice_fs, bob):
    await alice_fs.file_create("/foo.txt")
    await alice_fs.folder_create("/spam")

    for path in ["/foo.txt", "/spam"]:
        with pytest.raises(SharingNotAWorkspace) as exc:
            await alice_fs.share(path, recipient="dummy")
        assert exc.value.args == (f"`{path}` is not a workspace, hence cannot be shared",)


@pytest.mark.trio
async def test_share_invalid_recipient(running_backend, alice_fs, alice):
    await alice_fs.workspace_create("/foo")

    with pytest.raises(SharingRecipientError) as exc:
        await alice_fs.share("/foo", recipient=alice.user_id)
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
