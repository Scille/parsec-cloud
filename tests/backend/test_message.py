import pytest
from effect2.testing import const, perform_sequence, noop

from parsec.base import EEvent
from parsec.backend.session import EGetAuthenticatedUser
from parsec.backend.backend_api import execute_cmd
from parsec.backend.message import (
    EMessageNew, EMessageGet, InMemoryMessageComponent)
from parsec.tools import to_jsonb64


@pytest.fixture
def component():
    return InMemoryMessageComponent()


class TestMessageComponent:

    def test_message_get_empty(self, component):
        intent = EMessageGet(offset=0)
        eff = component.perform_message_get(intent)
        sequence = [
            (EGetAuthenticatedUser(), const('alice@test.com'))
        ]
        ret = perform_sequence(sequence, eff)
        assert ret == []

    def test_message_new(self, component):
        intent = EMessageNew(recipient='alice@test.com', body=b'This is for alice.')
        eff = component.perform_message_new(intent)
        sequence = [
            (EEvent('on_message_arrived', 'alice@test.com'), noop)
        ]
        ret = perform_sequence(sequence, eff)
        assert ret is None

    def test_message_retreive(self, component):
        def _recv_msg(recipient):
            intent = EMessageGet(offset=0)
            eff = component.perform_message_get(intent)
            sequence = [(EGetAuthenticatedUser(), const(recipient))]
            return perform_sequence(sequence, eff)

        def _send_msg(recipient):
            intent = EMessageNew(recipient=recipient, body=b'for %s' % recipient.encode())
            eff = component.perform_message_new(intent)
            sequence = [(EEvent('on_message_arrived', recipient), noop)]
            perform_sequence(sequence, eff)

        _send_msg('alice')
        _send_msg('bob')
        _send_msg('alice')
        _send_msg('bob')

        assert _recv_msg('alice') == [b'for alice', b'for alice']
        assert _recv_msg('bob') == [b'for bob', b'for bob']


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
