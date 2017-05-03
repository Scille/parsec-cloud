import pytest
import asyncio

from parsec.server import BaseServer
from parsec.backend import InMemoryMessageService

from .common import init_or_skiptest_parsec_postgresql


async def bootstrap_PostgreSQLMessageService(request, event_loop):
    module, url = await init_or_skiptest_parsec_postgresql()

    server = BaseServer()
    server.register_service(module.PostgreSQLService(url))
    msg_svc = module.PostgreSQLMessageService()
    server.register_service(msg_svc)
    await server.bootstrap_services()

    def finalize():
        event_loop.run_until_complete(server.teardown_services())

    request.addfinalizer(finalize)
    return msg_svc


@pytest.fixture(params=[InMemoryMessageService, bootstrap_PostgreSQLMessageService], ids=['in_memory', 'postgresql'])
def message_svc(request, event_loop):
    if asyncio.iscoroutinefunction(request.param):
        return event_loop.run_until_complete(request.param(request, event_loop))
    else:
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

    @pytest.mark.asyncio
    async def test_on_arrived_event(self, message_svc):
        called_with = '<not called>'

        def on_event(*args):
            nonlocal called_with
            called_with = args

        message_svc.on_arrived.connect(on_event, 'alice@test.com')
        # Message to wrong person doesn't trigger event
        ret = await message_svc.dispatch_msg({
            'cmd': 'message_new', 'recipient': 'bob@test.com', 'body': 'Hi dude !'})
        assert ret == {'status': 'ok'}
        assert called_with == '<not called>'
        ret = await message_svc.dispatch_msg({
            'cmd': 'message_new', 'recipient': 'alice@test.com', 'body': 'Hi dude !'})
        assert ret == {'status': 'ok'}
        assert called_with == ('alice@test.com', )
