import pytest
import trio
from unittest.mock import Mock
from trio.testing import trio_test
from freezegun import freeze_time

from parsec.utils import to_jsonb64, from_jsonb64

from tests.common import with_core, with_populated_local_storage, async_patch


def with_backend_conn_mocked(testfunc):
    class BackendConnectionMock:
        mock = Mock()

        def __init__(self, *args, **kwargs):
            pass

        async def block_get(self, id):
            return self.mock.block_get(id)

        async def block_save(self, block):
            return self.mock.block_save(block)

        async def send(self, req):
            return self.mock.send(req)

        async def init(self, nursery):
            pass

    return async_patch('parsec.core.local_fs.BackendConnection', BackendConnectionMock)(testfunc)


@pytest.mark.xfail
@trio_test
@with_backend_conn_mocked
@with_core(with_backend=False)
@with_populated_local_storage('alice')
async def test_sync_files(core, backend_conn_cls_mock):
    # Only `/dir/modifed.txt` needs to be sync to the backend
    bcm = backend_conn_cls_mock.mock
    bcm.block_save.side_effect = [
        '9f6dee67628e48ba9bea177c078f4896'
    ]
    bcm.send.side_effect = [
        {'status': 'ok'}  # vlob_update
    ]
    async with core.test_connect('alice@test') as sock:
        async with trio.open_nursery() as nursery:
            await core.fs.synchronizer._synchronize_files(nursery)
    assert bcm.block_save.call_count == 1
    assert bcm.send.call_count == 1
    # TODO: this test is really really raw...


@pytest.mark.xfail
@trio_test
@with_backend_conn_mocked
@with_core(with_backend=False)
@with_populated_local_storage('alice')
async def test_sync_user_manifest(core, backend_conn_cls_mock):
    # First placeholder file `/dir/new.txt` must be turned into a real file,
    # then the local user manifest can be sync
    bcm = backend_conn_cls_mock.mock
    bcm.block_save.side_effect = [
        '9f6dee67628e48ba9bea177c078f4896'
    ]
    bcm.send.side_effect = [
        {  # vlob_create
            'status': 'ok',
            'id': '0486103cf7d443fd95760f2204396ba1',
            'read_trust_seed': '<rts>',
            'write_trust_seed': '<wts>'
        }
    ]
    async with core.test_connect('alice@test') as sock:
        async with trio.open_nursery() as nursery:
            await core.fs.synchronizer._synchronize_user_manifest(nursery)
    assert bcm.block_save.call_count == 1
    assert bcm.send.call_count == 1
    # TODO: not finished...
