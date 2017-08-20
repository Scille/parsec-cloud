import pytest
from unittest.mock import patch
from effect2.testing import noop, perform_sequence

from parsec.base import EEvent
from parsec.core.identity import IdentityComponent, EIdentityLoad
from parsec.crypto import RSAPublicKey, RSAPrivateKey, AESKey

from tests.test_crypto import ALICE_PRIVATE_RSA


@pytest.fixture
def alice_identity():
    component = IdentityComponent()
    sequence = [
        (EEvent('identity_loaded', 'Alice'), noop),
    ]
    return perform_sequence(sequence, component.perform_identity_load(
        EIdentityLoad('Alice', ALICE_PRIVATE_RSA))
    )


@pytest.fixture
def mock_crypto_passthrough():
    count = 0

    def mocked_urandom(size):
        nonlocal count
        count += 1
        if size > 16:
            template = '<dummy-key-{:0>%s}>' % (size - 12)
        else:
            template = '{:0>%s}' % size
        return template.format(count).encode()

    with patch.object(RSAPublicKey, 'encrypt', new=lambda _, txt: txt), \
            patch.object(RSAPublicKey, 'verify', new=lambda _, sign, txt: None), \
            patch.object(RSAPrivateKey, 'decrypt', new=lambda _, txt: txt), \
            patch.object(RSAPrivateKey, 'sign', new=lambda _, txt: '<mock-signature>'), \
            patch.object(RSAPrivateKey, 'export',
                new=lambda _, pwd: ('<mock-exported-key with password %s>' % pwd).encode()), \
            patch.object(AESKey, 'encrypt', new=lambda _, txt: txt), \
            patch.object(AESKey, 'decrypt', new=lambda _, txt: txt), \
            patch('parsec.crypto.urandom', new=mocked_urandom), \
            patch('parsec.crypto.encrypt_with_password', new=lambda p, s, t: t), \
            patch('parsec.crypto.decrypt_with_password', new=lambda p, s, c: c):
        yield