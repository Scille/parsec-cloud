from marshmallow import fields

from parsec.service import cmd
from parsec.tools import BaseCmdSchema
from parsec.backend.vlob_service import BaseVlobService, MockedVlobService, Vlob


class cmd_CREATE_Schema(BaseCmdSchema):
    id = fields.String(required=True)
    blob = fields.String(missing='')


class BaseNamedVlobService(BaseVlobService):

    @cmd('create')
    async def _cmd_CREATE(self, session, msg):
        msg = cmd_CREATE_Schema().load(msg)
        vlob = await self.create(msg['id'], msg['blob'])
        return {
            'status': 'ok',
            'id': vlob.id,
            'read_trust_seed': vlob.read_trust_seed,
            'write_trust_seed': vlob.write_trust_seed
        }


class MockedNamedVlobService(MockedVlobService):
    def __init__(self):
        super().__init__()
        self._vlobs = {}

    async def create(self, id, blob=None):
        vlob = Vlob(id=id, blob=blob)
        # TODO: who cares about hash collision ?
        self._vlobs[vlob.id] = vlob
        return vlob
