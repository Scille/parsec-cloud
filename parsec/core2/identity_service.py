import attr
from marshmallow import fields
from effect import Effect, TypeDispatcher, base_dispatcher, ComposedDispatcher
from effect.do import do
from aioeffect import perform as asyncio_perform, performer as asyncio_performer
from blinker import signal

from parsec.service import BaseService, cmd, event
from parsec.tools import BaseCmdSchema
from parsec.exceptions import IdentityError, IdentityNotLoadedError
from parsec.crypto import load_private_key


@attr.s
class Event:
    name = attr.ib()
    sender = attr.ib()


@attr.s
class Identity:
    id = attr.ib()
    private_key = attr.ib()
    public_key = attr.ib()


@attr.s
class IdentityLoad:
    id = attr.ib()
    key = attr.ib()


@attr.s
class IdentityUnload:
    pass


@attr.s
class IdentityGet:
    pass


@do
def identity_load(id, key):
    identity = yield Effect(IdentityLoad(id, key))
    yield Effect(Event('identity_loaded', id))
    return identity


@do
def identity_unload():
    yield Effect(IdentityUnload())
    yield Effect(Event('identity_unloaded', None))


class cmd_IDENTITY_LOAD_Schema(BaseCmdSchema):
    id = fields.String(required=True)
    key = fields.String(required=True)
    password = fields.String()


@do
def api_identity_load(msg):
    msg = cmd_IDENTITY_LOAD_Schema().load(msg)
    yield identity_load(msg['id'], msg['key'])
    return {'status': 'ok'}


@do
def api_identity_unload(msg):
    yield identity_unload()
    return {'status': 'ok'}


@do
def api_identity_info(msg):
    identity = yield Effect(IdentityGet())
    return {'status': 'ok', 'id': identity.id}


class IdentityService(BaseService):
    name = 'IdentityService'

    def __init__(self, dispatcher):
        super().__init__()
        self._dispatcher = dispatcher

    on_identity_loaded = event('identity_loaded')
    on_identity_unloaded = event('identity_unloaded')

    @cmd('identity_load')
    async def _cmd_IDENTITY_LOAD(self, session, msg):
        return await asyncio_perform(self._dispatcher, api_identity_load(msg))

    @cmd('identity_info')
    async def _cmd_IDENTITY_INFO(self, session, msg):
        return await asyncio_perform(self._dispatcher, api_identity_info(msg))
        if not self.id:
            raise IdentityError('Identity not loaded')
        return {'status': 'ok', 'id': self.id}

    @cmd('identity_unload')
    async def _cmd_IDENTITY_UNLOAD(self, session, msg):
        return await asyncio_perform(self._dispatcher, api_identity_unload(msg))

    async def load(self, id, key):
        return await asyncio_perform(self._dispatcher, identity_load(id, key))

    async def get(self):
        return await asyncio_perform(self._dispatcher, Effect(IdentityGet()))


def identity_service_factory():
    loaded_identity = None

    @asyncio_performer
    async def perform_identity_load(dispatcher, identity_load):
        nonlocal loaded_identity
        if loaded_identity:
            raise IdentityError('Identity already loaded')
        # TODO: handle invalid key
        private_key = load_private_key(identity_load.key)
        loaded_identity = Identity(identity_load.id, private_key, private_key.pub_key)
        return loaded_identity

    @asyncio_performer
    async def perform_identity_unload(dispatcher, identity_unload):
        nonlocal loaded_identity
        if not loaded_identity:
            raise IdentityNotLoadedError('Identity not loaded')
        loaded_identity = None

    @asyncio_performer
    async def perform_identity_get(dispatcher, identity_get):
        nonlocal loaded_identity
        if not loaded_identity:
            raise IdentityNotLoadedError('Identity not loaded')
        return loaded_identity

    @asyncio_performer
    async def perform_event(dispatcher, event):
        signal(event.name).send(event.sender)

    dispatcher = ComposedDispatcher([base_dispatcher, TypeDispatcher({
        IdentityLoad: perform_identity_load,
        IdentityUnload: perform_identity_unload,
        IdentityGet: perform_identity_get,
        Event: perform_event
    })])

    return IdentityService(dispatcher)
