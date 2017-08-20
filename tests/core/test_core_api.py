import pytest
from effect2.testing import conste, noop, perform_sequence

from parsec.core.core_api import execute_cmd, execute_raw_cmd, EClientSubscribeEvent
from parsec.exceptions import ParsecError


def test_execute_cmd():
    eff = execute_cmd('subscribe_event', {'event': 'foo', 'sender': 'bar'})
    sequence = [
        (EClientSubscribeEvent('foo', 'bar'), noop)
    ]
    resp = perform_sequence(sequence, eff)
    assert resp == {'status': 'ok'}


def test_catch_parsec_exception():
    eff = execute_cmd('subscribe_event', {'event': 'foo', 'sender': 'bar'})
    sequence = [
        (EClientSubscribeEvent('foo', 'bar'), conste(ParsecError('error', 'msg'))),
    ]
    resp = perform_sequence(sequence, eff)
    assert resp == {'status': 'error', 'label': 'msg'}


class TestRawCMD:
    def test_execute(self):
        raw_cmd = b'{"cmd": "subscribe_event", "event": "foo", "sender": "bar"}'
        eff = execute_raw_cmd(raw_cmd)
        sequence = [
            (EClientSubscribeEvent('foo', 'bar'), noop)
        ]
        resp = perform_sequence(sequence, eff)
        assert resp == b'{"status": "ok"}'

    @pytest.mark.parametrize('bad_raw_cmd', [
        b'',
        b'not a json',
        b'{}',
    ])
    def test_bad_cmd(self, bad_raw_cmd):
        eff = execute_raw_cmd(bad_raw_cmd)
        sequence = [
        ]
        resp = perform_sequence(sequence, eff)
        assert resp == b'{"label": "Message is not a valid JSON.", "status": "bad_msg"}'
