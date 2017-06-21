import pytest
from effect import raise_
from effect.testing import perform_sequence

from parsec.core2.identity_service import (
    Identity, IdentityLoad, IdentityUnload, Event, IdentityError,
    IdentityNotLoadedError, identity_load, identity_unload
)


def test_load_identity():
    identity = Identity('John', None, None)
    eff = identity_load('John', b'fake_key')
    sequence = [
        (IdentityLoad('John', b'fake_key'), lambda _: identity),
        (Event('identity_loaded', 'John'), lambda _: None)
    ]
    ret = perform_sequence(sequence, eff)
    assert ret == identity


def test_unload_identity():
    eff = identity_unload()
    sequence = [
        (IdentityUnload(), lambda _: None),
        (Event('identity_unloaded', None), lambda _: None)
    ]
    ret = perform_sequence(sequence, eff)
    assert ret is None


def test_already_loaded_identity():
    eff = identity_load('John', b'fake_key')

    sequence = [
        (IdentityLoad('John', b'fake_key'), lambda _: raise_(IdentityError())),
    ]
    with pytest.raises(IdentityError):
        perform_sequence(sequence, eff)


def test_not_loaded_identity_unload():
    eff = identity_unload()
    sequence = [
        (IdentityUnload(), lambda _: raise_(IdentityNotLoadedError())),
    ]
    with pytest.raises(IdentityNotLoadedError):
        perform_sequence(sequence, eff)
