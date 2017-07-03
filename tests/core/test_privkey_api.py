import pytest
from effect2.testing import perform_sequence, const, noop

from parsec.core.core_api import execute_cmd
from parsec.core.privkey import EPrivkeyAdd, EPrivkeyGet, EPrivkeyLoad


def test_api_privkey_add():
    eff = execute_cmd('privkey_add', {'id': 'JohnDoe', 'password': 'secret', 'key': 'MTIzNDU=\n'})
    sequence = [
        (EPrivkeyAdd('JohnDoe', 'secret', b'12345'), noop),
    ]
    resp = perform_sequence(sequence, eff)
    assert resp == {'status': 'ok'}


@pytest.mark.parametrize('bad_params', [
    {},
    {'id': 42, 'key': 'MTIzNDU=\n', 'password': 'secret'},
    {'id': 'JohnDoe', 'key': 42, 'password': 'secret'},
    {'id': 'JohnDoe', 'key': 'MTIzNDU=\n', 'password': 42},
    {'key': 'MTIzNDU=\n', 'password': 'secret'},
    {'id': 'JohnDoe', 'password': 'secret'},
])
def test_api_privkey_add_error(bad_params):
    eff = execute_cmd('privkey_add', bad_params)
    sequence = [
    ]
    resp = perform_sequence(sequence, eff)
    assert resp['status'] == 'bad_msg'


def test_api_privkey_get():
    eff = execute_cmd('privkey_get', {'id': 'JohnDoe', 'password': 'secret'})
    sequence = [
        (EPrivkeyGet('JohnDoe', 'secret'), const(b'12345')),
    ]
    resp = perform_sequence(sequence, eff)
    assert resp == {'status': 'ok', 'key': '12345'}


@pytest.mark.parametrize('bad_params', [
    {},
    {'id': 42, 'password': 'secret'},
    {'id': 'JohnDoe', 'password': 42},
    {'password': 'secret'},
    {'id': 'JohnDoe'},
])
def test_api_privkey_get_error(bad_params):
    eff = execute_cmd('privkey_get', bad_params)
    sequence = [
    ]
    resp = perform_sequence(sequence, eff)
    assert resp['status'] == 'bad_msg'


def test_api_privkey_load():
    eff = execute_cmd('privkey_load', {'id': 'JohnDoe', 'password': 'secret'})
    sequence = [
        (EPrivkeyLoad('JohnDoe', 'secret'), noop)
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
