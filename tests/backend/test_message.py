import pytest
import asyncio
from effect2.testing import const, perform_sequence, asyncio_perform_sequence, noop

from parsec.base import EEvent
from parsec.backend.session import EGetAuthenticatedUser
from parsec.backend.backend_api import execute_cmd
from parsec.backend.message import (
    EMessageNew, EMessageGet, InMemoryMessageComponent)
from parsec.tools import to_jsonb64

from tests.common import can_side_effect_or_skip
from tests.backend.common import init_or_skiptest_parsec_postgresql


async def bootstrap_PostgreSQLMessageComponent(request, loop):
    can_side_effect_or_skip()
    module, url = await init_or_skiptest_parsec_postgresql()

    conn = module.PostgreSQLConnection(url)
    await conn.open_connection()

    def finalize():
        loop.run_until_complete(conn.close_connection())

    request.addfinalizer(finalize)
    return module.PostgreSQLMessageComponent(conn)


@pytest.fixture(params=[InMemoryMessageComponent, bootstrap_PostgreSQLMessageComponent],
                ids=['in_memory', 'postgresql'])
def component(request, loop):
    if asyncio.iscoroutinefunction(request.param):
        return loop.run_until_complete(request.param(request, loop))
    else:
        return request.param()


class TestMessageComponent:

    async def test_message_get_empty(self, component):
        intent = EMessageGet(offset=0)
        eff = component.perform_message_get(intent)
        sequence = [
            (EGetAuthenticatedUser(), const('alice@test.com'))
        ]
        ret = await asyncio_perform_sequence(sequence, eff)
        assert ret == []

    async def test_message_new(self, component):
        intent = EMessageNew(recipient='alice@test.com', body=b'This is for alice.')
        eff = component.perform_message_new(intent)
        sequence = [
            (EEvent('message_arrived', 'alice@test.com'), noop)
        ]
        ret = await asyncio_perform_sequence(sequence, eff)
        assert ret is None

    async def test_message_retreive(self, component):
        async def _recv_msg(recipient):
            intent = EMessageGet(offset=0)
            eff = component.perform_message_get(intent)
            sequence = [(EGetAuthenticatedUser(), const(recipient))]
            return await asyncio_perform_sequence(sequence, eff)

        async def _send_msg(recipient):
            intent = EMessageNew(recipient=recipient, body=b'for %s' % recipient.encode())
            eff = component.perform_message_new(intent)
            sequence = [(EEvent('message_arrived', recipient), noop)]
            await asyncio_perform_sequence(sequence, eff)

        await _send_msg('alice')
        await _send_msg('bob')
        await _send_msg('alice')
        await _send_msg('bob')

        alice_messages = await _recv_msg('alice')
        assert alice_messages == [b'for alice', b'for alice']
        bob_messages = await _recv_msg('bob')
        assert bob_messages == [b'for bob', b'for bob']


class TestMessageAPI:

    def test_message_get_ok(self):
        eff = execute_cmd('message_get', {})
        sequence = [
            (EMessageGet(offset=0), const([b'zero', b'one', b'two']))
        ]
        ret = perform_sequence(sequence, eff)
        assert ret == {
            'status': 'ok',
            'messages': [
                {'count': 1, 'body': to_jsonb64(b'zero')},
                {'count': 2, 'body': to_jsonb64(b'one')},
                {'count': 3, 'body': to_jsonb64(b'two')}
            ]
        }

    def test_message_get_with_offset(self):
        eff = execute_cmd('message_get', {'offset': 42})
        sequence = [
            (EMessageGet(offset=42), const([b'zero', b'one', b'two']))
        ]
        ret = perform_sequence(sequence, eff)
        assert ret == {
            'status': 'ok',
            'messages': [
                {'count': 43, 'body': to_jsonb64(b'zero')},
                {'count': 44, 'body': to_jsonb64(b'one')},
                {'count': 45, 'body': to_jsonb64(b'two')}
            ]
        }

    @pytest.mark.parametrize('bad_msg', [
        {'bad_field': 'foo'},
        {'offset': 0, 'bad_field': 'foo'},
        {'offset': None},
        {'offset': 'zero'}
    ])
    def test_message_get_bad_msg(self, bad_msg):
        eff = execute_cmd('message_get', bad_msg)
        sequence = [
        ]
        ret = perform_sequence(sequence, eff)
        assert ret['status'] == 'bad_msg'

    def test_message_new_ok(self):
        eff = execute_cmd('message_new', {
            'recipient': 'alice@test.com', 'body': to_jsonb64(b'This is for alice.')})
        sequence = [
            (EMessageNew(recipient='alice@test.com', body=b'This is for alice.'), noop)
        ]
        ret = perform_sequence(sequence, eff)
        assert ret == {'status': 'ok'}

    @pytest.mark.parametrize('bad_msg', [
        {'recipient': 'jdoe@test.com',
         'body': to_jsonb64(b'...'), 'bad_field': 'foo'},
        {'recipient': 'jdoe@test.com', 'body': 42},
        {'recipient': 'jdoe@test.com', 'body': None},
        {'recipient': 42, 'body': to_jsonb64(b'...')},
        {'recipient': None, 'body': to_jsonb64(b'...')},
        {}
    ])
    def test_message_new_bad_msg(self, bad_msg):
        eff = execute_cmd('message_new', bad_msg)
        sequence = [
        ]
        ret = perform_sequence(sequence, eff)
        assert ret['status'] == 'bad_msg'
