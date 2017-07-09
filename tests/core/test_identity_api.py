import pytest
from effect2.testing import perform_sequence
from unittest.mock import Mock

from parsec.core.core_api import execute_cmd
from parsec.core.identity import EIdentityGet, EIdentityLoad, EIdentityUnload, Identity


def test_api_identity_load():
    eff = execute_cmd('identity_load', {'id': 'JohnDoe', 'key': 'MTIzNDU=\n'})
    sequence = [
        (EIdentityLoad('JohnDoe', b'12345', None),
            lambda _: Identity('JohnDoe', Mock(), Mock())),
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
def test_api_identity_load_error(bad_params):
    eff = execute_cmd('identity_load', bad_params)
    sequence = [
    ]
    resp = perform_sequence(sequence, eff)
    assert resp['status'] == 'bad_msg'


def test_api_identity_unload():
    eff = execute_cmd('identity_unload', {})
    sequence = [
        (EIdentityUnload(),
            lambda _: None),
    ]
    resp = perform_sequence(sequence, eff)
    assert resp == {'status': 'ok'}


@pytest.mark.parametrize('bad_params', [
    {'bad_field': 42},
])
def test_api_identity_unload_error(bad_params):
    eff = execute_cmd('identity_unload', bad_params)
    sequence = [
    ]
    resp = perform_sequence(sequence, eff)
    assert resp['status'] == 'bad_msg'


def test_api_identity_info():
    # Not loaded
    eff = execute_cmd('identity_info', {})
    sequence = [
        (EIdentityGet(),
            lambda _: None),
    ]
    resp = perform_sequence(sequence, eff)
    assert resp == {'loaded': False, 'status': 'ok'}
    # Loaded
    eff = execute_cmd('identity_info', {})
    sequence = [
        (EIdentityGet(),
            lambda _: Identity('JohnDoe', Mock(), Mock())),
    ]
    resp = perform_sequence(sequence, eff)
    assert resp == {'id': 'JohnDoe', 'loaded': True, 'status': 'ok'}


@pytest.mark.parametrize('bad_params', [
    {'bad_field': 42},
])
def test_api_identity_info_error(bad_params):
    eff = execute_cmd('identity_info', bad_params)
    sequence = [
    ]
    resp = perform_sequence(sequence, eff)
    assert resp['status'] == 'bad_msg'
