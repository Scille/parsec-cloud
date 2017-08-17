import pytest
from effect2.testing import perform_sequence, const

from parsec.tools import to_jsonb64
from parsec.core.backend import BackendCmd
from parsec.core.backend_pubkey import (
    EBackendPubKeyGet, perform_pubkey_get, PubKey,
)
from parsec.exceptions import PubKeyNotFound


def test_perform_pubkey_get():
    eff = perform_pubkey_get(EBackendPubKeyGet('alice@test.com'))
    backend_response = {
        'status': 'ok',
        'id': 'alice@test.com',
        'key': to_jsonb64(b"<alice's key>"),
    }
    sequence = [
        (BackendCmd('pubkey_get', {'id': 'alice@test.com'}), const(backend_response))
    ]
    ret = perform_sequence(sequence, eff)
    assert ret == PubKey('alice@test.com', b"<alice's key>")


def test_perform_pubkey_get_missing_key():
    eff = perform_pubkey_get(EBackendPubKeyGet('alice@test.com'))
    backend_response = {
        'status': 'pubkey_not_found',
        'label': 'Public key not found'
    }
    sequence = [
        (BackendCmd('pubkey_get', {'id': 'alice@test.com'}), const(backend_response))
    ]
    with pytest.raises(PubKeyNotFound):
        perform_sequence(sequence, eff)
