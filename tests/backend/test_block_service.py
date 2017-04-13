from freezegun import freeze_time
import pytest

from parsec.backend import MockedBlockService


@pytest.fixture(params=[MockedBlockService, ])
def block_svc(request):
    return request.param()


@pytest.fixture
@freeze_time("2012-01-01")
def block(block_svc, event_loop, content='Whatever.'):
    return event_loop.run_until_complete(block_svc.create(content)), content


class TestBlockServiceAPI:

    @pytest.mark.asyncio
    async def test_create(self, block_svc):
        ret = await block_svc.dispatch_msg({'cmd': 'block_create', 'content': 'Foo.'})
        assert ret['status'] == 'ok'
        assert ret['id']

    @pytest.mark.asyncio
    @pytest.mark.parametrize('bad_msg', [
        {'cmd': 'block_create', 'content': '...', 'bad_field': 'foo'},
        {'cmd': 'block_create', 'content': 42},
        {'cmd': 'block_create', 'content': None},
        {'cmd': 'block_create'}, {}])
    async def test_bad_msg_create(self, block_svc, bad_msg):
        ret = await block_svc.dispatch_msg(bad_msg)
        assert ret['status'] == 'bad_msg'

    @pytest.mark.asyncio
    @freeze_time("2012-01-01")
    async def test_read(self, block_svc, block):
        block_id, block_content = block
        with freeze_time('2012-01-01') as frozen_datetime:
            creation_timestamp = frozen_datetime().timestamp()
            frozen_datetime.tick()
            access_timestamp = frozen_datetime().timestamp()
            ret = await block_svc.dispatch_msg({'cmd': 'block_read', 'id': block_id})
            assert {'status': 'ok',
                    'access_timestamp': access_timestamp,
                    'creation_timestamp': creation_timestamp,
                    'content': block_content} == ret
        # Unknown block
        ret = await block_svc.dispatch_msg({'cmd': 'block_read', 'id': '1234'})
        assert {'label': 'Block not found.', 'status': 'not_found'} == ret

    @pytest.mark.asyncio
    @pytest.mark.parametrize('bad_msg', [
        {'cmd': 'block_read', 'id': '<insert_id_here>', 'bad_field': 'foo'},
        {'cmd': 'block_read', 'id': 42},
        {'cmd': 'block_read', 'id': None},
        {'cmd': 'block_read'}, {}])
    async def test_bad_msg_read(self, block_svc, block, bad_msg):
        block_id, block_content = block
        if bad_msg.get('id') == '<insert_id_here>':
            bad_msg['id'] = block_id
        ret = await block_svc.dispatch_msg(bad_msg)
        assert ret['status'] == 'bad_msg'

    @pytest.mark.asyncio
    @freeze_time("2012-01-01")
    async def test_stat(self, block_svc, block):
        block_id, block_content = block
        with freeze_time('2012-01-01') as frozen_datetime:
            creation_timestamp = frozen_datetime().timestamp()
            ret = await block_svc.dispatch_msg({'cmd': 'block_stat', 'id': block_id})
            assert {'status': 'ok',
                    'access_timestamp': creation_timestamp,
                    'creation_timestamp': creation_timestamp} == ret
        # Unknown block
        ret = await block_svc.dispatch_msg({'cmd': 'block_stat', 'id': '1234'})
        assert {'label': 'Block not found.', 'status': 'not_found'} == ret

    @pytest.mark.asyncio
    @pytest.mark.parametrize('bad_msg', [
        {'cmd': 'block_stat', 'id': '<insert_id_here>', 'bad_field': 'foo'},
        {'cmd': 'block_stat', 'id': 42},
        {'cmd': 'block_stat', 'id': None},
        {'cmd': 'block_stat'}, {}])
    async def test_bad_msg_stat(self, block_svc, block, bad_msg):
        block_id, block_content = block
        if bad_msg.get('id') == '<insert_id_here>':
            bad_msg['id'] = block_id
        ret = await block_svc.dispatch_msg(bad_msg)
        assert ret['status'] == 'bad_msg'
