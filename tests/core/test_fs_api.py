import pytest
from trio.testing import trio_test
from freezegun import freeze_time

from parsec.utils import to_jsonb64, from_jsonb64

from tests.common import with_core, with_populated_local_storage


@trio_test
@with_core()
@with_populated_local_storage('alice')
async def test_stat_folder(core):
    async with core.test_connect('alice@test') as sock:
        await sock.send({'cmd': 'stat', 'path': '/'})
        rep = await sock.recv()
        assert rep == {
            'status': 'ok',
            'type': 'folder',
            'created': '2017-12-02T12:30:23+00:00',
            'children': ['dir', 'empty_dir']
        }
        # Test nested folder as well
        await sock.send({'cmd': 'stat', 'path': '/dir'})
        rep = await sock.recv()
        assert rep == {
            'status': 'ok',
            'type': 'folder',
            'created': '2017-12-02T12:30:23+00:00',
            'children': ['modified.txt', 'new.txt', 'up_to_date.txt']
        }
    # No sync for a stat
    assert core.mocked_local_storage_cls.return_value.save_dirty_file_manifest.call_count == 0
    assert core.mocked_local_storage_cls.return_value.save_placeholder_file_manifest.call_count == 0


@trio_test
@with_core()
@with_populated_local_storage('alice')
async def test_stat_file(core):
    # Also test the sync here
    async def _stat(path):
        await sock.send({'cmd': 'stat', 'path': path})
        rep = await sock.recv()
        return rep

    async with core.test_connect('alice@test') as sock:
        # Blocks only
        rep = await _stat('/dir/up_to_date.txt')
        assert rep == {
            'status': 'ok',
            'type': 'file',
            'is_dirty': False,
            'is_placeholder': False,
            'version': 2,
            'created': '2017-12-02T12:30:30+00:00',
            'updated': '2017-12-02T12:30:45+00:00',
            'size': 27
        }
        # Blocks + dirty blocks
        rep = await _stat('/dir/modified.txt')
        assert rep == {
            'status': 'ok',
            'type': 'file',
            'is_dirty': True,
            'is_placeholder': False,
            'version': 2,
            'created': '2017-12-02T12:50:30+00:00',
            'updated': '2017-12-02T12:51:00+00:00',
            'size': 19
        }
        # Dirty blocks only
        rep = await _stat('/dir/new.txt')
        assert rep == {
            'status': 'ok',
            'type': 'file',
            'is_dirty': True,
            'is_placeholder': True,
            'version': 0,
            'created': '2017-12-02T12:50:30+00:00',
            'updated': '2017-12-02T12:51:00+00:00',
            'size': 24
        }
    # No sync for a stat
    assert core.mocked_local_storage_cls.return_value.save_dirty_file_manifest.call_count == 0
    assert core.mocked_local_storage_cls.return_value.save_placeholder_file_manifest.call_count == 0


    async with core.test_connect('alice@test') as sock:
        await sock.send({'cmd': 'stat', 'path': '/dir/up_to_date.txt'})
        rep = await sock.recv()


@trio_test
@with_core()
@with_populated_local_storage('alice')
async def test_create_folder(core):
    async with core.test_connect('alice@test') as sock:
        await sock.send({'cmd': 'folder_create', 'path': '/new_folder'})
        rep = await sock.recv()
        assert rep == {'status': 'ok'}
        # Make sure folder is visible
        await sock.send({'cmd': 'stat', 'path': '/'})
        rep = await sock.recv()
        assert rep == {
            'status': 'ok',
            'type': 'folder',
            'created': '2017-12-02T12:30:23+00:00',
            'children': ['dir', 'empty_dir', 'new_folder']
        }
    # Local user manifest should have been flushed to local storage
    assert core.mocked_local_storage_cls.return_value.save_local_user_manifest.call_count == 1
    # Test nested as well
    async with core.test_connect('alice@test') as sock:
        await sock.send({'cmd': 'folder_create', 'path': '/dir/new_folder'})
        rep = await sock.recv()
        assert rep == {'status': 'ok'}
        # Make sure folder is visible
        await sock.send({'cmd': 'stat', 'path': '/dir'})
        rep = await sock.recv()
        assert rep == {
            'status': 'ok',
            'type': 'folder',
            'created': '2017-12-02T12:30:23+00:00',
            'children': ['modified.txt', 'new.txt', 'new_folder', 'up_to_date.txt']
        }
    # Local user manifest should have been flushed another time to local storage
    assert core.mocked_local_storage_cls.return_value.save_local_user_manifest.call_count == 2


