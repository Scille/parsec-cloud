import pytest
from effect2 import Effect
from effect2.testing import IntentType, noop, perform_sequence

from parsec.core import app_factory
from parsec.core.base import EEvent
from parsec.core.identity import IdentityComponent, Identity, EIdentityLoad

from tests.test_crypto import ALICE_PRIVATE_RSA


@pytest.fixture
def alice_identity():
    component = IdentityComponent()
    sequence = [
        (IntentType(EEvent), noop),
    ]
    return perform_sequence(sequence, component.perform_identity_load(
        EIdentityLoad('Alice', ALICE_PRIVATE_RSA))
    )


async def test_perform_identity_load():
    component = IdentityComponent()
    app = app_factory(component.get_dispatcher())
    assert component.identity is None
    intent = Effect(EIdentityLoad('Alice', ALICE_PRIVATE_RSA))
    ret = await app.async_perform(intent)
    assert isinstance(ret, Identity)
    assert ret == component.identity
    assert ret.id == 'Alice'
