import pytest
from trio.testing import trio_test

from parsec.utils import to_jsonb64, from_jsonb64

from tests.common import freeze_time, with_core, with_populated_local_storage


@pytest.mark.skip(reason='regression...')
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
    # No modifications, no flush
    assert core.mocked_local_storage_cls.return_value.flush_manifest.call_count == 0
    assert core.mocked_local_storage_cls.return_value.flush_user_manifest.call_count == 0


@pytest.mark.skip(reason='regression...')
@trio_test
@with_core()
@with_populated_local_storage('alice')
async def test_read_unknow_file(core):
    async with core.test_connect('alice@test') as sock:
        await sock.send({'cmd': 'file_read', 'path': '/dummy.txt'})
        rep = await sock.recv()
        assert rep == {'status': 'invalid_path', 'reason': "Path `/dummy.txt` doesn't exists"}
        # Try to read a folder, because why not ?
        await sock.send({'cmd': 'file_read', 'path': '/dir'})
        rep = await sock.recv()
        assert rep == {'status': 'invalid_path', 'reason': "Path `/dir` is not a file"}
    # No modifications, no flush
    assert core.mocked_local_storage_cls.return_value.flush_manifest.call_count == 0
    assert core.mocked_local_storage_cls.return_value.flush_user_manifest.call_count == 0


@pytest.mark.skip(reason='regression...')
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
    # No modifications, no flush
    assert core.mocked_local_storage_cls.return_value.flush_manifest.call_count == 0
    assert core.mocked_local_storage_cls.return_value.flush_user_manifest.call_count == 0
