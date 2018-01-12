import pytest
from trio.testing import trio_test
from libfaketime import fake_time

from parsec.utils import to_jsonb64, from_jsonb64

from tests.common import with_core, with_populated_local_storage


@pytest.mark.skip(reason='regression...')
@trio_test
@with_core()
@with_populated_local_storage('alice')
async def test_flush_folder(core):
    # Also test the sync here
    async def _stat(path):
        await sock.send({'cmd': 'stat', 'path': path})
        rep = await sock.recv()
        return rep

    async with core.test_connect('alice@test') as sock:
        await sock.send({'cmd': 'flush', 'path': '/'})
        rep = await sock.recv()
        assert rep == {'status': 'ok'}
        rep = await _stat('/')
        assert not rep['need_flush']
        # Test nested folder as well
        await sock.send({'cmd': 'flush', 'path': '/dir'})
        rep = await sock.recv()
        assert rep == {'status': 'ok'}
        rep = await _stat('/dir')
        assert not rep['need_flush']
    # Flush on folder is done automatically each time it is modified, so
    # calling flush command explicitly does nothing !
    assert core.mocked_local_storage_cls.return_value.flush_user_manifest.call_count == 0
    assert core.mocked_local_storage_cls.return_value.flush_manifest.call_count == 0


@pytest.mark.skip(reason='regression...')
@trio_test
@with_core()
@with_populated_local_storage('alice')
async def test_flush_file(core):
    # Also test the sync here
    async def _stat(path):
        await sock.send({'cmd': 'stat', 'path': path})
        rep = await sock.recv()
        return rep

    async def _modify(path):
        await sock.send({'cmd': 'file_write', 'path': path, 'content': to_jsonb64(b'foo')})
        rep = await sock.recv()
        return rep

    async with core.test_connect('alice@test') as sock:
        # File hasn't been touch, so no need to flush it so far
        rep = await _stat('/dir/up_to_date.txt')
        assert not rep['need_flush']
        # Unlike for folder, modifying the file doesn't trigger automatic flush
        await _modify('/dir/up_to_date.txt')
        assert core.mocked_local_storage_cls.return_value.flush_user_manifest.call_count == 0
        assert core.mocked_local_storage_cls.return_value.flush_manifest.call_count == 0
        # But now the need_flush flag is set !
        rep = await _stat('/dir/up_to_date.txt')
        assert rep['need_flush']
        # And using flush is useful ;-)
        await sock.send({'cmd': 'flush', 'path': '/dir/up_to_date.txt'})
        rep = await sock.recv()
        assert rep == {'status': 'ok'}
        assert core.mocked_local_storage_cls.return_value.flush_user_manifest.call_count == 0
        assert core.mocked_local_storage_cls.return_value.flush_manifest.call_count == 1

    # TODO: do the same thing for modified.txt: the file is modified (it need
    # to be synchronized with the backend) but already flushed on local storage
    # when the test start
