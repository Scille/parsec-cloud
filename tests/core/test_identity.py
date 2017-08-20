from effect2.testing import noop, perform_sequence

from parsec.base import EEvent
from parsec.core.identity import IdentityComponent, Identity, EIdentityLoad

from tests.test_crypto import ALICE_PRIVATE_RSA


async def test_perform_identity_load():
    component = IdentityComponent()
    assert component.identity is None
    intent = EIdentityLoad('Alice', ALICE_PRIVATE_RSA)
    eff = component.perform_identity_load(intent)
    ret = perform_sequence([
        (EEvent('identity_loaded', 'Alice'), noop),
    ], eff)
    assert isinstance(ret, Identity)
    assert ret == component.identity
    assert ret.id == 'Alice'
