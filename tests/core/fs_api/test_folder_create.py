import pytest
from trio.testing import trio_test

from parsec.utils import to_jsonb64, from_jsonb64

from tests.common import freeze_time, with_core, with_populated_local_storage


@pytest.mark.skip(reason='regression...')
@trio_test
@with_core()
@with_populated_local_storage('alice')
async def test_create_folder(core):
    async with core.test_connect('alice@test') as sock:
        with freeze_time('2017-12-10T12:00:00'):
            await sock.send({'cmd': 'folder_create', 'path': '/new_folder'})
            rep = await sock.recv()
        assert rep == {'status': 'ok'}
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
            'children': ['dir', 'empty_dir', 'new_folder']
        }
    # New folder manifest should have been flush along with it parent (i.e.
    # the user manifest in this case)
    assert core.mocked_local_storage_cls.return_value.flush_user_manifest.call_count == 1
    assert core.mocked_local_storage_cls.return_value.flush_manifest.call_count == 1


@pytest.mark.skip(reason='regression...')
@trio_test
@with_core()
@with_populated_local_storage('alice')
async def test_nested_create_folder(core):
    async with core.test_connect('alice@test') as sock:
        with freeze_time('2017-12-10T12:00:00'):
            await sock.send({'cmd': 'folder_create', 'path': '/dir/new_folder'})
            rep = await sock.recv()
        assert rep == {'status': 'ok'}
        # Make sure folder is visible
        await sock.send({'cmd': 'stat', 'path': '/dir'})
        rep = await sock.recv()
        assert rep == {
            'status': 'ok',
            'type': 'folder',
            'base_version': 1,
            'created': '2017-12-01T12:50:30+00:00',
            'updated': '2017-12-10T12:00:00+00:00',
            'is_placeholder': False,
            'need_flush': False,
            'need_sync': True,
            'children': ['modified.txt', 'new.txt', 'new_folder', 'up_to_date.txt']
        }
    # New folder manifest should have been flush along with it parent
    assert core.mocked_local_storage_cls.return_value.flush_manifest.call_count == 2
    assert core.mocked_local_storage_cls.return_value.flush_user_manifest.call_count == 0


@pytest.mark.skip(reason='regression...')
@trio_test
@with_core()
@with_populated_local_storage('alice')
async def test_create_duplicated_folder(core):
    async with core.test_connect('alice@test') as sock:
        await sock.send({'cmd': 'folder_create', 'path': '/dir'})
        rep = await sock.recv()
        assert rep == {'status': 'invalid_path', 'reason': 'Path `/dir` already exists'}
        # Try with existing file as well
        await sock.send({'cmd': 'folder_create', 'path': '/dir/up_to_date.txt'})
        rep = await sock.recv()
        assert rep == {'status': 'invalid_path', 'reason': 'Path `/dir/up_to_date.txt` already exists'}
    # No changes mean no flush to local storage
    assert core.mocked_local_storage_cls.return_value.save_local_user_manifest.call_count == 0
