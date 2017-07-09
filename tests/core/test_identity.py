import pytest
from effect2 import Effect
from effect2.asyncio import asyncio_perform
from parsec.core.identity import IdentityMixin, Identity, EIdentityLoad, identity_dispatcher_factory

from tests.test_crypto import ALICE_PRIVATE_RSA


@pytest.fixture
def app():
    return IdentityMixin()


@pytest.mark.asyncio
async def test_perform_identity_load(app):
    dispatcher = identity_dispatcher_factory(app)
    assert app.identity is None
    intent = Effect(EIdentityLoad('Alice', ALICE_PRIVATE_RSA))
    ret = await asyncio_perform(dispatcher, intent)
    assert isinstance(ret, Identity)
    assert ret == app.identity
    assert ret.id == 'Alice'
