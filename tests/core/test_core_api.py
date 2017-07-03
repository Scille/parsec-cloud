import pytest
from effect2.testing import perform_sequence, raise_
from unittest.mock import Mock

from parsec.core.core_api import execute_cmd, execute_raw_cmd, api_identity_load
from parsec.core.identity import EIdentityLoad, Identity
from parsec.exceptions import ParsecError


def test_execute_cmd():
    eff = execute_cmd('identity_load', {'id': 'JohnDoe', 'key': 'MTIzNDU=\n'})
    sequence = [
        (EIdentityLoad('JohnDoe', b'12345', None),
            lambda _: Identity('JohnDoe', Mock(), Mock())),
    ]
    resp = perform_sequence(sequence, eff)
    assert resp == {'status': 'ok'}


def test_catch_parsec_exception():
    eff = execute_cmd('identity_load', {'id': 'JohnDoe', 'key': 'MTIzNDU=\n'})
    sequence = [
        (EIdentityLoad('JohnDoe', b'12345', None),
            lambda _: raise_(ParsecError('error', 'msg'))),
    ]
    resp = perform_sequence(sequence, eff)
    assert resp == {'status': 'error', 'label': 'msg'}


class TestRawCMD:
    def test_execute(self):
        raw_cmd = b'{"cmd": "identity_load", "id": "JohnDoe", "key": "MTIzNDU=\\n"}'
        eff = execute_raw_cmd(raw_cmd)
        sequence = [
            (EIdentityLoad('JohnDoe', b'12345', None),
                lambda _: Identity('JohnDoe', Mock(), Mock())),
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
        assert resp == b'{"label": "Message is not a valid JSON.", "status": "bad_message"}'


class Test_api_identity_load:
    def test_call_execute_cmd(self):
        eff = execute_cmd('identity_load', {'id': 'JohnDoe', 'key': 'MTIzNDU=\n'})
        sequence = [
            (EIdentityLoad('JohnDoe', b'12345', None),
                lambda _: Identity('JohnDoe', Mock(), Mock())),
        ]
        resp = perform_sequence(sequence, eff)
        assert resp == {"status": "ok"}

    def test_call_api_identity_load(self):
        eff = api_identity_load({'id': 'JohnDoe', 'key': 'MTIzNDU=\n'})
        sequence = [
            (EIdentityLoad('JohnDoe', b'12345', None),
                lambda _: Identity('JohnDoe', Mock(), Mock())),
        ]
        resp = perform_sequence(sequence, eff)
        assert resp == {"status": "ok"}

    @pytest.mark.parametrize('bad_params', [
        {},
        {'id': 42, 'key': 'MTIzNDU=\n', 'password': 'secret'},
        {'id': 'JohnDoe', 'key': 42, 'password': 'secret'},
        {'id': 'JohnDoe', 'key': 'MTIzNDU=\n', 'password': 42},
        {'key': 'MTIzNDU=\n', 'password': 'secret'},
        {'id': 'JohnDoe', 'password': 'secret'},
    ])
    def test_api_identity_load_error(self, bad_params):
        eff = execute_cmd('identity_load', bad_params)
        sequence = [
        ]
        resp = perform_sequence(sequence, eff)
        assert resp['status'] == 'bad_msg'
