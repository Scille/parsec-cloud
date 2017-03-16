import pytest

from parsec.backend import BlockService


@pytest.fixture
def block_svc():
    return BlockService()


@pytest.fixture
def block(block_svc, event_loop, content='Whatever.'):
    return event_loop.run_until_complete(block_svc.create(content)), content


class TestBlockServiceAPI:

    @pytest.mark.asyncio
    async def test_create(self, block_svc):
        ret = await block_svc.dispatch_msg({'cmd': 'create', 'content': 'Foo.'})
        assert ret['status'] == 'ok'
        assert ret['id']

    @pytest.mark.asyncio
    async def test_read(self, block_svc, block):
        block_id, block_content = block
        ret = await block_svc.dispatch_msg({'cmd': 'read', 'id': block_id})
        assert ret['status'] == 'ok'
        assert ret['content'] == block_content
