import pytest
from trio.testing import trio_test
from freezegun import freeze_time

from parsec.utils import to_jsonb64, from_jsonb64

from tests.common import with_core, with_populated_local_storage


@trio_test
@with_core()
@with_populated_local_storage('alice')
async def test_delete_folder(core):
    async with core.test_connect('alice@test') as sock:
        with freeze_time('2017-12-10T12:00:00'):
            await sock.send({'cmd': 'delete', 'path': '/empty_dir'})
            rep = await sock.recv()
        assert rep == {'status': 'ok'}
        # Local user manifest should have been flushed to local storage,
        # deleted folder manifest is unknowing of what happened
        assert core.mocked_local_storage_cls.return_value.flush_manifest.call_count == 0
        assert core.mocked_local_storage_cls.return_value.flush_user_manifest.call_count == 1
        # Make sure the folder disappeared
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
            'children': ['dir']
        }
        await sock.send({'cmd': 'stat', 'path': '/empty_dir'})
        rep = await sock.recv()
        assert rep == {'status': 'invalid_path', 'reason': "Path `/empty_dir` doesn't exists"}


@trio_test
@with_core()
@with_populated_local_storage('alice')
async def test_delete_non_empty_folder(core):
    async with core.test_connect('alice@test') as sock:
        with freeze_time('2017-12-10T12:00:00'):
            await sock.send({'cmd': 'delete', 'path': '/dir'})
            rep = await sock.recv()
        assert rep == {'status': 'ok'}
        # Local user manifest should have been flushed to local storage,
        # deleted folder manifest is unknowing of what happened
        assert core.mocked_local_storage_cls.return_value.flush_manifest.call_count == 0
        assert core.mocked_local_storage_cls.return_value.flush_user_manifest.call_count == 1
        # Make sure the folder disappeared
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
            'children': ['empty_dir']
        }
        await sock.send({'cmd': 'stat', 'path': '/dir'})
        rep = await sock.recv()
        assert rep == {'status': 'invalid_path', 'reason': "Path `/dir` doesn't exists"}
        # Children should have disappeared as well
        await sock.send({'cmd': 'stat', 'path': '/dir/modified.txt'})
        rep = await sock.recv()
        assert rep == {'status': 'invalid_path', 'reason': "Path `/dir` doesn't exists"}


@trio_test
@with_core()
@with_populated_local_storage('alice')
async def test_delete_file(core):
    async with core.test_connect('alice@test') as sock:
        with freeze_time('2017-12-10T12:00:00'):
            await sock.send({'cmd': 'delete', 'path': '/dir/up_to_date.txt'})
            rep = await sock.recv()
        assert rep == {'status': 'ok'}
        # Local user manifest should have been flushed to local storage,
        # delete file manifest is unknowing of what happened
        assert core.mocked_local_storage_cls.return_value.flush_manifest.call_count == 1
        assert core.mocked_local_storage_cls.return_value.flush_user_manifest.call_count == 0
        # Make sure the folder disappeared
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
        await sock.send({'cmd': 'stat', 'path': '/dir/up_to_date.txt'})
        rep = await sock.recv()
        assert rep == {
            'status': 'invalid_path',
            'reason': "Path `/dir/up_to_date.txt` doesn't exists"
        }


@trio_test
@with_core()
@with_populated_local_storage('alice')
async def test_delete_unknow_file(core):
    async with core.test_connect('alice@test') as sock:
        await sock.send({'cmd': 'delete', 'path': '/dummy.txt'})
        rep = await sock.recv()
        assert rep == {'status': 'invalid_path', 'reason': "Path `/dummy.txt` doesn't exists"}
        await sock.send({'cmd': 'delete', 'path': '/dir/up_to_date.txt/foo'})
        rep = await sock.recv()
        assert rep == {'status': 'invalid_path', 'reason': 'Path `/dir/up_to_date.txt` is not a directory'}
    # No modifications, no flush
    assert core.mocked_local_storage_cls.return_value.flush_manifest.call_count == 0
    assert core.mocked_local_storage_cls.return_value.flush_user_manifest.call_count == 0
