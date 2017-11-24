import pytest
from trio.testing import trio_test
from freezegun import freeze_time

from parsec.utils import to_jsonb64, from_jsonb64

from tests.common import with_core, with_populated_local_storage


@trio_test
@with_core()
@with_populated_local_storage('alice')
async def test_sync_not_needed(core):
    # If folder hasn't got any modification locally...
    async with core.test_connect('alice@test') as sock:
        await sock.send({'cmd': 'synchronize', 'path': '/dir/up_to_date.txt'})
        rep = await sock.recv()
        assert rep == {'status': 'ok'}

        await sock.send({'cmd': 'stat', 'path': '/dir/up_to_date.txt'})
        rep = await sock.recv()
        assert not rep['need_flush']
        assert not rep['need_sync']

    # No flush, no sync occured !
    assert core.mocked_local_storage_cls.return_value.flush_user_manifest.call_count == 0
    assert core.mocked_local_storage_cls.return_value.flush_manifest.call_count == 0


@trio_test
@with_core()
@with_populated_local_storage('alice')
async def test_sync_file(core):
    async with core.test_connect('alice@test') as sock:
        await sock.send({'cmd': 'synchronize', 'path': '/dir/modified.txt'})
        rep = await sock.recv()
        assert rep == {'status': 'ok'}

        await sock.send({'cmd': 'stat', 'path': '/dir/modified.txt'})
        rep = await sock.recv()
        assert not rep['need_flush']
        assert not rep['need_sync']

    # This file was already flushed on local storage, but we re-flushed it
    # anyway once synchronized
    assert core.mocked_local_storage_cls.return_value.flush_user_manifest.call_count == 0
    assert core.mocked_local_storage_cls.return_value.flush_manifest.call_count == 1


@pytest.mark.xfail
@trio_test
@with_core()
@with_populated_local_storage('alice')
async def test_sync_placeholder_file(core):
    async with core.test_connect('alice@test') as sock:
        async def _stat(path):
            await sock.send({'cmd': 'stat', 'path': path})
            rep = await sock.recv()
            return rep

        await sock.send({'cmd': 'synchronize', 'path': '/dir/new.txt'})
        rep = await sock.recv()
        assert rep == {'status': 'ok'}

        rep = await _stat('/dir')
        assert not rep['need_flush']
        assert not rep['need_sync']

        rep = await _stat('/dir/new.txt')
        assert not rep['need_flush']
        assert not rep['need_sync']
        assert not rep['is_placeholder']

    # This file was already flushed on local storage, but we re-flushed it
    # anyway once synchronized
    # On top of that, the parent file should have been synchronized as well
    # given otherwise nobody (except this device) can access the file...
    assert core.mocked_local_storage_cls.return_value.flush_user_manifest.call_count == 0
    assert core.mocked_local_storage_cls.return_value.flush_manifest.call_count == 2
    # TODO: test cascade sync ?


@trio_test
@with_core()
@with_populated_local_storage('alice')
async def test_sync_folder(core):
    async with core.test_connect('alice@test') as sock:
        async def _stat(path):
            await sock.send({'cmd': 'stat', 'path': path})
            rep = await sock.recv()
            return rep

        await sock.send({'cmd': 'synchronize', 'path': '/dir'})
        rep = await sock.recv()
        assert rep == {'status': 'ok'}

        rep = await _stat('/dir')
        assert not rep['need_flush']
        assert not rep['need_sync']

        # new.txt is no longer a placeholder, but it still needs to be synchronized
        rep = await _stat('/dir/new.txt')
        assert not rep['need_flush']
        assert rep['need_sync']
        assert not rep['is_placeholder']

    # # Before synchronizing the folder, all it children placeholder should have
    # # been synchronized (here we only have `new.txt`)
    # assert core.mocked_local_storage_cls.return_value.flush_user_manifest.call_count == 0
    # assert core.mocked_local_storage_cls.return_value.flush_manifest.call_count == 1
    # assert core.mocked_local_storage_cls.return_value.move_manifest.call_count == 1

    # Test of truth: drop all data in the local storage and check if
    # synchronized data are still available
    core.mocked_local_storage_cls.test_storage.dirty_blocks.clear()
    core.mocked_local_storage_cls.test_storage.blocks.clear()
    core.mocked_local_storage_cls.test_storage.local_user_manifest = None
    core.mocked_local_storage_cls.test_storage.local_manifests.clear()

    async with core.test_connect('alice@test') as sock:
        await sock.send({'cmd': 'stat', 'path': '/dir'})
        rep = await sock.recv()
        assert rep == {
            'status': 'ok',
            'type': 'folder',
            'base_version': 3,
            'created': '2017-12-01T12:50:30+00:00',
            'updated': '2017-12-02T12:50:56+00:00',
            'is_placeholder': False,
            'need_flush': False,
            'need_sync': False,
            'children': [
                'modified.txt',
                'new.txt',
                'non_local.txt',
                'up_to_date.txt',
            ]
        }

        await sock.send({'cmd': 'stat', 'path': '/dir/new.txt'})
        rep = await sock.recv()
        assert rep == {
            'status': 'ok',
            'type': 'file',
            'base_version': 0,
            'created': '2017-12-02T12:50:30+00:00',
            'updated': '2017-12-02T12:51:00+00:00',
            'is_placeholder': False,
            'need_flush': False,
            'need_sync': True,
            'size': 24,
        }
