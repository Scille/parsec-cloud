import pytest
from trio.testing import trio_test

from parsec.utils import to_jsonb64, from_jsonb64

from tests.common import freeze_time, with_core, with_populated_local_storage


@pytest.mark.skip(reason='regression...')
@trio_test
@with_core()
@with_populated_local_storage('alice')
async def test_move_folder(core):
    async with core.test_connect('alice@test') as sock:
        with freeze_time('2017-12-10T12:00:00'):
            await sock.send({'cmd': 'move', 'src': '/dir', 'dst': '/renamed_dir'})
            rep = await sock.recv()
        assert rep == {'status': 'ok'}
        # Only the parent (i.e. user manifest in this case) has need to be flushed
        assert core.mocked_local_storage_cls.return_value.flush_manifest.call_count == 0
        assert core.mocked_local_storage_cls.return_value.flush_user_manifest.call_count == 1
        # Make sure folder is visible
        await sock.send({'cmd': 'stat', 'path': '/'})
        rep = await sock.recv()
        assert rep == {
            'status': 'ok',
            'type': 'folder',
            'base_version': 3,
            'created': '2017-12-01T12:50:30+00:00',
            'updated': '2017-12-10T12:00:00+00:00',
            'is_placeholder': False,
            'need_flush': False,
            'need_sync': True,
            'children': ['empty_dir', 'renamed_dir']
        }
        # Make sure folder still contains the same stuff
        await sock.send({'cmd': 'stat', 'path': '/renamed_dir'})
        rep = await sock.recv()
        assert rep == {
            'status': 'ok',
            'type': 'folder',
            'base_version': 1,
            'created': '2017-12-01T12:50:30+00:00',
            'updated': '2017-12-02T12:50:56+00:00',
            'is_placeholder': False,
            'need_flush': False,
            'need_sync': True,
            'children': ['modified.txt', 'new.txt', 'up_to_date.txt']
        }
        # And old folder nam is no longer available
        await sock.send({'cmd': 'stat', 'path': '/dir'})
        rep = await sock.recv()
        assert rep == {'status': 'invalid_path', 'reason': "Path `/dir` doesn't exists"}


@pytest.mark.skip(reason='regression...')
@trio_test
@with_core()
@with_populated_local_storage('alice')
async def test_move_folder_bad_dst(core):
    for src in ['/empty_dir', '/dir/up_to_date.txt']:
        async with core.test_connect('alice@test') as sock:
            # Destination already exists
            await sock.send({'cmd': 'move', 'src': src, 'dst': '/dir'})
            rep = await sock.recv()
            assert rep == {'status': 'invalid_path', 'reason': "Path `/dir` already exists"}
            # Destination already exists
            await sock.send({'cmd': 'move', 'src': src, 'dst': '/dir/modified.txt'})
            rep = await sock.recv()
            assert rep == {'status': 'invalid_path', 'reason': "Path `/dir/modified.txt` already exists"}
            # Cannot replace root !
            await sock.send({'cmd': 'move', 'src': src, 'dst': '/'})
            rep = await sock.recv()
            assert rep == {'status': 'invalid_path', 'reason': "Path `/` already exists"}
            # Destination contains non-existent folders
            await sock.send({'cmd': 'move', 'src': src, 'dst': '/dir/unknown/new_foo'})
            rep = await sock.recv()
            assert rep == {'status': 'invalid_path', 'reason': "Path `/dir/unknown` doesn't exists"}
            # Destination parent is exists, but is not a folder
            await sock.send({'cmd': 'move', 'src': src, 'dst': '/dir/up_to_date.txt/foo'})
            rep = await sock.recv()
            assert rep == {'status': 'invalid_path', 'reason': 'Path `/dir/up_to_date.txt` is not a directory'}
    # No changes means no flush to local storage
    assert core.mocked_local_storage_cls.return_value.flush_manifest.call_count == 0
    assert core.mocked_local_storage_cls.return_value.flush_user_manifest.call_count == 0


@pytest.mark.skip(reason='regression...')
@trio_test
@with_core()
@with_populated_local_storage('alice')
async def test_move_file(core):
    async with core.test_connect('alice@test') as sock:
        with freeze_time('2017-12-10T12:00:00'):
            await sock.send({'cmd': 'move', 'src': '/dir/up_to_date.txt', 'dst': '/renamed.txt'})
            rep = await sock.recv()
        assert rep == {'status': 'ok'}
        # Source and destination parent local folder manifests should have been flushed,
        # on the other hand file manifest stays untouched.
        assert core.mocked_local_storage_cls.return_value.flush_manifest.call_count == 1
        assert core.mocked_local_storage_cls.return_value.flush_user_manifest.call_count == 1
        # Check the destination exists
        await sock.send({'cmd': 'stat', 'path': '/'})
        rep = await sock.recv()
        assert rep == {
            'status': 'ok',
            'type': 'folder',
            'base_version': 3,
            'updated': '2017-12-10T12:00:00+00:00',
            'created': '2017-12-01T12:50:30+00:00',
            'is_placeholder': False,
            'need_flush': False,
            'need_sync': True,
            'children': ['dir', 'empty_dir', 'renamed.txt']
        }
        # Check the source no longer exits
        await sock.send({'cmd': 'stat', 'path': '/dir'})
        rep = await sock.recv()
        assert rep == {
            'status': 'ok',
            'type': 'folder',
            'base_version': 1,
            'updated': '2017-12-10T12:00:00+00:00',
            'created': '2017-12-01T12:50:30+00:00',
            'is_placeholder': False,
            'need_flush': False,
            'need_sync': True,
            'children': ['modified.txt', 'new.txt']
        }
        # Make sure we can no longer stat source name...
        await sock.send({'cmd': 'stat', 'path': '/dir/up_to_date.txt'})
        rep = await sock.recv()
        assert rep == {
            'status': 'invalid_path',
            'reason': "Path `/dir/up_to_date.txt` doesn't exists"
        }
        # ...and we can stat destination name
        await sock.send({'cmd': 'stat', 'path': '/renamed.txt'})
        rep = await sock.recv()
        assert rep == {
            'status': 'ok',
            'type': 'file',
            'base_version': 2,
            'created': '2017-12-02T12:30:30+00:00',
            'updated': '2017-12-02T12:30:45+00:00',
            'is_placeholder': False,
            'need_flush': False,
            'need_sync': False,
            'size': 27,
        }


@pytest.mark.skip(reason='regression...')
@trio_test
@with_core()
@with_populated_local_storage('alice')
async def test_move_unknow_file(core):
    async with core.test_connect('alice@test') as sock:
        await sock.send({'cmd': 'move', 'src': '/dummy.txt', 'dst': '/new_dummy.txt'})
        rep = await sock.recv()
        assert rep == {'status': 'invalid_path', 'reason': "Path `/dummy.txt` doesn't exists"}
        await sock.send({'cmd': 'move', 'src': '/dir/up_to_date.txt', 'dst': '/dir/up_to_date.txt/foo'})
        rep = await sock.recv()
        assert rep == {'status': 'invalid_path', 'reason': 'Path `/dir/up_to_date.txt` is not a directory'}
    # No changes mean no flush to local storage
    assert core.mocked_local_storage_cls.return_value.flush_manifest.call_count == 0
    assert core.mocked_local_storage_cls.return_value.flush_user_manifest.call_count == 0
