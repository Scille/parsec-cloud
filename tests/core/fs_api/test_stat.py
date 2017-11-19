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
            'base_version': 3,
            'created': '2017-12-01T12:50:30+00:00',
            'updated': '2017-12-03T12:50:30+00:00',
            'is_placeholder': False,
            'need_flush': False,
            'need_sync': False,
            'children': ['dir', 'empty_dir']
        }
        # Test nested folder as well
        await sock.send({'cmd': 'stat', 'path': '/dir'})
        rep = await sock.recv()
        assert rep == {
            'status': 'ok',
            'base_version': 1,
            'type': 'folder',
            'created': '2017-12-01T12:50:30+00:00',
            'updated': '2017-12-02T12:50:56+00:00',
            'is_placeholder': False,
            'need_flush': False,
            'need_sync': True,
            'children': ['modified.txt', 'new.txt', 'up_to_date.txt']
         }
    # No flush for a stat
    assert core.mocked_local_storage_cls.return_value.flush_user_manifest.call_count == 0
    assert core.mocked_local_storage_cls.return_value.flush_manifest.call_count == 0


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
            'base_version': 2,
            'created': '2017-12-02T12:30:30+00:00',
            'updated': '2017-12-02T12:30:45+00:00',
            'is_placeholder': False,
            'need_flush': False,
            'need_sync': False,
            'size': 27
        }
        # Blocks + dirty blocks
        rep = await _stat('/dir/modified.txt')
        assert rep == {
            'status': 'ok',
            'type': 'file',
            'base_version': 2,
            'created': '2017-12-02T12:50:30+00:00',
            'updated': '2017-12-02T12:51:00+00:00',
            'is_placeholder': False,
            'need_flush': False,
            'need_sync': True,
            'size': 19,
        }
        # Dirty blocks only
        rep = await _stat('/dir/new.txt')
        assert rep == {
            'status': 'ok',
            'type': 'file',
            'base_version': 0,
            'created': '2017-12-02T12:50:30+00:00',
            'updated': '2017-12-02T12:51:00+00:00',
            'is_placeholder': True,
            'need_flush': False,
            'need_sync': True,
            'size': 24,
        }
    # No sync for a stat
    assert core.mocked_local_storage_cls.return_value.flush_user_manifest.call_count == 0
    assert core.mocked_local_storage_cls.return_value.flush_manifest.call_count == 0
