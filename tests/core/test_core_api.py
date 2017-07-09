import pytest
from effect2.testing import perform_sequence, raise_
from unittest.mock import Mock

from parsec.core.core_api import execute_cmd, execute_raw_cmd
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
