import pytest
import trio

from tests.common import connect_signal_as_event


@pytest.mark.trio
@pytest.mark.parametrize("already_synced", [True, False])
async def test_share_workspace(
    already_synced, core, core2, alice_core_sock, bob_core2_sock, running_backend
):
    # Bob stays idle waiting for a sharing from alice
    core2_received_sharing = connect_signal_as_event(core2.signal_ns, "sharing.new")

    # First, create a folder and sync it on backend
    await core.fs.workspace_create("/foo")
    await core.fs.folder_create("/foo/spam")
    await core.fs.folder_create("/foo/spam/zob")
    await core.fs.file_create("/foo/spam/bar.txt")
    await core.fs.file_write("/foo/spam/bar.txt", b"Hello from Alice !")
    if already_synced:
        await core.fs.sync("/foo")

    # Now we can share this workspace with Bob
    await alice_core_sock.send({"cmd": "share", "path": "/foo", "recipient": "bob"})
    rep = await alice_core_sock.recv()
    assert rep == {"status": "ok"}

    # Bob should get a notification
    with trio.fail_after(seconds=1):
        await core2_received_sharing.wait()
        assert len(core2_received_sharing.cb.call_args_list) == 1

    # Now Bob can access the file just like Alice would do
    bob_foo_name = "foo (shared by alice)"
    bob_root_stat = await core2.fs.stat("/")
    assert bob_root_stat["children"] == [bob_foo_name]
    for path in ("", "/spam", "/spam/zob", "/spam/bar.txt"):
        alice_path = f"/foo{path}"
        bob_path = f"/{bob_foo_name}{path}"
        alice_file_stat = await core.fs.stat(alice_path)
        bob_file_stat = await core2.fs.stat(bob_path)
        assert bob_file_stat == alice_file_stat

    bob_file_data = await core2.fs.file_read(f"/{bob_foo_name}/spam/bar.txt")
    assert bob_file_data == b"Hello from Alice !"


@pytest.mark.trio
async def test_share_backend_offline(core, alice_core_sock, bob):
    await core.fs.workspace_create("/foo")

    await alice_core_sock.send({"cmd": "share", "path": "/foo", "recipient": bob.user_id})
    rep = await alice_core_sock.recv()
    assert rep == {"status": "backend_not_available", "reason": "Backend not available"}


@pytest.mark.trio
async def test_share_bad_entry(alice_core_sock, running_backend, bob):
    await alice_core_sock.send({"cmd": "share", "path": "/dummy", "recipient": bob.user_id})
    rep = await alice_core_sock.recv()
    assert rep == {
        "status": "invalid_path",
        "reason": "[Errno 2] No such file or directory: '/dummy'",
    }


@pytest.mark.trio
async def test_share_not_a_valid_path(alice_core_sock, running_backend, bob):
    await alice_core_sock.send({"cmd": "share", "path": "dummy", "recipient": bob.user_id})
    rep = await alice_core_sock.recv()
    assert rep == {"status": "bad_message", "errors": {"path": ["Path must be absolute"]}}


@pytest.mark.trio
async def test_share_bad_recipient(core, alice_core_sock, running_backend):
    await core.fs.workspace_create("/foo")

    await alice_core_sock.send({"cmd": "share", "path": "/foo", "recipient": "dummy"})
    rep = await alice_core_sock.recv()
    assert rep == {"status": "bad_recipient", "reason": "Cannot create message for `dummy`"}


@pytest.mark.trio
async def test_share_not_a_workspace(core, bob, alice_core_sock, running_backend):
    await core.fs.file_create("/foo.txt")
    await core.fs.folder_create("/spam")

    for path in ["/foo.txt", "/spam"]:
        await alice_core_sock.send({"cmd": "share", "path": path, "recipient": bob.user_id})
        rep = await alice_core_sock.recv()
        assert rep == {
            "status": "sharing_error",
            "reason": f"`{path}` is not a workspace, hence cannot be shared",
        }


@pytest.mark.trio
async def test_share_invalid_recipient(core, alice_core_sock, running_backend):
    await core.fs.workspace_create("/foo")

    await alice_core_sock.send({"cmd": "share", "path": "/foo.txt", "recipient": "alice"})
    rep = await alice_core_sock.recv()
    assert rep == {"status": "bad_recipient", "reason": "Cannot share to oneself."}


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