@trio_test
@with_core()
@with_populated_local_storage('alice')
async def test_create_duplicated_folder(core):
    async with core.test_connect('alice@test') as sock:
        await sock.send({'cmd': 'folder_create', 'path': '/dir'})
        rep = await sock.recv()
        assert rep == {'status': 'invalid_path', 'reason': 'Path `/dir` already exist'}
        # Try with existing file as well
        await sock.send({'cmd': 'folder_create', 'path': '/dir/up_to_date.txt'})
        rep = await sock.recv()
        assert rep == {'status': 'invalid_path', 'reason': 'Path `/dir/up_to_date.txt` already exist'}
    # No changes mean no flush to local storage
    assert core.mocked_local_storage_cls.return_value.save_local_user_manifest.call_count == 0


@trio_test
@with_core()
@with_populated_local_storage('alice')
async def test_move_folder(core):
    async with core.test_connect('alice@test') as sock:
        await sock.send({'cmd': 'move', 'src': '/dir', 'dst': '/renamed_dir'})
        rep = await sock.recv()
        assert rep == {'status': 'ok'}
        # Local user manifest should have been flushed to local storage
        assert core.mocked_local_storage_cls.return_value.save_local_user_manifest.call_count == 1
        # Make sure folder is visible
        await sock.send({'cmd': 'stat', 'path': '/'})
        rep = await sock.recv()
        assert rep == {
            'status': 'ok',
            'type': 'folder',
            'created': '2017-12-02T12:30:23+00:00',
            'children': ['empty_dir', 'renamed_dir'],
        }
        # Make sure folder still contains the same stuff
        await sock.send({'cmd': 'stat', 'path': '/renamed_dir'})
        rep = await sock.recv()
        assert rep == {
            'status': 'ok',
            'type': 'folder',
            'created': '2017-12-02T12:30:23+00:00',
            'children': ['modified.txt', 'new.txt', 'up_to_date.txt']
        }
        # And old folder nam is no longer available
        await sock.send({'cmd': 'stat', 'path': '/dir'})
        rep = await sock.recv()
        assert rep == {'status': 'invalid_path', 'reason': "Path `/dir` doesn't exist"}


@trio_test
@with_core()
@with_populated_local_storage('alice')
async def test_move_folder_bad_dst(core):
    for src in ['/empty_dir', '/dir/up_to_date.txt']:
        async with core.test_connect('alice@test') as sock:
            # Destination already exists
            await sock.send({'cmd': 'move', 'src': src, 'dst': '/dir'})
            rep = await sock.recv()
            assert rep == {'status': 'invalid_path', 'reason': "Path `/dir` already exist"}
            # Destination already exists
            await sock.send({'cmd': 'move', 'src': src, 'dst': '/dir/modified.txt'})
            rep = await sock.recv()
            assert rep == {'status': 'invalid_path', 'reason': "Path `/dir/modified.txt` already exist"}
            # Cannot replace root !
            await sock.send({'cmd': 'move', 'src': src, 'dst': '/'})
            rep = await sock.recv()
            assert rep == {'status': 'invalid_path', 'reason': "Root `/` folder always exists"}
            # Destination contains non-existent folders
            await sock.send({'cmd': 'move', 'src': src, 'dst': '/dir/unknown/new_foo'})
            rep = await sock.recv()
            assert rep == {'status': 'invalid_path', 'reason': "Path `/dir/unknown` doesn't exist"}
    # No changes mean no flush to local storage
    assert core.mocked_local_storage_cls.return_value.save_local_user_manifest.call_count == 0


