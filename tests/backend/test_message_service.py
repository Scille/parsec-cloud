import pytest

from parsec.backend import MessageService


@pytest.fixture
def message_svc():
    return MessageService()


class TestMessageServiceAPI:

    @pytest.mark.asyncio
    async def test_new(self, message_svc):
        ret = await message_svc.dispatch_msg({
            'cmd': 'new', 'recipient': 'jdoe@test.com', 'body': 'Hi dude !'})
        assert ret == {'status': 'ok'}

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
