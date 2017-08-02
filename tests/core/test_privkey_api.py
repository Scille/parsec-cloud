import pytest
from effect2.testing import perform_sequence, noop

from parsec.core.core_api import execute_cmd
from parsec.core.privkey import EPrivKeyExport, EPrivKeyLoad

from tests.core.test_identity import alice_identity


def test_api_privkey_export(alice_identity):
    eff = execute_cmd('privkey_export', {'password': 'secret'})
    sequence = [
        (EPrivKeyExport('secret'), noop),
    ]
    resp = perform_sequence(sequence, eff)
    assert resp == {'status': 'ok'}


@pytest.mark.parametrize('bad_params', [
    {},
    {'password': None},
    {'password': 42},
    {'password': 'P4ssw0rd.', 'unknown': 'field'}
])
def test_api_privkey_export_error(bad_params):
    eff = execute_cmd('privkey_export', bad_params)
    sequence = [
    ]
    resp = perform_sequence(sequence, eff)
    assert resp['status'] == 'bad_msg'


def test_api_privkey_load():
    eff = execute_cmd('privkey_load', {'id': 'JohnDoe', 'password': 'secret'})
    sequence = [
        (EPrivKeyLoad('JohnDoe', 'secret'), noop)
    ]
    resp = perform_sequence(sequence, eff)
    assert resp == {'status': 'ok'}


@pytest.mark.parametrize('bad_params', [
    {},
    {'id': 42, 'password': 'secret'},
    {'id': 'JohnDoe', 'password': 42},
    {'password': 'secret'},
    {'id': 'JohnDoe'},
])
def test_api_privkey_load_error(bad_params):
    eff = execute_cmd('privkey_load', bad_params)
    sequence = [
    ]
    resp = perform_sequence(sequence, eff)
    assert resp['status'] == 'bad_msg'