@trio_test
@with_core()
@with_populated_local_storage('alice')
async def test_move_file(core):
    async with core.test_connect('alice@test') as sock:
        await sock.send({'cmd': 'move', 'src': '/dir/up_to_date.txt', 'dst': '/renamed.txt'})
        rep = await sock.recv()
        assert rep == {'status': 'ok'}
        # Local user manifest should have been flushed to local storage
        assert core.mocked_local_storage_cls.return_value.save_local_user_manifest.call_count == 1
        # Check the destination exists
        await sock.send({'cmd': 'stat', 'path': '/'})
        rep = await sock.recv()
        assert rep == {
            'status': 'ok',
            'type': 'folder',
            'created': '2017-12-02T12:30:23+00:00',
            'children': ['dir', 'empty_dir', 'renamed.txt']
        }
        # Check the source no longer exits
        await sock.send({'cmd': 'stat', 'path': '/dir'})
        rep = await sock.recv()
        assert rep == {
            'status': 'ok',
            'type': 'folder',
            'created': '2017-12-02T12:30:23+00:00',
            'children': ['modified.txt', 'new.txt']
        }
        # Make sure we can no longer stat source name...
        await sock.send({'cmd': 'stat', 'path': '/dir/up_to_date.txt'})
        rep = await sock.recv()
        assert rep == {
            'status': 'invalid_path',
            'reason': "Path `/dir/up_to_date.txt` doesn't exist"
        }
        # ...and we can stat destination name
        await sock.send({'cmd': 'stat', 'path': '/renamed.txt'})
        rep = await sock.recv()
        assert rep == {
            'status': 'ok',
            'type': 'file',
            'is_dirty': False,
            'is_placeholder': False,
            'version': 2,
            'created': '2017-12-02T12:30:30+00:00',
            'updated': '2017-12-02T12:30:45+00:00',
            'size': 27
        }


@trio_test
@with_core()
@with_populated_local_storage('alice')
async def test_move_unknow_file(core):
    async with core.test_connect('alice@test') as sock:
        await sock.send({'cmd': 'move', 'src': '/dummy.txt', 'dst': '/new_dummy.txt'})
        rep = await sock.recv()
        assert rep == {'status': 'invalid_path', 'reason': "Path `/dummy.txt` doesn't exist"}
    # No changes mean no flush to local storage
    assert core.mocked_local_storage_cls.return_value.save_local_user_manifest.call_count == 0


@trio_test
@with_core()
@with_populated_local_storage('alice')
async def test_delete_folder(core):
    async with core.test_connect('alice@test') as sock:
        await sock.send({'cmd': 'delete', 'path': '/empty_dir'})
        rep = await sock.recv()
        assert rep == {'status': 'ok'}
        # Local user manifest should have been flushed to local storage
        assert core.mocked_local_storage_cls.return_value.save_local_user_manifest.call_count == 1
        # Make sure the folder disappeared
        await sock.send({'cmd': 'stat', 'path': '/'})
        rep = await sock.recv()
        assert rep == {
            'status': 'ok',
            'type': 'folder',
            'created': '2017-12-02T12:30:23+00:00',
            'children': ['dir'],
        }
        await sock.send({'cmd': 'stat', 'path': '/empty_dir'})
        rep = await sock.recv()
        assert rep == {'status': 'invalid_path', 'reason': "Path `/empty_dir` doesn't exist"}


@trio_test
@with_core()
@with_populated_local_storage('alice')
async def test_delete_non_empty_folder(core):
    async with core.test_connect('alice@test') as sock:
        await sock.send({'cmd': 'delete', 'path': '/dir'})
        rep = await sock.recv()
        assert rep == {'status': 'ok'}
        # Local user manifest should have been flushed to local storage
        assert core.mocked_local_storage_cls.return_value.save_local_user_manifest.call_count == 1
        # Make sure the folder disappeared
        await sock.send({'cmd': 'stat', 'path': '/'})
        rep = await sock.recv()
        assert rep == {
            'status': 'ok',
            'type': 'folder',
            'created': '2017-12-02T12:30:23+00:00',
            'children': ['empty_dir']
        }
        await sock.send({'cmd': 'stat', 'path': '/dir'})
        rep = await sock.recv()
        assert rep == {'status': 'invalid_path', 'reason': "Path `/dir` doesn't exist"}
        # Children should have disappeared as well
        await sock.send({'cmd': 'stat', 'path': '/dir/modified.txt'})
        rep = await sock.recv()
        assert rep == {'status': 'invalid_path', 'reason': "Path `/dir` doesn't exist"}


@trio_test
@with_core()
@with_populated_local_storage('alice')
async def test_delete_file(core):
    async with core.test_connect('alice@test') as sock:
        await sock.send({'cmd': 'delete', 'path': '/dir/up_to_date.txt'})
        rep = await sock.recv()
        assert rep == {'status': 'ok'}
        # Local user manifest should have been flushed to local storage
        assert core.mocked_local_storage_cls.return_value.save_local_user_manifest.call_count == 1
        # Make sure the folder disappeared
        await sock.send({'cmd': 'stat', 'path': '/dir'})
        rep = await sock.recv()
        assert rep == {
            'status': 'ok',
            'type': 'folder',
            'created': '2017-12-02T12:30:23+00:00',
            'children': ['modified.txt', 'new.txt']
        }
        await sock.send({'cmd': 'stat', 'path': '/dir/up_to_date.txt'})
        rep = await sock.recv()
        assert rep == {
            'status': 'invalid_path',
            'reason': "Path `/dir/up_to_date.txt` doesn't exist"
        }


