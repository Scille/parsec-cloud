import pytest

from parsec.core import (MockedBackendAPIService, MockedBlockService, SynchronizerService)
from parsec.exceptions import BlockNotFound, UserVlobNotFound, VlobNotFound
from parsec.server import BaseServer


@pytest.fixture
def backend_svc():
    return MockedBackendAPIService()


@pytest.fixture
def block_svc():
    return MockedBlockService()


@pytest.fixture
def synchronizer_svc(event_loop, backend_svc, block_svc):
    # block_service = MetaBlockService(backends=[MockedBlockService, MockedBlockService])
    service = SynchronizerService()
    server = BaseServer()
    server.register_service(service)
    server.register_service(backend_svc)
    server.register_service(block_svc)
    event_loop.run_until_complete(server.bootstrap_services())
    yield service
    event_loop.run_until_complete(server.teardown_services())


class TestSynchronizerService:

    @pytest.mark.asyncio
    async def test_block_create(self, block_svc, synchronizer_svc):
        content = 'foo'
        block_id = await synchronizer_svc.block_create(content)
        await synchronizer_svc.block_read(block_id)
        with pytest.raises(BlockNotFound):
            await block_svc.read(block_id)

    @pytest.mark.asyncio
    async def test_block_read(self, block_svc, synchronizer_svc):
        local_content = 'foo'
        local_block_id = await synchronizer_svc.block_create(local_content)
        ret = await synchronizer_svc.block_read(local_block_id)
        assert sorted(list(ret.keys())) == ['content', 'creation_date']
        assert ret['content'] == local_content
        remote_content = 'bar'
        remote_block_id = await block_svc.create(remote_content)
        ret = await synchronizer_svc.block_read(remote_block_id)
        assert sorted(list(ret.keys())) == ['content', 'creation_date']
        assert ret['content'] == remote_content

    @pytest.mark.asyncio
    async def test_block_stat(self, block_svc, synchronizer_svc):
        local_content = 'foo'
        local_block_id = await synchronizer_svc.block_create(local_content)
        ret = await synchronizer_svc.block_stat(local_block_id)
        assert sorted(list(ret.keys())) == ['creation_date']
        remote_content = 'bar'
        remote_block_id = await block_svc.create(remote_content)
        ret = await synchronizer_svc.block_stat(remote_block_id)
        assert sorted(list(ret.keys())) == ['creation_date']

    @pytest.mark.asyncio
    async def test_block_delete(self, block_svc, synchronizer_svc):
        block_id = await synchronizer_svc.block_create('foo')
        await synchronizer_svc.block_read(block_id)
        await synchronizer_svc.block_delete(block_id)
        with pytest.raises(BlockNotFound):
            await synchronizer_svc.block_read(block_id)
        with pytest.raises(BlockNotFound):
            await synchronizer_svc.block_delete(block_id)

    @pytest.mark.asyncio
    async def test_block_list(self, block_svc, synchronizer_svc):
        block_id = await synchronizer_svc.block_create('foo')
        block_2_id = await synchronizer_svc.block_create('bar')
        ret = await synchronizer_svc.block_list()
        assert isinstance(ret, list)
        assert set(ret) == set([block_id, block_2_id])

    @pytest.mark.asyncio
    async def test_block_synchronize(self, block_svc, synchronizer_svc):
        content = 'foo'
        block_id = await synchronizer_svc.block_create(content)
        with pytest.raises(BlockNotFound):
            await block_svc.read(block_id)
        await synchronizer_svc.block_synchronize(block_id)
        ret = await block_svc.read(block_id)
        assert sorted(list(ret.keys())) == ['content', 'creation_date']
        assert ret['content'] == content
        ret = await synchronizer_svc.block_list()
        assert ret == []

    @pytest.mark.asyncio
    async def test_user_vlob_read(self, backend_svc, synchronizer_svc):
        remote_blob = 'bar'
        await backend_svc.user_vlob_update(1, remote_blob)
        ret = await synchronizer_svc.user_vlob_read()
        assert sorted(list(ret.keys())) == ['blob', 'version']
        assert ret['blob'] == remote_blob
        assert ret['version'] == 1
        local_blob = 'foo'
        await synchronizer_svc.user_vlob_update(2, local_blob)
        ret = await synchronizer_svc.user_vlob_read()
        assert sorted(list(ret.keys())) == ['blob', 'version']
        assert ret['blob'] == local_blob
        assert ret['version'] == 2

    @pytest.mark.asyncio
    async def test_user_vlob_update(self, backend_svc, synchronizer_svc):
        await synchronizer_svc.user_vlob_update(1, 'foo')
        user_vlob_blob = 'bar'
        await synchronizer_svc.user_vlob_update(1, user_vlob_blob)
        ret = await synchronizer_svc.user_vlob_read()
        assert sorted(list(ret.keys())) == ['blob', 'version']
        assert ret['blob'] == user_vlob_blob
        assert ret['version'] == 1

    @pytest.mark.asyncio
    async def test_user_vlob_delete(self, synchronizer_svc):
        await synchronizer_svc.user_vlob_update('foo')
        await synchronizer_svc.user_vlob_read()
        await synchronizer_svc.user_vlob_delete()
        user_vlob = await synchronizer_svc.user_vlob_read()
        assert user_vlob['blob'] == ''
        with pytest.raises(UserVlobNotFound):
            await synchronizer_svc.user_vlob_delete()

    @pytest.mark.asyncio
    async def test_user_vlob_synchronize(self, synchronizer_svc):
        ret = await synchronizer_svc.user_vlob_read()
        assert sorted(list(ret.keys())) == ['blob', 'version']
        assert ret['blob'] == ''
        assert ret['version'] == 0
        blob = 'foo'
        await synchronizer_svc.user_vlob_update(1, blob)
        await synchronizer_svc.user_vlob_synchronize()
        ret = await synchronizer_svc.user_vlob_read()
        assert sorted(list(ret.keys())) == ['blob', 'version']
        assert ret['blob'] == blob
        assert ret['version'] == 1
        ret = await synchronizer_svc.user_vlob()
        assert ret is False
        # Do nothing
        await synchronizer_svc.user_vlob_synchronize()

    @pytest.mark.asyncio
    async def test_vlob_create(self, backend_svc, synchronizer_svc):
        blob = 'foo'
        ret = await synchronizer_svc.vlob_create(blob)
        vlob_id = ret['id']
        read_trust_seed = ret['read_trust_seed']
        assert sorted(list(ret.keys())) == ['id', 'read_trust_seed', 'write_trust_seed']
        await synchronizer_svc.vlob_read(vlob_id, read_trust_seed)
        with pytest.raises(VlobNotFound):
            await backend_svc.vlob_read(vlob_id, read_trust_seed)

    @pytest.mark.asyncio
    async def test_vlob_read(self, backend_svc, synchronizer_svc):
        local_blob = 'foo'
        local_vlob = await synchronizer_svc.vlob_create(local_blob)
        ret = await synchronizer_svc.vlob_read(local_vlob['id'], local_vlob['read_trust_seed'])
        assert sorted(list(ret.keys())) == ['blob', 'id', 'version']
        assert ret['id'] == local_vlob['id']
        assert ret['blob'] == local_blob
        assert ret['version'] == 1
        remote_blob = 'bar'
        remote_vlob = await backend_svc.vlob_create(remote_blob)
        ret = await synchronizer_svc.vlob_read(remote_vlob['id'], remote_vlob['read_trust_seed'])
        assert sorted(list(ret.keys())) == ['blob', 'id', 'version']
        assert ret['id'] == remote_vlob['id']
        assert ret['blob'] == remote_blob
        assert ret['version'] == 1

    @pytest.mark.asyncio
    async def test_vlob_update(self, backend_svc, synchronizer_svc):
        local_vlob = await synchronizer_svc.vlob_create()
        local_blob = 'foo'
        await synchronizer_svc.vlob_update(local_vlob['id'], 1, local_vlob['write_trust_seed'], local_blob)
        ret = await synchronizer_svc.vlob_read(local_vlob['id'], local_vlob['read_trust_seed'])
        assert sorted(list(ret.keys())) == ['blob', 'id', 'version']
        assert ret['id'] == local_vlob['id']
        assert ret['blob'] == local_blob
        assert ret['version'] == 1

    @pytest.mark.asyncio
    async def test_vlob_delete(self, backend_svc, synchronizer_svc):
        local_vlob = await synchronizer_svc.vlob_create('foo')
        await synchronizer_svc.vlob_read(local_vlob['id'], local_vlob['read_trust_seed'])
        await synchronizer_svc.vlob_delete(local_vlob['id'])
        with pytest.raises(VlobNotFound):
            await synchronizer_svc.vlob_read(local_vlob['id'], local_vlob['read_trust_seed'])
        with pytest.raises(VlobNotFound):
            await synchronizer_svc.vlob_delete(local_vlob['id'])

    @pytest.mark.asyncio
    async def test_vlob_list(self, backend_svc, synchronizer_svc):
        local_vlob = await synchronizer_svc.vlob_create('foo')
        local_vlob_2 = await synchronizer_svc.vlob_create('bar')
        ret = await synchronizer_svc.vlob_list()
        assert isinstance(ret, list)
        assert set(ret) == set([local_vlob['id'], local_vlob_2['id']])

    @pytest.mark.asyncio
    async def test_vlob_synchronize(self, backend_svc, synchronizer_svc):
        blob = 'foo'
        ret = await synchronizer_svc.vlob_create(blob)
        vlob_id = ret['id']
        new_vlob = await synchronizer_svc.vlob_synchronize(vlob_id)
        ret = await synchronizer_svc.vlob_read(new_vlob['id'], new_vlob['read_trust_seed'])
        assert sorted(list(ret.keys())) == ['blob', 'id', 'version']
        assert ret['id'] == new_vlob['id']
        assert ret['blob'] == blob
        assert ret['version'] == 1
        ret = await synchronizer_svc.vlob_list()
        assert ret == []
        blob = 'bar'
        await synchronizer_svc.vlob_update(new_vlob['id'], 2, new_vlob['write_trust_seed'], blob)
        await synchronizer_svc.vlob_synchronize(new_vlob['id'])
        ret = await synchronizer_svc.vlob_read(new_vlob['id'], new_vlob['read_trust_seed'])
        assert sorted(list(ret.keys())) == ['blob', 'id', 'version']
        assert ret['id'] == new_vlob['id']
        assert ret['blob'] == blob
        assert ret['version'] == 2
        # Do nothing
        await synchronizer_svc.vlob_synchronize(new_vlob['id'])

    @pytest.mark.asyncio
    async def test_synchronize(self, synchronizer_svc):
        await synchronizer_svc.block_create('foo')
        await synchronizer_svc.user_vlob_update(1, 'foo')
        await synchronizer_svc.vlob_create('foo')
        await synchronizer_svc.synchronize()
        ret = await synchronizer_svc.block_list()
        assert ret == []
        ret = await synchronizer_svc.user_vlob()
        assert ret is False
        ret = await synchronizer_svc.vlob_list()
        assert ret == []

    @pytest.mark.xfail
    @pytest.mark.asyncio
    async def test_periodic_synchronization(self):
        pass
