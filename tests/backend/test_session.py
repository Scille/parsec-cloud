import pytest
from unittest.mock import patch
from effect2.testing import const, perform_sequence, noop

from parsec.backend.session import (
    SessionComponent, EHandshakeSend, EHandshakeRecv, EGetAuthenticatedUser)
from parsec.backend.pubkey import EPubkeyGet
from parsec.exceptions import HandshakeError

from tests.test_crypto import mock_crypto_passthrough, alice


@pytest.fixture
def component():
    return SessionComponent()


def test_handshake(component, mock_crypto_passthrough, alice):
    with patch('parsec.backend.session._generate_challenge') as generate_challenge:
        generate_challenge.return_value = 'my-challenge'
        eff = component.handshake()
        sequence = [
            (EHandshakeSend(payload='{"challenge": "my-challenge", "handshake": "challenge"}'),
                noop),
            (EHandshakeRecv(),
                const('{"handshake": "answer", "answer": "bXktY2hhbGxlbmdlLWFuc3dlcg==", "identity": "alice@test.com"}')),
            (EPubkeyGet('alice@test.com', raw=False), const(alice.pub_key)),
            (EHandshakeSend(payload='{"status": "ok", "handshake": "done"}'),
                noop)
        ]
        perform_sequence(sequence, eff)
    # Now identity can retrieve authenticated user
        intent = EGetAuthenticatedUser()
        eff = component.perform_get_authenticated_user(intent)
        sequence = [
        ]
        ret = perform_sequence(sequence, eff)
        assert ret == 'alice@test.com'


# TODO: test bad handshake as well


def test_get_identity_before_handshake(component):
        intent = EGetAuthenticatedUser()
        with pytest.raises(HandshakeError):
            eff = component.perform_get_authenticated_user(intent)
            sequence = [
            ]
            perform_sequence(sequence, eff)