@trio_test
@with_core()
async def test_delete_unknow_file(core):
    async with core.test_connect('alice@test') as sock:
        await sock.send({'cmd': 'delete', 'path': '/dummy.txt'})
        rep = await sock.recv()
        assert rep == {'status': 'invalid_path', 'reason': "Path `/dummy.txt` doesn't exist"}
    # No changes mean no flush to local storage
    assert core.mocked_local_storage_cls.return_value.save_local_user_manifest.call_count == 0


@trio_test
@with_core()
@with_populated_local_storage('alice')
async def test_read(core):
    async with core.test_connect('alice@test') as sock:
        # Blocks only
        await sock.send({'cmd': 'file_read', 'path': '/dir/up_to_date.txt'})
        rep = await sock.recv()
        assert rep == {'status': 'ok', 'content': to_jsonb64(b'Hello from up_to_date.txt !')}
        # Blocks + dirty blocks
        await sock.send({'cmd': 'file_read', 'path': '/dir/modified.txt'})
        rep = await sock.recv()
        assert rep == {'status': 'ok', 'content': to_jsonb64(b'This is SPARTAAAA !')}
        # Dirty blocks only
        await sock.send({'cmd': 'file_read', 'path': '/dir/new.txt'})
        rep = await sock.recv()
        assert rep == {'status': 'ok', 'content': to_jsonb64(b'Welcome to the new file.')}
    # No changes mean no flush to local storage
    assert core.mocked_local_storage_cls.return_value.save_local_user_manifest.call_count == 0


@trio_test
@with_core()
@with_populated_local_storage('alice')
async def test_read_unknow_file(core):
    async with core.test_connect('alice@test') as sock:
        await sock.send({'cmd': 'file_read', 'path': '/dummy.txt'})
        rep = await sock.recv()
        assert rep == {'status': 'invalid_path', 'reason': "Path `/dummy.txt` doesn't exist"}
        # Try to read a folder, because why not ?
        await sock.send({'cmd': 'file_read', 'path': '/dir'})
        rep = await sock.recv()
        assert rep == {'status': 'invalid_path', 'reason': "Path `/dir` is not a file"}
    # No changes mean no flush to local storage
    assert core.mocked_local_storage_cls.return_value.save_local_user_manifest.call_count == 0


@trio_test
@with_core()
@with_populated_local_storage('alice')
async def test_read_with_offset(core):
    async with core.test_connect('alice@test') as sock:
        # Blocks only
        await sock.send({'cmd': 'file_read', 'path': '/dir/up_to_date.txt', 'offset': 6, 'size': 7})
        rep = await sock.recv()
        assert rep == {'status': 'ok', 'content': to_jsonb64(b'from up')}
        # Blocks + dirty blocks
        await sock.send({'cmd': 'file_read', 'path': '/dir/modified.txt', 'offset': 3, 'size': 6})
        rep = await sock.recv()
        assert rep == {'status': 'ok', 'content': to_jsonb64(b's is S')}
        # Dirty blocks only
        await sock.send({'cmd': 'file_read', 'path': '/dir/new.txt', 'offset': 9, 'size': 5})
        rep = await sock.recv()
        assert rep == {'status': 'ok', 'content': to_jsonb64(b'o the')}
    # No changes mean no flush to local storage
    assert core.mocked_local_storage_cls.return_value.save_local_user_manifest.call_count == 0


