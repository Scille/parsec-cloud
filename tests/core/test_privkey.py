from unittest.mock import Mock
from effect2.testing import perform_sequence, const

from parsec.core.privkey import EPrivkeyAdd, EPrivkeyGet, EPrivkeyLoad, PrivKeyComponent
from parsec.core.identity import EIdentityLoad, Identity

from tests.test_crypto import ALICE_PRIVATE_RSA


def test_perform_privkey_add():
    app = PrivKeyComponent()
    assert app._encrypted_keys == {}
    eff = app.perform_add_privkey(EPrivkeyAdd('Alice', 'secret', ALICE_PRIVATE_RSA))
    sequence = [
    ]
    ret = perform_sequence(sequence, eff)
    assert ret is None
    assert len(app._encrypted_keys) == 1


def test_perform_privkey_get():
    app = PrivKeyComponent()
    # Add privkey
    eff = app.perform_add_privkey(EPrivkeyAdd('Alice', 'secret', ALICE_PRIVATE_RSA))
    perform_sequence([], eff)
    # Get loaded privkey
    eff = app.perform_get_privkey(EPrivkeyGet('Alice', 'secret'))
    sequence = [
    ]
    ret = perform_sequence(sequence, eff)
    assert ret == ALICE_PRIVATE_RSA


def test_perform_privkey_load_identity():
    app = PrivKeyComponent()
    # Add privkey
    eff = app.perform_add_privkey(EPrivkeyAdd('Alice', 'secret', ALICE_PRIVATE_RSA))
    perform_sequence([], eff)
    # Get loaded privkey
    eff = app.perform_privkey_load_identity(EPrivkeyLoad('Alice', 'secret'))
    sequence = [
        (EIdentityLoad('Alice', ALICE_PRIVATE_RSA, None), const(Identity('Alice', Mock(), Mock())))
    ]
    ret = perform_sequence(sequence, eff)
    assert ret is None
