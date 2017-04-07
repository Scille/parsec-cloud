import pytest

from parsec.backend import InMemoryMessageService


@pytest.fixture(params=[InMemoryMessageService, ])
def message_svc(request):
    return request.param()


class TestMessageServiceAPI:

    @pytest.mark.asyncio
    async def test_new(self, message_svc):
        ret = await message_svc.dispatch_msg({
            'cmd': 'new', 'recipient': 'jdoe@test.com', 'body': 'Hi dude !'})
        assert ret == {'status': 'ok'}

    @pytest.mark.asyncio
    @pytest.mark.parametrize('bad_msg', [
        {'cmd': 'new', 'recipient': 'jdoe@test.com', 'body': '...', 'bad_field': 'foo'},
        {'cmd': 'new', 'recipient': 'jdoe@test.com', 'body': 42},
        {'cmd': 'new', 'recipient': 'jdoe@test.com', 'body': None},
        {'cmd': 'new', 'recipient': 42, 'body': '...'},
        {'cmd': 'new', 'recipient': None, 'body': '...'},
        {'cmd': 'new'}, {'recipient': 'jdoe@test.com', 'body': '...'}, {}])
    async def test_bad_msg_new(self, message_svc, bad_msg):
        ret = await message_svc.dispatch_msg(bad_msg)
        assert ret['status'] == 'bad_msg'

    @pytest.mark.asyncio
    async def test_get_empty(self, message_svc):
        ret = await message_svc.dispatch_msg({
            'cmd': 'get', 'recipient': 'unknown@test.com'})
        assert ret == {'status': 'ok', 'messages': []}

    @pytest.mark.asyncio
    async def test_ordered(self, message_svc):
        for recipient, body in (
                ('alice@test.com', 'Alice1'),
                ('bob@test.com', 'Bob1'),
                ('foo@test.com', 'Foo1'),
                ('alice@test.com', 'Alice2'),
                ('alice@test.com', 'Alice3'),
                ('bob@test.com', 'Bob2')):
            ret = await message_svc.dispatch_msg({
                'cmd': 'new', 'recipient': recipient, 'body': body})
        assert ret == {'status': 'ok'}
        ret = await message_svc.dispatch_msg({
            'cmd': 'get', 'recipient': 'alice@test.com'})
        assert ret == {'status': 'ok', 'messages': [
            {'count': 1, 'body': 'Alice1'},
            {'count': 2, 'body': 'Alice2'},
            {'count': 3, 'body': 'Alice3'}
        ]}
        ret = await message_svc.dispatch_msg({
            'cmd': 'get', 'recipient': 'alice@test.com', 'offset': 2})
        assert ret == {'status': 'ok', 'messages': [
            {'count': 3, 'body': 'Alice3'}
        ]}

    @pytest.mark.asyncio
    @pytest.mark.parametrize('bad_msg', [
        {'cmd': 'get', 'recipient': 'jdoe@test.com', 'bad_field': 'foo'},
        {'cmd': 'get', 'recipient': 'jdoe@test.com', 'offset': 0, 'bad_field': 'foo'},
        {'cmd': 'get', 'recipient': 'jdoe@test.com', 'offset': None},
        {'cmd': 'get', 'recipient': 'jdoe@test.com', 'offset': 'zero'},
        {'cmd': 'get', 'recipient': 42},
        {'cmd': 'get', 'recipient': None},
        {'cmd': 'get'}, {'recipient': 'jdoe@test.com'}, {}])
    async def test_bad_msg_get(self, message_svc, bad_msg):
        ret = await message_svc.dispatch_msg(bad_msg)
        assert ret['status'] == 'bad_msg'