@trio_test
@with_core()
@with_populated_local_storage('alice')
async def test_write_then_sync(core):

    async def _write(path, offset, content):
        await sock.send({'cmd': 'file_write', 'path': path, 'offset': offset, 'content': to_jsonb64(content)})
        rep = await sock.recv()
        assert rep == {'status': 'ok'}

    async with core.test_connect('alice@test') as sock:
        # Blocks only
        await _write('/dir/up_to_date.txt', 6, b'*^__^*')
        # Blocks + dirty blocks
        await _write('/dir/modified.txt', 3, b'*^__^*')
        # Dirty blocks only
        await _write('/dir/new.txt', 9, b'*^__^*')
    # Write doesn't change the local user manifest
    assert core.mocked_local_storage_cls.return_value.save_local_user_manifest.call_count == 0
    # Also write doesn't sync automatically
    assert core.mocked_local_storage_cls.return_value.save_dirty_file_manifest.call_count == 0
    assert core.mocked_local_storage_cls.return_value.save_placeholder_file_manifest.call_count == 0

    # Make sure the data are readable

    async def _read(path, expected):
        await sock.send({'cmd': 'file_read', 'path': path})
        rep = await sock.recv()
        assert rep['status'] == 'ok'
        assert from_jsonb64(rep['content']) == expected

    async with core.test_connect('alice@test') as sock:
        # Blocks only
        await _read('/dir/up_to_date.txt', b'Hello *^__^*p_to_date.txt !')
        # Blocks + dirty blocks
        await _read('/dir/modified.txt', b'Thi*^__^*PARTAAAA !')
        # Dirty blocks only
        await _read('/dir/new.txt', b'Welcome t*^__^*new file.')

    # Also test the sync here
    async def _sync(path):
        await sock.send({'cmd': 'file_sync', 'path': path})
        rep = await sock.recv()
        assert rep == {'status': 'ok'}

    async with core.test_connect('alice@test') as sock:
        # Blocks only
        await _sync('/dir/up_to_date.txt')
        # Sync should have been called
        assert core.mocked_local_storage_cls.return_value.save_dirty_file_manifest.call_count == 1
        assert core.mocked_local_storage_cls.return_value.save_placeholder_file_manifest.call_count == 0
        # Blocks + dirty blocks
        await _sync('/dir/modified.txt')
        # Sync should have been called
        assert core.mocked_local_storage_cls.return_value.save_dirty_file_manifest.call_count == 2
        assert core.mocked_local_storage_cls.return_value.save_placeholder_file_manifest.call_count == 0
        # Dirty blocks only
        await _sync('/dir/new.txt')
        # Sync should have been called
        assert core.mocked_local_storage_cls.return_value.save_dirty_file_manifest.call_count == 2
        assert core.mocked_local_storage_cls.return_value.save_placeholder_file_manifest.call_count == 1
    # Write doesn't change the local user manifest
    assert core.mocked_local_storage_cls.return_value.save_local_user_manifest.call_count == 0

    # Finally check again that data are still readable
    async with core.test_connect('alice@test') as sock:
        # Blocks only
        await _read('/dir/up_to_date.txt', b'Hello *^__^*p_to_date.txt !')
        # Blocks + dirty blocks
        await _read('/dir/modified.txt', b'Thi*^__^*PARTAAAA !')
        # Dirty blocks only
        await _read('/dir/new.txt', b'Welcome t*^__^*new file.')


@trio_test
@with_core()
@with_populated_local_storage('alice')
async def test_write_bad_file(core):
    async with core.test_connect('alice@test') as sock:
        await sock.send({'cmd': 'file_write', 'path': '/dummy.txt', 'content': to_jsonb64(b'foo')})
        rep = await sock.recv()
        assert rep == {'status': 'invalid_path', 'reason': "Path `/dummy.txt` doesn't exist"}
        await sock.send({'cmd': 'file_write', 'path': '/dir', 'content': to_jsonb64(b'foo')})
        rep = await sock.recv()
        assert rep == {'status': 'invalid_path', 'reason': "Path `/dir` is not a file"}


