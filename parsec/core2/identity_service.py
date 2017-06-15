from marshmallow import fields

from parsec.service import BaseService, cmd, event
from parsec.tools import BaseCmdSchema
from parsec.exceptions import IdentityError, IdentityNotLoadedError
from parsec.crypto import load_private_key


class cmd_IDENTITY_LOAD_Schema(BaseCmdSchema):
    id = fields.String(required=True)
    key = fields.String(required=True)


class BaseIdentityService(BaseService):
    name = 'IdentityService'

    on_identity_loaded = event('identity_loaded')
    on_identity_unloaded = event('identity_unloaded')

    id = NotImplemented  # Overwritten by implementations
    private_key = NotImplemented  # Overwritten by implementations

    @cmd('identity_load')
    async def _cmd_IDENTITY_LOAD(self, session, msg):
        msg = cmd_IDENTITY_LOAD_Schema().load(msg)
        await self.load(**msg)
        return {'status': 'ok'}

    @cmd('identity_info')
    async def _cmd_IDENTITY_INFO(self, session, msg):
        if not self.id:
            raise IdentityError('Identity not loaded')
        return {'status': 'ok', 'id': self.id}

    @cmd('identity_unload')
    async def _cmd_IDENTITY_UNLOAD(self, session, msg):
        await self.unload()
        return {'status': 'ok'}

    async def load(self, id, raw_key):
        raise NotImplementedError()

    async def unload(self):
        raise NotImplementedError()


class IdentityService(BaseIdentityService):


    def __init__(self):
        super().__init__()
        # TODO raise exception if accessing id/private_key before doing a `load`
        self.id = None
        self.private_key = None
        self.public_key = None

    async def load(self, id, raw_key):
        # TODO encrypt private key
        if self.private_key:
            raise IdentityError('User already logged in')
        self.id = id
        self.private_key = load_private_key(raw_key)
        self.public_key = self.private_key.pub_key
        self.on_identity_loaded.send(id)

    async def unload(self):
        if not self.id:
            raise IdentityNotLoadedError('Identity not loaded')
        self.id = None
        self.private_key = None
        self.public_key = None
        self.on_identity_unloaded.send(self.id)
