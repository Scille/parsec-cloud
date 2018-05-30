from nacl.public import PublicKey, SealedBox
import pytest
import trio

from parsec.utils import ejson_dumps


@pytest.mark.trio
@pytest.mark.parametrize("already_synced", [True, False])
async def test_share_file(
    already_synced, core, core2, alice_core_sock, bob_core2_sock, running_backend
):
    assert core.fs.get_last_processed_message() == 0
    assert core2.fs.get_last_processed_message() == 0

    # Bob stays idle waiting for a sharing from alice
    await bob_core2_sock.send({"cmd": "event_subscribe", "event": "new_sharing"})
    rep = await bob_core2_sock.recv()
    assert rep == {"status": "ok"}
    await bob_core2_sock.send({"cmd": "event_listen"})

    # First, create a file and sync it on backend
    await core.fs.file_create("/foo.txt")
    await core.fs.file_write("/foo.txt", b"Hello from Alice !")
    if already_synced:
        await core.fs.sync("/foo.txt")

    # Now we can share this file with Bob
    await alice_core_sock.send({"cmd": "share", "path": "/foo.txt", "recipient": "bob"})
    rep = await alice_core_sock.recv()
    assert rep == {"status": "ok"}

    # Bob should get a notification
    with trio.move_on_after(seconds=1) as cancel_scope:
        rep = await bob_core2_sock.recv()
    assert not cancel_scope.cancelled_caught
    assert rep == {"status": "ok", "event": "new_sharing", "subject": "/shared-with-alice/foo.txt"}

    # Now Bob can access the file just like Alice would do
    alice_file_stat = await core.fs.stat("/foo.txt")
    bob_file_stat = await core2.fs.stat("/shared-with-alice/foo.txt")
    assert bob_file_stat == alice_file_stat

    bob_file_data = await core.fs.file_read("/foo.txt")
    assert bob_file_data == b"Hello from Alice !"

    assert core.fs.get_last_processed_message() == 0
    assert core2.fs.get_last_processed_message() == 1


@pytest.mark.trio
@pytest.mark.parametrize("already_synced", [True, False])
async def test_multiple_messages(
    already_synced, core, core2, alice_core_sock, bob_core2_sock, running_backend, bob
):
    def _build_ping_body(destination):
        ping_body = {"type": "ping", "ping": destination}
        broadcast_key = PublicKey(bob.user_privkey.public_key.encode())
        box = SealedBox(broadcast_key)
        sharing_msg_clear = ejson_dumps(ping_body).encode("utf8")
        sharing_msg_signed = core.auth_device.device_signkey.sign(sharing_msg_clear)
        return box.encrypt(sharing_msg_signed)

    assert core.fs.get_last_processed_message() == 0
    assert core2.fs.get_last_processed_message() == 0

    await bob_core2_sock.send({"cmd": "event_subscribe", "event": "ping"})
    rep = await bob_core2_sock.recv()
    assert rep == {"status": "ok"}

    # Two messages received at once
    await running_backend.backend.message.perform_message_new(
        sender_device_id="alice@test", recipient_user_id="bob", body=_build_ping_body("foo")
    )
    await running_backend.backend.message.perform_message_new(
        sender_device_id="alice@test", recipient_user_id="bob", body=_build_ping_body("bar")
    )

    await bob_core2_sock.send({"cmd": "event_listen"})
    with trio.move_on_after(seconds=1) as cancel_scope:
        rep = await bob_core2_sock.recv()
    assert not cancel_scope.cancelled_caught
    assert rep == {"event": "ping", "status": "ok", "subject": "foo"}

    await bob_core2_sock.send({"cmd": "event_listen"})
    with trio.move_on_after(seconds=1) as cancel_scope:
        rep = await bob_core2_sock.recv()
    assert not cancel_scope.cancelled_caught
    assert rep == {"event": "ping", "status": "ok", "subject": "bar"}

    assert core.fs.get_last_processed_message() == 0
    assert core2.fs.get_last_processed_message() == 2

    # Next message received
    await running_backend.backend.message.perform_message_new(
        sender_device_id="alice@test", recipient_user_id="bob", body=_build_ping_body("baz")
    )

    await bob_core2_sock.send({"cmd": "event_listen"})
    with trio.move_on_after(seconds=1) as cancel_scope:
        rep = await bob_core2_sock.recv()
    assert not cancel_scope.cancelled_caught
    assert rep == {"event": "ping", "status": "ok", "subject": "baz"}

    assert core.fs.get_last_processed_message() == 0
    assert core2.fs.get_last_processed_message() == 3


# @pytest.mark.trio
# @pytest.mark.parametrize('already_synced', [True, False])
# async def test_share_folder(already_synced, alice_core_sock, bob_core2_sock, backend):
#     # TODO
#     pass


@pytest.mark.trio
async def test_share_backend_offline(core, alice_core_sock, bob):
    await core.fs.file_create("/foo.txt")

    await alice_core_sock.send({"cmd": "share", "path": "/foo.txt", "recipient": bob.user_id})
    rep = await alice_core_sock.recv()
    assert rep == {"status": "backend_not_availabled", "reason": "Backend not available"}


@pytest.mark.trio
async def test_share_bad_entry(alice_core_sock, running_backend, bob):
    await alice_core_sock.send({"cmd": "share", "path": "/dummy.txt", "recipient": bob.user_id})
    rep = await alice_core_sock.recv()
    assert rep == {"status": "invalid_path", "reason": "Path `/dummy.txt` doesn't exists"}


@pytest.mark.trio
async def test_share_bad_recipient(core, alice_core_sock, running_backend):
    await core.fs.file_create("/foo.txt")

    await alice_core_sock.send({"cmd": "share", "path": "/foo.txt", "recipient": "dummy"})
    rep = await alice_core_sock.recv()
    assert rep == {"status": "not_found", "reason": "No user with id `dummy`."}


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