@trio_test
@with_core()
@with_populated_local_storage('alice')
async def test_truncate_then_sync(core):
    async def _truncate(path, length):
        await sock.send({'cmd': 'file_truncate', 'path': path, 'length': length})
        rep = await sock.recv()
        assert rep == {'status': 'ok'}

    async with core.test_connect('alice@test') as sock:
        # Blocks only
        await _truncate('/dir/up_to_date.txt', 15)
        # Blocks + dirty blocks
        await _truncate('/dir/modified.txt', 15)
        # Dirty blocks only
        await _truncate('/dir/new.txt', 15)
    # Write doesn't change the local user manifest
    assert core.mocked_local_storage_cls.return_value.save_local_user_manifest.call_count == 0
    # Also write doesn't sync automatically
    assert core.mocked_local_storage_cls.return_value.save_dirty_file_manifest.call_count == 0
    assert core.mocked_local_storage_cls.return_value.save_placeholder_file_manifest.call_count == 0

    # Make sure the data are readable

    async def _read(path, expected):
        await sock.send({'cmd': 'file_read', 'path': path})
        rep = await sock.recv()
        assert rep['status'] == 'ok'
        assert from_jsonb64(rep['content']) == expected

    async with core.test_connect('alice@test') as sock:
        # Blocks only
        await _read('/dir/up_to_date.txt', b'Hello from up_t')
        # Blocks + dirty blocks
        await _read('/dir/modified.txt', b'This is SPARTAA')
        # Dirty blocks only
        await _read('/dir/new.txt', b'Welcome to the ')

    # Also test the sync here
    async def _sync(path):
        await sock.send({'cmd': 'file_sync', 'path': path})
        rep = await sock.recv()
        assert rep == {'status': 'ok'}

    async with core.test_connect('alice@test') as sock:
        # Blocks only
        await _sync('/dir/up_to_date.txt')
        # Sync should have been called
        assert core.mocked_local_storage_cls.return_value.save_dirty_file_manifest.call_count == 1
        assert core.mocked_local_storage_cls.return_value.save_placeholder_file_manifest.call_count == 0
        # Blocks + dirty blocks
        await _sync('/dir/modified.txt')
        # Sync should have been called
        assert core.mocked_local_storage_cls.return_value.save_dirty_file_manifest.call_count == 2
        assert core.mocked_local_storage_cls.return_value.save_placeholder_file_manifest.call_count == 0
        # Dirty blocks only
        await _sync('/dir/new.txt')
        # Sync should have been called
        assert core.mocked_local_storage_cls.return_value.save_dirty_file_manifest.call_count == 2
        assert core.mocked_local_storage_cls.return_value.save_placeholder_file_manifest.call_count == 1
    # Write doesn't change the local user manifest
    assert core.mocked_local_storage_cls.return_value.save_local_user_manifest.call_count == 0

    # Finally check again that data are still readable
    async with core.test_connect('alice@test') as sock:
        # Blocks only
        await _read('/dir/up_to_date.txt', b'Hello from up_t')
        # Blocks + dirty blocks
        await _read('/dir/modified.txt', b'This is SPARTAA')
        # Dirty blocks only
        await _read('/dir/new.txt', b'Welcome to the ')


@trio_test
@with_core()
@with_populated_local_storage('alice')
async def test_truncate_bad_file(core):
    async with core.test_connect('alice@test') as sock:
        await sock.send({'cmd': 'file_truncate', 'path': '/dummy.txt', 'length': 4})
        rep = await sock.recv()
        assert rep == {'status': 'invalid_path', 'reason': "Path `/dummy.txt` doesn't exist"}
        await sock.send({'cmd': 'file_truncate', 'path': '/dir', 'length': 4})
        rep = await sock.recv()
        assert rep == {'status': 'invalid_path', 'reason': "Path `/dir` is not a file"}


@trio_test
@with_core()
async def test_create_file(core):
    async with core.test_connect('alice@test') as sock:
        with freeze_time('2017-12-10T12:00:00'):
            await sock.send({'cmd': 'file_create', 'path': '/new.txt'})
            rep = await sock.recv()
        assert rep == {'status': 'ok'}
        await sock.send({'cmd': 'stat', 'path': '/new.txt'})
        rep = await sock.recv()
        assert rep == {
            'status': 'ok',
            'type': 'file',
            'is_dirty': True,
            'is_placeholder': True,
            'version': 0,
            'created': '2017-12-10T12:00:00+00:00',
            'updated': '2017-12-10T12:00:00+00:00',
            'size': 0
        }
    # Placeholder file and local user manifest should have been both flushed
    # to local storage
    assert core.mocked_local_storage_cls.return_value.save_placeholder_file_manifest.call_count == 1
    assert core.mocked_local_storage_cls.return_value.save_local_user_manifest.call_count == 1


@trio_test
@with_core()
@with_populated_local_storage('alice')
async def test_create_bad_file(core):
    async with core.test_connect('alice@test') as sock:
        await sock.send({'cmd': 'file_create', 'path': '/dir/up_to_date.txt'})
        rep = await sock.recv()
        assert rep == {'status': 'invalid_path', 'reason': 'Path `/dir/up_to_date.txt` already exist'}
        await sock.send({'cmd': 'file_create', 'path': '/dir'})
        rep = await sock.recv()
        assert rep == {'status': 'invalid_path', 'reason': 'Path `/dir` already exist'}
