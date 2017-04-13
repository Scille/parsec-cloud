import pytest

from parsec.backend import InMemoryMessageService


@pytest.fixture(params=[InMemoryMessageService, ])
def message_svc(request):
    return request.param()


class TestMessageServiceAPI:

    @pytest.mark.asyncio
    async def test_new(self, message_svc):
        ret = await message_svc.dispatch_msg({
            'cmd': 'message_new', 'recipient': 'jdoe@test.com', 'body': 'Hi dude !'})
        assert ret == {'status': 'ok'}

    @pytest.mark.asyncio
    @pytest.mark.parametrize('bad_msg', [
        {'cmd': 'message_new', 'recipient': 'jdoe@test.com', 'body': '...', 'bad_field': 'foo'},
        {'cmd': 'message_new', 'recipient': 'jdoe@test.com', 'body': 42},
        {'cmd': 'message_new', 'recipient': 'jdoe@test.com', 'body': None},
        {'cmd': 'message_new', 'recipient': 42, 'body': '...'},
        {'cmd': 'message_new', 'recipient': None, 'body': '...'},
        {'cmd': 'message_new'}, {'recipient': 'jdoe@test.com', 'body': '...'}, {}])
    async def test_bad_msg_new(self, message_svc, bad_msg):
        ret = await message_svc.dispatch_msg(bad_msg)
        assert ret['status'] == 'bad_msg'

    @pytest.mark.asyncio
    async def test_get_empty(self, message_svc):
        ret = await message_svc.dispatch_msg({
            'cmd': 'message_get', 'recipient': 'unknown@test.com'})
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
                'cmd': 'message_new', 'recipient': recipient, 'body': body})
        assert ret == {'status': 'ok'}
        ret = await message_svc.dispatch_msg({
            'cmd': 'message_get', 'recipient': 'alice@test.com'})
        assert ret == {'status': 'ok', 'messages': [
            {'count': 1, 'body': 'Alice1'},
            {'count': 2, 'body': 'Alice2'},
            {'count': 3, 'body': 'Alice3'}
        ]}
        ret = await message_svc.dispatch_msg({
            'cmd': 'message_get', 'recipient': 'alice@test.com', 'offset': 2})
        assert ret == {'status': 'ok', 'messages': [
            {'count': 3, 'body': 'Alice3'}
        ]}

    @pytest.mark.asyncio
    @pytest.mark.parametrize('bad_msg', [
        {'cmd': 'message_get', 'recipient': 'jdoe@test.com', 'bad_field': 'foo'},
        {'cmd': 'message_get', 'recipient': 'jdoe@test.com', 'offset': 0, 'bad_field': 'foo'},
        {'cmd': 'message_get', 'recipient': 'jdoe@test.com', 'offset': None},
        {'cmd': 'message_get', 'recipient': 'jdoe@test.com', 'offset': 'zero'},
        {'cmd': 'message_get', 'recipient': 42},
        {'cmd': 'message_get', 'recipient': None},
        {'cmd': 'message_get'}, {'recipient': 'jdoe@test.com'}, {}])
    async def test_bad_msg_get(self, message_svc, bad_msg):
        ret = await message_svc.dispatch_msg(bad_msg)
        assert ret['status'] == 'bad_msg'
