from marshmallow import fields
from collections import defaultdict

from parsec.service import BaseService, cmd, event
from parsec.tools import BaseCmdSchema
from parsec.exceptions import UserVlobError


class cmd_READ_Schema(BaseCmdSchema):
    version = fields.Int(validate=lambda n: n >= 0)


class cmd_UPDATE_Schema(BaseCmdSchema):
    version = fields.Int(validate=lambda n: n >= 1)
    blob = fields.String(required=True)


class BaseUserVlobService(BaseService):

    name = 'NamedVlobService'

    on_updated = event('on_user_vlob_updated')

    @cmd('user_vlob_read')
    async def _cmd_READ(self, session, msg):
        msg = cmd_READ_Schema().load(msg)
        atom = await self.read(session.identity, msg.get('version'))
        return {'status': 'ok', 'blob': atom.blob, 'version': atom.version}

    @cmd('user_vlob_update')
    async def _cmd_UPDATE(self, session, msg):
        msg = cmd_UPDATE_Schema().load(msg)
        await self.update(session.identity, msg['version'], msg['blob'])
        return {'status': 'ok'}

    async def read(self, id, version=None):
        raise NotImplementedError()

    async def update(self, id, version, blob):
        raise NotImplementedError()


class UserVlobAtom:
    def __init__(self, id, version=0, blob=None):
        # Generate opaque id if not provided
        self.id = id
        self.blob = blob or ''
        self.version = version


class MockedUserVlobService(BaseUserVlobService):
    def __init__(self):
        super().__init__()
        self._vlobs = defaultdict(list)

    async def read(self, id, version=None):
        vlobs = self._vlobs[id]
        if version == 0 or (version is None and not vlobs):
            return UserVlobAtom(id=id)
        try:
            if version is None:
                return vlobs[-1]
            else:
                return vlobs[version - 1]
        except IndexError:
            raise UserVlobError('Wrong blob version.')

    async def update(self, id, version, blob):
        vlobs = self._vlobs[id]
        if len(vlobs) != version - 1:
            raise UserVlobError('Wrong blob version.')
        vlobs.append(UserVlobAtom(id=id, version=version, blob=blob))
        self.on_updated.send(id)
