import pytest
import trio


@pytest.mark.trio
@pytest.mark.parametrize('already_synced', [True, False])
async def test_share_file(already_synced, core, core2, alice_core_sock, bob_core2_sock, running_backend):
    # Bob stays idle waiting for a sharing from alice
    await bob_core2_sock.send({'cmd': 'event_subscribe', 'event': 'new_sharing'})
    rep = await bob_core2_sock.recv()
    assert rep == {'status': 'ok'}
    await bob_core2_sock.send({'cmd': 'event_listen'})

    # First, create a file and sync it on backend
    alice_file = await core.fs.root.create_file('foo.txt')
    await alice_file.write(b'Hello from Alice !')
    if already_synced:
        await alice_file.sync()

    # Now we can share this file with Bob
    await alice_core_sock.send({'cmd': 'share', 'path': '/foo.txt', 'recipient': 'bob'})
    rep = await alice_core_sock.recv()
    assert rep == {'status': 'ok'}

    # Bob should get a notification
    with trio.move_on_after(seconds=1) as cancel_scope:
        rep = await bob_core2_sock.recv()
    assert not cancel_scope.cancelled_caught
    assert rep == {
        'status': 'ok',
        'event': 'new_sharing',
        'subject': '/shared-with-alice/foo.txt'
    }

    # Now Bob can access the file just like Alice would do
    bob_file = await core2.fs.fetch_path('/shared-with-alice/foo.txt')
    assert bob_file.created == alice_file.created
    assert bob_file.updated == alice_file.updated
    assert bob_file.base_version == alice_file.base_version
    bob_file_data = await bob_file.read()
    assert bob_file_data == b'Hello from Alice !'


# @pytest.mark.trio
# @pytest.mark.parametrize('already_synced', [True, False])
# async def test_share_folder(already_synced, alice_core_sock, bob_core2_sock, backend):
#     # TODO
#     pass


@pytest.mark.trio
async def test_share_backend_offline(core, alice_core_sock, bob):
    await core.fs.root.create_file('foo.txt')

    await alice_core_sock.send({
        'cmd': 'share',
        'path': '/foo.txt',
        'recipient': bob.user_id
    })
    rep = await alice_core_sock.recv()
    assert rep == {'status': 'backend_not_availabled'}


@pytest.mark.trio
async def test_share_bad_entry(alice_core_sock, running_backend, bob):
    await alice_core_sock.send({
        'cmd': 'share',
        'path': '/dummy.txt',
        'recipient': bob.user_id
    })
    rep = await alice_core_sock.recv()
    assert rep == {
        'status': 'invalid_path',
        'reason': "Path `/dummy.txt` doesn't exists"
    }


@pytest.mark.trio
async def test_share_bad_recipient(core, alice_core_sock, running_backend):
    await core.fs.root.create_file('foo.txt')

    await alice_core_sock.send({
        'cmd': 'share',
        'path': '/foo.txt',
        'recipient': 'dummy'
    })
    rep = await alice_core_sock.recv()
    assert rep == {
        'status': 'not_found',
        'reason': 'No user with id `dummy`.'
    }

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
