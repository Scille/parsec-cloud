import pytest
from trio.testing import trio_test

from parsec.utils import to_jsonb64, from_jsonb64

from tests.common import freeze_time, with_core, with_populated_local_storage


@pytest.mark.skip(reason='regression...')
@trio_test
@with_core()
@with_populated_local_storage('alice')
async def test_write_then_flush(core):
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
        await _read('/dir/up_to_date.txt', b'Hello *^__^*p_to_date.txt !')
        # Blocks + dirty blocks
        await _read('/dir/modified.txt', b'Thi*^__^*PARTAAAA !')
        # Dirty blocks only
        await _read('/dir/new.txt', b'Welcome t*^__^*new file.')

    # Also test the flush here
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
        await _read('/dir/up_to_date.txt', b'Hello *^__^*p_to_date.txt !')
        # Blocks + dirty blocks
        await _read('/dir/modified.txt', b'Thi*^__^*PARTAAAA !')
        # Dirty blocks only
        await _read('/dir/new.txt', b'Welcome t*^__^*new file.')

    # TODO: should be able to spaw&test another core only sharing the local_storage


@pytest.mark.skip(reason='regression...')
@trio_test
@with_core()
@with_populated_local_storage('alice')
async def test_write_bad_file(core):
    async with core.test_connect('alice@test') as sock:
        await sock.send({'cmd': 'file_write', 'path': '/dummy.txt', 'content': to_jsonb64(b'foo')})
        rep = await sock.recv()
        assert rep == {'status': 'invalid_path', 'reason': "Path `/dummy.txt` doesn't exists"}
        await sock.send({'cmd': 'file_write', 'path': '/dir', 'content': to_jsonb64(b'foo')})
        rep = await sock.recv()
        assert rep == {'status': 'invalid_path', 'reason': "Path `/dir` is not a file"}
    # No modifications, no flush
    assert core.mocked_local_storage_cls.return_value.flush_manifest.call_count == 0
    assert core.mocked_local_storage_cls.return_value.flush_user_manifest.call_count == 0


# TODO: test write with offset
