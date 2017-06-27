import pytest
from effect import Effect
from aioeffect import perform as asyncio_perform
from parsec.core.identity import IdentityMixin, Identity, EIdentityLoad, identity_dispatcher_factory

from tests.test_crypto import ALICE_PRIVATE_RSA


@pytest.mark.asyncio
async def test_perform_identity_load():
    app = IdentityMixin()
    dispatcher = identity_dispatcher_factory(app)
    assert app.identity is None
    intent = Effect(EIdentityLoad('Alice', ALICE_PRIVATE_RSA))
    ret = await asyncio_perform(dispatcher, intent)
    assert isinstance(ret, Identity)
    assert ret == app.identity
    assert ret.id == 'Alice'
