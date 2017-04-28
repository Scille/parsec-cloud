import pytest

from parsec.backend import InMemoryMessageService
from .common import postgresql_url


async def bootstrap_PostgreSQLMessageService(request, event_loop):
    module = pytest.importorskip('parsec.backend.postgresql')
    await module._init_db(postgresql_url(), force=True)
    svc = module.PostgreSQLMessageService(postgresql_url())
    await svc.bootstrap()

    def finalize():
        event_loop.run_until_complete(svc.teardown())

    request.addfinalizer(finalize)
    return svc


def bootstrap_InMemoryMessageService(request, event_loop):
    return InMemoryMessageService()


@pytest.fixture(params=[bootstrap_InMemoryMessageService, bootstrap_PostgreSQLMessageService], ids=['in_memory', 'postgresql'])
def message_svc(request, event_loop):
    return request.param(request, event_loop)


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
