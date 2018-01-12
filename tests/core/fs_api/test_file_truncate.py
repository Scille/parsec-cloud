import pytest
from trio.testing import trio_test
from libfaketime import fake_time

from parsec.utils import to_jsonb64, from_jsonb64

from tests.common import with_core, with_populated_local_storage


@pytest.mark.skip(reason='regression...')
@trio_test
@with_core()
@with_populated_local_storage('alice')
async def test_truncate_then_flush(core):
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
    # Also write doesn't flush anything automatically
    assert core.mocked_local_storage_cls.return_value.flush_user_manifest.call_count == 0
    assert core.mocked_local_storage_cls.return_value.flush_manifest.call_count == 0

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
    async def _flush(path):
        await sock.send({'cmd': 'flush', 'path': path})
        rep = await sock.recv()
        assert rep == {'status': 'ok'}

    async with core.test_connect('alice@test') as sock:
        # Blocks only
        await _flush('/dir/up_to_date.txt')
        # Flush should have been called
        assert core.mocked_local_storage_cls.return_value.flush_manifest.call_count == 1
        # Blocks + dirty blocks
        await _flush('/dir/modified.txt')
        # Flush should have been called
        assert core.mocked_local_storage_cls.return_value.flush_manifest.call_count == 2
        # Dirty blocks only
        await _flush('/dir/new.txt')
        # Flush should have been called
        assert core.mocked_local_storage_cls.return_value.flush_manifest.call_count == 3
    # Write doesn't change the local user manifest
    assert core.mocked_local_storage_cls.return_value.flush_user_manifest.call_count == 0

    # Finally check again that data are still readable
    async with core.test_connect('alice@test') as sock:
        # Blocks only
        await _read('/dir/up_to_date.txt', b'Hello from up_t')
        # Blocks + dirty blocks
        await _read('/dir/modified.txt', b'This is SPARTAA')
        # Dirty blocks only
        await _read('/dir/new.txt', b'Welcome to the ')



@pytest.mark.skip(reason='regression...')
@trio_test
@with_core()
@with_populated_local_storage('alice')
async def test_truncate_bad_file(core):
    async with core.test_connect('alice@test') as sock:
        await sock.send({'cmd': 'file_truncate', 'path': '/dummy.txt', 'length': 4})
        rep = await sock.recv()
        assert rep == {'status': 'invalid_path', 'reason': "Path `/dummy.txt` doesn't exists"}
        await sock.send({'cmd': 'file_truncate', 'path': '/dir', 'length': 4})
        rep = await sock.recv()
        assert rep == {'status': 'invalid_path', 'reason': "Path `/dir` is not a file"}
    # No modifications, no flush
    assert core.mocked_local_storage_cls.return_value.flush_manifest.call_count == 0
    assert core.mocked_local_storage_cls.return_value.flush_user_manifest.call_count == 0
