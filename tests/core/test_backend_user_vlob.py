import pytest
from effect2.testing import perform_sequence, const

from parsec.tools import to_jsonb64
from parsec.core.backend import BackendCmd
from parsec.core.backend_user_vlob import (
    EBackendUserVlobRead, perform_user_vlob_read, UserVlobAtom,
    EBackendUserVlobUpdate, perform_user_vlob_update
)


def test_perform_vlob_read():
    eff = perform_user_vlob_read(EBackendUserVlobRead(3))
    backend_response = {
        'status': 'ok',
        'version': 3,
        'blob': to_jsonb64(b'fooo')
    }
    sequence = [
        (BackendCmd('user_vlob_read', {'version': 3}), const(backend_response))
    ]
    ret = perform_sequence(sequence, eff)
    assert ret == UserVlobAtom(3, b'fooo')


def test_perform_vlob_update():
    eff = perform_user_vlob_update(EBackendUserVlobUpdate(3, b'bar'))
    backend_response = {'status': 'ok'}
    sequence = [
        (BackendCmd('user_vlob_update', {'version': 3, 'blob': to_jsonb64(b'bar')}),
            const(backend_response))
    ]
    ret = perform_sequence(sequence, eff)
    assert ret is None
