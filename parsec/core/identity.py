import asyncio
import attr
from effect import TypeDispatcher
from aioeffect import performer as asyncio_performer

from parsec.exceptions import IdentityNotLoadedError, IdentityError
from parsec.crypto import load_private_key


@attr.s
class Identity:
    id = attr.ib()
    private_key = attr.ib()
    public_key = attr.ib()


@attr.s
class EIdentityLoad:
    id = attr.ib()
    key = attr.ib()
    password = attr.ib(default=None)


@attr.s
class EIdentityUnload:
    pass


@attr.s
class EIdentityGet:
    pass


@attr.s
class IdentityMixin:
    identity = attr.ib(default=None)

    @asyncio_performer
    async def perform_identity_load(self, dispatcher, intent):
        if self.identity:
            raise IdentityError('Identity already loaded')
        # TODO: handle invalid key with more precise exception
        try:
            private_key = load_private_key(intent.key, intent.password)
        except Exception as e:
            raise IdentityError('Invalid private key (%s)' % e)
        self.identity = Identity(intent.id, private_key, private_key.pub_key)
        return self.identity

    @asyncio_performer
    async def perform_identity_unload(self, dispatcher, intent):
        if not self.identity:
            raise IdentityNotLoadedError('Identity not loaded')
        self.identity = None

    @asyncio_performer
    async def perform_identity_get(self, dispatcher, intent):
        if not self.identity:
            return None
        return self.identity


def identity_dispatcher_factory(app):
    return TypeDispatcher({
        EIdentityLoad: app.perform_identity_load,
        EIdentityUnload: app.perform_identity_unload,
        EIdentityGet: app.perform_identity_get,
    })
