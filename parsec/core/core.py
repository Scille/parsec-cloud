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
    password = attr.ib()


@attr.s
class EIdentityUnload:
    pass


@attr.s
class EIdentityGet:
    pass


class IdentityMixin:
    identity = attr.ib()

    @asyncio_performer
    async def perform_identity_load(self, dispatcher, intent):
        if self.identity:
            raise IdentityError('Identity already loaded')
        # TODO: handle invalid key with more precise exception
        try:
            private_key = load_private_key(intent.key, intent.password)
        except Exception as e:
            raise IdentityError('Invalid private key (%s)' % e)
        return Identity(intent.id, private_key, private_key.pub_key)

    @asyncio_performer
    async def perform_identity_unload(self, dispatcher, intent):
        if not self.identity:
            raise IdentityNotLoadedError('Identity not loaded')
        self.identity = None

    @asyncio_performer
    async def perform_identity_get(self, dispatcher, intent):
        if not self.identity:
            raise IdentityNotLoadedError('Identity not loaded')
        return self.identity


@attr.s
class App(IdentityMixin):
    pass


def identity_dispatcher_factory(app):
    return TypeDispatcher({
        EIdentityLoad: app.perform_identity_load,
        EIdentityUnload: app.perform_identity_unload,
        EIdentityGet: app.perform_identity_get,
    })

# def core_factory():
#     app = Core()

#     @asyncio_performer
#     async def perform_identity_load(dispatcher, identity_load):
#         nonlocal loaded_identity
#         if loaded_identity:
#             raise IdentityError('Identity already loaded')
#         # TODO: handle invalid key
#         private_key = load_private_key(identity_load.key)
#         loaded_identity = Identity(identity_load.id, private_key, private_key.pub_key)
#         return loaded_identity

#     @asyncio_performer
#     async def perform_identity_unload(dispatcher, identity_unload):
#         nonlocal loaded_identity
#         if not loaded_identity:
#             raise IdentityNotLoadedError('Identity not loaded')
#         loaded_identity = None

#     @asyncio_performer
#     async def perform_identity_get(dispatcher, identity_get):
#         nonlocal loaded_identity
#         if not loaded_identity:
#             raise IdentityNotLoadedError('Identity not loaded')
#         return loaded_identity

#     @asyncio_performer
#     async def perform_event(dispatcher, event):
#         signal(event.name).send(event.sender)

#     dispatcher = ComposedDispatcher([base_dispatcher, TypeDispatcher({
#         IdentityLoad: perform_identity_load,
#         IdentityUnload: perform_identity_unload,
#         IdentityGet: perform_identity_get,
#         Event: perform_event
#     })])

#     return IdentityService(dispatcher)
