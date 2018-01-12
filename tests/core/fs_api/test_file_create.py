import pytest
from trio.testing import trio_test

from parsec.utils import to_jsonb64, from_jsonb64

from tests.common import freeze_time, with_core, with_populated_local_storage


@pytest.mark.skip(reason='regression...')
@trio_test
@with_core()
async def test_create_file(core):
    async with core.test_connect('alice@test') as sock:
        with freeze_time('2017-12-10T12:00:00'):
            await sock.send({'cmd': 'file_create', 'path': '/new.txt'})
            rep = await sock.recv()
        assert rep == {'status': 'ok'}
        # Parent manifest and newly created file manifest shoul have been
        # both flushed
        assert core.mocked_local_storage_cls.return_value.flush_manifest.call_count == 1
        assert core.mocked_local_storage_cls.return_value.flush_user_manifest.call_count == 1
        await sock.send({'cmd': 'stat', 'path': '/new.txt'})
        rep = await sock.recv()
        assert rep == {
            'status': 'ok',
            'type': 'file',
            'base_version': 0,
            'created': '2017-12-10T12:00:00+00:00',
            'updated': '2017-12-10T12:00:00+00:00',
            'is_placeholder': True,
            'need_flush': False,
            'need_sync': True,
            'size': 0,
        }


@pytest.mark.skip(reason='regression...')
@trio_test
@with_core()
@with_populated_local_storage('alice')
async def test_create_bad_file(core):
    async with core.test_connect('alice@test') as sock:
        await sock.send({'cmd': 'file_create', 'path': '/dir/up_to_date.txt'})
        rep = await sock.recv()
        assert rep == {'status': 'invalid_path', 'reason': 'Path `/dir/up_to_date.txt` already exists'}
        await sock.send({'cmd': 'file_create', 'path': '/dir'})
        rep = await sock.recv()
        assert rep == {'status': 'invalid_path', 'reason': 'Path `/dir` already exists'}
        await sock.send({'cmd': 'file_create', 'path': '/dir/up_to_date.txt/foo'})
        rep = await sock.recv()
        assert rep == {'status': 'invalid_path', 'reason': 'Path `/dir/up_to_date.txt` is not a directory'}
    # No modifications, no flush
    assert core.mocked_local_storage_cls.return_value.flush_manifest.call_count == 0
    assert core.mocked_local_storage_cls.return_value.flush_user_manifest.call_count == 0
