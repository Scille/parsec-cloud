from effect.testing import perform_sequence

from parsec.core2.bbbackend_api_service import (
    BackendCmd, VlobAtom, VlobAccess, vlob_read, vlob_create)
from parsec.tools import to_jsonb64


def test_vlob_read():
    eff = vlob_read('1234567890', 'dummy-trust-seed')
    sequence = [
        (BackendCmd('vlob_read', {'id': '1234567890', 'trust_seed': 'dummy-trust-seed'}),
            lambda _: {'status': 'ok', 'id': '1234567890', 'version': 1, 'blob': to_jsonb64(b'foo')}),
    ]
    ret = perform_sequence(sequence, eff)
    assert ret == VlobAtom('1234567890', 1, b'foo')


def test_block_create():
    eff = vlob_create(b'foo')
    sequence = [
        (BackendCmd('vlob_create', {'blob': to_jsonb64(b'foo')}),
            lambda _: {'status': 'ok', 'id': '42', 'read_trust_seed': '123',
                       'write_trust_seed': 'ABC'}),
    ]
    ret = perform_sequence(sequence, eff)
    assert ret == VlobAccess('42', '123', 'ABC')
