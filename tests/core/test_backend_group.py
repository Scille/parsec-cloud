from effect2.testing import perform_sequence, const

from parsec.core.backend import BackendCmd
from parsec.core.backend_group import (
    Group, perform_group_read, EBackendGroupRead,
    perform_group_create, EBackendGroupCreate,
    perform_group_add_identities, EBackendGroupAddIdentities,
    perform_group_remove_identities, EBackendGroupRemoveIdentities,
)


def test_perform_group_create():
    eff = perform_group_create(EBackendGroupCreate('teamA'))
    backend_response = {'status': 'ok'}
    sequence = [
        (BackendCmd('group_create', {'name': 'teamA'}), const(backend_response))
    ]
    ret = perform_sequence(sequence, eff)
    assert ret is None


def test_perform_group_read():
    eff = perform_group_read(EBackendGroupRead('teamA'))
    backend_response = {'status': 'ok', 'admins': ['John'], 'users': ['Bob', 'Alice']}
    sequence = [
        (BackendCmd('group_read', {'name': 'teamA'}), const(backend_response))
    ]
    ret = perform_sequence(sequence, eff)
    assert ret == Group('teamA', ['John'], ['Bob', 'Alice'])


def test_perform_group_add_identities():
    eff = perform_group_add_identities(EBackendGroupAddIdentities('teamA', ['John'], False))
    backend_response = {'status': 'ok'}
    sequence = [
        (BackendCmd('group_add_identities',
                    {'name': 'teamA', 'identities': ['John'], 'admin': False}),
         const(backend_response))
    ]
    ret = perform_sequence(sequence, eff)
    assert ret is None


def test_perform_group_remove_identities():
    eff = perform_group_remove_identities(EBackendGroupRemoveIdentities('teamA', ['John'], False))
    backend_response = {'status': 'ok'}
    sequence = [
        (BackendCmd('group_remove_identities',
                    {'name': 'teamA', 'identities': ['John'], 'admin': False}),
         const(backend_response))
    ]
    ret = perform_sequence(sequence, eff)
    assert ret is None
