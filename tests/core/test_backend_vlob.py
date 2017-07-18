from effect2.testing import perform_sequence, const

from parsec.tools import to_jsonb64
from parsec.core.backend import BackendCmd
from parsec.core.backend_vlob import (
    EBackendVlobCreate, perform_vlob_create, VlobAccess,
    EBackendVlobRead, perform_vlob_read, VlobAtom,
    EBackendVlobUpdate, perform_vlob_update
)


def test_perform_vlob_create():
    eff = perform_vlob_create(EBackendVlobCreate(b'fooo'))
    backend_response = {
        'status': 'ok',
        'id': '42',
        'read_trust_seed': 'RTS42',
        'write_trust_seed': 'WTS42',
    }
    sequence = [
        (BackendCmd('vlob_create', {'blob': to_jsonb64(b'fooo')}), const(backend_response))
    ]
    ret = perform_sequence(sequence, eff)
    assert ret == VlobAccess('42', 'RTS42', 'WTS42')


def test_perform_vlob_read():
    eff = perform_vlob_read(EBackendVlobRead('42', 'RTS42', 3))
    backend_response = {
        'status': 'ok',
        'id': '42',
        'version': 3,
        'blob': to_jsonb64(b'fooo')
    }
    sequence = [
        (BackendCmd('vlob_read', {'id': '42', 'trust_seed': 'RTS42', 'version': 3}),
            const(backend_response))
    ]
    ret = perform_sequence(sequence, eff)
    assert ret == VlobAtom('42', 3, b'fooo')


def test_perform_vlob_update():
    eff = perform_vlob_update(EBackendVlobUpdate('42', 'WTS42', 3, b'bar'))
    backend_response = {
        'status': 'ok',
    }
    sequence = [
        (BackendCmd('vlob_update',
                    {'id': '42', 'trust_seed': 'WTS42', 'version': 3, 'blob': to_jsonb64(b'bar')}),
            const(backend_response))
    ]
    ret = perform_sequence(sequence, eff)
    assert ret is None
