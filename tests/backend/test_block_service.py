from freezegun import freeze_time
import pytest

from parsec.backend import BlockService


@pytest.fixture
def block_svc():
    return BlockService()


@pytest.fixture
@freeze_time("2012-01-01")
def block(block_svc, event_loop, content='Whatever.'):
    return event_loop.run_until_complete(block_svc.create(content)), content


class TestBlockServiceAPI:

    @pytest.mark.asyncio
    async def test_create(self, block_svc):
        ret = await block_svc.dispatch_msg({'cmd': 'create', 'content': 'Foo.'})
        assert ret['status'] == 'ok'
        assert ret['id']

    @pytest.mark.asyncio
    @freeze_time("2012-01-01")
    async def test_read(self, block_svc, block):
        block_id, block_content = block
        with freeze_time('2012-01-01') as frozen_datetime:
            creation_timestamp = frozen_datetime().timestamp()
            frozen_datetime.tick()
            access_timestamp = frozen_datetime().timestamp()
            ret = await block_svc.dispatch_msg({'cmd': 'read', 'id': block_id})
            assert {'status': 'ok',
                    'access_timestamp': access_timestamp,
                    'creation_timestamp': creation_timestamp,
                    'content': block_content} == ret
        # Unknown block
        ret = await block_svc.dispatch_msg({'cmd': 'read', 'id': '1234'})
        assert {'label': 'Block not found.', 'status': 'not_found'} == ret

    @pytest.mark.asyncio
    @freeze_time("2012-01-01")
    async def test_stat(self, block_svc, block):
        block_id, block_content = block
        with freeze_time('2012-01-01') as frozen_datetime:
            creation_timestamp = frozen_datetime().timestamp()
            ret = await block_svc.dispatch_msg({'cmd': 'stat', 'id': block_id})
            assert {'status': 'ok',
                    'access_timestamp': creation_timestamp,
                    'creation_timestamp': creation_timestamp} == ret
        # Unknown block
        ret = await block_svc.dispatch_msg({'cmd': 'stat', 'id': '1234'})
        assert {'label': 'Block not found.', 'status': 'not_found'} == ret
